# OntoCore LSP

**OntoCore LSP** is currently provided by the **`ontocore-lsp`** binary and crate. The VS Code extension bundles this server and communicates over stdio.

## Binary

| Platform artifact | Name |
|-------------------|------|
| Language server | `ontocore-lsp` |
| Bundled path (extension) | `extension/server/<platform>-<arch>/ontocore-lsp` |

A rename to `ontocore-lsp` is planned after the public API stabilizes (v1.0 target).

## Custom methods

OntoCore exposes workspace operations via `ontocore/*` LSP methods:

| Method | Purpose |
|--------|---------|
| `ontocore/indexWorkspace` | Index workspace folder |
| `ontocore/getCatalogSnapshot` | Explorer tree data |
| `ontocore/getEntity` | Entity inspector payload |
| `ontocore/getGraph` | Graph visualization data |
| `ontocore/query` / `ontocore/sparql` | Query workbench |
| `ontocore/applyAxiomPatch` | Turtle write-back |
| `ontocore/parseManchester` | Manchester editor |
| `ontocore/runReasoner` / `ontocore/getExplanation` | Reasoning |
| `ontocore/findUsages` / `ontocore/previewRefactor` / `ontocore/applyRefactor` | Refactoring |

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

LSP publishes diagnostics with `source: "ontocore"`. This identifier is unchanged in v0.9 for compatibility.
