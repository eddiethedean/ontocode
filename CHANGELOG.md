# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-06-24

### Added

- **Simple write-back (v0.4a):** create/edit/delete entities; labels, comments, simple `SubClassOf` in Turtle
- New **`ontoindex-owl`** crate — Horned-OWL facade for axiom modeling and patch write-back
- LSP **`ontoindex/applyAxiomPatch`** — preview and apply patch operations
- CLI **`ontoindex patch`** — apply patches from JSON
- Editable **Entity Inspector** and explorer create/delete commands in VS Code
- **`EntityDetail.editable`** and `document_path` for authoring UI
- Oxigraph ↔ Horned-OWL **consistency tests** and `examples/protege-roundtrip/` fixtures
- [docs/authoring.md](docs/authoring.md)

### Changed

- Turtle catalog entities/axioms sourced from Horned-OWL when parse succeeds (dual stack per ADR-0013)
- Workspace MSRV bumped to **1.88** (Horned-OWL 1.4)
- Label strings in catalog normalized (no extra RDF literal quotes from Horned-OWL bridge)

[0.4.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.4.0

## [0.3.0] - 2026-06-23

### Added

- **Ontology diagnostics (v0.3):** parse errors, broken imports, undefined prefixes, duplicate/missing labels, orphan classes
- New `ontoindex-diagnostics` crate with catalog lint rules
- `diagnostics` SQL virtual table (`SELECT * FROM diagnostics`)
- LSP `textDocument/publishDiagnostics` after workspace reindex (VS Code Problems panel)
- Diagnostics explorer tree grouped by severity
- `ontoindex validate` prints all diagnostics; non-zero exit on errors
- Open-buffer parsing for inline diagnostics on unsaved edits

### Fixed

- Diagnostic file paths when entity `ontology_id` is an ontology IRI (not `doc-N`)
- LSP always responds to hover/definition/symbol requests (`null` when no result)
- LSP advertises `textDocumentSync` so unsaved-buffer diagnostics work in VS Code
- RDF/XML `xmlns` prefix extraction; fewer false `undefined_prefix` reports
- Orphan-class heuristic skips taxonomy roots with in-workspace children
- Import IRI normalization (trailing slash); stale Problems panel entries cleared after reindex

[0.3.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.3.0

## [0.2.3] - 2026-06-11

### Fixed

- **VS Code extension:** **Open Entity Inspector** and **Jump to Source** from the explorer context menu (tree items pass an object, not an IRI string)

### Changed

- Extension Marketplace README — step-by-step usage guide, troubleshooting, and preview image
- Marketplace listing description and search keywords

[0.2.3]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.3

## [0.2.2] - 2026-06-11

### Fixed

- **VS Code extension:** await `LanguageClient.start()` (v9 removed `onReady()` — startup was broken)
- **macOS:** clear `com.apple.quarantine` on bundled `ontoindex-lsp` when present

### Added

- Extension startup regression guards (`clientStartup.guard.test.ts`) — block `onReady()` and non-awaited `start()` from shipping again
- VS Code integration tests (`@vscode/test-electron`) and CI matrix across Ubuntu, macOS, and Windows on VS Code 1.85.0 and stable

[0.2.2]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.2

## [0.2.1] - 2026-06-11

### Fixed

- **VS Code extension:** set executable permission on bundled `ontoindex-lsp` before spawn (fixes `EACCES` on macOS/Linux after Marketplace or VSIX install)

### Added

- Extension e2e tests: simulate Marketplace `chmod 644` on bundled LSP and verify spawn after fix; CI VSIX unpack regression test

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
