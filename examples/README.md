# Examples

Runnable assets for OntoCore and OntoCode. **Canonical documentation:** [docs/examples/index.md](../docs/examples/index.md) on Read the Docs.

## Quick start (git clone)

```bash
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
cargo run -p ontocode --example index_and_query
```

## Contents

| Path | Description |
|------|-------------|
| `index_and_query.rs` | IndexBuilder + SQL |
| `ontocore_workspace.rs` | `Workspace` API |
| `workspace_operations.rs` | Classify, import graph, docs export |
| `error_handling.rs` | Error handling |
| `semantic_diff.rs` | Semantic diff |
| `obo-workflow/` | OBO smoke workspace |
| `protege-roundtrip/` | Turtle + OWL/XML fixtures |

Query cookbook: [docs/examples/queries.md](../docs/examples/queries.md) (not `examples/queries.md`).
