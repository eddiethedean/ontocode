# ADR-0002 — Use Horned-OWL for OWL Modeling

## Status

**Superseded** — v0.1–v0.2 use Oxigraph-based RDF parsing and entity extraction ([ADR-0003](0003-use-oxigraph.md)). Horned-OWL may be revisited for OWL-native authoring and write-back (v0.4+).

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

Oxigraph + catalog entity extraction for the shipped MVP. See [adr/README.md](README.md).
