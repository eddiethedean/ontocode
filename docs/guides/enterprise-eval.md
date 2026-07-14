# Evaluating OntoCode for your organization

This page helps security, platform, and ontology teams decide whether OntoCode **v0.23.0** (latest tagged) fits your workflow. It is honest about **what ships today** vs the v1.0 Protégé-competitive target.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## When not to use OntoCode (today)

Prefer Protégé or other tools (or wait for v1.0) if you need:

- Full **SQL** analytics (JOINs, aggregates) — only SQL-like virtual tables ship today
- A **stable, semver-guaranteed plugin API** or production owlmake integration without subprocess scaffolding — plugin host **MVP shipped in v0.14**; stable ecosystem API planned **v1.0**

## Enterprise documentation pack

**Start here:** [Procurement and enterprise appendix](procurement-appendix.md) — single index for security, legal, deployment, and governance questionnaires.

| Document | Audience |
|----------|----------|
| [Production readiness](production-readiness.md) | Engineering leadership — pilot vs production criteria |
| [Protégé vs OntoCode](protege-decision.md) | Ontology teams — when to adopt, keep Protégé, or split |
| [Production evidence protocol](production-evidence.md) | DevOps / QA — self-benchmark on your corpus |
| [Enterprise deployment](enterprise-deployment.md) | Platform / IT — VSIX mirror, CI, air-gap |
| [Platform compatibility](platform-compatibility.md) | Platform — VS Code versions, OS/arch, remote dev |
| [Performance and sizing](performance-sizing.md) | DevOps — limits, pilot benchmarks |
| [Governance](governance.md) | Leadership — sustainability, releases, security policy |
| [Release timeline (non-commitment)](release-timeline.md) | Planning — v0.9/v1.0 goals without fixed dates |
| [LGPL compliance](lgpl-compliance.md) | Legal — horned-owl obligations |
| [Protégé coexistence](protege-coexistence.md) | Ontology teams — split workflow with Protégé |
| [Plugin authoring](plugins.md) | Platform — v0.16+ plugin API (permissions, views, preferences, context actions), manifests, subprocess plugins |

## What ships today (v0.23.0)

| Capability | Status |
|------------|--------|
| Browse OWL/RDF in VS Code | Shipped |
| Turtle (`.ttl`) write-back (labels, parents, create/delete) | Shipped |
| CLI SQL/SPARQL queries and `validate` for CI | Shipped |
| Inline diagnostics (Problems panel) | Shipped |
| Query workbench + Manchester editor in VS Code | Shipped |
| EL/RL/RDFS/DL reasoning + inferred hierarchy | **Shipped** (OntoLogos 1.x) |
| OWL 2 DL classification (`dl` / `auto` profiles) | **Shipped** (OntoLogos 1.x; not certified HermiT-identical) |
| Explanations (OntoLogos) | **Shipped** (DL-first on DL; EL/RL/RDFS alternatives where supported) |
| React entity inspector + graph visualization | **Shipped** |
| OBO format index + `obo_id` in explorer | **Shipped** |
| OBO write-back in VS Code + CLI (`ontocore-obo`) | **Shipped** (engine v0.12; inspector v0.13) |
| Turtle domain/range/chains/individual assertions | **Shipped** (v0.12) |
| OWL/XML · RDF/XML catalog (`.owl`, `.owx`) | **Shipped** (read v0.12; write-back v0.21) |
| RDF/XML + OWL/XML write-back (semantic re-serialize) | **Shipped** (v0.21; OWL 2 mutate depth expanded in v0.22) |
| ROBOT CLI interop (`ontocore robot`, LSP `runRobot`) | **Shipped** (requires Java + `robot` on PATH) |
| Full OWL 2 DL axiom catalog (all axiom kinds editable) | **Partial** (richest on Turtle; XML core ops) |
| Semantic diff | **Shipped** (v0.10 — CLI, LSP, VS Code panel) |
| Incremental indexing + multi-root workspaces | **Shipped** (v0.10) |
| Turtle completion + diagnostic quick fixes | **Shipped** (v0.11) |
| Manage Imports UI + import patch ops | **Shipped** (v0.11) |
| `ontocore docs` export | **Shipped** (v0.11) |
| Open VSX / Cursor marketplace | **Shipped** (v0.11) |
| Plugin host (`ontocore plugins`, LSP `listPlugins`/`runPlugin`) | **Shipped** (v0.14) |
| Plugin permissions + UI views (`api_version = "1"`) | **Shipped** (v0.15) |
| Explanation alternatives + staleness detection | **Shipped** (v0.15) |
| Graph asserted/inferred/combined modes | **Shipped** (v0.15) |
| Reference plugins (naming validator, Markdown exporter, SHACL scaffold) | **Shipped** (v0.14) |
| owlmake workflow scaffold (`ontocore workflow run`) | **Shipped** (v0.14 — subprocess; not production-hardened) |
| Realization / instance checking (`realize`, `check-instance`) | **Shipped** (v0.23) |
| SWRL rule browser / editor / validate | **Shipped** (v0.23; DLSafe) |
| Engine-level reasoner cancel | **Shipped** (v0.23) |

