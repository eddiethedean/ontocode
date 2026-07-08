# Evaluating OntoCode for your organization

This page helps security, platform, and ontology teams decide whether OntoCode **v0.15.0** fits your workflow. It is honest about **what ships today** vs the v1.0 Protégé-competitive target.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## When not to use OntoCode (today)

Prefer Protégé or other tools (or wait for v1.0) if you need:

- Full **SQL** analytics (JOINs, aggregates) — only SQL-like virtual tables ship today
- A **stable, semver-guaranteed plugin API** or production owlmake integration without subprocess scaffolding — plugin host **MVP shipped in v0.14**; stable ecosystem API planned **v1.0**

## Enterprise documentation pack

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
| [Plugin authoring](plugins.md) | Platform — v0.14 plugin host MVP, manifests, subprocess plugins |

## What ships today (v0.14.0)

| Capability | Status |
|------------|--------|
| Browse OWL/RDF in VS Code | Shipped |
| Turtle (`.ttl`) write-back (labels, parents, create/delete) | Shipped |
| CLI SQL/SPARQL queries and `validate` for CI | Shipped |
| Inline diagnostics (Problems panel) | Shipped |
| Query workbench + Manchester editor in VS Code | Shipped |
| EL/RL/RDFS/DL reasoning + inferred hierarchy | **Shipped** (OntoLogos 1.x) |
| OWL 2 DL classification (`dl` / `auto` profiles) | **Shipped** (OntoLogos 1.x; HermiT parity) |
| EL explanations (where OntoLogos supports) | **Shipped** (EL-first; DL clash traces partial) |
| React entity inspector + graph visualization | **Shipped** |
| OBO format index + `obo_id` in explorer | **Shipped** |
| OBO write-back in VS Code + CLI (`ontocore-obo`) | **Shipped** (engine v0.12; inspector v0.13) |
| Turtle domain/range/chains/individual assertions | **Shipped** (v0.12) |
| OWL/XML read-only catalog (`.owl`, `.owx`) | **Shipped** (v0.12) |
| ROBOT CLI interop (`ontocore robot`, LSP `runRobot`) | **Shipped** (requires Java + `robot` on PATH) |
| Full OWL 2 DL axiom catalog (all axiom kinds editable) | **Partial** (Turtle + OBO; Horned formats read-only) |
| Semantic diff | **Shipped** (v0.10 — CLI, LSP, VS Code panel) |
| Incremental indexing + multi-root workspaces | **Shipped** (v0.10) |
| Turtle completion + diagnostic quick fixes | **Shipped** (v0.11) |
| Manage Imports UI + import patch ops | **Shipped** (v0.11) |
| `ontocore docs` export | **Shipped** (v0.11) |
| Open VSX / Cursor marketplace | **Shipped** (v0.11) |
| Plugin host MVP (`ontocore plugins`, LSP `listPlugins`/`runPlugin`) | **Shipped** (v0.14) |
| Reference plugins (naming validator, Markdown exporter, SHACL scaffold) | **Shipped** (v0.14) |
| owlmake workflow scaffold (`ontocore workflow run`) | **Shipped** (v0.14 — subprocess; not production-hardened) |

Full gap analysis: [Protégé parity matrix](../design/PROTEGE_PARITY.md).

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
| Supply chain | SHA256 checksums on release artifacts; `cargo audit` in CI. Code signing planned — not shipped yet |

## Licensing

- OntoCore/OntoCode crates: **MIT OR Apache-2.0**
- **LGPL:** [`horned-owl`](https://crates.io/crates/horned-owl) is used for OWL modeling and Turtle write-back — review LGPL obligations ([LGPL compliance](lgpl-compliance.md), [LICENSES.md](../design/LICENSES.md), [FAQ](../faq.md))
- **NOTICES file:** Regenerate before releases per [LICENSES.md](../design/LICENSES.md); verify your release process includes third-party attribution

## Known limitations for enterprise layouts

| Limitation | Impact |
|------------|--------|
| **Multi-root VS Code workspaces** | **All folders indexed** (v0.10+) |
| **Write-back** | **Turtle and OBO (`.obo`)**; RDF/XML and OWL/XML read-only in the inspector |
| **Reasoning** | EL/RL/RDFS/DL/auto via OntoLogos 1.x; explanations EL-first; results may differ from Protégé on partial OWL mappings |
| **CLI release binaries** | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| **Scale** | Workspaces above [workspace limits](../workspace-limits.md) may fail indexing — prefer CLI batch workflows for very large terminologies |
| **ROBOT / Java** | `ontocore robot` and LSP `runRobot` spawn an external Java `robot` process — not JVM-free for that workflow |

## Protégé coexistence

A [first-week Protégé migration guide](protege-migration.md) ships today. Round-trip workflows (Protégé export → OntoCode edit → Protégé verify) and OWL/XML-heavy migration playbooks are **v1.0 targets**. Today:

- [Protégé coexistence guide](protege-coexistence.md) — split workflow when keeping Protégé for specific features

- Use OntoCode for **Turtle and OBO editing in VS Code**, **CI validation**, **SQL/SPARQL queries**, **Manchester axioms** (including disjoint classes), **workspace refactoring**, **property chain editing**, and **EL/RL/RDFS/DL classification**
- Keep Protégé for **full OWL 2 DL axiom editing in OWL/XML**, **Protégé-specific plugins**, and axiom types not yet in the [Protégé parity matrix](../design/PROTEGE_PARITY.md) until v1.0
- See [Protégé parity matrix](../design/PROTEGE_PARITY.md) and [What ships today](../SHIPPED.md)

## Evaluation checklist

1. Install from [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor), or offline VSIX
2. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
3. Run the [production evidence protocol](production-evidence.md) on your corpus
4. Run `ontocore validate` and optionally `ontocore classify --profile el` in a test CI job ([ci-integration.md](../ci-integration.md))
5. Review [Protégé decision matrix](protege-decision.md) and [platform compatibility](platform-compatibility.md)
6. Review [security policy](../security.md) and [governance](governance.md) with your platform team
7. Compare [Protégé parity matrix](../design/PROTEGE_PARITY.md) against your requirements; read [release timeline](release-timeline.md) for planning (no fixed v1.0 date)

## Questions

[FAQ](../faq.md) · [Troubleshooting](../troubleshooting.md) · [errors reference](../errors.md) · [Report an issue](https://github.com/eddiethedean/ontocode/issues)
