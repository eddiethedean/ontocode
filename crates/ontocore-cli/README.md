# ontocore-cli

> Part of **OntoCore** (semantic workspace engine).

Command-line interface for [OntoCore](https://github.com/eddiethedean/ontocode) — index ontology workspaces, run SQL/SPARQL queries, validate, patch Turtle and OBO files, and classify with OntoLogos.

## Prerequisites

- Rust **1.88+** (`rustup update stable`; check with `rustc --version`)
- `~/.cargo/bin` on your `PATH` after `cargo install`

## Install (pinned)

```bash
cargo install ontocore-cli --locked --version 0.18.2
```

## Linux x64 without Rust

Download `ontocore-v*-x86_64-unknown-linux-gnu.tar.gz` from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) and verify `SHA256SUMS` — [release integrity](https://ontocode-vs.readthedocs.io/en/latest/release-integrity/).

## Quick example

```bash
ontocore inspect /path/to/ontologies
ontocore query /path/to/ontologies "SELECT short_name FROM classes"
ontocore validate /path/to/ontologies
ontocore classify /path/to/ontologies --profile el --format json

# Requires a git repository at /path/to/repo
ontocore diff /path/to/repo HEAD..WORKTREE
ontocore diff --left-ref main --right-ref feature --format markdown --breaking-only
ontocore docs /path/to/ontologies --format markdown --output ./docs-out
```

Semantic diff compares catalogs from git refs, directories, or workspace snapshots. See [semantic diff guide](https://ontocode-vs.readthedocs.io/en/latest/ontocode/semantic-diff/).

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [CLI reference](https://ontocode-vs.readthedocs.io/en/latest/cli-reference/)
- [Getting started](https://ontocode-vs.readthedocs.io/en/latest/getting-started/)
- [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/)

See [crates.io](https://crates.io/crates/ontocore-cli) for the latest published version.

## License

MIT OR Apache-2.0
