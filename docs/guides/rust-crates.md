# Rust & CLI (OntoIndex)

**OntoIndex** is the Rust engine behind OntoCode: the `ontoindex` CLI, published `ontoindex-*` crates on [crates.io](https://crates.io/search?q=ontoindex), and the `ontoindex-lsp` language server (also bundled inside the VS Code extension).

> **Looking for the VS Code extension only?** See the [VS Code extension path](vscode-extension.md).

## Quick start

```bash
cargo install ontoindex-cli --locked
ontoindex query /path/to/ontologies "SELECT * FROM classes"
ontoindex validate /path/to/ontologies
```

No clone required. Release binaries: [release integrity](../release-integrity.md).

[:octicons-arrow-right-24: CLI getting started](../getting-started.md) — install options, first commands, fixtures.

## CLI workflows

| Task | Guide / command |
|------|-----------------|
| Index and inspect | `ontoindex inspect <workspace>` — [CLI reference](../cli-reference.md) |
| SQL virtual tables | `ontoindex query` — [SQL reference](../sql-reference.md) |
| SPARQL | `ontoindex sparql` — [SPARQL reference](../sparql-reference.md) |
| Lint / CI gate | `ontoindex validate` — [CI integration](../ci-integration.md) |
| EL / RL / RDFS classify | `ontoindex classify` — [Reasoner](reasoner.md) |
| Turtle patches | `ontoindex patch` — [Patch reference](../patch-reference.md) |
| Workspace refactor | `ontoindex refactor` — [Refactoring guide](refactoring.md) |

## Rust library embedding

| Topic | Guide |
|-------|-------|
| Crate map, examples | [Rust library guide](rust-library.md) |
| Index + query in code | [`examples/index_and_query.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/index_and_query.rs) |
| Per-crate READMEs | [`crates/`](https://github.com/eddiethedean/ontocode/tree/main/crates) on GitHub |

Published crates (dependency order): `ontoindex-core` → `ontoindex-parser` → `ontoindex-owl` → `ontoindex-diagnostics` → `ontoindex-catalog` → `ontoindex-query` → `ontoindex-reasoner` → `ontoindex-refactor` → `ontoindex-robot` → `ontoindex-lsp` → `ontoindex-cli`.

## Integrators

| Topic | Guide |
|-------|-------|
| Custom editor / tool via LSP | [LSP API](../lsp-api.md) |
| Patch JSON wire format | [Patch reference](../patch-reference.md) |
| Limits and exit codes | [Workspace limits](../workspace-limits.md) · [Errors](../errors.md) |

## Enterprise & CI

- [CI integration](../ci-integration.md)
- [Production readiness](production-readiness.md)
- [Performance and sizing](performance-sizing.md)

## Help

- [FAQ](../faq.md) — `cargo install` vs clone, MSRV, pre-1.0 API stability
- [Troubleshooting](../troubleshooting.md)
- [Best practices](best-practices.md)

## Related

- [VS Code extension path](vscode-extension.md) — Marketplace install, explorer, inspector
- [Documentation home](../index.md)
