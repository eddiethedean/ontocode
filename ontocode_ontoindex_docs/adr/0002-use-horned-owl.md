# ADR-0002 — Use Horned-OWL for OWL Modeling

## Status
Accepted

## Context
A Protégé replacement requires OWL 2 awareness and cannot treat everything as anonymous triples.

## Decision
Use Horned-OWL as a primary OWL modeling and manipulation dependency where feasible.

## Consequences
Positive:
- OWL-specific abstractions
- avoids inventing OWL model from scratch
- better axiom-level workflows

Negative:
- may need adapters for RDF-centric workflows
- may not cover every desired write-back use case
