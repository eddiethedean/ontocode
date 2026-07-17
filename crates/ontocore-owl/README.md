# ontocore-owl

> Part of **OntoCore** (semantic workspace engine).

Horned-OWL facade for OWL axiom modeling, Turtle/XML patch write-back (including `add_import` / `remove_import`), IRI remap/merge, and Manchester syntax for [OntoCore](https://github.com/eddiethedean/ontocode).

## Install

```toml
ontocore-owl = "0.26"
```

## Quick sample

```rust
use std::collections::BTreeMap;
use ontocore_owl::{apply_patches_to_text, PatchOp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = r#"@prefix ex: <http://example.org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Person a owl:Class .
"#;
    let result = apply_patches_to_text(
        src,
        &[PatchOp::AddLabel {
            entity_iri: "http://example.org#Person".into(),
            value: "Person".into(),
        }],
        false,
        &BTreeMap::new(),
    )?;
    println!("{}", result.preview_text.unwrap_or_default());
    Ok(())
}
```

## Documentation

- [Patch reference](https://ontocode-vs.readthedocs.io/en/latest/patch-reference/)
- [Authoring guide](https://ontocode-vs.readthedocs.io/en/latest/authoring/)
- [docs.rs](https://docs.rs/ontocore-owl)
- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)

## License

MIT OR Apache-2.0 (links LGPL `horned-owl` — see [LICENSES](https://ontocode-vs.readthedocs.io/en/latest/design/LICENSES/))
