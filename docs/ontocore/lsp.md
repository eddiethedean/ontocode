# OntoCore LSP

**OntoCore LSP** is currently provided by the **`ontoindex-lsp`** binary and crate. The VS Code extension bundles this server and communicates over stdio.

## Binary

| Platform artifact | Name |
|-------------------|------|
| Language server | `ontoindex-lsp` |
| Bundled path (extension) | `extension/server/<platform>-<arch>/ontoindex-lsp` |

A rename to `ontocore-lsp` is planned after the public API stabilizes (v1.0 target).

## Custom methods

OntoCore exposes workspace operations via `ontoindex/*` LSP methods:

| Method | Purpose |
|--------|---------|
| `ontoindex/indexWorkspace` | Index workspace folder |
| `ontoindex/getCatalogSnapshot` | Explorer tree data |
| `ontoindex/getEntity` | Entity inspector payload |
| `ontoindex/getGraph` | Graph visualization data |
| `ontoindex/query` / `ontoindex/sparql` | Query workbench |
| `ontoindex/applyAxiomPatch` | Turtle write-back |
| `ontoindex/parseManchester` | Manchester editor |
| `ontoindex/runReasoner` / `ontoindex/getExplanation` | Reasoning |
| `ontoindex/findUsages` / `ontoindex/previewRefactor` / `ontoindex/applyRefactor` | Refactoring |

Full wire format: **[LSP API reference](../lsp-api.md)**.

## Rust library

```rust
use ontocore::lsp::catalog_snapshot_json;
use ontocore::catalog::IndexBuilder;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let json = catalog_snapshot_json(&catalog)?;
```

Protocol types: `ontocore::lsp::protocol`.

## Diagnostics

LSP publishes diagnostics with `source: "ontoindex"`. This identifier is unchanged in v0.9 for compatibility.
