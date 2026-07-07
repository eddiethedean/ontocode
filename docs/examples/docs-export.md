# Documentation export cookbook

Export Markdown or HTML documentation from an indexed workspace. See [Documentation export guide](../guides/docs-export.md).

## Markdown export

```bash
ontocore docs /path/to/ontologies --format markdown --output ./docs-out
```

Output includes `index.md` with class hierarchy and property index sections (v0.13+).

## HTML export

```bash
ontocore docs /path/to/ontologies --format html --output ./docs-out
```

## Limit to one ontology

```bash
ontocore docs . --format markdown --output ./docs-out \
  --ontology-id http://example.org/people
```

## From a git clone

```bash
cargo run -- docs fixtures --format markdown --output /tmp/onto-docs
```

## Related

- [CLI reference — docs](../cli-reference.md)
- [Rust API — export_docs](../ontocore/rust-api.md)
