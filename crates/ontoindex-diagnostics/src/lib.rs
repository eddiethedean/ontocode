//! Catalog lint rules and diagnostic collection for OntoIndex v0.3.
//!
//! # API stability
//!
//! **Pre-1.0:** diagnostic rule codes and severities are stable within a minor release
//! but new rules may be added.

mod engine;
mod input;
mod location;
mod rules;

pub use engine::{collect_diagnostics, collect_diagnostics_with_sources};
pub use input::DiagnosticInput;
