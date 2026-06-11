# ADR-0008 — Use Reasoner Adapters Instead of Building a Native Reasoner First

## Status
Accepted

## Context
Reasoners are complex, and existing OWL reasoners are mature.

## Decision
Integrate reasoners through adapters rather than inventing a novel calculus from scratch on day one.

## Amendment ([ADR-0014](0014-rust-native-reasoners-only.md))

v0.6+ adapters are **Rust-native only** (`whelk-rs`, `reasonable`, in-tree DL). JVM reasoners (ELK, HermiT, Pellet) are **explicitly excluded**.

## Consequences
Positive:
- pluggable reasoner profiles (EL / RL / DL)
- avoids maintaining one monolithic reasoner API surface in core
- Rust adapters ship inside the binary — no external runtime

Negative:
- in-tree OWL 2 DL engine is substantial work ([REASONER_SPEC.md](../REASONER_SPEC.md))
- must validate against fixtures rather than delegating to HermiT
