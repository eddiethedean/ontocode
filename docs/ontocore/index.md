# OntoCore

**OntoCore** is the semantic workspace engine for ontology development. It lives in the [OntoCode repository](https://github.com/eddiethedean/ontocode) and powers the OntoCode VS Code IDE.

**Current release: v0.20.0** · [crates.io search](https://crates.io/search?q=ontocore)

[![ontocore](https://img.shields.io/badge/ontocore-0.20.0-blue)](https://crates.io/crates/ontocore)
[![core](https://img.shields.io/badge/core-0.20.0-blue)](https://crates.io/crates/ontocore-core)
[![parser](https://img.shields.io/badge/parser-0.20.0-blue)](https://crates.io/crates/ontocore-parser)
[![catalog](https://img.shields.io/badge/catalog-0.20.0-blue)](https://crates.io/crates/ontocore-catalog)
[![query](https://img.shields.io/badge/query-0.20.0-blue)](https://crates.io/crates/ontocore-query)
[![cli](https://img.shields.io/badge/cli-0.20.0-blue)](https://crates.io/crates/ontocore-cli)
[![lsp](https://img.shields.io/badge/lsp-0.20.0-blue)](https://crates.io/crates/ontocore-lsp)
[![owl](https://img.shields.io/badge/owl-0.20.0-blue)](https://crates.io/crates/ontocore-owl)
[![edit](https://img.shields.io/badge/edit-0.20.0-blue)](https://crates.io/crates/ontocore-edit)
[![reasoner](https://img.shields.io/badge/reasoner-0.20.0-blue)](https://crates.io/crates/ontocore-reasoner)
[![diff](https://img.shields.io/badge/diff-0.20.0-blue)](https://crates.io/crates/ontocore-diff)
[![refactor](https://img.shields.io/badge/refactor-0.20.0-blue)](https://crates.io/crates/ontocore-refactor)
[![docs](https://img.shields.io/badge/docs-0.20.0-blue)](https://crates.io/crates/ontocore-docs)

OntoCore indexes ontology workspaces on disk and provides:

- Workspace discovery and indexing
- RDF/OWL/OBO parsing
- Entity catalog and symbol graph
- SQL virtual tables and SPARQL
- Diagnostics and lint rules
- Refactoring (rename, migrate, move, extract)
- Reasoning integration via [OntoLogos](https://github.com/eddiethedean/ontologos)
- Patch write-back for Turtle and OBO (`.ttl`, `.obo`)
- Semantic diff (version refs, directories, breaking-change heuristics)
- Documentation export (`ontocore docs`)
- CLI (`ontocore`) and LSP (`ontocore-lsp`)

## Relationship to OntoCode and OntoLogos

| Product | Role |
|---------|------|
| **OntoCore** | Rust platform — indexing, queries, diagnostics, CLI, LSP |
| **OntoCode** | VS Code extension — explorer, inspector, webviews, marketplace |
| **OntoLogos** | OWL reasoning — classification, consistency, explanations |

OntoCode is the flagship IDE built on OntoCore. OntoLogos is a separate reasoning stack that OntoCore integrates through `ontocore-reasoner`.

## Public API

Use the [`ontocore`](https://crates.io/crates/ontocore) façade crate:

```rust
use ontocore::Workspace;

let ws = Workspace::open("./ontology")?;
let diagnostics = ws.diagnostics();
let results = ws.query("SELECT short_name FROM classes")?;
let hits = ws.search("Person")?;
let graph = ws.import_graph()?;
let diff = ws.diff_against_path("./other")?;
```

**Stable since v0.10:** `Workspace`, `WorkspaceOptions`, and `ontocore::diff`. Other `ontocore-*` internals remain pre-1.0 until v1.0.

```rust
use ontocore::{Workspace, WorkspaceOptions};

let ws = Workspace::open_with_options(
    WorkspaceOptions::single("./ontology").with_disk_cache(true),
)?;
ws.reindex_incremental()?;
```

Lower-level access remains available through `ontocore-*` crates. See [crate map](crate-map.md).

## Compatibility (v0.10+)

All crates, binaries, and LSP methods use **`ontocore`** naming. Upgrading from **v0.11.x**? See [v0.13 migration](../migration/v0.13.md). From v0.10? See [v0.11 migration](../migration/v0.11.md). From v0.8? See [v0.9 migration](../migration/v0.9.md).

| Surface | Name |
|---------|------|
| Implementation crates | `ontocore-*` |
| CLI binary | `ontocore` |
| LSP binary | `ontocore-lsp` |
| LSP methods | `ontocore/*` |
| Diagnostic source | `ontocore` |

## Next steps

- [Rust API reference](rust-api.md)
- [Architecture](architecture.md)
- [Crate map](crate-map.md)
- [Workspace engine](workspace-engine.md)
- [Roadmap](roadmap.md)
- [Rust & CLI guide](../guides/rust-crates.md)
