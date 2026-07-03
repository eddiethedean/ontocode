# OntoCore crate map

OntoCore is currently implemented by the `ontoindex-*` crates. The [`ontocore`](https://crates.io/crates/ontocore) crate is the public façade.

## Façade

| Crate | Role |
|-------|------|
| `ontocore` | Public API — `Workspace`, module re-exports |

```toml
[dependencies]
ontocore = "0.9"
```

```rust
use ontocore::workspace::Workspace;
use ontocore::catalog::IndexBuilder;  // lower-level
```

## Implementation crates

| Crate | Role |
|-------|------|
| `ontoindex-core` | Types, workspace scanner, limits, path jail |
| `ontoindex-parser` | RDF parsing (Oxigraph), OBO index |
| `ontoindex-catalog` | Index builder, entity API, graph payloads |
| `ontoindex-query` | SQL virtual tables, SPARQL |
| `ontoindex-owl` | Horned-OWL facade, patches, Manchester |
| `ontoindex-diagnostics` | Lint rules |
| `ontoindex-reasoner` | OntoLogos classification facade |
| `ontoindex-refactor` | Workspace refactoring |
| `ontoindex-robot` | ROBOT CLI wrappers |
| `ontoindex-lsp` | Language server binary + protocol types |
| `ontoindex-cli` | `ontoindex` binary |

`ontoindex-robot` is not re-exported by `ontocore` — use `ontoindex-robot` or the CLI directly for ROBOT interop.

## Module map (`ontocore`)

| `ontocore` module | Source crate |
|-------------------|--------------|
| `workspace` | Wraps `ontoindex-catalog` |
| `catalog` | `ontoindex-catalog` |
| `query` | `ontoindex-query` |
| `diagnostics` | `ontoindex-diagnostics` |
| `parser` | `ontoindex-parser` |
| `owl` | `ontoindex-owl` |
| `reasoner` | `ontoindex-reasoner` |
| `refactor` | `ontoindex-refactor` |
| `lsp` | `ontoindex-lsp` (feature `lsp`, default on) |

## Examples

```bash
cargo run -p ontocode --example ontocore_workspace
cargo run -p ontocode --example index_and_query
```

See [Rust library guide](../guides/rust-library.md) for classification and error-handling examples.
