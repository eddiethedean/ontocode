# Using OntoCore as a Rust library

Embed **OntoCore** in tools, pipelines, or custom CLIs via the [`ontocore`](https://crates.io/crates/ontocore) façade crate from **crates.io**. You do **not** need to clone this repository.

> OntoCore (previously branded **OntoIndex** / `ontoindex-*`) is implemented by the `ontocore-*` crates. See [v0.9 migration](../migration/v0.9.md).

Pre-1.0: public APIs may change between minor releases until v1.0. Pin minors in production. Crates are at **0.23.x**.

!!! tip "Prefer `Workspace`"
    For new code, use the **`Workspace` API** (`ontocore = "0.23"`). Lower-level `IndexBuilder` remains available for specialized pipelines — see [Rust API](../ontocore/rust-api.md).

## crates.io first (5 minutes)

1. Create a crate (or open an existing one).
2. Add OntoCore:

```toml
[dependencies]
ontocore = "0.23"
```

3. Point `Workspace::open` at **your** ontology directory (any folder of `.ttl` / `.obo` / other indexed formats):

```rust
use ontocore::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = Workspace::open("./ontologies")?;
    let result = ws.query("SELECT short_name, labels FROM classes")?;
    for row in &result.rows {
        println!("{} — {}", row["short_name"], row["labels"]);
    }
    Ok(())
}
```

4. Run with `cargo run`. First compile pulls OntoCore dependencies (can take several minutes cold).

**Errors:** `Workspace::open` returns `CatalogError`; `query` / `sparql` return `QueryError`. The façade also exposes a unified [`ontocore::Error`](https://docs.rs/ontocore/latest/ontocore/enum.Error.html) for `From` conversion — see [Errors reference](../errors.md#rust-library-errors).

Method-level params / returns / side effects: [Rust API — Workspace methods](../ontocore/rust-api.md#workspace-method-reference).

## Minimal `Cargo.toml` recipes

**Query / index only** (façade defaults):

```toml
[dependencies]
ontocore = "0.23"
```

**Classify + explain** (same crate — reasoner is included):

```toml
[dependencies]
ontocore = "0.23"
```

```rust
use ontocore::Workspace;
use ontocore::reasoner::ReasonerId;

let ws = Workspace::open("./ontologies")?;
let result = ws.classify(ReasonerId::El)?;
```

**Semantic patch / transactions** (extra crates):

```toml
[dependencies]
ontocore = "0.23"
ontocore-edit = "0.23"
ontocore-owl = "0.21"
```

See [Semantic transactions](#semantic-transactions-ontocore-edit) below.

## Optional: monorepo examples (clone only)

In-repo examples under the unpublished `ontocode` package need a git clone:

```bash
git clone https://github.com/eddiethedean/ontocode.git && cd ontocode
cargo run -p ontocode --example ontocore_workspace
cargo run -p ontocode --example workspace_operations
cargo run -p ontocode --example error_handling
```

Those examples use `fixtures/` — that directory exists only in a clone, not after `cargo add ontocore`.

## Lower-level: index and query

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
use ontocore::Workspace;
use ontocore::reasoner::ReasonerId;

let ws = Workspace::open(".")?;
let result = ws.classify(ReasonerId::El)?;
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

**v0.19+** ships **`ontocore-edit`** for ordered, invertible Turtle/OBO edit batches. Use when building undo/redo, audit trails, or multi-step apply pipelines:

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

Dependency: `ontocore-edit = "0.23"`. Full API: [Rust API — semantic transactions](../ontocore/rust-api.md#semantic-transactions-ontocore-edit-v019) · [docs.rs/ontocore-edit](https://docs.rs/ontocore-edit).

## Next steps

| Goal | Doc |
|------|-----|
| Method reference | [Rust API](../ontocore/rust-api.md) |
| Error types | [Errors](../errors.md#rust-library-errors) |
| CLI instead of embed | [Install CLI & CI](../getting-started.md) |
| Stability expectations | [API stability](api-stability.md) |
