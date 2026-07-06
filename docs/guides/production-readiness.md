# Production readiness and pilot criteria

This page states what OntoCode / OntoCore **v0.11.1** is appropriate for in production-like environments. It is not legal advice and does not replace your organization's risk review.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Maturity model

| Level | Version | Meaning |
|-------|---------|---------|
| **Pre-1.0** | **0.11.x (current)** | Active development. Library APIs, LSP JSON, and SQL virtual table columns may change between minor releases until [v1.0](../design/v1.0_BACKLOG.md). |
| **Stable CI gates** | 0.11.x | `ontocore validate`, `ontocore classify`, and `ontocore diff` are documented for CI — see [workspace limits](../workspace-limits.md). |
| **v1.0 target** | Planned | Protégé-competitive OWL 2 DL + OBO in VS Code per [Protégé parity](../design/PROTEGE_PARITY.md). |

OntoCode v0.11 is **not** documented as a general-availability replacement for Protégé or full OWL 2 DL axiom engineering (property chains, full catalog, OBO write-back).

## Approved use cases (pilot or production)

| Use case | v0.11 readiness | Notes |
|----------|----------------|-------|
| CI lint gate on ontology repos | **Suitable** | `ontocore validate` — [CI integration](../ci-integration.md) |
| CI consistency gate (EL profile) | **Suitable** | `ontocore classify --profile el` — profile must match ontology |
| CI consistency gate (DL profile) | **Pilot** | `ontocore classify --profile dl` or `auto` — OntoLogos 1.0.0; verify on your corpus |
| Developer IDE for Turtle authoring | **Pilot** | Turtle write-back only; pre-1.0 extension APIs |
| Workspace refactoring (rename, migrate, move, extract) | **Pilot** | Turtle only; preview before apply — [Refactoring guide](refactoring.md) |
| Semantic diff in PR review | **Pilot** | `ontocore diff` + VS Code panel — [Semantic diff](../ontocode/semantic-diff.md) |
| Ontology browse/query in VS Code with Git | **Pilot** | Local-first; multi-root supported — [enterprise deployment](enterprise-deployment.md) |
| Air-gapped VS Code install | **Pilot** | VSIX + SHA256 — [enterprise deployment](enterprise-deployment.md) |
| OBO index + ROBOT CLI in CI | **Pilot** | Index `.obo`; `ontocore robot validate` — requires Java + `robot` on PATH — [ROBOT interop](robot-interop.md) |
| Replace Protégé for full OWL 2 DL engineering | **Not supported** | DL classification shipped; full axiom catalog, chains, OBO write-back remain v1.0 — [Protégé coexistence](protege-coexistence.md) |
| Full OBO write-back in VS Code | **Not supported** | OBO is read-only in inspector; Turtle write-back only |
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

| Surface | Stability (v0.11) |
|---------|-------------------|
| `ontocore validate` exit codes | Documented for CI |
| `ontocore classify` exit codes | Documented for CI |
| `ontocore diff` output | Documented for CI; pre-1.0 field names may evolve |
| `ontocore::Workspace` API | Stable since v0.10; other crates pre-1.0 |
| SQL virtual table column names | May change pre-1.0 |
| LSP `ontocore/*` JSON | May change pre-1.0 |
| Rust `ontocore-*` crate APIs | May change pre-1.0 |

Pin CLI version in CI: release binary with `VERSION=0.11.1` or `cargo install ontocore-cli --locked --version 0.11.1`.

## Support and incident response

| Topic | v0.9 policy |
|-------|-------------|
| Commercial support | **Not offered** — community / GitHub issues |
| Security reports | [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories/new) — not public issues |
| Acknowledgment target | Within a few business days ([SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md)) |
| Patch SLA | **No committed SLA** — track [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories) for your version |
| Supported versions | 0.11.x ([security policy](../security.md)) |

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
  Git repo (.ttl primary, .obo index, .owl read-only)
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
