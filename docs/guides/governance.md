# Project governance and sustainability

What enterprise evaluators can determine from **published documentation and repository policy**. This is not a commercial vendor statement.

## Project model

| Aspect | Status |
|--------|--------|
| **Product** | Open-source OntoCode (VS Code IDE) + OntoCore (Rust engine) |
| **License** | MIT OR Apache-2.0 (application crates); third-party licenses in [LICENSES.md](../design/LICENSES.md) |
| **Distribution** | GitHub Releases (VSIX, CLI, LSP), [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode), [crates.io](https://crates.io/search?q=ontocore) |
| **Commercial support** | **Not offered** — community via [GitHub issues](https://github.com/eddiethedean/ontocode/issues) |
| **Vendor / company** | Not documented as a separate legal entity |

Fortune 500 teams should plan **internal OSS adoption** with their own escalation path and pinned versions.

## Release cadence (observed)

Recent documented releases (see [changelog](../changelog.md)):

| Version | Date (changelog) |
|---------|------------------|
| 0.18.0 | 2026-07-09 |
| 0.15.0 | 2026-07-08 |
| 0.14.0 | 2026-07-09 |
| 0.13.0 | 2026-07-08 |
| 0.12.0 | 2026-07-06 |
| 0.11.3 | 2026-07-06 |
| 0.11.2 | 2026-07-06 |
| 0.11.1 | 2026-07-06 |
| 0.11.0 | 2026-07-05 |
| 0.10.0 | 2026-07-04 |
| 0.9.0 | 2026-07-03 |
| 0.8.0 | 2026-06-26 |
| 0.7.0 | 2026-06-25 |
| 0.6.0 | 2026-06-24 |

Pre-1.0 releases may ship frequently. **No committed future cadence** is documented.

Maintainers follow [releasing.md](../releasing.md): version bump, CHANGELOG, SHIPPED matrix, `mkdocs build --strict`, `./scripts/check-doc-versions.sh`, GitHub Release artifacts with `SHA256SUMS` and `NOTICES`.

## Version support policy

| Stream | Security support (documented) |
|--------|-------------------------------|
| **0.18.x** | Yes — current release |
| **0.14.x** | Best effort |
| **0.11.x** | No |
| **0.10.x** | Best effort |
| **≤ 0.9.x** | No |

Pin versions in CI and desktop rollouts; do not assume automatic long-term backports. Canonical table: [security policy](../security.md).

## Security response

- Report via [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories/new) — not public issues
- Acknowledgment target: within a few business days ([SECURITY.md](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md))
- **No published SLA** for patch delivery
- Historical advisories: check the repository **Security** tab (not summarized in docs)

Supply chain: `cargo audit` in CI; release integrity via SHA256 — [release integrity](../release-integrity.md). Code signing: **not shipped**.

## Quality gates (documented)

| Gate | Where documented |
|------|------------------|
| Rust CI (fmt, clippy, tests) | README, [contributing.md](../contributing.md) |
| Extension tests + VS Code E2E | README, contributing |
| MkDocs strict build | [releasing.md](../releasing.md) |
| Doc version sync | `./scripts/check-doc-versions.sh` |

## Roadmap governance

- **Target specs** live under Contributing → Design (may describe future behavior)
- **Shipped behavior** is canonical in [SHIPPED.md](../SHIPPED.md)
- **v1.0** is a product goal, not a committed date — [Release timeline (non-commitment)](release-timeline.md)

## Contributing

Community contributions welcome — [contributing.md](../contributing.md). No documented contributor license agreement beyond standard GitHub inbound licensing.

## Enterprise implications

| Question | Documented answer |
|----------|-------------------|
| Bus factor / team size | Not documented |
| Funding model | Not documented |
| Paid enterprise tier | Not offered |
| Partner program | Not documented |
| SOC 2 / ISO | Not claimed — [production readiness](production-readiness.md) |

## Related

- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
- [Release timeline](release-timeline.md)
- [Security policy](../security.md)
