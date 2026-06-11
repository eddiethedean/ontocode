# ADR-0014 — Rust-Native Reasoners Only (No JVM)

## Status

Accepted

## Context

Earlier specs assumed **ELK** and **HermiT** JVM adapters for v1.0 ([ADR-0008](0008-reasoner-adapters-not-built-in-reasoner.md), prior [REASONER_SPEC.md](../REASONER_SPEC.md)). That conflicts with OntoIndex’s Rust-first, local-first goals: Java install friction, packaging complexity, and a hard dependency outside the shipped binary.

Rust OWL reasoners exist today, but none is a drop-in HermiT replacement with identical coverage and battle history:

| Crate / component | Profile | Notes |
|-------------------|---------|-------|
| [whelk-rs](https://github.com/INCATools/whelk-rs) | OWL EL | Horned-OWL ecosystem; experimental; ideal for OBO-scale TBoxes |
| [reasonable](https://github.com/gtfierro/reasonable) | OWL 2 RL | Datalog materialization; fast; not full DL |
| Horned-OWL reasoner interface | — | Trait defined; no bundled DL engine ([Horned-OWL paper](https://drops.dagstuhl.de/entities/document/10.4230/TGDK.2.2.9)) |

Full **OWL 2 DL** classification, consistency, and unsatisfiability explanations therefore require either an in-tree Rust implementation in `ontoindex-reasoner` or careful integration of a maturing Rust DL crate after evaluation.

## Decision

1. **Never depend on Java or JVM reasoners** (ELK, HermiT, Pellet, etc.) in OntoIndex or OntoCode — not at v1.0, not as optional adapters.
2. Ship **Rust reasoner adapters** behind the existing `ReasonerAdapter` trait ([REASONER_SPEC.md](../REASONER_SPEC.md)):
   - **`whelk`** — `whelk-rs` for OWL EL classification (default for OBO / large terminologies).
   - **`reasonable`** — OWL 2 RL materialization where RL semantics suffice.
   - **`dl`** — native OWL 2 DL engine in `ontoindex-reasoner` (classification, consistency, unsatisfiable classes, clash-trace explanations).
3. **Plugin reasoners** must be native binaries or WASM — not JVM subprocesses ([PLUGIN_SPEC.md](../PLUGIN_SPEC.md)).
4. **ROBOT** remains an optional external CLI for OBO release pipelines; it is not used for in-IDE classification ([OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md)).

## Consequences

Positive:

- Single-language stack; no Java bootstrap wizard or JAR management.
- Reasoning works offline in the VSIX / `ontoindex` binary.
- Aligns with Horned-OWL + Oxigraph dual stack ([ADR-0013](0013-dual-stack-oxigraph-horned-owl.md)).

Negative:

- Must **build or integrate** a production DL reasoner in Rust — more engineering than wrapping HermiT.
- EL-only ontologies may classify faster via `whelk` than full DL; users need profile guidance in UI.
- Parity with every Protégé + HermiT edge case is validated by tests, not assumed.

## Implementation notes

- v0.6: `whelk` adapter + DL reasoner MVP (consistency + unsat reporting).
- v1.0: DL classification + **real** clash-trace explanations (P0 in [PROTEGE_PARITY.md](../PROTEGE_PARITY.md)).
- Evaluate candidate DL implementations against Protégé-exported fixtures; do not block on JVM cross-checks in CI.

## Related

- Supersedes JVM-specific guidance in prior REASONER_SPEC and amends [ADR-0008](0008-reasoner-adapters-not-built-in-reasoner.md) (adapters remain; JVM adapters are excluded).
