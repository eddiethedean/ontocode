# Production readiness and pilot criteria

This page states what OntoCode / OntoIndex **v0.6.0** is appropriate for in production-like environments. It is not legal advice and does not replace your organization's risk review.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Maturity model

| Level | Version | Meaning |
|-------|---------|---------|
| **Pre-1.0** | **0.6.x (current)** | Active development. Library APIs, LSP JSON, and SQL virtual table columns may change between minor releases until [v1.0](../design/v1.0_BACKLOG.md). |
| **Stable CI gates** | 0.6.x | `ontoindex validate` and `ontoindex classify` exit codes are documented and intended for CI — see [workspace limits](../workspace-limits.md). |
| **v1.0 target** | Planned | Protégé-competitive OWL 2 DL + OBO in VS Code per [Protégé parity](../design/PROTEGE_PARITY.md). |

OntoCode v0.6 is **not** documented as a general-availability replacement for Protégé or full OWL 2 DL engineering.

## Approved use cases (pilot or production)

| Use case | v0.6 readiness | Notes |
|----------|----------------|-------|
| CI lint gate on ontology repos | **Suitable** | `ontoindex validate` — [CI integration](../ci-integration.md) |
| CI consistency gate (EL profile) | **Suitable** | `ontoindex classify --profile el` — profile must match ontology |
| Developer IDE for Turtle authoring | **Pilot** | Turtle write-back only; pre-1.0 extension APIs |
| Git-native ontology browse/query in VS Code | **Pilot** | Local-first; see [enterprise deployment](enterprise-deployment.md) |
| Air-gapped VS Code install | **Pilot** | VSIX + SHA256 — [enterprise deployment](enterprise-deployment.md) |
| Replace Protégé for OWL 2 DL | **Not supported** | Keep Protégé — [Protégé coexistence](protege-coexistence.md) |
| OBO / ROBOT biomedical workflows | **Not supported** | Planned v0.7b |
| Org-wide mandatory IDE standard | **Defer** | Complete pilot + legal review first |

## Pilot criteria (recommended before wider rollout)

Complete these on a **representative** ontology project (not only tutorial fixtures):

1. **Functional fit** — Compare [Protégé parity](../design/PROTEGE_PARITY.md) against required axiom types and reasoning profile.
2. **Sizing** — Confirm workspace within [limits](../workspace-limits.md); run timing on your corpus — [performance and sizing](performance-sizing.md).
3. **Security** — Platform review of [security policy](../security.md) and [enterprise deployment](enterprise-deployment.md) (LSP stdio, Restricted Mode, path jail).
4. **Legal** — Review LGPL (`horned-owl`) and third-party notices — [LGPL compliance](lgpl-compliance.md).
5. **CI proof** — `validate` and optional `classify` in a test pipeline on real branches.
6. **Coexistence** — If migrating from Protégé, follow [Protégé coexistence](protege-coexistence.md) split workflow for 1–2 release cycles.
7. **Exit criteria** — Define what would trigger rollback (e.g. unsatisfiable-class false positives, scale failures, DL axiom editing gaps).

Suggested pilot duration: **4–8 weeks** with 3–10 engineers on one ontology repository.

## What is stable enough for automation

| Surface | Stability (v0.6) |
|---------|------------------|
| `ontoindex validate` exit codes | Documented for CI |
| `ontoindex classify` exit codes | Documented for CI |
| SQL virtual table column names | May change pre-1.0 |
| LSP `ontoindex/*` JSON | May change pre-1.0 |
| Rust `ontoindex-*` crate APIs | May change pre-1.0 |

Pin CLI version in CI: release binary with `VERSION=0.6.0` or `cargo install ontoindex-cli --locked --version 0.6.0`.

## Support and incident response

| Topic | v0.6 policy |
|-------|-------------|
| Commercial support | **Not offered** — community / GitHub issues |
| Security reports | [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories/new) — not public issues |
| Acknowledgment target | Within a few business days ([SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md)) |
| Patch SLA | **No committed SLA** — track [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories) for your version |
| Supported versions | 0.6.x, 0.5.x ([security policy](../security.md)) |

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
  ontoindex-lsp (stdio, local)
        │
        ▼
  Git repo (.ttl primary, .owl read-only)
        │
        ▼
  CI pipeline (ontoindex validate / classify)
        │
        ▼
  Optional: Protégé for DL/OBO review
```

## Related

- [Enterprise evaluation](enterprise-eval.md)
- [Enterprise deployment](enterprise-deployment.md)
- [Performance and sizing](performance-sizing.md)
- [LGPL compliance](lgpl-compliance.md)
