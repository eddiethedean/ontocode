# ADR-0008 — Use Reasoner Adapters Instead of Building a Native Reasoner First

## Status
Accepted

## Context
Reasoners are complex, and existing OWL reasoners are mature.

## Decision
Integrate external reasoners through adapters.

## Consequences
Positive:
- faster path to v1.0 reasoning
- supports established reasoners
- avoids deep research implementation

Negative:
- JVM dependency for some adapters
- packaging complexity
