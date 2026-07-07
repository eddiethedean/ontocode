//! Workspace operations beyond query: classify, import graph, docs export.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p ontocode --example workspace_operations
//! ```

use ontocore::docs::ExportOptions;
use ontocore::reasoner::ReasonerId;
use ontocore::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = std::path::Path::new("fixtures");
    let ws = Workspace::open(workspace)?;

    let stats = ws.stats();
    println!("Indexed {} classes across {} ontologies", stats.class_count, stats.ontology_count);

    let graph = ws.import_graph()?;
    println!("Import graph: {} nodes, {} edges", graph.nodes.len(), graph.edges.len());

    let classification = ws.classify(ReasonerId::El)?;
    println!(
        "EL classification: consistent={}, unsatisfiable classes={}",
        classification.consistent,
        classification.unsatisfiable.len()
    );

    let out = std::env::temp_dir().join("ontocore-workspace-ops-docs");
    ws.export_docs(ExportOptions::markdown(&out))?;
    println!("Exported Markdown docs to {}", out.display());

    Ok(())
}