Full gap analysis for evaluators: [Known limitations](../known-limitations.md) · [What ships today](../SHIPPED.md) · [Protégé decision](protege-decision.md).

## Deployment model

- **Local-first:** OntoCore indexes files on disk. Ontology content is **not uploaded** to a cloud service by default.
- **Language server:** `ontocore-lsp` runs as a child process of VS Code over stdio. **Do not expose it to the network** without authentication — see [security policy](../security.md).
- **Offline install:** VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) + `SHA256SUMS` verification ([release-integrity.md](../release-integrity.md)).
- **CI-only usage:** Teams can run `ontocore validate` and `ontocore classify` in pipelines without installing the VS Code extension ([ci-integration.md](../ci-integration.md)).

## Security and compliance

| Topic | Guidance |
|-------|----------|
| Threat model | [SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — path jail, resource limits, Restricted Mode |
| Vulnerability reporting | GitHub Security Advisories (not public issues) |
| `ontocode.lspPath` | Trusted workspaces only; ignored in VS Code Restricted Mode |
| Resource limits | [workspace-limits.md](../workspace-limits.md) — file count, size, triple caps |
| Telemetry | **No telemetry**. AI features are opt-in per [ADR-0010](../design/adr/0010-ai-features-opt-in.md) (not shipped) |
| Supply chain | SHA256 checksums on release artifacts; `cargo audit` in CI. **Code signing not shipped.** Open VSX publish is automated on release; **VS Code Marketplace publish is manual** ([releasing](../releasing.md)). |

## Licensing

- OntoCore/OntoCode crates: **MIT OR Apache-2.0**
- **LGPL:** [`horned-owl`](https://crates.io/crates/horned-owl) is used for OWL modeling and Turtle write-back — review LGPL obligations ([LGPL compliance](lgpl-compliance.md), [LICENSES.md](../design/LICENSES.md), [FAQ](../faq.md))
- **NOTICES file:** Regenerate before releases per [LICENSES.md](../design/LICENSES.md); verify your release process includes third-party attribution

## Known limitations for enterprise layouts

| Limitation | Impact |
|------------|--------|
| **Multi-root VS Code workspaces** | **All folders indexed** (v0.10+) |
| **Write-back** | **Turtle, OBO, RDF/XML, OWL/XML**; JSON-LD / TriG / N-Triples read-only. XML is semantic re-serialize (not byte-identical) |
| **Reasoning** | EL/RL/RDFS/DL/auto via OntoLogos 1.x; explanations DL-first on DL (EL/RL/RDFS alternatives); results may differ from Protégé on partial OWL mappings |
| **CLI release binaries** | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| **Scale** | Workspaces above [workspace limits](../workspace-limits.md) may fail indexing — prefer CLI batch workflows for very large terminologies |
| **ROBOT / Java** | `ontocore robot` and LSP `runRobot` spawn an external Java `robot` process — not JVM-free for that workflow |

## Protégé coexistence

A [first-week Protégé migration guide](protege-migration.md) ships today. Round-trip workflows (Protégé export → OntoCode edit → Protégé verify) and byte-identical XML playbooks are **v1.0 targets**. Today:

- [Protégé coexistence guide](protege-coexistence.md) — split workflow when keeping Protégé for specific features
- [OWL/XML and RDF/XML write-back](owl-xml-workflow.md) — semantic re-serialize caveats

- Use OntoCode for **Turtle / OBO / RDF/XML / OWL/XML editing in VS Code** (XML with caveats), **CI validation**, **SQL/SPARQL queries**, **Manchester axioms** (richest on Turtle), **workspace refactoring** (Turtle), **property chain editing**, and **EL/RL/RDFS/DL classification**
- Keep Protégé for **byte-identical XML layout**, **Protégé-specific plugins**, and axiom types still listed under [known limitations](../known-limitations.md) until v1.0
- See [Protégé vs OntoCode](protege-decision.md) and [What ships today](../SHIPPED.md)

## Evaluation checklist

1. Install from [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor), or offline VSIX
2. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
3. Run the [production evidence protocol](production-evidence.md) on your corpus
4. Run `ontocore validate` and optionally `ontocore classify --profile el` or `--profile dl` in a test CI job ([ci-integration.md](../ci-integration.md))
5. Optional: `ontocore realize` on a representative ABox corpus and a SWRL dual-check ([realize](../examples/realize.md) · [swrl](../examples/swrl.md) · [week-2](enterprise-week-2.md))
6. Confirm Query Workbench is acceptable without Protégé DL Query — [dl-query.md](dl-query.md)
7. Review [Protégé decision matrix](protege-decision.md) and [platform compatibility](platform-compatibility.md)
8. Review [security policy](../security.md) and [governance](governance.md) with your platform team
9. Compare [What ships today](../SHIPPED.md) and [known limitations](../known-limitations.md) against your requirements; read [release timeline](release-timeline.md) for planning (no fixed v1.0 date)

## Questions

[FAQ](../faq.md) · [Troubleshooting](../troubleshooting.md) · [errors reference](../errors.md) · [Report an issue](https://github.com/eddiethedean/ontocode/issues)
