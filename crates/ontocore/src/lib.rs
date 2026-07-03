//! OntoCore — semantic workspace engine for ontology development.
//!
//! OntoCore indexes ontology workspaces and provides search, diagnostics, refactoring,
//! SQL, SPARQL, reasoning integration, and LSP services.
//!
//! Implementation is currently provided by the `ontoindex-*` crates.
//! Those crate names remain stable until the public API reaches 1.0.

pub mod catalog;
pub mod diagnostics;
pub mod owl;
pub mod parser;
pub mod query;
pub mod reasoner;
pub mod refactor;
pub mod workspace;

#[cfg(feature = "lsp")]
pub mod lsp;

pub use ontoindex_core::{Diagnostic, Entity, OntoIndexError};
