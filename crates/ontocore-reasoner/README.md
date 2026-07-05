# ontocore-reasoner

> Part of **OntoCore** (semantic workspace engine).

Thin [OntoLogos](https://github.com/eddiethedean/ontologos) **1.0.0** facade for OntoCore — EL, RL, RDFS, DL, and auto-routed classification, hierarchy merge, and EL-first explanations.

## Usage

```rust
use ontocore_catalog::IndexBuilder;
use ontocore_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let input = WorkspaceInputLoader::new("fixtures").load(catalog.class_hierarchy())?;
let result = classify(ReasonerId::Dl, &input, false)?;
// `consistent` is true when no *named class* is unsatisfiable (not full ABox consistency).
println!("consistent: {}", result.consistent);
```

CLI equivalent: `ontocore classify <workspace> --profile dl`.

## Install

```toml
ontocore-reasoner = "0.11"
```

## Profiles

| Profile | Status |
|---------|--------|
| `el`, `rl`, `rdfs`, `dl`, `auto` | Shipped (OntoLogos 1.0) |

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [Reasoner guide](https://ontocode-vs.readthedocs.io/en/latest/guides/reasoner/)
- [REASONER_SPEC](https://ontocode-vs.readthedocs.io/en/latest/design/REASONER_SPEC/)

## License

MIT OR Apache-2.0
