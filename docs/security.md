# Security policy

OntoCore and OntoCode are **local-first** tools: they index and parse files on disk and do not upload ontology content by default.

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.14.x   | Yes       |
| 0.13.x   | Best effort |
| 0.12.x  | Best effort |
| 0.11.x  | No        |
| 0.10.x  | Best effort |
| ≤ 0.9.x | No        |

Full policy: **[SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md)**

## Threat model summary

- **`ontocore-lsp` has no authentication.** Treat it like any local dev server — do not expose it to the internet or untrusted networks.
- **Workspace path jail:** The language server operates on the opened workspace folder. Custom `document_uri` values in patch requests must resolve within the workspace.
- **Resource limits:** File count, size, entity, and triple caps reduce DoS risk when opening untrusted repositories — see [workspace-limits.md](workspace-limits.md).
- **VS Code Restricted Mode:** `ontocode.lspPath` is **ignored** in untrusted workspaces; the bundled server is used instead.

## Reporting vulnerabilities

Report via [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories/new) — **not** public GitHub issues.

The canonical policy (supported versions, scope, hardening table) is maintained in the repository:

**[SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md)**

## Quick hardening checklist

| Control | Recommendation |
|---------|----------------|
| LSP exposure | Local stdio only; never port-forward `ontocore-lsp` |
| Custom LSP binary | Set `ontocode.lspPath` only in **trusted** workspaces |
| Release artifacts | Verify `SHA256SUMS` from official [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) — [release-integrity.md](release-integrity.md) |
| CI validation | Use `ontocore validate` to gate merges — [ci-integration.md](ci-integration.md) |
| Dependency audit | `cargo audit` runs in project CI (config: [`.cargo/audit.toml`](https://github.com/eddiethedean/ontocode/blob/main/.cargo/audit.toml)) |

## Documented `cargo audit` exceptions

CI runs `cargo audit` on every pull request. Some **transitive** advisories are documented in [`.cargo/audit.toml`](https://github.com/eddiethedean/ontocode/blob/main/.cargo/audit.toml) when they cannot be upgraded without breaking semver pins across the RDF/OWL stack:

| Advisory | Crate (transitive) | Why ignored | Review |
|----------|-------------------|-------------|--------|
| RUSTSEC-2026-0194 | `quick-xml` | DoS in unused XML paths via horned-owl / oxigraph | 2026-10-01 |
| RUSTSEC-2026-0195 | `quick-xml` | Same dependency graph | 2026-10-01 |

**Not ignored:** advisories on direct dependencies are fixed in-tree (for example `crossbeam-epoch` bumps). Enterprise questionnaires: cite this section and [release integrity](release-integrity.md).

`git2` (semantic diff) links `libgit2` — see [LGPL / licenses](guides/lgpl-compliance.md) and [DEPENDENCY_MATRIX](design/DEPENDENCY_MATRIX.md) for linkage notes.

## Enterprise evaluation

Procurement-oriented summary: [enterprise evaluation guide](guides/enterprise-eval.md) · [production readiness](guides/production-readiness.md) · [LGPL compliance](guides/lgpl-compliance.md)

## Related

- [FAQ — security and LGPL](faq.md)
- [Errors reference](errors.md)
- [LICENSES.md](design/LICENSES.md) — third-party licenses including LGPL (`horned-owl`)
