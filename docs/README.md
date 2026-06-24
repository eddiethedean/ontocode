# Documentation index

**Browse online:** [ontocode.readthedocs.io](https://ontocode.readthedocs.io/)

## Start here

| I want to… | Read |
|------------|------|
| Install VS Code extension | [vscode-install.md](vscode-install.md) |
| Run CLI queries in 5 minutes | [getting-started.md](getting-started.md) |
| Edit Turtle ontologies | [authoring.md](authoring.md) |
| Automate edits (patch JSON) | [patch-reference.md](patch-reference.md) |
| Integrate with CI | [ci-integration.md](ci-integration.md) |
| SQL virtual tables | [sql-reference.md](sql-reference.md) |
| SPARQL queries | [sparql-reference.md](sparql-reference.md) |
| LSP / extension integration | [lsp-api.md](lsp-api.md) |
| Troubleshooting & FAQ | [faq.md](faq.md) |
| Workspace limits & validate exit codes | [workspace-limits.md](workspace-limits.md) |
| Verify release downloads | [release-integrity.md](release-integrity.md) |
| Security | [security.md](security.md) |
| Contribute / build from source | [contributing.md](contributing.md) |

**Shipped in v0.4:** All guides above except where noted as future in design specs.

## Reference

| Path | Audience | Content |
|------|----------|---------|
| [lsp-api.md](lsp-api.md) | Extension / LSP integrators | **Shipped v0.4** LSP methods and wire format |
| [sql-reference.md](sql-reference.md) | CLI / query users | **Shipped v0.4** virtual SQL tables |
| [sparql-reference.md](sparql-reference.md) | CLI / query users | **Shipped v0.4** SPARQL over indexed triples |
| [patch-reference.md](patch-reference.md) | CLI / automation | **Shipped v0.4** Turtle patch JSON |
| [authoring.md](authoring.md) | VS Code / CLI users | **Shipped v0.4** write-back workflow |
| [vscode-install.md](vscode-install.md) | VS Code users | Install VSIX, troubleshooting |
| [getting-started.md](getting-started.md) | New users | 5-minute CLI and VS Code paths |
| [ci-integration.md](ci-integration.md) | DevOps | `validate` in CI pipelines |
| [faq.md](faq.md) | All users | Common questions and troubleshooting |
| [releasing.md](releasing.md) | Maintainers | crates.io and GitHub release process |

## Shipped vs planned

- **Trust for v0.4 behavior:** `docs/lsp-api.md`, `docs/sql-reference.md`, `docs/authoring.md`, `docs/patch-reference.md`, crate `lib.rs` module docs, and tests.
- **Target / future:** `docs/design/SPEC.md`, `docs/design/LSP_SPEC.md`, `docs/design/ROADMAP.md` — read banners; many features are not built yet.
- **v1.0 bar:** all P0 items in [design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) must be green before release.

## Design specs (contributors / planners)

Product vision, roadmap, ADRs, and target architecture live in [design/README.md](design/README.md). These describe **future** behavior as well as shipped features — do not treat them as user manuals.
