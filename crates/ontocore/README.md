# ontocore

**OntoCore** is the public façade API for the semantic workspace engine.

This crate re-exports the stable surface of OntoCore for Rust applications. Implementation lives in the `ontocore-*` crates (`ontocore-core`, `ontocore-catalog`, `ontocore-query`, `ontocore-edit`, etc.) — use those directly when you need lower-level control.

## Quick start

```rust
use ontocore::Workspace;

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
| `workspace` | High-level `Workspace::open` API — incremental index, diff, import graph |
| `diff` | Semantic catalog diff, version refs, breaking-change heuristics |
| `catalog` | Index builder and entity catalog |
| `query` | SQL virtual tables and SPARQL |
| `diagnostics` | Lint rule collection |
| `parser` | RDF/OBO parsing |
| `owl` | Horned-OWL bridge, patches, Manchester |
| `reasoner` | Ontologos classification facade |
| `refactor` | Workspace refactoring |
| `docs` | Markdown/HTML documentation export |
| `lsp` | LSP protocol types (feature `lsp`, enabled by default in docs.rs via `all-features`) |

## Ecosystem

| Component | Role |
|-----------|------|
| **OntoCore** (`ontocore`, `ontocore-*`) | Semantic workspace engine — this crate |
| **Ontologos** | Reasoning engine (classification, consistency, explanations) |
| **OntoCode** | VS Code IDE powered by OntoCore |
| **External plugins** (e.g. [owlmake](https://github.com/INCATools/owlmake)) | Workflow/build/release automation — integrate via OntoCore plugin APIs, not core dependencies |

## Features

| Feature | Description |
|---------|-------------|
| `lsp` | Re-export `ontocore_lsp::protocol` and `catalog_snapshot_json` |
| `plugins` | Plugin manifest discovery via `ontocore::plugin` (v0.14 foundation) |

Default features are **empty** — enable `lsp` when you need LSP wire types:

```toml
ontocore = { version = "0.17", features = ["lsp"] }
```

## Binaries

- **CLI:** `ontocore` (`ontocore-cli` crate) — `cargo install ontocore-cli --locked`
- **LSP:** `ontocore-lsp` — bundled in the OntoCode VS Code extension

## Documentation

- [docs.rs](https://docs.rs/ontocore)
- [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)
- [docs/ontocore/](https://github.com/eddiethedean/ontocode/tree/main/docs/ontocore) in this repository

**Current version: 0.19.0**
