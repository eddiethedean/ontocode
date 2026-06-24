# Evaluating OntoCode for your organization

This page helps security, platform, and ontology teams decide whether OntoCode v0.4 fits your workflow. It is honest about **what ships today** vs the v1.0 Protégé-competitive target.

## What v0.4.0 delivers

| Capability | Status |
|------------|--------|
| Browse OWL/RDF in VS Code | Shipped |
| Turtle (`.ttl`) write-back (labels, parents, create/delete) | Shipped |
| CLI SQL/SPARQL queries and `validate` for CI | Shipped |
| Inline diagnostics (Problems panel) | Shipped |
| Reasoning, Manchester editor, query workbench in VS Code | **Not shipped** (roadmap) |
| OBO format + ROBOT interop | **Not shipped** (v0.7b target) |
| Semantic Git diff | **Not shipped** (v0.9 target) |

Full gap analysis: [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## Deployment model

- **Local-first:** OntoIndex indexes files on disk. Ontology content is **not uploaded** to a cloud service by default.
- **Language server:** `ontoindex-lsp` runs as a child process of VS Code over stdio. **Do not expose it to the network** without authentication — see [security policy](../security.md).
- **Offline install:** VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) + `SHA256SUMS` verification ([release-integrity.md](../release-integrity.md)).
- **CI-only usage:** Teams can run `ontoindex validate` in pipelines without installing the VS Code extension ([ci-integration.md](../ci-integration.md)).

## Security and compliance

| Topic | Guidance |
|-------|----------|
| Threat model | [SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — path jail, resource limits, Restricted Mode |
| Vulnerability reporting | GitHub Security Advisories (not public issues) |
| `ontocode.lspPath` | Trusted workspaces only; ignored in VS Code Restricted Mode |
| Resource limits | [workspace-limits.md](../workspace-limits.md) — file count, size, triple caps |
| Telemetry | **No telemetry** in v0.4. AI features are opt-in per [ADR-0010](../design/adr/0010-ai-features-opt-in.md) (not shipped) |
| Supply chain | SHA256 checksums on release artifacts; `cargo audit` in CI. Code signing planned — not shipped yet |

## Licensing

- OntoIndex/OntoCode crates: **MIT OR Apache-2.0**
- **LGPL:** [`horned-owl`](https://crates.io/crates/horned-owl) is used for OWL modeling and Turtle write-back — review LGPL obligations for your distribution model ([LICENSES.md](../design/LICENSES.md), [FAQ](../faq.md))
- **NOTICES file:** Regenerate before releases per [LICENSES.md](../design/LICENSES.md); verify your release process includes third-party attribution

## Known limitations for enterprise layouts

| Limitation | Impact |
|------------|--------|
| **Multi-root VS Code workspaces** | Only the **first** folder is indexed |
| **Write-back** | **Turtle only** in v0.4; OWL/XML is read-only in the inspector |
| **CLI release binaries** | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| **Scale** | Workspaces above [workspace limits](../workspace-limits.md) may fail indexing — prefer CLI batch workflows for very large terminologies |
| **ROBOT / Java** | Planned ROBOT CLI interop (v0.7b) requires an external Java process — not JVM-free for that workflow |

## Protégé migration

A full migration guide is a **v1.0 deliverable**. Today:

- Use OntoCode for **Git-native Turtle editing** and **CI validation** alongside or instead of Protégé for simple edits
- Expect to keep Protégé for **DL reasoning**, **Manchester axioms**, and **complex OWL editing** until v1.0
- See [Protégé parity matrix](../design/PROTEGE_PARITY.md) for the canonical P0 checklist

## Evaluation checklist

1. Install from [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or offline VSIX
2. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
3. Run `ontoindex validate` in a test CI job ([ci-integration.md](../ci-integration.md))
4. Review [security policy](../security.md) with your platform team
5. Compare [Protégé parity matrix](../design/PROTEGE_PARITY.md) against your requirements

## Questions

[FAQ](../faq.md) · [errors reference](../errors.md) · [Report an issue](https://github.com/eddiethedean/ontocode/issues)
