//! OntoCore — semantic workspace engine for ontology development.
//!
//! OntoCore indexes ontology workspaces and provides search, diagnostics, refactoring,
//! SQL, SPARQL, reasoning integration, and LSP services.
//!
//! Implementation is provided by the `ontocore-*` crates.

pub mod catalog;
pub mod diagnostics;
pub mod diff;
pub mod docs;
pub mod owl;
pub mod parser;
pub mod query;
pub mod reasoner;
pub mod refactor;
pub mod workspace;

#[cfg(feature = "lsp")]
pub mod lsp;

pub use ontocore_core::{Diagnostic, Entity, OntoCoreError};
pub use workspace::{Workspace, WorkspaceOptions};
