# Using OntoIndex as a Rust library

Embed the OntoIndex crates in tools, pipelines, or custom CLIs. Pre-1.0: public APIs may change between minor releases until v1.0.

Published crates: [crates.io search — ontoindex](https://crates.io/search?q=ontoindex)

## Quick example: index and query

From the repository root:

```bash
cargo run -p ontocode --example index_and_query
```

Source: [`examples/index_and_query.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/index_and_query.rs)

```rust
use ontoindex_catalog::IndexBuilder;
use ontoindex_query::query_catalog;

let catalog = IndexBuilder::new()
    .workspace("fixtures")
    .build()?;

let result = query_catalog(&catalog, "SELECT short_name, labels FROM classes")?;
for row in &result.rows {
    println!("{} — {}", row["short_name"], row["labels"]);
}
```

## Crate map

| Crate | Role |
|-------|------|
| `ontoindex-core` | Types, workspace scanner, limits |
| `ontoindex-parser` | RDF parsing (Oxigraph) |
| `ontoindex-catalog` | Index builder, entity API |
| `ontoindex-query` | SQL virtual tables, SPARQL |
| `ontoindex-owl` | Horned-OWL facade, patches, Manchester |
| `ontoindex-diagnostics` | Lint rules |
| `ontoindex-reasoner` | OntoLogos classification facade |
| `ontoindex-lsp` | Language server binary + library |
| `ontoindex-cli` | `ontoindex` binary |

## Classification example

```rust
use ontoindex_catalog::IndexBuilder;
use ontoindex_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let input = WorkspaceInputLoader::new("fixtures").load(catalog.class_hierarchy())?;
let result = classify(ReasonerId::El, &input, false)?;
println!("consistent: {}", result.consistent);
```

See [`crates/ontoindex-reasoner/README.md`](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-reasoner/README.md).

## Error handling

```bash
cargo run -p ontocode --example error_handling
```

Uses `OntoIndexError` from `ontoindex-core` — see [`examples/error_handling.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/error_handling.rs).

## API stability

- Crates are at **0.6.x** on crates.io
- LSP wire JSON: [LSP API](../lsp-api.md)
- SQL tables: [SQL reference](../sql-reference.md)
- Exit codes: [workspace limits](../workspace-limits.md)

Pin exact versions in `Cargo.toml` and use `--locked` when installing the CLI.

## Related

- [CLI reference](../cli-reference.md)
- [LSP API](../lsp-api.md) — integrate `ontoindex-lsp` over stdio
- [Contributing](../contributing.md)
