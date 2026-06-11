# ADR-0011 — Use sqlparser for SQL Virtual Tables

## Status

Accepted (supersedes [ADR-0004](0004-use-datafusion-for-sql.md) for v0.1–v0.2)

## Context

OntoIndex exposes ontology catalog data as SQL-queryable virtual tables. [ADR-0004](0004-use-datafusion-for-sql.md) proposed evaluating DataFusion; the MVP needed a lightweight `SELECT`/`FROM`/`WHERE` path without pulling the full Arrow/DataFusion stack.

## Decision

Implement SQL-like queries with the [`sqlparser`](https://crates.io/crates/sqlparser) crate and hand-rolled virtual table projection in `ontoindex-query`.

## Consequences

Positive:

- Small dependency footprint
- Straightforward mapping from catalog structs to rows
- Easy to document the supported SQL subset

Negative:

- Limited SQL (single table, simple filters)
- No Arrow/Parquet integration until a future migration or DataFusion adapter

## Implementation

See [`crates/ontoindex-query/src/sql.rs`](../../crates/ontoindex-query/src/sql.rs) and [docs/sql-reference.md](../../docs/sql-reference.md).
