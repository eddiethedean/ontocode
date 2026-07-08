# Using OntoCore as a Rust library

Embed **OntoCore** in tools, pipelines, or custom CLIs via the [`ontocore`](https://crates.io/crates/ontocore) façade crate or individual `ontocore-*` implementation crates.

> OntoCore (previously branded **OntoIndex** / `ontoindex-*`) is implemented by the `ontocore-*` crates. See [v0.9 migration](../migration/v0.9.md).

Pre-1.0: public APIs may change between minor releases until v1.0.

## Quick example: `Workspace` API

```bash
cargo run -p ontocode --example ontocore_workspace
cargo run -p ontocode --example workspace_operations
```

```rust
use ontocore::workspace::Workspace;

// Point at any directory of ontology files (use your path, not a repo-only fixtures folder).
let ws = Workspace::open(".")?;
let result = ws.query("SELECT short_name, labels FROM classes")?;
for row in &result.rows {
    println!("{} — {}", row["short_name"], row["labels"]);
}
```

In-repo examples use `fixtures/` only when you clone this repository:

```bash
cargo run -p ontocode --example ontocore_workspace
```

## Lower-level: index and query

```bash
cargo run -p ontocode --example index_and_query   # clone only (uses fixtures/)
```

```rust
use ontocore::catalog::IndexBuilder;
use ontocore::query::query_catalog;

let catalog = IndexBuilder::new().workspace(".").build()?;
let result = query_catalog(&catalog, "SELECT short_name, labels FROM classes")?;
```

## Crate map

See [OntoCore crate map](../ontocore/crate-map.md) for the full table. Summary:

| Crate | Role |
|-------|------|
| `ontocore` | Public façade — `Workspace`, module re-exports |
| `ontocore-*` | Implementation crates (stable names until v1.0) |

## Classification example

```rust
use ontocore::catalog::IndexBuilder;
use ontocore::reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace(".").build()?;
let input = WorkspaceInputLoader::new(".").load(catalog.class_hierarchy())?;
let result = classify(ReasonerId::El, &input, false)?;
println!("consistent: {}", result.consistent);
```

## Workspace options (v0.10+)

```rust
use ontocore::{Workspace, WorkspaceOptions};

let ws = Workspace::open_with_options(
    WorkspaceOptions::single("./ontology")
        .with_disk_cache(true),
)?;
ws.reindex_incremental()?;
let diff = ws.diff_against_path("./baseline")?;
```

| Option | Purpose |
|--------|---------|
| `WorkspaceOptions::single(path)` | Primary workspace root |
| `with_disk_cache(true)` | Persist parse cache under `.ontocore/cache/` |
| `reindex_incremental()` | Reuse unchanged documents by content hash |

Semantic diff: `ws.diff()`, `ws.diff_against_path()`, or `ontocore::diff::diff_git_refs` — see [Semantic diff](../ontocode/semantic-diff.md).

## Error handling

```bash
cargo run -p ontocode --example error_handling
```

Uses `OntoCoreError` from `ontocore-core` (re-exported as `ontocore::OntoCoreError`).

## API stability

- Crates are at **0.11.x** on crates.io
- `Workspace` and `WorkspaceOptions` are **stable since v0.10** (pre-1.0 policy still applies to other crates)
- LSP wire JSON: [LSP API](../lsp-api.md)
- SQL tables: [SQL reference](../sql-reference.md)

Pin exact versions in `Cargo.toml` and use `--locked` when installing the CLI.

## Related

- [OntoCore overview](../ontocore/index.md)
- [CLI reference](../cli-reference.md)
- [OntoCore LSP](../ontocore/lsp.md)
