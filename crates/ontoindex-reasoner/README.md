# ontoindex-reasoner

Thin [OntoLogos](https://github.com/eddiethedean/ontologos) 0.9.0 facade for OntoIndex — EL, RL, and RDFS classification, hierarchy merge, and EL explanations.

## Usage

```rust
use ontoindex_catalog::IndexBuilder;
use ontoindex_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let input = WorkspaceInputLoader::new("fixtures").load(catalog.class_hierarchy())?;
let result = classify(ReasonerId::El, &input, false)?;
println!("consistent: {}", result.consistent);
```

CLI equivalent: `ontoindex classify <workspace> --profile el`.

## Profiles

| Profile | Status |
|---------|--------|
| `el`, `rl`, `rdfs` | Shipped (OntoLogos 0.9) |
| `dl`, `auto` | Stubbed until OntoLogos 1.0 |

## Documentation

- [Reasoner guide](https://onto-code.readthedocs.io/en/latest/guides/reasoner/)
- [REASONER_SPEC](https://onto-code.readthedocs.io/en/latest/design/REASONER_SPEC/)

## License

MIT OR Apache-2.0
