# Rust & CLI (OntoCore)

**OntoCore** is the Rust semantic workspace engine behind OntoCode: the `ontoindex` CLI (OntoCore CLI), published `ontocore` and `ontoindex-*` crates on [crates.io](https://crates.io/search?q=ontoindex), and the `ontoindex-lsp` language server (OntoCore LSP, bundled in the VS Code extension).

> OntoCore is currently invoked as **`ontoindex`** on the command line. An `ontocore` alias is planned for v0.10.

> **Looking for the VS Code extension only?** See [OntoCode VS Code extension](../ontocode/vscode-extension.md).

## Quick start

```bash
cargo install ontoindex-cli --locked
ontoindex query /path/to/ontologies "SELECT * FROM classes"
ontoindex validate /path/to/ontologies
```

No clone required. Release binaries: [release integrity](../release-integrity.md).

[:octicons-arrow-right-24: CLI getting started](../getting-started.md)

## CLI workflows

| Task | Guide / command |
|------|-----------------|
| Index and inspect | `ontoindex inspect <workspace>` — [CLI reference](../cli-reference.md) |
| SQL virtual tables | `ontoindex query` — [OntoCore SQL views](../ontocore/sql-views.md) |
| SPARQL | `ontoindex sparql` — [SPARQL reference](../sparql-reference.md) |
| Lint / CI gate | `ontoindex validate` — [CI integration](../ci-integration.md) |
| EL / RL / RDFS classify | `ontoindex classify` — [Reasoner](reasoner.md) |
| Turtle patches | `ontoindex patch` — [Patch reference](../patch-reference.md) |
| Workspace refactor | `ontoindex refactor` — [Refactoring guide](refactoring.md) |

## Rust library embedding

| Topic | Guide |
|-------|-------|
| OntoCore overview | [ontocore/index.md](../ontocore/index.md) |
| Crate map, examples | [Rust library guide](rust-library.md) |
| `Workspace` API | [`examples/ontocore_workspace.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/ontocore_workspace.rs) |

Primary dependency: `ontocore = "0.9"`. Implementation crates remain `ontoindex-*` for compatibility.

## LSP integration

OntoCore LSP is provided by `ontoindex-lsp`. Wire format: [OntoCore LSP](../ontocore/lsp.md) · [LSP API](../lsp-api.md).

## Related

- [OntoCore architecture](../ontocore/architecture.md)
- [OntoCore roadmap](../ontocore/roadmap.md)
- [What ships today](../SHIPPED.md)
