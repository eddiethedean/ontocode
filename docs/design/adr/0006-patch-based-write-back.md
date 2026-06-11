# ADR-0006 — Use Patch-Based Write-Back

## Status
Accepted

## Context
Ontology files often contain hand formatting, comments, and structure users care about.

## Decision
Editing and refactoring should prefer targeted source patches instead of whole-file rewrites.

## Consequences
Positive:
- preserves user formatting
- produces clean Git diffs
- builds trust

Negative:
- more complex implementation
- source location fidelity is required
