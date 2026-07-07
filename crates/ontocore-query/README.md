# ontocore-query

> Part of **OntoCore** (semantic workspace engine).

SQL-like virtual tables and SPARQL query engine for [OntoCore](https://github.com/eddiethedean/ontocode).

## Install

```toml
ontocore-query = "0.13"
```

## Quick example

```rust
use ontocore_catalog::IndexBuilder;
use ontocore_query::query_catalog;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let result = query_catalog(&catalog, "SELECT short_name FROM classes")?;
```

## Documentation

- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [SQL reference](https://ontocode-vs.readthedocs.io/en/latest/sql-reference/)
- [SPARQL reference](https://ontocode-vs.readthedocs.io/en/latest/sparql-reference/)
- [Query cookbook](https://ontocode-vs.readthedocs.io/en/latest/examples/queries/)
- [docs.rs](https://docs.rs/ontocore-query)

## License

MIT OR Apache-2.0
