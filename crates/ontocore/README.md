# ontocore

**OntoCore** is the semantic workspace engine for ontology development.

This crate is the public façade for OntoCore. Implementation is currently provided by the `ontoindex-*` crates; those names remain stable until the public API reaches 1.0.

## Quick start

```rust
use ontocore::workspace::Workspace;

let ws = Workspace::open("./ontology")?;
let stats = ws.catalog().data().stats();
println!("{} classes indexed", stats.class_count);

let result = ws.query("SELECT short_name, labels FROM classes")?;
for row in &result.rows {
    println!("{:?}", row);
}
```

## Modules

| Module | Role |
|--------|------|
| `workspace` | High-level `Workspace::open` API (experimental pre-1.0) |
| `catalog` | Index builder and entity catalog |
| `query` | SQL virtual tables and SPARQL |
| `diagnostics` | Lint rule collection |
| `parser` | RDF/OBO parsing |
| `owl` | Horned-OWL bridge, patches, Manchester |
| `reasoner` | OntoLogos classification facade |
| `refactor` | Workspace refactoring |
| `lsp` | LSP protocol types (feature `lsp`, enabled by default) |

## Compatibility

- CLI: `ontoindex` (OntoCore CLI alias planned for v0.10)
- LSP: `ontoindex-lsp` binary and `ontoindex/*` methods

See [docs/ontocore/](https://github.com/eddiethedean/ontocode/tree/main/docs/ontocore) in the OntoCode repository.
