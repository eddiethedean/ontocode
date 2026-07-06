# OBO workflows

OntoCore v0.11.2 indexes **OBO Format** (`.obo`) files via `fastobo` and exposes `obo_id` in the catalog and SQL virtual tables. Write-back in VS Code remains **Turtle only** — OBO files are read-only in the Entity Inspector.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Prerequisites

- OntoCode v0.11.2+ or `ontocore-cli` 0.11.2+
- Workspace containing `.obo` files (or mixed `.obo` + RDF)

## Index and browse

1. Open a folder with `.obo` files in VS Code.
2. Wait for indexing (or run **OntoCode: Index Workspace**).
3. Browse **Classes** — entities from OBO terms appear with labels; explorer may show `obo_id` when present.

Supported extensions: `.obo` (syntax highlighting included).

## Query `obo_id` from SQL

```bash
ontocore query /path/to/workspace "SELECT obo_id, short_name, labels FROM entities WHERE obo_id IS NOT NULL"
```

See [SQL reference](../sql-reference.md) for the `obo_id` column.

## Write-back policy

| Format | Index / query | VS Code inspector edit | `ontocore patch` |
|--------|---------------|------------------------|-------------------|
| `.obo` | Yes | Read-only | Not supported |
| `.ttl` | Yes | Yes | Yes |

Edit OBO content with external tools or convert to Turtle for OntoCode write-back.

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

- **Minimal OBO parser** — line-based indexing for common term headers and relationships; not full fastobo parity.
- **No OBO write-back** in v0.8 — planned for v1.0.
- **Multi-root workspaces** — all folders indexed since v0.10; ensure each root contains ontology files

## Related

- [ROBOT interop](robot-interop.md)
- [Authoring](../authoring.md) (Turtle write-back)
- [FAQ](../faq.md)
