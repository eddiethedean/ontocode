# SQL query reference (OntoCore v0.14)

> **Status:** Documents behavior in **OntoCore v0.14.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

OntoCore exposes indexed ontology data as **virtual tables** queried with a SQL-like `SELECT` syntax. The CLI (`ontocore query`) and Rust API (`query_catalog`) use the same engine.

**Source of truth:** [`sql.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-query/src/sql.rs)

## Supported SQL subset

- `SELECT *` or `SELECT col1, col2, ...` from a single virtual table
- `FROM table_name` (one table only)
- `WHERE` with comparisons and boolean combinations:
  - `column = 'value'` or `column != 'value'` (string literals)
  - **Boolean column:** `deprecated = 'true'` or `deprecated = 'false'` (values are strings in the virtual table)
  - **Boolean shorthand:** bare `deprecated` is true when the column value is the string `"true"`
  - Combine with `AND` / `OR` only — **no parentheses**, no `NOT`, no `LIKE`, no `IN`
- Output formats: text (default), JSON (`--format json`), CSV (`--format csv`)

Not supported: `JOIN`, subqueries, `GROUP BY`, `ORDER BY`, SQL functions, `NOT`, parentheses, `LIKE`, `IN`, or multiple tables.

```bash
ontocore query fixtures "SELECT short_name FROM classes WHERE short_name = 'Person'"
ontocore query fixtures "SELECT short_name FROM classes WHERE deprecated = 'false' AND short_name = 'Person'"

# Fails at parse/eval time
ontocore query fixtures "SELECT short_name FROM classes WHERE NOT deprecated"
```

**Expected output (text):** tab-separated header plus one row for `Person` when run on `fixtures/`.

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

### Horned-OWL axiom projections (v0.13)

These tables project structured axioms from the Horned-OWL catalog (Turtle, OWL/XML, `.owx`):

| Table | Columns |
|-------|---------|
| `restrictions` | `class_iri`, `property_iri`, `restriction_kind`, `filler` |
| `equivalent_class_axioms` | `class_iri`, `expression` |
| `disjoint_class_axioms` | `class_iri`, `disjoint_with` |
| `domain_axioms` | `property_iri`, `domain` |
| `range_axioms` | `property_iri`, `range` |

Browse live schema in the Query Workbench **Schema** sidebar (`ontocore/listSqlSchema`).

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
ontocore query ./fixtures "SELECT * FROM classes"
ontocore query ./fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontocore query ./fixtures "SELECT * FROM annotations" --format json
ontocore query ./fixtures "SELECT code, message FROM diagnostics WHERE severity = 'warning'"
```
