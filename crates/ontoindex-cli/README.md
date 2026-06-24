# ontoindex-cli

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

- [CLI reference](https://onto-code.readthedocs.io/en/latest/cli-reference/)
- [Getting started](https://onto-code.readthedocs.io/en/latest/getting-started/)
- [What ships today](https://onto-code.readthedocs.io/en/latest/SHIPPED/)

## License

MIT OR Apache-2.0
