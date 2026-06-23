# SQL query reference (OntoIndex v0.3)

OntoIndex exposes indexed ontology data as **virtual tables** queried with a SQL-like `SELECT` syntax. The CLI (`ontoindex query`) and Rust API (`query_catalog`) use the same engine.

**Source of truth:** [`crates/ontoindex-query/src/sql.rs`](../crates/ontoindex-query/src/sql.rs)

## Supported SQL subset

- `SELECT *` or `SELECT col1, col2, ...` from a single virtual table
- `FROM table_name` (one table only)
- `WHERE` with simple comparisons: `column = 'value'` (string literals only)
- Output formats: text (default), JSON (`--format json`), CSV (`--format csv`)

Not supported in v0.3: `JOIN`, subqueries, `GROUP BY`, `ORDER BY`, functions, or multiple tables.

**v1.0 plan:** extend [`sqlparser`](https://crates.io/crates/sqlparser) virtual-table joins/aggregations first ([ADR-0011](../docs/design/adr/0011-use-sqlparser-for-sql.md) amendment). Revisit [DataFusion](https://crates.io/crates/datafusion) only if scope exceeds hand-rolled implementation — see [DEPENDENCY_MATRIX.md](../docs/design/DEPENDENCY_MATRIX.md).

**Limits:** query strings up to 1 MiB; result sets capped at 100,000 rows (see `ontoindex-core::limits`).

## Virtual tables and columns

### `ontologies`

| Column | Description |
|--------|-------------|
| `id` | Document id (`doc-1`, …) |
| `path` | Filesystem path |
| `format` | `turtle`, `rdf_xml`, `owl`, … |
| `base_iri` | Declared base IRI |
| `parse_status` | `Ok`, `Warning`, or `Error` |
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
| `axiom_kind` | e.g. `SubClassOf` |

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

### `diagnostics` (v0.3)

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

See [examples/queries.md](../examples/queries.md) for a copy-paste cookbook.

```bash
ontoindex query ./fixtures "SELECT * FROM classes"
ontoindex query ./fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontoindex query ./fixtures "SELECT * FROM annotations" --format json
ontoindex query ./fixtures "SELECT code, message FROM diagnostics WHERE severity = 'warning'"
```
