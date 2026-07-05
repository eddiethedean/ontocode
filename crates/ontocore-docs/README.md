# ontocore-docs

> Part of **OntoCore** (semantic workspace engine). **New in v0.11.**

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

## Documentation

- [CLI reference](https://ontocode-vs.readthedocs.io/en/latest/cli-reference/)
- [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/)

## License

MIT OR Apache-2.0
