# OntoCore

**OntoCore** is the semantic workspace engine for ontology development. It lives in the [OntoCode repository](https://github.com/eddiethedean/ontocode) and powers the OntoCode VS Code IDE.

OntoCore indexes ontology workspaces on disk and provides:

- Workspace discovery and indexing
- RDF/OWL/OBO parsing
- Entity catalog and symbol graph
- SQL virtual tables and SPARQL
- Diagnostics and lint rules
- Refactoring (rename, migrate, move, extract)
- Reasoning integration via [OntoLogos](https://github.com/eddiethedean/ontologos)
- Patch write-back for Turtle
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
use ontocore::workspace::Workspace;

let ws = Workspace::open("./ontology")?;
let diagnostics = ws.diagnostics();
let results = ws.query("SELECT short_name FROM classes")?;
let hits = ws.search("Person")?;
```

**Pre-1.0:** `Workspace` and related APIs are experimental until v0.10.

Lower-level access remains available through `ontocore-*` crates. See [crate map](crate-map.md).

## Compatibility (v0.9+)

All crates, binaries, and LSP methods use **`ontocore`** naming. Upgrading from v0.8? See [migration guide](../migration/v0.9.md).

| Surface | Name |
|---------|------|
| Implementation crates | `ontocore-*` |
| CLI binary | `ontocore` |
| LSP binary | `ontocore-lsp` |
| LSP methods | `ontocore/*` |
| Diagnostic source | `ontocore` |

## Next steps

- [Architecture](architecture.md)
- [Crate map](crate-map.md)
- [Workspace engine](workspace-engine.md)
- [Roadmap](roadmap.md)
- [Rust & CLI guide](../guides/rust-crates.md)
