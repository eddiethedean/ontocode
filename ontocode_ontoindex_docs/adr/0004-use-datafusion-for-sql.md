# ADR-0004 — Use DataFusion for SQL-Style Query Execution

## Status
Proposed

## Context
OntoIndex should expose ontology concepts as SQL-queryable virtual tables.

## Decision
Evaluate DataFusion as the SQL execution layer.

## Consequences
Positive:
- mature query execution
- Arrow ecosystem
- future Parquet/CSV integrations

Negative:
- virtual table integration complexity
- may be heavier than needed for early MVP
