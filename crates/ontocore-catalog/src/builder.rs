use crate::OntologyCatalogData;
use ontocore_core::{
    limits::{MAX_ENTITIES, MAX_TOTAL_TRIPLES, MAX_TRIPLES_PER_FILE},
    read_to_string_capped, Annotation, Axiom, Diagnostic, DiagnosticCode, DiagnosticSeverity,
    Entity, Import, Namespace, OntologyDocument, OntologyFormat, ParseStatus, SourceLocation,
    WorkspaceScanner, MAX_FILE_BYTES,
};
use ontocore_diagnostics::{collect_diagnostics_with_sources, DiagnosticInput};
use ontocore_owl::{load_turtle_text, supports_horned_load};
use ontocore_parser::{parse_ontology_file, parse_ontology_text, ParsedOntology};
use oxigraph::model::Quad;
use oxigraph::store::Store;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("core error: {0}")]
    Core(#[from] ontocore_core::OntoCoreError),

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

        let mut bridge_diagnostics = Vec::new();

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
                return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
                ))));
            }

            if parsed.parse_status != ParseStatus::Error {
                load_quads_into_store(&store, parsed.quads(), triple_count)?;
            }

            documents.push(OntologyDocument {
                id: doc_id.clone(),
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

            let semantics = semantics_for_document(
                &file.path,
                file.format,
                &doc_id,
                &parsed,
                self.document_override_text(&file.path),
            )?;

            if let Some(diag) = semantics.bridge_warning {
                bridge_diagnostics.push(diag);
            }

            for entity in semantics.entities {
                if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
                    return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(
                        format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                    )));
                }
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
            annotations.extend(semantics.annotations);
            axioms.extend(semantics.axioms);
            namespaces.extend(semantics.namespace_rows);
            imports.extend(semantics.imports);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_ENTITIES} entities"
                ))));
            }
        }

        for (override_path, override_text) in &self.document_overrides {
            if files.iter().any(|f| paths_equal(&f.path, override_path)) {
                continue;
            }
            let format = OntologyFormat::from_extension(
                override_path.extension().and_then(|e| e.to_str()).unwrap_or("ttl"),
            );
            if matches!(format, OntologyFormat::Unknown) {
                continue;
            }
            let doc_id = format!("doc-{}", documents.len() + 1);
            let parsed = parse_ontology_text(
                override_path,
                format,
                &doc_id,
                override_text,
                override_text.as_bytes(),
            )
            .map_err(|e| CatalogError::Parse {
                path: override_path.clone(),
                message: e.to_string(),
            })?;

            triple_count += parsed.triple_count;
            if triple_count > MAX_TOTAL_TRIPLES {
                return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
                ))));
            }

            if parsed.parse_status != ParseStatus::Error {
                load_quads_into_store(&store, parsed.quads(), triple_count)?;
            }

            documents.push(OntologyDocument {
                id: doc_id.clone(),
                path: override_path.clone(),
                format,
                base_iri: parsed.base_iri.clone(),
                imports: parsed.imports.clone(),
                namespaces: parsed.namespaces.clone(),
                parse_status: parsed.parse_status,
                content_hash: buffer_content_hash(override_text),
                modified_time: 0,
                parse_message: parsed.parse_message.clone(),
                parse_error_location: parsed.parse_error_location.clone(),
            });

            let doc_idx = documents.len() - 1;
            let mut doc_entity_iris = Vec::new();

            let semantics = semantics_for_document(
                override_path,
                format,
                &doc_id,
                &parsed,
                Some(override_text),
            )?;

            if let Some(diag) = semantics.bridge_warning {
                bridge_diagnostics.push(diag);
            }

            for entity in semantics.entities {
                if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
                    return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(
                        format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                    )));
                }
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
            annotations.extend(semantics.annotations);
            axioms.extend(semantics.axioms);
            namespaces.extend(semantics.namespace_rows);
            imports.extend(semantics.imports);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
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
        data.diagnostics.extend(bridge_diagnostics);

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

    /// Oxigraph triple store for SPARQL — not a stable public API; use [`ontocore_query::sparql_catalog`].
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

struct DocumentSemantics {
    entities: Vec<Entity>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
    namespace_rows: Vec<Namespace>,
    imports: Vec<Import>,
    bridge_warning: Option<Diagnostic>,
}

fn semantics_for_document(
    path: &Path,
    format: OntologyFormat,
    doc_id: &str,
    parsed: &ParsedOntology,
    override_text: Option<&String>,
) -> Result<DocumentSemantics> {
    if parsed.parse_status == ParseStatus::Error || !supports_horned_load(format) {
        return Ok(DocumentSemantics {
            entities: parsed.entities.clone(),
            annotations: parsed.annotations.clone(),
            axioms: parsed.axioms.clone(),
            namespace_rows: parsed.namespace_rows.clone(),
            imports: parsed.import_rows.clone(),
            bridge_warning: None,
        });
    }

    let source_text = if let Some(text) = override_text {
        text.clone()
    } else {
        read_to_string_capped(path, MAX_FILE_BYTES).map_err(CatalogError::Core)?
    };

    match load_turtle_text(path, doc_id, &source_text, parsed.quads(), &parsed.namespaces) {
        Ok(owl) => Ok(DocumentSemantics {
            entities: owl.bridge.entities,
            annotations: owl.bridge.annotations,
            axioms: owl.bridge.axioms,
            namespace_rows: owl.bridge.namespace_rows,
            imports: owl.bridge.imports,
            bridge_warning: None,
        }),
        Err(e) => {
            eprintln!(
                "ontocore-catalog: Horned-OWL load failed for {}: {e}; using parser entities",
                path.display()
            );
            Ok(DocumentSemantics {
                entities: parsed.entities.clone(),
                annotations: parsed.annotations.clone(),
                axioms: parsed.axioms.clone(),
                namespace_rows: parsed.namespace_rows.clone(),
                imports: parsed.import_rows.clone(),
                bridge_warning: Some(Diagnostic {
                    code: DiagnosticCode::OwlBridgeFailed,
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "Horned-OWL bridge failed; using parser-only entities and axioms: {e}"
                    ),
                    file: path.to_path_buf(),
                    range: SourceLocation::default(),
                    entity_iri: None,
                    quick_fix: None,
                }),
            })
        }
    }
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

fn buffer_content_hash(text: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("buffer:{:016x}", hasher.finish())
}

fn load_quads_into_store(store: &Store, quads: &[Quad], triple_count_so_far: usize) -> Result<()> {
    let mut file_triples = 0usize;
    for quad in quads {
        file_triples += 1;
        if file_triples > MAX_TRIPLES_PER_FILE {
            return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                "file exceeds {MAX_TRIPLES_PER_FILE} triples"
            ))));
        }
        if triple_count_so_far > MAX_TOTAL_TRIPLES {
            return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
            ))));
        }
        store.insert(quad).map_err(|e| CatalogError::Store(e.to_string()))?;
    }
    Ok(())
}
