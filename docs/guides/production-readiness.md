# Production readiness and pilot criteria

This page states what OntoCode / OntoCore **v0.20.0** (latest tagged) is appropriate for in production-like environments. It is not legal advice and does not replace your organization's risk review.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Maturity model

| Level | Version | Meaning |
|-------|---------|---------|
| **Pre-1.0** | **0.20.x (latest tagged)** | Pin `cargo install ontocore-cli --locked --version 0.20.0` in CI. Library APIs may change until [v1.0](../design/v1.0_BACKLOG.md). |
| **In development** | **0.20.x (unreleased on `main`)** | Workspace runtime and patch hardening — not on Marketplace/crates.io until tagged. |
| **Stable CI gates** | 0.19.x | `ontocore validate`, `ontocore classify`, and `ontocore diff` are documented for CI — see [workspace limits](../workspace-limits.md). |
| **v1.0 target** | Planned | Protégé-competitive OWL 2 DL + OBO in VS Code per [Protégé parity](../design/PROTEGE_PARITY.md). |

OntoCode v0.20 is **not** documented as a general-availability replacement for Protégé for every advanced OWL 2 DL workflow (e.g. full Manchester axiom coverage for all formats; RDF/XML and OWL/XML write-back planned **v0.21**).

## Approved use cases (pilot or production)

| Use case | v0.20 readiness | Notes |
|----------|----------------|-------|
| CI lint gate on ontology repos | **Suitable** | `ontocore validate` — [CI integration](../ci-integration.md) |
| CI consistency gate (EL profile) | **Suitable** | `ontocore classify --profile el` — profile must match ontology |
| CI consistency gate (DL profile) | **Pilot** | `ontocore classify --profile dl` or `auto` — OntoLogos 1.0.0; verify on your corpus |
| Developer IDE for Turtle/OBO authoring | **Pilot** | Turtle + OBO write-back; pre-1.0 extension APIs |
| Workspace refactoring (rename, migrate, move, extract) | **Pilot** | Turtle only; preview before apply — [Refactoring guide](refactoring.md) |
| Semantic diff in PR review | **Pilot** | `ontocore diff` + VS Code panel — [Semantic diff](../ontocode/semantic-diff.md) |
| Ontology browse/query in VS Code | **Pilot** | Local-first; multi-root supported — [enterprise deployment](enterprise-deployment.md) |
| Air-gapped VS Code install | **Pilot** | VSIX + SHA256 — [enterprise deployment](enterprise-deployment.md) |
| OBO index + write-back + ROBOT CLI in CI | **Pilot** | Index and edit `.obo`; `ontocore robot validate` — requires Java + `robot` on PATH — [ROBOT interop](robot-interop.md) |
| Replace Protégé for full OWL 2 DL engineering | **Not supported** | DL classification shipped; full axiom catalog for all formats remains v1.0; RDF/XML and OWL/XML write-back planned **v0.21** — [Protégé coexistence](protege-coexistence.md) |
| Org-wide mandatory IDE standard | **Defer** | Complete pilot + legal review first |

## Pilot criteria (recommended before wider rollout)

Complete these on a **representative** ontology project (not only tutorial fixtures):

1. **Functional fit** — Compare [Protégé parity](../design/PROTEGE_PARITY.md) against required axiom types and reasoning profile.
2. **Sizing** — Confirm workspace within [limits](../workspace-limits.md); run [production evidence protocol](production-evidence.md) on your corpus — [performance and sizing](performance-sizing.md).
3. **Security** — Platform review of [security policy](../security.md) and [enterprise deployment](enterprise-deployment.md) (LSP stdio, Restricted Mode, path jail).
4. **Legal** — Review LGPL (`horned-owl`) and third-party notices — [LGPL compliance](lgpl-compliance.md).
5. **CI proof** — `validate` and optional `classify` in a test pipeline on real branches.
6. **Coexistence** — If migrating from Protégé, follow [Protégé coexistence](protege-coexistence.md) split workflow for 1–2 release cycles.
7. **Exit criteria** — Define what would trigger rollback (e.g. unsatisfiable-class false positives, scale failures, DL axiom editing gaps).

Suggested pilot duration: **4–8 weeks** with 3–10 engineers on one ontology repository.

## What is stable enough for automation

| Surface | Stability (v0.20 tagged) |
|---------|-------------------|
| `ontocore validate` exit codes | Documented for CI |
| `ontocore classify` exit codes | Documented for CI |
| `ontocore diff` output | Documented for CI; pre-1.0 field names may evolve |
| `ontocore::Workspace` API | Stable since v0.10; other crates pre-1.0 |
| SQL virtual table column names | May change pre-1.0 |
| LSP `ontocore/*` JSON | May change pre-1.0 |
| Rust `ontocore-*` crate APIs | May change pre-1.0 |

Pin CLI version in CI: release binary with `VERSION=0.20.0` or `cargo install ontocore-cli --locked --version 0.20.0`.

## Support and incident response

| Topic | v0.20 policy |
|-------|-------------|
| Commercial support | **Not offered** — community / GitHub issues |
| Security reports | [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories/new) — not public issues |
| Acknowledgment target | Within a few business days ([SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md)) |
| Patch SLA | **No committed SLA** — track [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories) for your version |
| Supported versions | 0.19.x latest tagged; 0.20.x in progress ([security policy](../security.md)) |

Enterprises requiring contractual SLAs should treat OntoCode as **internal OSS adoption** with your own escalation path to maintainers via GitHub.

## Compliance questionnaires (honest answers)

| Question | Answer from documentation |
|----------|---------------------------|
| Is data sent to the vendor cloud? | **No** by default — local-first ([security](../security.md)) |
| SOC 2 / ISO 27001 certified? | **No** — not claimed |
| HIPAA BAA available? | **No** |
| Telemetry? | **None** shipped |
| Code-signed binaries? | **Not yet** — SHA256 only ([release integrity](../release-integrity.md)) |

## Reference architecture (pilot)

```text
Developers (VS Code + OntoCode VSIX)
        │
        ▼
  ontocore-lsp (stdio, local)
        │
        ▼
  Ontology workspace (.ttl primary, .obo index, .owl read-only)
        │
        ▼
  CI pipeline (ontocore validate / classify / robot)
        │
        ▼
  Optional: Protégé for DL review
```

## Related

- [Enterprise evaluation](enterprise-eval.md)
- [Production evidence protocol](production-evidence.md)
- [Enterprise deployment](enterprise-deployment.md)
- [Performance and sizing](performance-sizing.md)
- [LGPL compliance](lgpl-compliance.md)
