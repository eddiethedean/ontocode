# Rust API reference

OntoCore exposes a Rust library through the [`ontocore`](https://crates.io/crates/ontocore) façade crate and individual `ontocore-*` implementation crates.

## Stability

| Surface | Status |
|---------|--------|
| `ontocore::Workspace` | **Stable since v0.10** — preferred high-level API |
| `ontocore::catalog::IndexBuilder` | Stable for custom pipelines |
| LSP JSON (`ontocore/*` methods) | Pre-1.0 — may change between minor releases |
| SQL virtual table columns | Pre-1.0 — pin versions in production |

Pin dependencies in `Cargo.toml`:

```toml
[dependencies]
ontocore = "0.13"
```

For CI and reproducible builds: `cargo install ontocore-cli --locked --version 0.13.0`.

## docs.rs

Generated API documentation is published on [docs.rs](https://docs.rs/ontocore):

| Crate | docs.rs |
|-------|---------|
| `ontocore` | [docs.rs/ontocore](https://docs.rs/ontocore) |
| `ontocore-core` | [docs.rs/ontocore-core](https://docs.rs/ontocore-core) |
| `ontocore-parser` | [docs.rs/ontocore-parser](https://docs.rs/ontocore-parser) |
| `ontocore-catalog` | [docs.rs/ontocore-catalog](https://docs.rs/ontocore-catalog) |
| `ontocore-diagnostics` | [docs.rs/ontocore-diagnostics](https://docs.rs/ontocore-diagnostics) |
| `ontocore-query` | [docs.rs/ontocore-query](https://docs.rs/ontocore-query) |
| `ontocore-reasoner` | [docs.rs/ontocore-reasoner](https://docs.rs/ontocore-reasoner) |
| `ontocore-robot` | [docs.rs/ontocore-robot](https://docs.rs/ontocore-robot) |
| `ontocore-owl` | [docs.rs/ontocore-owl](https://docs.rs/ontocore-owl) |
| `ontocore-obo` | [docs.rs/ontocore-obo](https://docs.rs/ontocore-obo) |
| `ontocore-lsp` | [docs.rs/ontocore-lsp](https://docs.rs/ontocore-lsp) |
| `ontocore-diff` | [docs.rs/ontocore-diff](https://docs.rs/ontocore-diff) |
| `ontocore-docs` | [docs.rs/ontocore-docs](https://docs.rs/ontocore-docs) |
| `ontocore-refactor` | [docs.rs/ontocore-refactor](https://docs.rs/ontocore-refactor) |
| `ontocore-plugin` | [docs.rs/ontocore-plugin](https://docs.rs/ontocore-plugin) (experimental v0.14 foundation) |
| `ontocore-cli` | [docs.rs/ontocore-cli](https://docs.rs/ontocore-cli) |

Search all crates: [crates.io search?q=ontocore](https://crates.io/search?q=ontocore).

## Book ↔ docs.rs crosswalk

Use **this book** for workflows, limits, and LSP JSON; use **docs.rs** for Rust type signatures and module layout.

| You need… | Start in the book | Rust API (docs.rs) |
|-----------|-------------------|---------------------|
| Open and query a workspace | [Rust library guide](../guides/rust-library.md), [Workspace engine](workspace-engine.md) | [`Workspace`](https://docs.rs/ontocore/latest/ontocore/struct.Workspace.html), [`WorkspaceOptions`](https://docs.rs/ontocore/latest/ontocore/struct.WorkspaceOptions.html) |
| SQL virtual tables | [SQL reference](../sql-reference.md), [SQL views](sql-views.md) | [`ontocore::query`](https://docs.rs/ontocore/latest/ontocore/query/index.html) |
| SPARQL | [SPARQL reference](../sparql-reference.md) | [`Workspace::sparql`](https://docs.rs/ontocore/latest/ontocore/struct.Workspace.html#method.sparql) |
| Turtle patch apply | [Patch JSON](../patch-reference.md), [Authoring](../authoring.md) | [`ontocore::owl`](https://docs.rs/ontocore/latest/ontocore/owl/index.html) |
| OBO patch apply | [OBO authoring](../ontocode/obo-authoring.md) | [`ontocore::obo`](https://docs.rs/ontocore-obo/latest/ontocore_obo/index.html) |
| Semantic diff | [Semantic diff](../ontocode/semantic-diff.md) | [`ontocore-diff`](https://docs.rs/ontocore-diff/latest/ontocore_diff/index.html) |
| Refactoring | [Refactoring guide](../guides/refactoring.md) | [`ontocore-refactor`](https://docs.rs/ontocore-refactor/latest/ontocore_refactor/index.html) |
| Docs export | [Docs export](../guides/docs-export.md) | [`ontocore::docs`](https://docs.rs/ontocore/latest/ontocore/docs/index.html) |
| LSP integration | [LSP API](../lsp-api.md), [LSP overview](lsp.md) | [`ontocore-lsp`](https://docs.rs/ontocore-lsp/latest/ontocore_lsp/index.html) |
| Custom LSP client | [LSP hello world](../guides/lsp-hello-world.md) | — |
| Error codes / exit behavior | [Errors reference](../errors.md) | Crate `thiserror` types per module |
| Resource limits | [Workspace limits](../workspace-limits.md) | Index builder options in `ontocore-catalog` |

## Recommended entry point: `Workspace`

```rust
use ontocore::Workspace;

let ws = Workspace::open("./ontologies")?;

// Catalog stats
let stats = ws.stats();
println!("{} classes", stats.class_count);

// SQL virtual tables
let result = ws.query("SELECT short_name, labels FROM classes")?;

// SPARQL over indexed triples
let sparql = ws.sparql("SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10")?;

// Lint diagnostics from indexing
for d in ws.diagnostics() {
    println!("{:?}: {}", d.code, d.message);
}

// Entity search by IRI fragment, short name, or label
let hits = ws.search("Person");
```

### `WorkspaceOptions`

```rust
use ontocore::{Workspace, WorkspaceOptions};

let ws = Workspace::open_with_options(
    WorkspaceOptions::single("./ontology")
        .with_disk_cache(true)
        .with_scan_roots(vec!["./imports".into()]),
)?;
ws.reindex_incremental()?;
let diff = ws.diff_against_path("./baseline")?;
```

| Option | Purpose |
|--------|---------|
| `single(path)` | Primary workspace root |
| `with_scan_roots(vec![...])` | Multi-root indexing (mirrors v0.10 LSP behavior) |
| `with_disk_cache(true)` | Persist parse snapshots under `.ontocore/cache/` |

## Lower-level API

When you need buffer overrides, partial rebuilds, or direct catalog access:

```rust
use ontocore::catalog::IndexBuilder;
use ontocore::query::query_catalog;

let catalog = IndexBuilder::new().workspace(".").build()?;
let result = query_catalog(&catalog, "SELECT * FROM classes")?;
```

See [Workspace engine](workspace-engine.md) for the indexing pipeline and [crate map](crate-map.md) for module boundaries.

## Documentation export (`docs` module)

```rust
use ontocore::{Workspace, docs::{export_workspace, ExportOptions}};

let ws = Workspace::open("./fixtures")?;
export_workspace(
    ws.catalog(),
    ExportOptions::markdown("/tmp/onto-docs"),
)?;
```

See [Documentation export guide](../guides/docs-export.md).

## Examples in this repository

```bash
cargo run -p ontocode --example ontocore_workspace   # Workspace API
cargo run -p ontocode --example workspace_operations # classify, graph, docs export
cargo run -p ontocode --example index_and_query      # IndexBuilder + query (uses fixtures/)
cargo run -p ontocode --example error_handling        # Error handling patterns
cargo run -p ontocode --example semantic_diff         # Git/workspace semantic diff (requires git repo)
```

See [Examples index](../examples/index.md) for CLI cookbooks and fixture workflows.

## Related guides

| Topic | Document |
|-------|----------|
| Embedding walkthrough | [Rust library guide](../guides/rust-library.md) |
| CLI and crates overview | [Rust & CLI guide](../guides/rust-crates.md) |
| LSP wire format | [LSP API](../lsp-api.md) |
| SQL virtual tables | [SQL reference](../sql-reference.md) |
| Error codes | [Errors reference](../errors.md) |
| Resource limits | [Workspace limits](../workspace-limits.md) |
