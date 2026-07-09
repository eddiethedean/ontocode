# ontocore-refactor

> Part of **OntoCore** (semantic workspace engine).

Workspace refactoring for OntoCore — find usages, safe IRI rename, namespace migration, move entity, and extract module.

## Usage

```rust
use ontocore_catalog::IndexBuilder;
use ontocore_refactor::{find_usages, preview_rename_iri, apply_refactor_plan};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let usages = find_usages(&catalog, "http://example.org/people#Person");
let plan = preview_rename_iri(&catalog, "http://example.org/people#Person", "http://example.org/people#Human")?;
apply_refactor_plan(&plan, false, workspace_root)?;
```

CLI: `ontocore refactor usages|rename|migrate-namespace|move|extract`.

## Install

```toml
ontocore-refactor = "0.16"
```

## Documentation

- [Refactoring guide](https://ontocode-vs.readthedocs.io/en/latest/guides/refactoring/)
- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)

## License

MIT OR Apache-2.0
