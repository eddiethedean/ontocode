# ontocore-cli

> Part of **OntoCore** (semantic workspace engine).

Command-line interface for [OntoCore](https://github.com/eddiethedean/ontocode) — index ontology workspaces, run SQL/SPARQL queries, validate, patch Turtle files, and classify with OntoLogos.

## Install

```bash
cargo install ontocore-cli --locked
```

## Quick example

```bash
ontocore inspect /path/to/ontologies
ontocore query /path/to/ontologies "SELECT short_name FROM classes"
ontocore validate /path/to/ontologies
ontocore classify /path/to/ontologies --profile el --format json
ontocore diff /path/to/repo HEAD..WORKTREE
ontocore diff --left-ref main --right-ref feature --format markdown --breaking-only
ontocore docs /path/to/ontologies --format markdown --output ./docs-out
```

Semantic diff compares indexed catalogs (directories, git refs, or two `Workspace` snapshots). See [migration v0.10](https://github.com/eddiethedean/ontocode/blob/main/docs/migration/v0.10.md) and [migration v0.11](https://github.com/eddiethedean/ontocode/blob/main/docs/migration/v0.11.md) for docs export and import patch ops.

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [CLI reference](https://ontocode-vs.readthedocs.io/en/latest/cli-reference/)
- [Getting started](https://ontocode-vs.readthedocs.io/en/latest/getting-started/)
- [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/)

**Current version: 0.11.1**

## License

MIT OR Apache-2.0
