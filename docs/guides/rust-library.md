# Using OntoCore as a Rust library

Embed **OntoCore** in tools, pipelines, or custom CLIs via the [`ontocore`](https://crates.io/crates/ontocore) façade crate or individual `ontocore-*` implementation crates.

> OntoCore (formerly referred to as **OntoCore** in older docs) is implemented by the `ontocore-*` crates.

Pre-1.0: public APIs may change between minor releases until v1.0.

## Quick example: `Workspace` API

```bash
cargo run -p ontocode --example ontocore_workspace
```

```rust
use ontocore::workspace::Workspace;

let ws = Workspace::open("fixtures")?;
let result = ws.query("SELECT short_name, labels FROM classes")?;
for row in &result.rows {
    println!("{} — {}", row["short_name"], row["labels"]);
}
```

## Lower-level: index and query

```bash
cargo run -p ontocode --example index_and_query
```

```rust
use ontocore::catalog::IndexBuilder;
use ontocore::query::query_catalog;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let result = query_catalog(&catalog, "SELECT short_name, labels FROM classes")?;
```

## Crate map

See [OntoCore crate map](../ontocore/crate-map.md) for the full table. Summary:

| Crate | Role |
|-------|------|
| `ontocore` | Public façade — `Workspace`, module re-exports |
| `ontocore-*` | Implementation crates (stable names until v1.0) |

## Classification example

```rust
use ontocore::catalog::IndexBuilder;
use ontocore::reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let input = WorkspaceInputLoader::new("fixtures").load(catalog.class_hierarchy())?;
let result = classify(ReasonerId::El, &input, false)?;
println!("consistent: {}", result.consistent);
```

## Error handling

```bash
cargo run -p ontocode --example error_handling
```

Uses `OntoCoreError` from `ontocore-core` (re-exported as `ontocore::OntoCoreError`).

## API stability

- Crates are at **0.9.x** on crates.io
- `Workspace` API is experimental until v0.10
- LSP wire JSON: [LSP API](../lsp-api.md)
- SQL tables: [SQL reference](../sql-reference.md)

Pin exact versions in `Cargo.toml` and use `--locked` when installing the CLI.

## Related

- [OntoCore overview](../ontocore/index.md)
- [CLI reference](../cli-reference.md)
- [OntoCore LSP](../ontocore/lsp.md)
