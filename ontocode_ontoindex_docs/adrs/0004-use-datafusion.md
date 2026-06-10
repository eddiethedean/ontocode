# ADR-0004 — Use DataFusion for SQL-style Query Execution

## Status
Proposed

## Context
The product promise includes DuckDB-like local SQL-style queries over ontology repositories.

## Decision
Use DataFusion for SQL execution over virtual ontology tables.

## Consequences
This gives a mature query engine but requires careful virtual table design.