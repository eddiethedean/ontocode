# ontoindex-catalog

Semantic catalog and index builder for [OntoIndex](https://github.com/eddiethedean/ontocode).

## Install

```toml
ontoindex-catalog = "0.6"
```

## Quick example

```rust
use ontoindex_catalog::IndexBuilder;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
println!("{:?}", catalog.data().stats());
```

## Documentation

- [Rust library guide](https://onto-code.readthedocs.io/en/latest/guides/rust-library/)
- [SQL reference](https://onto-code.readthedocs.io/en/latest/sql-reference/)
- [docs.rs](https://docs.rs/ontoindex-catalog)

## License

MIT OR Apache-2.0
