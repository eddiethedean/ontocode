//! Match structured errors from OntoCore crates.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p ontocode --example error_handling
//! ```

use ontocore_catalog::{CatalogError, IndexBuilder};
use ontocore_core::OntoCoreError;
use ontocore_parser::{parse_ontology_file, ParseError};
use ontocore_query::{query_catalog, QueryError};
use std::path::Path;

fn main() {
    let missing = Path::new("fixtures/does-not-exist.ttl");
    if let Err(err) =
        parse_ontology_file(missing, ontocore_core::OntologyFormat::Turtle, "doc-1", "h", 0)
    {
        match err {
            ParseError::Io(_) => eprintln!("parse: file not found or unreadable"),
            ParseError::LimitExceeded(msg) => eprintln!("parse: {msg}"),
            other => eprintln!("parse: {other}"),
        }
    }

    if let Err(err) = IndexBuilder::new().workspace("fixtures").build() {
        match err {
            CatalogError::Parse { path, message } => {
                eprintln!("catalog parse error in {}: {message}", path.display());
            }
            CatalogError::Core(OntoCoreError::Scanner(msg)) => eprintln!("catalog scanner: {msg}"),
            CatalogError::Store(msg) => eprintln!("catalog store: {msg}"),
            CatalogError::Core(other) => eprintln!("catalog core: {other}"),
        }
    }

    let catalog = IndexBuilder::new().workspace("fixtures").build().expect("fixtures index");
    if let Err(err) = query_catalog(&catalog, "SELECT * FROM not_a_table") {
        match err {
            QueryError::Sql(msg) => eprintln!("query: {msg}"),
            QueryError::Sparql(msg) => eprintln!("sparql: {msg}"),
            QueryError::Export(msg) => eprintln!("export: {msg}"),
        }
    }
}
