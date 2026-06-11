//! Index a workspace and run SQL-like queries over the ontology catalog.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p ontocode --example index_and_query
//! ```

use ontoindex_catalog::IndexBuilder;
use ontoindex_query::query_catalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = std::path::Path::new("fixtures");
    let catalog = IndexBuilder::new().workspace(workspace).build()?;

    let stats = catalog.data().stats();
    println!(
        "Indexed {} ontologies, {} classes, {} triples",
        stats.ontology_count, stats.class_count, stats.triple_count
    );

    let result = query_catalog(&catalog, "SELECT short_name, labels FROM classes")?;
    for row in &result.rows {
        println!(
            "{} — {}",
            row.get("short_name").map(String::as_str).unwrap_or(""),
            row.get("labels").map(String::as_str).unwrap_or("")
        );
    }

    Ok(())
}
