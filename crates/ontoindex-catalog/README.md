# ontoindex-catalog

> Part of **OntoCore** (semantic workspace engine). Crate name remains `ontoindex-catalog` for compatibility.

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

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [Rust library guide](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-library/)
- [SQL reference](https://ontocode-vs.readthedocs.io/en/latest/sql-reference/)
- [docs.rs](https://docs.rs/ontoindex-catalog)

## License

MIT OR Apache-2.0
