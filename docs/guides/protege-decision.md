# Protégé vs OntoCode decision matrix

Use this page to decide **when OntoCode fits**, **when to keep Protégé**, and **when to run both**. It reflects **v0.22.0** (latest tagged) — see [What ships today](../SHIPPED.md). A [first-week migration guide](protege-migration.md) ships today; extended round-trip playbooks are planned for **v1.0**.

!!! note "Non-goals today (v0.22)"
    - **WebProtégé-style collaboration** — out of scope until post-1.0
    - **Byte-identical Protégé OWL/XML / RDF/XML layout** — OntoCode re-serializes for semantic fidelity ([ADR-0021](../design/adr/0021-deterministic-xml-serializers.md)); write-back itself **ships** in v0.21
    - **Stable semver-guaranteed third-party plugin marketplace API** — plugin host MVP shipped; ecosystem hardening is v1.0
    - **Language SDKs** (Python/TypeScript ontology clients) — embed via Rust `ontocore`, CLI, or LSP instead

## Quick decision

| Your situation | Recommendation |
|----------------|----------------|
| VS Code + Turtle-primary ontologies | **Adopt OntoCode** (pilot IDE + CI) |
| CI lint/consistency gates without desktop Protégé | **Adopt OntoCore CLI** (`ontocore validate` / `classify`) |
| Protégé `.owl` / `.owx` corpora that need light inspector edits | **Adopt OntoCode** for core write-back; keep Protégé when you need byte-identical XML layout, full axiom catalog, or Protégé-only plugins — [OWL/XML write-back](owl-xml-workflow.md) |
| Full OWL 2 DL axiom catalog on every format | **Split workflow** — OntoCode for browse/query/CI/classification + supported authoring; keep Protégé for uncovered axiom types until [v1.0](../roadmap.md) |
| OBO release pipelines with in-editor OBO write-back | **Adopt OntoCode** (inspector since v0.13; patch engine since v0.12) for OBO inspector + `ontocore patch`; validate with ROBOT in CI |
| Enterprise requires vendor SLA / SOC 2 | **Defer** or run limited CI pilot — [Production readiness](production-readiness.md) |
| Air-gapped VS Code + internal artifact mirror | **Pilot** — [Enterprise deployment](enterprise-deployment.md) |

## Capability comparison (v0.22 tagged)

| Capability | Protégé | OntoCode v0.22 | Notes |
|------------|---------|----------------|-------|
| OWL 2 DL classification | Yes | Yes (`dl` / `auto` via OntoLogos 1.x) | Explanations EL-first; see [Reasoner guide](reasoner.md) |
| Turtle authoring | Manual / plugins | Native write-back | OntoCode inspector + patches |
| OBO authoring | Native | Native write-back | Patch engine since v0.12; Entity Inspector since v0.13 |
| RDF/XML in-place editing | Yes | Yes (semantic re-serialize) | Not byte-identical to Protégé — [OWL/XML write-back](owl-xml-workflow.md) |
| OWL/XML in-place editing | Yes | Yes (semantic re-serialize) | Core ops; Manchester/refactor limited |
| Manchester axiom editing | Full | MVP subset | Disjoint + property chains shipped (v0.12); richest on Turtle |
| Disjoint classes | Yes | Yes (v0.9) | Via Manchester / patch JSON |
| Property chain editing | Yes | Yes (v0.12) | Inspector + patch JSON |
| OBO format | Native | Index + write-back | v0.12 inspector + patch |
| ROBOT integration | Common | CLI wrapper | Java + `robot` required |
| SQL/SPARQL over repo | Plugins / external | Built-in workbench + CLI | |
| Semantic diff in pull requests | Weak default | Strong default | Shipped v0.10 |
| Workspace refactoring | Limited | Rename, migrate, move, extract | Turtle only; preview + apply |
| CI automation | External scripts | `ontocore validate` / `classify` | Documented exit codes |
| Local-first / no telemetry | Desktop app | Yes | No cloud upload by default |
| Commercial support | Ecosystem / third parties | **None** — community OSS | |

## Adoption paths

### Path A — CI only (lowest risk)

1. Pin `ontocore-cli` **0.22.0** in Linux CI
2. Gate merges with `ontocore validate`
3. Optional: `ontocore classify --profile el` when ontology is EL
4. Keep Protégé on engineer desktops unchanged

Docs: [CI integration](../ci-integration.md) · [Production evidence protocol](production-evidence.md)

### Path B — Split workflow (recommended pilot)

1. Author `.ttl` (or editable XML with caveats) in OntoCode; validate in CI
2. Use Protégé for Protégé-specific plugins, byte-identical XML layout, or axiom types not yet covered in [SHIPPED](../SHIPPED.md) / [known limitations](../known-limitations.md)
3. Standardize on Turtle for shared authoring when you need byte-stable diffs or refactor apply
4. Run 4–8 week pilot — [Production readiness](production-readiness.md)

Docs: [Protégé coexistence](protege-coexistence.md) · [OWL/XML write-back](owl-xml-workflow.md)

### Path C — Protégé replacement (not supported today)

Do **not** plan org-wide Protégé retirement on pre-1.0 alone. Re-evaluate at **v1.0** against [What ships today](../SHIPPED.md) and [known limitations](../known-limitations.md).

## When OntoCode is a poor fit

- Needs HermiT-/Protégé-grade DL **explanation UX** or a **full axiom editor for every OWL construct** as the sole gate — OntoCode ships DL classification for CI/IDE (`dl` / `auto`), but is not a drop-in Protégé replacement for all axiom authoring
- Primary artifacts require **byte-identical Protégé XML** (OntoCode re-serializes)
- Workspaces exceed [workspace limits](../workspace-limits.md) without split-repo strategy
- Legal cannot accept **LGPL** (`horned-owl`) for desktop distribution — [LGPL compliance](lgpl-compliance.md)
- Procurement requires **SOC 2, HIPAA BAA, or vendor SLA**

## Related

- [Protégé coexistence](protege-coexistence.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
- [Release timeline (non-commitment)](release-timeline.md)
