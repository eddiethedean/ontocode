# OntoCore crate map

OntoCore is currently implemented by the `ontocore-*` crates. The [`ontocore`](https://crates.io/crates/ontocore) crate is the public façade.

## Façade

| Crate | Role |
|-------|------|
| `ontocore` | Public API — `Workspace`, module re-exports |

```toml
[dependencies]
ontocore = "0.24"
# Optional: ontocore = { version = "0.19", features = ["lsp", "plugins"] }
```

```rust
use ontocore::workspace::Workspace;
use ontocore::catalog::IndexBuilder;  // lower-level
```

## Implementation crates

| Crate | Role |
|-------|------|
| `ontocore-core` | Types, workspace scanner, limits, path jail |
| `ontocore-parser` | RDF parsing (Oxigraph), OBO index |
| `ontocore-catalog` | Index builder, entity API, graph payloads |
| `ontocore-query` | SQL virtual tables, SPARQL |
| `ontocore-owl` | Horned-OWL facade, patches, Manchester |
| `ontocore-obo` | OBO Format 1.4 patch write-back |
| `ontocore-edit` | Semantic transaction apply path (v0.20) |
| `ontocore-diagnostics` | Lint rules |
| `ontocore-reasoner` | OntoLogos classification facade |
| `ontocore-swrl` | SWRL rule IR, validation, Turtle helpers (v0.24) |
| `ontocore-refactor` | Workspace refactoring |
| `ontocore-diff` | Semantic catalog diff, git compare |
| `ontocore-docs` | Markdown/HTML documentation export |
| `ontocore-robot` | ROBOT CLI wrappers |
| `ontocore-lsp` | Language server binary + protocol types |
| `ontocore-plugin` | Plugin manifest discovery and host runtime (v0.14+) |
| `ontocore-plugin-naming` | Reference naming validator plugin |
| `ontocore-plugin-markdown-export` | Reference Markdown export plugin |
| `ontocore-plugin-shacl` | SHACL validator scaffold plugin |
| `ontocore-plugin-builtins` | Built-in plugin wiring for CLI/LSP |
| `ontocore-cli` | `ontocore` binary |

`ontocore-robot` is not re-exported by `ontocore` — use `ontocore-robot` or the CLI directly for ROBOT interop.

## Module map (`ontocore`)

| `ontocore` module | Source crate |
|-------------------|--------------|
| `workspace` | Wraps `ontocore-catalog` |
| `catalog` | `ontocore-catalog` |
| `query` | `ontocore-query` |
| `diagnostics` | `ontocore-diagnostics` |
| `parser` | `ontocore-parser` |
| `owl` | `ontocore-owl` |
| `obo` | `ontocore-obo` |
| `edit` | `ontocore-edit` |
| `reasoner` | `ontocore-reasoner` |
| `swrl` | `ontocore-swrl` |
| `refactor` | `ontocore-refactor` |
| `diff` | `ontocore-diff` |
| `docs` | `ontocore-docs` |
| `lsp` | `ontocore-lsp` (feature `lsp`, opt-in) |
| `plugin` | `ontocore-plugin` (feature `plugins`, opt-in) |

## Examples

```bash
cargo run -p ontocode --example ontocore_workspace
cargo run -p ontocode --example index_and_query
```

See [Rust library guide](../guides/rust-library.md) for classification and error-handling examples.
