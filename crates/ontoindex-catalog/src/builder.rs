use crate::OntologyCatalogData;
use ontoindex_core::{
    limits::{MAX_ENTITIES, MAX_FILE_BYTES, MAX_TOTAL_TRIPLES, MAX_TRIPLES_PER_FILE},
    OntologyDocument, OntologyFormat, ParseStatus, WorkspaceScanner,
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

    pub fn build(self) -> Result<OntologyCatalog> {
        let scanner = WorkspaceScanner::new(&self.workspace);
        let files = scanner.scan()?;

        let mut documents = Vec::new();
        let mut entities = Vec::new();
        let mut annotations = Vec::new();
        let mut axioms = Vec::new();
        let mut namespaces = Vec::new();
        let mut imports = Vec::new();
        let mut triple_count = 0usize;
        let store = Store::new().map_err(|e| CatalogError::Store(e.to_string()))?;

        for (idx, file) in files.iter().enumerate() {
            let doc_id = format!("doc-{}", idx + 1);
            let parsed = if let Some(text) = self.document_overrides.get(&file.path) {
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
                if let Some(text) = self.document_overrides.get(&file.path) {
                    load_text_into_store(&store, &file.path, file.format, text, triple_count)?;
                } else {
                    load_into_store(&store, &file.path, file.format, triple_count)?;
                }
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

            entities.extend(parsed.entities);
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

        Ok(OntologyCatalog { workspace: self.workspace, data, store })
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
}

impl OntologyCatalog {
    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn data(&self) -> &OntologyCatalogData {
        &self.data
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}

fn load_text_into_store(
    store: &Store,
    path: &Path,
    format: ontoindex_core::OntologyFormat,
    content: &str,
    triple_count_so_far: usize,
) -> Result<()> {
    if content.len() as u64 > MAX_FILE_BYTES {
        return Err(CatalogError::Parse {
            path: path.to_path_buf(),
            message: format!("file exceeds {MAX_FILE_BYTES} bytes"),
        });
    }
    load_bytes_into_store(store, path, format, content.as_bytes(), triple_count_so_far)
}

fn load_into_store(
    store: &Store,
    path: &Path,
    format: ontoindex_core::OntologyFormat,
    triple_count_so_far: usize,
) -> Result<()> {
    use std::fs;

    let metadata = fs::metadata(path)
        .map_err(|e| CatalogError::Parse { path: path.to_path_buf(), message: e.to_string() })?;
    if metadata.len() > MAX_FILE_BYTES {
        return Err(CatalogError::Parse {
            path: path.to_path_buf(),
            message: format!("file exceeds {MAX_FILE_BYTES} bytes"),
        });
    }

    let content = fs::read(path)
        .map_err(|e| CatalogError::Parse { path: path.to_path_buf(), message: e.to_string() })?;

    load_bytes_into_store(store, path, format, &content, triple_count_so_far)
}

fn load_bytes_into_store(
    store: &Store,
    path: &Path,
    format: OntologyFormat,
    content: &[u8],
    triple_count_so_far: usize,
) -> Result<()> {
    use ontoindex_core::OntologyFormat;
    use oxigraph::io::{RdfFormat, RdfParser};

    let rdf_format = match format {
        OntologyFormat::Turtle => RdfFormat::Turtle,
        OntologyFormat::RdfXml | OntologyFormat::Owl => RdfFormat::RdfXml,
        OntologyFormat::JsonLd => RdfFormat::JsonLd { profile: Default::default() },
        OntologyFormat::NTriples => RdfFormat::NTriples,
        OntologyFormat::NQuads => RdfFormat::NQuads,
        OntologyFormat::TriG => RdfFormat::TriG,
        OntologyFormat::Unknown => {
            return Err(CatalogError::Parse {
                path: path.to_path_buf(),
                message: "unsupported format".to_string(),
            })
        }
    };

    let parser = RdfParser::from_format(rdf_format);
    let mut file_triples = 0usize;
    for quad in parser.for_reader(content) {
        let quad: Quad = quad.map_err(|e| CatalogError::Parse {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;
        file_triples += 1;
        if file_triples > MAX_TRIPLES_PER_FILE {
            return Err(CatalogError::Parse {
                path: path.to_path_buf(),
                message: format!("file exceeds {MAX_TRIPLES_PER_FILE} triples"),
            });
        }
        if triple_count_so_far + file_triples > MAX_TOTAL_TRIPLES {
            return Err(CatalogError::Core(ontoindex_core::OntoIndexError::Scanner(format!(
                "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
            ))));
        }
        store.insert(&quad).map_err(|e| CatalogError::Store(e.to_string()))?;
    }

    Ok(())
}
