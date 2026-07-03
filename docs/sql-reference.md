# SQL query reference (OntoCore v0.9)

> **Status:** Documents behavior in **OntoIndex v0.8.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

OntoIndex exposes indexed ontology data as **virtual tables** queried with a SQL-like `SELECT` syntax. The CLI (`ontoindex query`) and Rust API (`query_catalog`) use the same engine.

**Source of truth:** [`sql.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-query/src/sql.rs)

## Supported SQL subset

- `SELECT *` or `SELECT col1, col2, ...` from a single virtual table
- `FROM table_name` (one table only)
- `WHERE` with comparisons and boolean combinations:
  - `column = 'value'` or `column != 'value'` (string literals)
  - `column` or `NOT column` (boolean column identifiers, e.g. `deprecated`)
  - Combine with `AND` / `OR` (no parentheses)
- Output formats: text (default), JSON (`--format json`), CSV (`--format csv`)

Not supported: `JOIN`, subqueries, `GROUP BY`, `ORDER BY`, SQL functions, `LIKE`, `IN`, or multiple tables.

```bash
ontoindex query /path/to/ontologies "SELECT short_name FROM classes WHERE short_name != 'Person'"
ontoindex query /path/to/ontologies "SELECT short_name FROM classes WHERE deprecated = 'false' AND short_name = 'Person'"
```

From a git clone, replace `/path/to/ontologies` with `fixtures`.

SPARQL over indexed triples: [sparql-reference.md](sparql-reference.md).

**v1.0 plan:** extend [`sqlparser`](https://crates.io/crates/sqlparser) virtual-table joins/aggregations first ([ADR-0011](design/adr/0011-use-sqlparser-for-sql.md) amendment). Revisit [DataFusion](https://crates.io/crates/datafusion) only if scope exceeds hand-rolled implementation — see [DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md).

**Limits:** query strings up to 1 MiB; result sets capped at 100,000 rows (see [workspace-limits.md](workspace-limits.md)).

> **Warning:** Both SQL and SPARQL silently truncate at 100,000 rows. The CLI does not exit non-zero for truncation. LSP responses set `truncated: true`. Do not use row counts as strict CI gates without checking [workspace-limits.md](workspace-limits.md).

## Virtual tables and columns

### `ontologies`

| Column | Description |
|--------|-------------|
| `id` | Document id (`doc-1`, …) |
| `path` | Filesystem path |
| `format` | `turtle`, `rdf_xml`, `owl`, … |
| `base_iri` | Declared base IRI |
| `parse_status` | `ok`, `warning`, or `error` |
| `content_hash` | SHA-256 content hash |
| `modified_time` | File mtime (seconds) |

### `classes`, `object_properties`, `data_properties`, `annotation_properties`, `individuals`, `entities`, `properties`

Entity tables share these columns (`properties` is the union of all property kinds):

| Column | Description |
|--------|-------------|
| `iri` | Full IRI |
| `short_name` | Local name |
| `kind` | `class`, `object_property`, … |
| `ontology_id` | Owning ontology id |
| `labels` | Semicolon-separated labels |
| `comments` | Semicolon-separated comments |
| `deprecated` | `true` or `false` |
| `obo_id` | OBO id when indexed from `.obo` (empty for RDF-only entities) |

### `annotations`

| Column | Description |
|--------|-------------|
| `subject` | Annotation subject IRI |
| `predicate` | Predicate IRI |
| `object` | Object value |
| `ontology_id` | Document id |

### `axioms`

| Column | Description |
|--------|-------------|
| `id` | Axiom id |
| `ontology_id` | Document id |
| `subject` | Subject IRI |
| `predicate` | Predicate IRI |
| `object` | Object IRI or value |
| `axiom_kind` | e.g. `sub_class_of` |

### `namespaces`

| Column | Description |
|--------|-------------|
| `prefix` | Prefix name |
| `iri` | Namespace IRI |
| `ontology_id` | Document id |

### `imports`

| Column | Description |
|--------|-------------|
| `ontology_id` | Importing document id |
| `import_iri` | Imported ontology IRI |

### `diagnostics`

| Column | Description |
|--------|-------------|
| `code` | `parse_error`, `broken_import`, `undefined_prefix`, `duplicate_label`, `missing_label`, `orphan_class` |
| `severity` | `error`, `warning`, or `info` |
| `message` | Human-readable description |
| `file` | Filesystem path |
| `line` | 1-based line number (empty if unknown) |
| `column` | 0-based column (empty if unknown) |
| `entity_iri` | Related entity IRI, if any |

## Examples

See [query cookbook](examples/queries.md) for a copy-paste cookbook.

```bash
ontoindex query ./fixtures "SELECT * FROM classes"
ontoindex query ./fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontoindex query ./fixtures "SELECT * FROM annotations" --format json
ontoindex query ./fixtures "SELECT code, message FROM diagnostics WHERE severity = 'warning'"
```
