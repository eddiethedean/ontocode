# Rust & CLI (OntoCore)

**OntoCore** is the Rust semantic workspace engine behind OntoCode: the `ontocore` CLI (OntoCore CLI), published `ontocore` and `ontocore-*` crates on [crates.io](https://crates.io/search?q=ontocore), and the `ontocore-lsp` language server (OntoCore LSP, bundled in the VS Code extension).

> **Looking for the VS Code extension only?** See [OntoCode VS Code extension](../ontocode/vscode-extension.md).

## Quick start

```bash
cargo install ontocore-cli --locked
ontocore query /path/to/ontologies "SELECT * FROM classes"
ontocore validate /path/to/ontologies
```

No clone required. Release binaries: [release integrity](../release-integrity.md).

[:octicons-arrow-right-24: CLI getting started](../getting-started.md)

## CLI workflows

| Task | Guide / command |
|------|-----------------|
| Index and inspect | `ontocore inspect <workspace>` — [CLI reference](../cli-reference.md) |
| SQL virtual tables | `ontocore query` — [OntoCore SQL views](../ontocore/sql-views.md) |
| SPARQL | `ontocore sparql` — [SPARQL reference](../sparql-reference.md) |
| Lint / CI gate | `ontocore validate` — [CI integration](../ci-integration.md) |
| EL / RL / RDFS classify | `ontocore classify` — [Reasoner](reasoner.md) |
| Turtle patches | `ontocore patch` — [Patch reference](../patch-reference.md) |
| Workspace refactor | `ontocore refactor` — [Refactoring guide](refactoring.md) |

## Rust library embedding

| Topic | Guide |
|-------|-------|
| OntoCore overview | [ontocore/index.md](../ontocore/index.md) |
| Crate map, examples | [Rust library guide](rust-library.md) · [Rust API reference](../ontocore/rust-api.md) |
| `Workspace` API | [`examples/ontocore_workspace.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/ontocore_workspace.rs) |

Primary dependency: `ontocore = "0.12"`. Pin an exact patch in CI: `cargo install ontocore-cli --locked --version 0.12.0`. See [Rust API reference](../ontocore/rust-api.md).

## LSP integration

OntoCore LSP is provided by `ontocore-lsp`. Wire format: [OntoCore LSP](../ontocore/lsp.md) · [LSP API](../lsp-api.md).

## Related

- [OntoCore architecture](../ontocore/architecture.md)
- [OntoCore roadmap](../ontocore/roadmap.md)
- [What ships today](../SHIPPED.md)
