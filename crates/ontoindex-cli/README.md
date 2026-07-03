# ontoindex-cli

> Part of **OntoCore** (semantic workspace engine). Crate name remains `ontoindex-cli` for compatibility.

Command-line interface for [OntoIndex](https://github.com/eddiethedean/ontocode) — index ontology workspaces, run SQL/SPARQL queries, validate, patch Turtle files, and classify with OntoLogos.

## Install

```bash
cargo install ontoindex-cli --locked
```

## Quick example

```bash
ontoindex inspect /path/to/ontologies
ontoindex query /path/to/ontologies "SELECT short_name FROM classes"
ontoindex validate /path/to/ontologies
ontoindex classify /path/to/ontologies --profile el --format json
```

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [CLI reference](https://ontocode-vs.readthedocs.io/en/latest/cli-reference/)
- [Getting started](https://ontocode-vs.readthedocs.io/en/latest/getting-started/)
- [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/)

## License

MIT OR Apache-2.0
