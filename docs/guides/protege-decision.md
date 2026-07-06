# Protégé vs OntoCode decision matrix

Use this page to decide **when OntoCode fits**, **when to keep Protégé**, and **when to run both**. It reflects **v0.11.0** — see [What ships today](../SHIPPED.md). A full Protégé migration guide is planned for **v1.0**.

## Quick decision

| Your situation | Recommendation |
|----------------|----------------|
| Git + VS Code + Turtle-primary ontologies | **Adopt OntoCode** (pilot IDE + CI) |
| CI lint/consistency gates without desktop Protégé | **Adopt OntoCore CLI** (`ontocore validate` / `classify`) |
| Full OWL 2 DL axiom catalog + property chain editing | **Split workflow** — use OntoCode for DL classification; keep Protégé for chains and full axiom editing until v1.0 |
| OBO release pipelines with in-editor OBO write-back | **Keep Protégé** or external OBO tools; use OntoCode for index/ROBOT CI |
| Enterprise requires vendor SLA / SOC 2 | **Defer** or run limited CI pilot — [Production readiness](production-readiness.md) |
| Air-gapped VS Code + internal artifact mirror | **Pilot** — [Enterprise deployment](enterprise-deployment.md) |

## Capability comparison (v0.11)

| Capability | Protégé | OntoCode v0.11 | Notes |
|------------|---------|---------------|-------|
| OWL 2 DL classification | Yes | Yes (`dl` / `auto` via OntoLogos 1.0) | Explanations EL-first; see [Reasoner guide](reasoner.md) |
| Turtle authoring in Git | Manual / plugins | Native write-back | OntoCode inspector + patches |
| RDF/XML in-place editing | Yes | Read-only index | Write-back Turtle only |
| Manchester axiom editing | Full | MVP subset | Disjoint shipped; chains view-only |
| Disjoint classes | Yes | Yes (v0.9) | Via Manchester / patch JSON |
| Property chain editing | Yes | View-only | v1.0 target |
| OBO format | Native | Index + syntax highlight | No OBO write-back in VS Code |
| ROBOT integration | Common | CLI wrapper | Java + `robot` required |
| SQL/SPARQL over repo | Plugins / external | Built-in workbench + CLI | |
| Git-native PR workflow | Weak default | Strong default | Semantic diff v0.10 target |
| Workspace refactoring | Limited | Rename, migrate, move, extract | Turtle only; preview + apply |
| CI automation | External scripts | `ontocore validate` / `classify` | Documented exit codes |
| Local-first / no telemetry | Desktop app | Yes | No cloud upload by default |
| Commercial support | Ecosystem / third parties | **None** — community OSS | |

## Adoption paths

### Path A — CI only (lowest risk)

1. Pin `ontocore-cli` **0.11.0** in Linux CI
2. Gate merges with `ontocore validate`
3. Optional: `ontocore classify --profile el` when ontology is EL
4. Keep Protégé on engineer desktops unchanged

Docs: [CI integration](../ci-integration.md) · [Production evidence protocol](production-evidence.md)

### Path B — Split workflow (recommended pilot)

1. Author `.ttl` in OntoCode; validate in CI
2. Use Protégé for DL review, property chains, or OBO edits that OntoCode does not support
3. Standardize on Turtle in Git where possible
4. Run 4–8 week pilot — [Production readiness](production-readiness.md)

Docs: [Protégé coexistence](protege-coexistence.md)

### Path C — Protégé replacement (not supported today)

Do **not** plan org-wide Protégé retirement on v0.9 documentation alone. Re-evaluate at **v1.0** against [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## When OntoCode is a poor fit

- Ontology program **requires OWL 2 DL** reasoning as the primary gate
- Primary artifacts are **OWL/XML or OBO** with no Turtle migration plan
- Workspaces exceed [workspace limits](../workspace-limits.md) without split-repo strategy
- Legal cannot accept **LGPL** (`horned-owl`) for desktop distribution — [LGPL compliance](lgpl-compliance.md)
- Procurement requires **SOC 2, HIPAA BAA, or vendor SLA**

## Related

- [Protégé coexistence](protege-coexistence.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
- [Release timeline (non-commitment)](release-timeline.md)
