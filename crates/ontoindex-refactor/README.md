# ontoindex-refactor

> Part of **OntoCore** (semantic workspace engine). Crate name remains `ontoindex-refactor` for compatibility.

Workspace refactoring for OntoIndex — find usages, safe IRI rename, namespace migration, move entity, and extract module (v0.8).

## Usage

```rust
use ontoindex_catalog::IndexBuilder;
use ontoindex_refactor::{find_usages, preview_rename_iri, apply_refactor_plan};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let usages = find_usages(&catalog, "http://example.org/people#Person");
let plan = preview_rename_iri(&catalog, "http://example.org/people#Person", "http://example.org/people#Human")?;
apply_refactor_plan(&plan, false)?;
```

CLI: `ontoindex refactor usages|rename|migrate-namespace|move|extract`.

## License

MIT OR Apache-2.0
