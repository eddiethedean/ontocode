# Examples index

Runnable examples for OntoCore CLI, Rust embedding, and OntoCode workflows.

## CLI cookbooks (copy-paste)

| Guide | Description |
|-------|-------------|
| [Query cookbook](queries.md) | SQL and SPARQL over virtual tables |
| [Sample patches](patches.md) | `ontocore patch` JSON for Turtle, OBO, RDF/XML, and OWL/XML write-back |
| [Refactoring](refactoring.md) | Rename, migrate namespace, move, extract |
| [Classify](classify.md) | Reasoner profiles and CI exit semantics |
| [Realize / instance check](realize.md) | ABox realization and `check-instance` (v0.24) |
| [SWRL](swrl.md) | Rule patches + LSP validate/list (v0.24) |
| [Semantic diff](diff.md) | Git refs, PR summary, directory compare |
| [Docs export](docs-export.md) | Markdown/HTML documentation export |
| [Index vs inspect](inspect.md) | Stats-only vs diagnostic summary |

From a git clone, prefix commands with `cargo run --` (e.g. `cargo run -- query fixtures "SELECT * FROM classes"`). With `cargo install ontocore-cli`, use `ontocore` directly.

## End-to-end workflow (clone)

```bash
cargo run -- validate fixtures
cargo run -- classify fixtures --profile el --format json
cargo run -- diff HEAD..WORKTREE --format markdown
```

Then try a patch preview: [Sample patches](patches.md). VS Code path: [First success (~10 min)](../guides/first-success.md).

## Rust examples (`examples/`)

| Example | Run | Description |
|---------|-----|-------------|
| `index_and_query` | `cargo run -p ontocode --example index_and_query` | `Workspace` + SQL query on `fixtures/` |
| `ontocore_workspace` | `cargo run -p ontocode --example ontocore_workspace` | High-level `Workspace` API |
| `workspace_operations` | `cargo run -p ontocode --example workspace_operations` | Classify, import graph, docs export |
| `error_handling` | `cargo run -p ontocode --example error_handling` | `ontocore::Error` handling |
| `semantic_diff` | `cargo run -p ontocode --example semantic_diff` | Git/workspace semantic diff (optional git repo) |

## Fixture workspaces

| Location | Description |
|----------|-------------|
| [`fixtures/` on GitHub](https://github.com/eddiethedean/ontocode/tree/main/fixtures) | Primary tutorial corpus (`example.ttl`, `complex-classes.ttl`, …) |
| [Fixtures README](https://github.com/eddiethedean/ontocode/blob/main/fixtures/README.md) | Per-file purpose and smoke commands |
| [`examples/obo-workflow/`](https://github.com/eddiethedean/ontocode/tree/main/examples/obo-workflow) | Minimal OBO workspace — see [OBO workflow guide](../guides/obo-workflow.md) |
| [`examples/protege-roundtrip/`](https://github.com/eddiethedean/ontocode/tree/main/examples/protege-roundtrip) | Protégé-style Turtle + OWL/XML / RDF/XML fixtures (v0.18) |
| [`examples/plugin-workspace/`](https://github.com/eddiethedean/ontocode/tree/main/examples/plugin-workspace) | Sample plugin manifests — see [Plugin authoring](../guides/plugins.md) |

Download tutorial files without cloning:

```bash
mkdir ontocode-tutorial && cd ontocode-tutorial
curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.24.0/fixtures/example.ttl
curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.24.0/fixtures/complex-classes.ttl
```

## VS Code tutorial

[First success (~10 min)](../guides/first-success.md) — install extension, open tutorial pack, browse and edit.

## Related

- [CLI reference](../cli-reference.md)
- [Rust API](../ontocore/rust-api.md)
- [Documentation export](../guides/docs-export.md)
- [Semantic diff](../ontocode/semantic-diff.md)
