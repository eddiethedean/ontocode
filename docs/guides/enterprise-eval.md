# Evaluating OntoCode for your organization

This page helps security, platform, and ontology teams decide whether OntoCode **v0.7** fits your workflow. It is honest about **what ships today** vs the v1.0 Protégé-competitive target.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Enterprise documentation pack

| Document | Audience |
|----------|----------|
| [Production readiness](production-readiness.md) | Engineering leadership — pilot vs production criteria |
| [Enterprise deployment](enterprise-deployment.md) | Platform / IT — VSIX mirror, CI, air-gap |
| [Performance and sizing](performance-sizing.md) | DevOps — limits, pilot benchmarks |
| [LGPL compliance](lgpl-compliance.md) | Legal — horned-owl obligations |
| [Protégé coexistence](protege-coexistence.md) | Ontology teams — split workflow with Protégé |

## What v0.7.0 delivers

| Capability | Status |
|------------|--------|
| Browse OWL/RDF in VS Code | Shipped |
| Turtle (`.ttl`) write-back (labels, parents, create/delete) | Shipped |
| CLI SQL/SPARQL queries and `validate` for CI | Shipped |
| Inline diagnostics (Problems panel) | Shipped |
| Query workbench + Manchester editor in VS Code | Shipped |
| EL/RL/RDFS reasoning + inferred hierarchy | **Shipped** (OntoLogos 0.9.0) |
| EL explanations (where OntoLogos supports) | **Shipped** (EL-first) |
| React entity inspector + graph visualization | **Shipped** |
| OBO format index + `obo_id` in explorer | **Shipped** (write-back: Turtle only in VS Code) |
| ROBOT CLI interop (`ontoindex robot`, LSP `runRobot`) | **Shipped** (requires Java + `robot` on PATH) |
| Full OWL 2 DL reasoning (`dl` / `auto` profiles) | **Not shipped** (OntoLogos 1.0 target) |
| Full OBO write-back in VS Code | **Not shipped** (v1.0 target) |
| Semantic Git diff | **Not shipped** (v0.9 target) |

Full gap analysis: [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## Deployment model

- **Local-first:** OntoIndex indexes files on disk. Ontology content is **not uploaded** to a cloud service by default.
- **Language server:** `ontoindex-lsp` runs as a child process of VS Code over stdio. **Do not expose it to the network** without authentication — see [security policy](../security.md).
- **Offline install:** VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) + `SHA256SUMS` verification ([release-integrity.md](../release-integrity.md)).
- **CI-only usage:** Teams can run `ontoindex validate` and `ontoindex classify` in pipelines without installing the VS Code extension ([ci-integration.md](../ci-integration.md)).

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

- OntoIndex/OntoCode crates: **MIT OR Apache-2.0**
- **LGPL:** [`horned-owl`](https://crates.io/crates/horned-owl) is used for OWL modeling and Turtle write-back — review LGPL obligations ([LGPL compliance](lgpl-compliance.md), [LICENSES.md](../design/LICENSES.md), [FAQ](../faq.md))
- **NOTICES file:** Regenerate before releases per [LICENSES.md](../design/LICENSES.md); verify your release process includes third-party attribution

## Known limitations for enterprise layouts

| Limitation | Impact |
|------------|--------|
| **Multi-root VS Code workspaces** | Only the **first** folder is indexed |
| **Write-back** | **Turtle only**; OWL/XML is read-only in the inspector |
| **Reasoning** | EL/RL/RDFS via OntoLogos 0.9; **DL/auto** stubbed until OntoLogos 1.0; results may differ from Protégé on partial OWL mappings |
| **CLI release binaries** | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| **Scale** | Workspaces above [workspace limits](../workspace-limits.md) may fail indexing — prefer CLI batch workflows for very large terminologies |
| **ROBOT / Java** | `ontoindex robot` and LSP `runRobot` spawn an external Java `robot` process — not JVM-free for that workflow |

## Protégé coexistence

A full migration guide is a **v1.0 deliverable**. Today:

- [Protégé coexistence guide](protege-coexistence.md) — interim split workflow

- Use OntoCode for **Git-native Turtle editing**, **CI validation**, **SQL/SPARQL queries**, **Manchester MVP**, and **EL/RL/RDFS classification**
- **Manchester MVP (v0.5+)** covers restrictions, `and`/`or`, and cardinality — not disjoint axioms, property chains, or the full axiom catalog
- Keep Protégé for **full OWL 2 DL reasoning**, **disjoint axioms**, **full OBO write-back**, and **full OWL 2 DL editing** until v0.8–v1.0
- See [Protégé parity matrix](../design/PROTEGE_PARITY.md) and [What ships today](../SHIPPED.md)

## Evaluation checklist

1. Install from [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or offline VSIX
2. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
3. Run `ontoindex validate` and optionally `ontoindex classify --profile el` in a test CI job ([ci-integration.md](../ci-integration.md))
4. Review [security policy](../security.md) with your platform team
5. Compare [Protégé parity matrix](../design/PROTEGE_PARITY.md) against your requirements

## Questions

[FAQ](../faq.md) · [Troubleshooting](../troubleshooting.md) · [errors reference](../errors.md) · [Report an issue](https://github.com/eddiethedean/ontocode/issues)
