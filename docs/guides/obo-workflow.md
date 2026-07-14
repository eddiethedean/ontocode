# OBO workflows

OntoCore **v0.12.0** indexes **OBO Format** (`.obo`) files via `fastobo` and exposes `obo_id` in the catalog and SQL virtual tables. **OBO write-back** ships in v0.12 via the `ontocore-obo` crate — Entity Inspector forms and `ontocore patch` for `.obo` files.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Prerequisites

- OntoCode **v0.12.0+** or `ontocore-cli` **0.12.0+**
- Workspace containing `.obo` files (or mixed `.obo` + RDF)

## Index and browse

1. Open a folder with `.obo` files in VS Code.
2. Wait for indexing (or run **OntoCode: Index Workspace**).
3. Browse **Classes** — entities from OBO terms appear with labels; explorer may show `obo_id` when present.

Supported extensions: `.obo` (syntax highlighting included).

## Edit in VS Code (v0.12+)

1. Open an OBO term in the **Entity Inspector**.
2. Edit name, synonym, definition, `is_a` parent, namespace, or deprecated flag.
3. Use **Preview** then **Apply** (same preview-before-apply flow as Turtle).

Patch ops use `term_id` (not `entity_iri`). See [patch reference](../patch-reference.md) and [migration/v0.12.md](../migration/v0.12.md).

## CLI patch

```bash
ontocore patch path/to/terms.obo --preview '[{"op":"set_name","term_id":"EX:001","value":"renamed"}]'
```

## Query `obo_id` from SQL

```bash
ontocore query /path/to/workspace "SELECT obo_id, short_name, labels FROM entities WHERE obo_id IS NOT NULL"
```

See [SQL reference](../sql-reference.md) for the `obo_id` column.

## Write-back policy

| Format | Index / query | VS Code inspector edit | `ontocore patch` |
|--------|---------------|------------------------|------------------|
| `.obo` | Yes | Yes (v0.12+) | Yes (v0.12+) |
| `.ttl` | Yes | Yes | Yes |
| `.owl` / `.owx` / `.rdf` | Yes | Yes (v0.21+; semantic re-serialize; subset of Turtle ops) | Yes (v0.21+; see [OWL/XML workflow](owl-xml-workflow.md) and [patch reference](../patch-reference.md)) |

## Example workspace

Repository example: [`examples/obo-workflow/`](https://github.com/eddiethedean/ontocode/tree/main/examples/obo-workflow)

```bash
git clone https://github.com/eddiethedean/ontocode.git
cd ontocode
cargo run -- inspect examples/obo-workflow
cargo run -- query examples/obo-workflow "SELECT obo_id, labels FROM entities"
```

## ROBOT validation

OBO pipelines often use [ROBOT](http://robot.obolibrary.org/). See [ROBOT interop guide](robot-interop.md) for `ontocore robot validate` and CI recipes.

## Limitations

- **OBO patch scope** — v0.12 covers common term headers (`name`, synonyms, `def`, `xref`, `is_a`, namespace, deprecated); not every OBO construct.
- **Multi-root workspaces** — all folders indexed since v0.10; ensure each root contains ontology files

## Related

- [ROBOT interop](robot-interop.md)
- [Authoring](../authoring.md)
- [FAQ](../faq.md)
- [v0.12 migration](../migration/v0.12.md)
