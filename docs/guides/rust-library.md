# Using OntoCore as a Rust library

Embed **OntoCore** in tools, pipelines, or custom CLIs via the [`ontocore`](https://crates.io/crates/ontocore) façade crate or individual `ontocore-*` implementation crates.

> OntoCore (previously branded **OntoIndex** / `ontoindex-*`) is implemented by the `ontocore-*` crates. See [v0.9 migration](../migration/v0.9.md).

Pre-1.0: public APIs may change between minor releases until v1.0.

!!! tip "Prefer `Workspace`"
    For new code, use the **`Workspace` API** (`ontocore = "0.19"`). Lower-level `IndexBuilder` remains available for specialized pipelines — see [Rust API](../ontocore/rust-api.md).

## Quick example: `Workspace` API

```bash
cargo run -p ontocode --example ontocore_workspace
cargo run -p ontocode --example workspace_operations
```

```rust
use ontocore::Workspace;

// Point at any directory of ontology files (use your path, not a repo-only fixtures folder).
let ws = Workspace::open(".")?;
let result = ws.query("SELECT short_name, labels FROM classes")?;
for row in &result.rows {
    println!("{} — {}", row["short_name"], row["labels"]);
}
```

Errors are crate `thiserror` types — see [Errors reference](../errors.md) for CLI/LSP codes that map to common failure modes.

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
let input = WorkspaceInputLoader::new(".").load()?;
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

## Semantic transactions (`ontocore-edit`)

v0.19 ships **`ontocore-edit`** for ordered, invertible Turtle/OBO edit batches. Use when building undo/redo, audit trails, or multi-step apply pipelines:

```rust
use ontocore_edit::Transaction;
use ontocore_owl::PatchOp;

let txn = Transaction::from_turtle(vec![
    PatchOp::SetLabel {
        entity_iri: "http://example.org/Person".into(),
        value: "Person".into(),
    },
]);

let undo = txn.invert()?;
```

Dependency: `ontocore-edit = "0.19"`. Full API: [Rust API — semantic transactions](../ontocore/rust-api.md#semantic-transactions-ontocore-edit-v019) · [docs.rs/ontocore-edit](https://docs.rs/ontocore-edit).

## Error handling

```bash
cargo run -p ontocode --example error_handling
```

Uses `OntoCoreError` from `ontocore-core` (re-exported as `ontocore::OntoCoreError`).

## API stability

- Crates are at **0.19.x** on crates.io (`ontocore = "0.19"`)
- Prefer the `Workspace` API for new code; see [Rust API](../ontocore/rust-api.md)
- `Workspace` and `WorkspaceOptions` are **stable since v0.10** (pre-1.0 policy still applies to other crates)
- LSP wire JSON: [LSP API](../lsp-api.md)
- SQL tables: [SQL reference](../sql-reference.md)

Pin exact versions in `Cargo.toml` and use `--locked` when installing the CLI.

## Related

- [OntoCore overview](../ontocore/index.md)
- [CLI reference](../cli-reference.md)
- [OntoCore LSP](../ontocore/lsp.md)
