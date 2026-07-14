# Rust & CLI (OntoCore)

> **Canonical embedder guide:** [Rust library guide](rust-library.md) — `Workspace`, classification, transactions, and examples.
>
> This page is a short CLI-oriented index. For VS Code only, see [OntoCode VS Code extension](../ontocode/vscode-extension.md).

**OntoCore** is the Rust semantic workspace engine: `ontocore` CLI, `ontocore-*` crates on [crates.io](https://crates.io/search?q=ontocore), and `ontocore-lsp` (bundled in the VS Code extension).

## Quick start

```bash
cargo install ontocore-cli --locked --version 0.23.0
ontocore query /path/to/ontologies "SELECT * FROM classes"
ontocore validate /path/to/ontologies
```

**Linux CI:** prefer the [release binary](../ci-integration.md) over `cargo install` on every job.

[:octicons-arrow-right-24: CLI getting started](../getting-started.md)

## CLI workflows

| Task | Guide / command |
|------|-----------------|
| Index and inspect | `ontocore inspect <workspace>` — [CLI reference](../cli-reference.md) |
| SQL virtual tables | `ontocore query` — [OntoCore SQL views](../ontocore/sql-views.md) |
| SPARQL | `ontocore sparql` — [SPARQL reference](../sparql-reference.md) |
| Lint / CI gate | `ontocore validate` — [CI integration](../ci-integration.md) |
| EL / RL / RDFS classify | `ontocore classify` — [Reasoner](reasoner.md) |
| Turtle / OBO patches | `ontocore patch` — [Patch reference](../patch-reference.md) |
| Workspace refactor | `ontocore refactor` — [Refactoring guide](refactoring.md) |

## Rust library embedding

| Topic | Guide |
|-------|-------|
| **Start here** | [Rust library guide](rust-library.md) |
| API crosswalk | [Rust API reference](../ontocore/rust-api.md) |
| Crate map | [ontocore/crate-map.md](../ontocore/crate-map.md) |
| `Workspace` example | [`examples/ontocore_workspace.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/ontocore_workspace.rs) |

Primary dependency: `ontocore = "0.23"`.

## Related

- [OntoCore overview](../ontocore/index.md)
- [Which artifact?](which-artifact.md)
- [API stability (pre-1.0)](api-stability.md)
