use crate::OntologyCatalogData;
use ontoindex_core::{
    limits::{MAX_ENTITIES, MAX_TOTAL_TRIPLES, MAX_TRIPLES_PER_FILE},
    Entity, OntologyDocument, ParseStatus, WorkspaceScanner,
};
use ontoindex_diagnostics::{collect_diagnostics_with_sources, DiagnosticInput};
use ontoindex_parser::{parse_ontology_file, parse_ontology_text};
use oxigraph::model::Quad;
use oxigraph::store::Store;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("core error: {0}")]
    Core(#[from] ontoindex_core::OntoIndexError),

    #[error("parse error in {path}: {message}")]
    Parse { path: PathBuf, message: String },

    #[error("store error: {0}")]
    Store(String),
}

pub type Result<T> = std::result::Result<T, CatalogError>;

pub struct IndexBuilder {
    workspace: PathBuf,
    document_overrides: HashMap<PathBuf, String>,
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self { workspace: PathBuf::from("."), document_overrides: HashMap::new() }
    }

    pub fn workspace(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace = path.into();
        self
    }

    /// Use in-memory text instead of disk for specific paths (LSP open buffers).
    pub fn document_overrides(mut self, overrides: HashMap<PathBuf, String>) -> Self {
        self.document_overrides = overrides;
        self
    }

    fn document_override_text(&self, path: &Path) -> Option<&String> {
        self.document_overrides
            .get(path)
            .or_else(|| path.canonicalize().ok().and_then(|p| self.document_overrides.get(&p)))
    }

    pub fn build(self) -> Result<OntologyCatalog> {
        let scanner = WorkspaceScanner::new(&self.workspace);
        let files = scanner.scan()?;

        let mut documents = Vec::new();
        let mut entities: Vec<Entity> = Vec::new();
        let mut entity_index: HashMap<String, usize> = HashMap::new();
        let mut entity_to_document: HashMap<String, usize> = HashMap::new();
        let mut document_entity_iris: Vec<Vec<String>> = Vec::new();
        let mut annotations = Vec::new();
        let mut axioms = Vec::new();
        let mut namespaces = Vec::new();
        let mut imports = Vec::new();
        let mut triple_count = 0usize;
        let store = Store::new().map_err(|e| CatalogError::Store(e.to_string()))?;

        for (idx, file) in files.iter().enumerate() {
            let doc_id = format!("doc-{}", idx + 1);
            let parsed = if let Some(text) = self.document_override_text(&file.path) {
                parse_ontology_text(&file.path, file.format, &doc_id, text, text.as_bytes())
                    .map_err(|e| CatalogError::Parse {
                        path: file.path.clone(),
                        message: e.to_string(),
                    })?
            } else {
                parse_ontology_file(
                    &file.path,
                    file.format,
                    &doc_id,
                    &file.content_hash,
                    file.modified_time,
                )
                .map_err(|e| CatalogError::Parse {
                    path: file.path.clone(),
                    message: e.to_string(),
                })?
            };

            triple_count += parsed.triple_count;
            if triple_count > MAX_TOTAL_TRIPLES {
                return Err(CatalogError::Core(ontoindex_core::OntoIndexError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
                ))));
            }

            if parsed.parse_status != ParseStatus::Error {
                load_quads_into_store(&store, parsed.quads(), triple_count)?;
            }

            documents.push(OntologyDocument {
                id: doc_id,
                path: file.path.clone(),
                format: file.format,
                base_iri: parsed.base_iri.clone(),
                imports: parsed.imports.clone(),
                namespaces: parsed.namespaces.clone(),
                parse_status: parsed.parse_status,
                content_hash: file.content_hash.clone(),
                modified_time: file.modified_time,
                parse_message: parsed.parse_message.clone(),
                parse_error_location: parsed.parse_error_location.clone(),
            });

            let doc_idx = documents.len() - 1;
            let mut doc_entity_iris = Vec::new();

            for entity in parsed.entities {
                if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
                    if prev_doc_idx != doc_idx {
                        document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
                    }
                }
                entity_to_document.insert(entity.iri.clone(), doc_idx);
                doc_entity_iris.push(entity.iri.clone());
                if let Some(&existing_idx) = entity_index.get(&entity.iri) {
                    merge_entity(&mut entities[existing_idx], &entity);
                } else {
                    let idx = entities.len();
                    entity_index.insert(entity.iri.clone(), idx);
                    entities.push(entity);
                }
            }
            document_entity_iris.push(doc_entity_iris);
            annotations.extend(parsed.annotations);
            axioms.extend(parsed.axioms);
            namespaces.extend(parsed.namespace_rows);
            imports.extend(parsed.import_rows);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(ontoindex_core::OntoIndexError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_ENTITIES} entities"
                ))));
            }
        }

        let mut data = OntologyCatalogData {
            documents,
            entities,
            annotations,
            axioms,
            namespaces,
            imports,
            triple_count,
            diagnostics: Vec::new(),
        };
        let lint_input = DiagnosticInput {
            documents: &data.documents,
            entities: &data.entities,
            annotations: &data.annotations,
            axioms: &data.axioms,
            namespaces: &data.namespaces,
            imports: &data.imports,
        };
        data.diagnostics = collect_diagnostics_with_sources(&lint_input, &self.document_overrides);

        Ok(OntologyCatalog {
            workspace: self.workspace,
            data,
            store,
            entity_to_document,
            document_entity_iris,
        })
    }
}

impl Default for IndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OntologyCatalog {
    workspace: PathBuf,
    data: OntologyCatalogData,
    store: Store,
    /// Entity IRI → index in [`OntologyCatalogData::documents`].
    pub(crate) entity_to_document: HashMap<String, usize>,
    /// Entity IRIs declared per document (parallel to `documents`).
    pub(crate) document_entity_iris: Vec<Vec<String>>,
}

impl OntologyCatalog {
    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn data(&self) -> &OntologyCatalogData {
        &self.data
    }

    /// Oxigraph triple store for SPARQL — not a stable public API; use [`ontoindex_query::sparql_catalog`].
    #[doc(hidden)]
    pub fn store(&self) -> &Store {
        &self.store
    }
}

fn merge_entity(existing: &mut Entity, incoming: &Entity) {
    for label in &incoming.labels {
        if !existing.labels.contains(label) {
            existing.labels.push(label.clone());
        }
    }
    for comment in &incoming.comments {
        if !existing.comments.contains(comment) {
            existing.comments.push(comment.clone());
        }
    }
    existing.deprecated |= incoming.deprecated;
    existing.ontology_id = incoming.ontology_id.clone();
    if existing.short_name.is_empty() {
        existing.short_name = incoming.short_name.clone();
    }
}

fn load_quads_into_store(store: &Store, quads: &[Quad], triple_count_so_far: usize) -> Result<()> {
    let mut file_triples = 0usize;
    for quad in quads {
        file_triples += 1;
        if file_triples > MAX_TRIPLES_PER_FILE {
            return Err(CatalogError::Core(ontoindex_core::OntoIndexError::Scanner(format!(
                "file exceeds {MAX_TRIPLES_PER_FILE} triples"
            ))));
        }
        if triple_count_so_far > MAX_TOTAL_TRIPLES {
            return Err(CatalogError::Core(ontoindex_core::OntoIndexError::Scanner(format!(
                "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
            ))));
        }
        store.insert(quad).map_err(|e| CatalogError::Store(e.to_string()))?;
    }
    Ok(())
}
