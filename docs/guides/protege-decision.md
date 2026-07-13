# Protégé vs OntoCode decision matrix

Use this page to decide **when OntoCode fits**, **when to keep Protégé**, and **when to run both**. It reflects **v0.20.0** (latest tagged) — see [What ships today](../SHIPPED.md). A [first-week migration guide](protege-migration.md) ships today; extended round-trip playbooks are planned for **v1.0**.

!!! note "Non-goals today (v0.19)"
    - **WebProtégé-style collaboration** — out of scope until post-1.0
    - **In-place OWL/XML or RDF/XML write-back** — browse/query only; edit Turtle/OBO or use Protégé
    - **Stable semver-guaranteed third-party plugin marketplace API** — plugin host MVP shipped; ecosystem hardening is v1.0
    - **Language SDKs** (Python/TypeScript ontology clients) — embed via Rust `ontocore`, CLI, or LSP instead

## Quick decision

| Your situation | Recommendation |
|----------------|----------------|
| VS Code + Turtle-primary ontologies | **Adopt OntoCode** (pilot IDE + CI) |
| CI lint/consistency gates without desktop Protégé | **Adopt OntoCore CLI** (`ontocore validate` / `classify`) |
| Full OWL 2 DL axiom catalog + OWL/XML write-back | **Split workflow** — use OntoCode for Turtle/OBO authoring and DL classification; keep Protégé for OWL/XML-heavy modules until [v0.21](../roadmap.md) |
| OBO release pipelines with in-editor OBO write-back | **Adopt OntoCode** (inspector since v0.13; patch engine since v0.12) for OBO inspector + `ontocore patch`; validate with ROBOT in CI |
| Enterprise requires vendor SLA / SOC 2 | **Defer** or run limited CI pilot — [Production readiness](production-readiness.md) |
| Air-gapped VS Code + internal artifact mirror | **Pilot** — [Enterprise deployment](enterprise-deployment.md) |

## Capability comparison (v0.19 tagged)

| Capability | Protégé | OntoCode v0.19 | Notes |
|------------|---------|----------------|-------|
| OWL 2 DL classification | Yes | Yes (`dl` / `auto` via OntoLogos 1.x) | Explanations EL-first; see [Reasoner guide](reasoner.md) |
| Turtle authoring | Manual / plugins | Native write-back | OntoCode inspector + patches |
| OBO authoring | Native | Native write-back | Patch engine since v0.12; Entity Inspector since v0.13 |
| RDF/XML in-place editing | Yes | Read-only index | Write-back Turtle and OBO |
| Manchester axiom editing | Full | MVP subset | Disjoint + property chains shipped (v0.12) |
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

1. Pin `ontocore-cli` **0.20.0** in Linux CI
2. Gate merges with `ontocore validate`
3. Optional: `ontocore classify --profile el` when ontology is EL
4. Keep Protégé on engineer desktops unchanged

Docs: [CI integration](../ci-integration.md) · [Production evidence protocol](production-evidence.md)

### Path B — Split workflow (recommended pilot)

1. Author `.ttl` in OntoCode; validate in CI
2. Use Protégé for OWL/XML-heavy modules, Protégé-specific plugins, or axiom types not yet in the parity matrix
3. Standardize on Turtle for shared authoring where possible
4. Run 4–8 week pilot — [Production readiness](production-readiness.md)

Docs: [Protégé coexistence](protege-coexistence.md)

### Path C — Protégé replacement (not supported today)

Do **not** plan org-wide Protégé retirement on v0.9 documentation alone. Re-evaluate at **v1.0** against [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## When OntoCode is a poor fit

- Ontology program **requires OWL 2 DL** reasoning as the **only** gate with no CI alternative
- Primary artifacts are **OWL/XML or RDF/XML** with no Turtle/OBO migration plan
- Workspaces exceed [workspace limits](../workspace-limits.md) without split-repo strategy
- Legal cannot accept **LGPL** (`horned-owl`) for desktop distribution — [LGPL compliance](lgpl-compliance.md)
- Procurement requires **SOC 2, HIPAA BAA, or vendor SLA**

## Related

- [Protégé coexistence](protege-coexistence.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
- [Release timeline (non-commitment)](release-timeline.md)
