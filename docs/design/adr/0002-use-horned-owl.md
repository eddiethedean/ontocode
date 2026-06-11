# ADR-0002 — Use Horned-OWL for OWL Modeling

## Status

**Superseded by [ADR-0013](0013-dual-stack-oxigraph-horned-owl.md) for v0.4b+** — v0.1–v0.2 used Oxigraph-based RDF parsing and entity extraction only ([ADR-0003](0003-use-oxigraph.md)).

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

## Superseded by

- v0.1–v0.2: Oxigraph + catalog entity extraction (MVP)
- v0.4b+: [ADR-0013](0013-dual-stack-oxigraph-horned-owl.md) dual stack (Oxigraph + Horned-OWL)
