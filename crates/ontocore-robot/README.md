# ontocore-robot

> Part of **OntoCore** (semantic workspace engine).

Thin wrappers around the [ROBOT](https://github.com/ontodev/robot) CLI for OntoCore.

## Install

```toml
ontocore-robot = "0.17"
```

## CLI

```bash
ontocore robot validate ./ontology.obo
ontocore robot merge --inputs a.owl b.owl --output merged.owl
ontocore robot report ./ontology --report report.tsv
```

## Library

```rust
use ontocore_robot::{robot_validate, run_robot};

let output = robot_validate(None, path.as_path())?;
```

## License

MIT OR Apache-2.0
