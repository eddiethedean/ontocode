# ontocore-docs

Export ontology workspace catalogs to Markdown and HTML documentation.

## CLI

```bash
ontocore docs ./ontologies --format markdown --output ./docs-out
ontocore docs ./ontologies --format html --output ./docs-out
```

## Library

```rust
use ontocore_docs::{export_workspace, ExportFormat, ExportOptions};
use ontocore_catalog::OntologyCatalog;

let catalog = /* from Workspace */;
export_workspace(&catalog, ExportOptions::markdown("./out"))?;
```
