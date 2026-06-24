//! RDF/OWL parsing into [`ontoindex_core`] entities via Oxigraph.
//!
//! Entry point: [`parse_ontology_file`].
//!
//! # API stability
//!
//! **Pre-1.0:** parsing APIs and [`ParsedOntology`] fields may change between minor releases.

mod rdf;
mod vocab;

pub use rdf::{parse_ontology_file, parse_ontology_text, ParsedOntology};
pub use vocab::OWL;
