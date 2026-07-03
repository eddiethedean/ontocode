# ontocore-cli

> Part of **OntoCore** (semantic workspace engine). Crate name remains `ontocore-cli` for compatibility.

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
```

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [CLI reference](https://ontocode-vs.readthedocs.io/en/latest/cli-reference/)
- [Getting started](https://ontocode-vs.readthedocs.io/en/latest/getting-started/)
- [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/)

## License

MIT OR Apache-2.0
