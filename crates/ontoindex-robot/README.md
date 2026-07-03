# ontoindex-robot

> Part of **OntoCore** (semantic workspace engine). Crate name remains `ontoindex-robot` for compatibility.

Thin wrappers around the [ROBOT](https://github.com/ontodev/robot) CLI for OntoIndex.

## CLI

```bash
ontoindex robot validate ./ontology.obo
ontoindex robot merge --inputs a.owl b.owl --output merged.owl
ontoindex robot report ./ontology --report report.tsv
```

## Library

```rust
use ontoindex_robot::{robot_validate, run_robot};

let output = robot_validate(None, path.as_path())?;
```

## License

MIT OR Apache-2.0
