use crate::OntologyCatalogData;
use ontoindex_core::{OntologyDocument, ParseStatus, WorkspaceScanner};
use ontoindex_parser::parse_ontology_file;
use oxigraph::model::Quad;
use oxigraph::store::Store;
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
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self { workspace: PathBuf::from(".") }
    }

    pub fn workspace(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace = path.into();
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
            let parsed = parse_ontology_file(
                &file.path,
                file.format,
                &doc_id,
                &file.content_hash,
                file.modified_time,
            )
            .map_err(|e| CatalogError::Parse { path: file.path.clone(), message: e.to_string() })?;

            triple_count += parsed.triple_count;

            if parsed.parse_status != ParseStatus::Error {
                load_into_store(&store, &file.path, file.format)?;
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
            });

            entities.extend(parsed.entities);
            annotations.extend(parsed.annotations);
            axioms.extend(parsed.axioms);
            namespaces.extend(parsed.namespace_rows);
            imports.extend(parsed.import_rows);
        }

        Ok(OntologyCatalog {
            workspace: self.workspace,
            data: OntologyCatalogData {
                documents,
                entities,
                annotations,
                axioms,
                namespaces,
                imports,
                triple_count,
            },
            store,
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

fn load_into_store(
    store: &Store,
    path: &Path,
    format: ontoindex_core::OntologyFormat,
) -> Result<()> {
    use ontoindex_core::OntologyFormat;
    use oxigraph::io::{RdfFormat, RdfParser};
    use std::fs;

    let content = fs::read(path)
        .map_err(|e| CatalogError::Parse { path: path.to_path_buf(), message: e.to_string() })?;

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
    for quad in parser.for_reader(content.as_slice()) {
        let quad: Quad = quad.map_err(|e| CatalogError::Parse {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;
        store.insert(&quad).map_err(|e| CatalogError::Store(e.to_string()))?;
    }

    Ok(())
}
