# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-06-11

### Fixed

- **VS Code extension:** set executable permission on bundled `ontoindex-lsp` before spawn (fixes `EACCES` on macOS/Linux after Marketplace or VSIX install)

[0.2.1]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.1

## [0.2.0] - 2026-06-11

### Added

- **OntoCode Explorer** — VS Code extension (`extension/`) with ontology tree views and entity inspector
- `ontoindex-lsp` — language server with custom methods (`indexWorkspace`, `getCatalogSnapshot`, `getEntity`)
- LSP browsing aids — hover, document/workspace symbols, go-to-definition, debounced re-index on file changes
- Catalog entity APIs — hierarchy, entity detail, and jump-to-source helpers in `ontoindex-catalog`
- LSP smoke integration test and CI jobs for LSP + extension builds
- Release workflow assets for `ontoindex-lsp` binary and multi-platform extension VSIX (Linux, macOS, Windows)
- User docs: `docs/lsp-api.md`, `docs/vscode-install.md`, `docs/release-integrity.md`
- Design docs under `docs/design/` including v1.0 Protégé parity matrix and Rust-native reasoner strategy (ADR-0014)

### Fixed

- LSP JSON wire format uses snake_case enums (`class`, `ok`, …) — aligned with extension, tests, and `docs/lsp-api.md` (ADR-0012)
- Workspace reindex debouncing and notifications when open documents change
- Jump-to-source for prefixed Turtle entity names
- Explorer shows classes whose parents are declared in another ontology file
- Structured `LspErrorPayload` for custom LSP error responses

[0.2.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.0

## [0.1.0] - 2026-06-10

### Added

- **OntoIndex foundation** — Rust workspace for local-first ontology indexing
- `ontoindex-core` — workspace scanner, shared types, content hashing
- `ontoindex-parser` — RDF/OWL parsing and entity extraction via Oxigraph
- `ontoindex-catalog` — semantic catalog and triple store
- `ontoindex-query` — SQL-like virtual tables and SPARQL queries
- `ontoindex-cli` — `ontoindex` binary with `index`, `query`, `sparql`, `validate`, and `inspect` commands
- Fixture ontology and integration/golden snapshot tests
- CI and crates.io release workflows

[0.1.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.1.0
