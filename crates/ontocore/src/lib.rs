//! OntoCore — semantic workspace engine for ontology development.
//!
//! OntoCore indexes ontology workspaces and provides search, diagnostics, refactoring,
//! SQL, SPARQL, reasoning integration, and LSP services.
//!
//! Implementation is provided by the `ontocore-*` crates.
//!
//! # Quick start
//!
//! ```no_run
//! use ontocore::Workspace;
//!
//! # fn demo() -> Result<(), ontocore::Error> {
//! let workspace = Workspace::open(".")?;
//! let result = workspace.query("SELECT short_name, labels FROM classes")?;
//! for row in &result.rows {
//!     println!("{:?}", row);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Validate
//!
//! ```no_run
//! use ontocore::Workspace;
//!
//! # fn demo() -> Result<(), ontocore::Error> {
//! let workspace = Workspace::open(".")?;
//! let diagnostics = workspace.diagnostics();
//! println!("{} diagnostics", diagnostics.len());
//! # Ok(())
//! # }
//! ```

pub mod catalog;
pub mod diagnostics;
pub mod diff;
pub mod docs;
pub mod edit;
pub mod error;
pub mod obo;
pub mod owl;
pub mod parser;
pub mod query;
pub mod reasoner;
pub mod refactor;
pub mod swrl;
pub mod workspace;

#[cfg(feature = "lsp")]
pub mod lsp;

#[cfg(feature = "plugins")]
pub mod plugin;

pub use error::Error;
pub use ontocore_core::{Diagnostic, Entity, OntoCoreError};
pub use workspace::{Workspace, WorkspaceOptions};
