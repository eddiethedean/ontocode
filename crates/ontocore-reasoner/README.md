# ontocore-reasoner

> Part of **OntoCore** (semantic workspace engine). Crate name remains `ontocore-reasoner` for compatibility.

Thin [OntoLogos](https://github.com/eddiethedean/ontologos) 0.9.0 facade for OntoCore — EL, RL, and RDFS classification, hierarchy merge, and EL explanations.

## Usage

```rust
use ontocore_catalog::IndexBuilder;
use ontocore_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let input = WorkspaceInputLoader::new("fixtures").load(catalog.class_hierarchy())?;
let result = classify(ReasonerId::El, &input, false)?;
println!("consistent: {}", result.consistent);
```

CLI equivalent: `ontocore classify <workspace> --profile el`.

## Profiles

| Profile | Status |
|---------|--------|
| `el`, `rl`, `rdfs` | Shipped (OntoLogos 0.9) |
| `dl`, `auto` | Stubbed until OntoLogos 1.0 |

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [Reasoner guide](https://ontocode-vs.readthedocs.io/en/latest/guides/reasoner/)
- [REASONER_SPEC](https://ontocode-vs.readthedocs.io/en/latest/design/REASONER_SPEC/)

## License

MIT OR Apache-2.0
