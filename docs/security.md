# Security policy

OntoIndex and OntoCode are **local-first** tools: they index and parse files on disk and do not upload ontology content by default.

The canonical security policy (supported versions, reporting, threat model) is maintained in the repository:

**[SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md)**

## Quick summary

- Report vulnerabilities via [GitHub Security Advisories](https://github.com/eddiethedean/ontocode/security/advisories/new) — not public issues.
- **Do not** expose `ontoindex-lsp` to the internet without authentication.
- `ontocode.lspPath` is ignored in VS Code Restricted Mode.
- Release artifact verification: [release-integrity.md](release-integrity.md).
