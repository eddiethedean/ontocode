//! Core types, workspace scanning, path sandboxing, and resource limits for OntoIndex.
//!
//! Published as [`ontoindex-core`](https://crates.io/crates/ontoindex-core).
//!
//! # API stability
//!
//! **Pre-1.0:** public types and constants may change between minor releases until
//! [v1.0 stable core](https://github.com/eddiethedean/ontocode/blob/main/docs/design/v1.0_BACKLOG.md)
//! is complete. See [workspace limits](https://github.com/eddiethedean/ontocode/blob/main/docs/workspace-limits.md).

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
pub use limits::{
    MAX_ENTITIES, MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS, MAX_QUERY_BYTES, MAX_SCAN_FILES,
    MAX_SPARQL_RESULT_ROWS, MAX_SQL_RESULT_ROWS, MAX_TOTAL_TRIPLES, MAX_TRIPLES_PER_FILE,
};
pub use model::{
    Annotation, Axiom, Diagnostic, DiagnosticCode, DiagnosticSeverity, Entity, EntityKind, Import,
    Namespace, OntologyDocument, OntologyFormat, ParseStatus, SourceLocation,
    AXIOM_KIND_EQUIVALENT_CLASS, AXIOM_KIND_SUB_CLASS_OF,
};
pub use path_jail::{
    canonical_workspace_root, file_uri_to_path, is_path_within, resolve_document_path,
    resolve_lsp_document_path, validate_workspace_scope, workspace_uri_to_path,
};
pub use scanner::{OntologyFile, WorkspaceScanner};
