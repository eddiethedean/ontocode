# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.18.x  | Yes       |
| 0.16.x  | Yes       |
| 0.15.x  | Yes       |
| 0.14.x  | Best effort |
| 0.13.x  | Best effort |
| 0.11.x  | No        |
| 0.10.x  | Best effort |
| ≤ 0.9.x | No        |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Report security issues privately by opening a [GitHub Security Advisory](https://github.com/eddiethedean/ontocode/security/advisories/new) or contacting the repository maintainers through GitHub.

Include:

- Description of the issue and potential impact
- Steps to reproduce (if applicable)
- Affected version(s)

We aim to acknowledge reports within a few business days.

## Threat model

OntoCore and OntoCode are **local-first** tools: they index and parse files on disk and do not upload ontology content by default.

### Intended deployment

- `ontocore` CLI run by the operator on paths they choose
- `ontocore-lsp` connected over **stdio** to a trusted editor (VS Code + OntoCode extension)

### Do not expose raw LSP to the internet

`ontocore-lsp` has **no authentication, authorization, or rate limiting**. If the language server is reachable over a network (TCP proxy, shared container socket, misconfigured debug transport), an attacker can:

- Request reads of arbitrary files via LSP document URIs (mitigated in v0.2+ by workspace jail when a workspace is indexed)
- Trigger indexing of directories outside the intended project via `ontocore/indexWorkspace` (mitigated by workspace scope validation)
- Exhaust CPU/memory with large ontologies or expensive queries (partially mitigated by resource limits — see below)

**Never bind `ontocore-lsp` to a public interface without an authenticated reverse proxy and strict path sandboxing.**

## Hardening in v0.2+ (extended in v0.3, v0.4 write-back)

| Control | Where |
|---------|--------|
| Workspace path jail (LSP document reads) | `ontocore-core::path_jail`, `ontocore-lsp` handlers |
| `indexWorkspace` scope validation | `ontocore-lsp` state |
| File size / file count / triple / entity limits | `ontocore-core::limits`, scanner, parser, catalog |
| SQL/SPARQL query size and result row caps | `ontocore-query` |
| Symlink skip + `follow_links(false)` in scanner | `ontocore-core::scanner` |
| Markdown escaping in LSP hover | `ontocore-lsp` handlers |
| `ontocode.lspPath` ignored in VS Code Restricted Mode | OntoCode extension |

See [docs/release-integrity.md](docs/release-integrity.md) for verifying release binaries.

## Scope

Reports in scope include:

- Path traversal or arbitrary file read via LSP or indexing
- Remote code execution via extension language-server path settings
- Denial of service via malicious ontology files or queries
- Supply-chain issues in release artifacts or dependencies

Ontology **content** may contain sensitive IRIs or literals; treat indexed catalogs as confidential when workspaces contain private data.

## Safe harbor

We appreciate responsible disclosure and will work with reporters to understand and address valid issues.
