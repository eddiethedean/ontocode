# ontocore-catalog

> Part of **OntoCore** (semantic workspace engine).

Semantic catalog and index builder for [OntoCore](https://github.com/eddiethedean/ontocode).

## Install

```toml
ontocore-catalog = "0.22"
```

Supports incremental rebuilds (content-hash reuse), optional disk cache (`.ontocore/cache/`), and config fingerprinting for CI.

## Quick example

```rust
use ontocore_catalog::IndexBuilder;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
println!("{:?}", catalog.data().stats());
```

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [Rust library guide](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-library/)
- [SQL reference](https://ontocode-vs.readthedocs.io/en/latest/sql-reference/)
- [docs.rs](https://docs.rs/ontocore-catalog)

## License

MIT OR Apache-2.0
