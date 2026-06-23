//! Catalog lint rules and diagnostic collection for OntoIndex v0.3.

mod engine;
mod input;
mod location;
mod rules;

pub use engine::{collect_diagnostics, collect_diagnostics_with_sources};
pub use input::DiagnosticInput;
