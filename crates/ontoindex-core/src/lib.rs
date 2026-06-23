//! Core types, workspace scanning, path sandboxing, and resource limits for OntoIndex.
//!
//! Published as [`ontoindex-core`](https://crates.io/crates/ontoindex-core).
//! See [SQL reference](https://github.com/eddiethedean/ontocode/blob/main/docs/sql-reference.md)
//! and [LSP API](https://github.com/eddiethedean/ontocode/blob/main/docs/lsp-api.md) for v0.3 behavior.

pub mod document_lookup;
pub mod error;
pub mod limits;
pub mod model;
pub mod path_jail;
pub mod scanner;

pub use document_lookup::{
    document_for_entity, document_for_ontology_id, document_matches_entity,
    document_matches_ontology_id, file_uri_for_path, normalize_iri,
};
pub use error::{OntoIndexError, Result};
pub use limits::*;
pub use model::*;
pub use path_jail::{
    canonical_workspace_root, file_uri_to_path, is_path_within, resolve_document_path,
    validate_workspace_scope, workspace_uri_to_path,
};
pub use scanner::{OntologyFile, WorkspaceScanner};
