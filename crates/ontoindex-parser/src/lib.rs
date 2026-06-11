//! RDF/OWL parsing into [`ontoindex_core`] entities via Oxigraph.
//!
//! Entry point: [`parse_ontology_file`].

mod rdf;
mod vocab;

pub use rdf::{parse_ontology_file, ParsedOntology};
pub use vocab::OWL;
