# ontocore

**OntoCore** is the public façade API for the semantic workspace engine.

This crate re-exports the stable surface of OntoCore for Rust applications. Implementation lives in the `ontocore-*` crates (`ontocore-core`, `ontocore-catalog`, `ontocore-query`, etc.) — use those directly when you need lower-level control.

## Quick start

```rust
use ontocore::workspace::Workspace;

let workspace = Workspace::open(".")?;
let diagnostics = workspace.diagnostics();

let result = workspace.query("SELECT short_name, labels FROM classes")?;
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

## Ecosystem

| Component | Role |
|-----------|------|
| **OntoCore** (`ontocore`, `ontocore-*`) | Semantic workspace engine — this crate |
| **OntoLogos** | Reasoning engine (classification, consistency, explanations) |
| **OntoCode** | VS Code IDE powered by OntoCore |
| **External plugins** (e.g. [owlmake](https://github.com/INCATools/owlmake)) | Workflow/build/release automation — integrate via OntoCore plugin APIs, not core dependencies |

## Binaries

- **CLI:** `ontocore` (`ontocore-cli` crate) — `cargo install ontocore-cli --locked`
- **LSP:** `ontocore-lsp` — bundled in the OntoCode VS Code extension

See [docs/ontocore/](https://github.com/eddiethedean/ontocode/tree/main/docs/ontocore) in the OntoCode repository.
