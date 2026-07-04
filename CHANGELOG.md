# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.0] - 2026-07-04

### Added

- **Incremental workspace indexing** — content-hash reuse in `ontocore-catalog`; LSP debounced reindex avoids reparsing unchanged files
- **Multi-root LSP workspaces** — all VS Code folders indexed; `path_jail` and `didChangeWorkspaceFolders` support
- **Stable `ontocore::Workspace` API** — `open_with_options`, `reindex` / `reindex_incremental`, `import_graph`, `diff`, `stats`
- **`ontocore-diff` crate** — catalog semantic diff, breaking-change heuristics, git ref compare
- **`ontocore diff` CLI** — text/json/markdown output; directory and git range modes
- **LSP `ontocore/semanticDiff`** and VS Code **Semantic Diff** React panel
- **Optional disk cache** — `.ontocore/cache/` keyed by content hash (`ontocode.indexCache` / `WorkspaceOptions::disk_cache`)
- Migration guide [docs/migration/v0.10.md](docs/migration/v0.10.md); example `semantic_diff`

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.10.0**
- Extension marketplace version **0.10.0**

## [0.9.0] - 2026-07-03

### Added

- **`ontocore` crate** — public façade with `Workspace::open`, module re-exports, and `lsp` feature
- **OntoCore documentation** — `docs/ontocore/` and `docs/ontocode/` trees; ADR-0018; platform architecture (`VISION.md`, `ARCHITECTURE.md`, `ROADMAP.md`)
- Example `ontocore_workspace` using `Workspace` API
- Diagnostic codes `owl_bridge_failed` and `io_read_error`
- Release pipeline publishes `ontocore` façade; extended `check-doc-versions.sh`
- **OntoLogos 1.0.0** integration — real `dl` and `auto` reasoner adapters (`ontocore-reasoner`)
- DL/auto classification tests (library, CLI, LSP) and reasoner panel enablement in VS Code extension
- Plugin platform design — [PLUGIN_SPEC.md](docs/design/PLUGIN_SPEC.md) with build/workflow/release plugin categories; [owlmake](https://github.com/INCATools/owlmake) as reference external workflow plugin
- OBO/ROBOT spec — owlmake integration path and ODK workflow goals ([OBO_ROBOT_SPEC.md](docs/design/OBO_ROBOT_SPEC.md))

### Changed

- **Breaking:** rename all `ontoindex-*` crates to **`ontocore-*`**
- **Breaking:** CLI binary `ontoindex` → **`ontocore`** (`ontocore-cli` crate)
- **Breaking:** LSP binary `ontoindex-lsp` → **`ontocore-lsp`**; custom methods `ontoindex/*` → **`ontocore/*`**
- **Breaking:** `OntoIndexError` → **`OntoCoreError`**
- **OntoCore** platform branding across README, docs, extension output channel, and GitHub templates
- `apply_refactor_plan` requires `workspace_root`; diagnostic engine surfaces IO read failures
- Horned-OWL bridge failures emit catalog diagnostics instead of silent fallback
- OntoLogos workspace dependencies bumped from 0.9.0 → **1.0.0**
- Enterprise adoption docs reconciled with shipped DL/auto classification capability
- Extension marketplace metadata — OntoCore-powered description and expanded keywords
- `ontocore` crate README repositioned as public façade API

### Fixed

- LSP reasoner integration test updated for shipped DL/auto profiles
- MkDocs strict-mode documentation link fixes (ADR rename, concepts, contributing)
- Release packaging: license files for `ontocore` and `ontocore-robot`; crate READMEs and include lists for crates.io; release dry-run only on leaf crates

### Notes

- See [migration/v0.9.md](docs/migration/v0.9.md) for upgrade steps from v0.8
- `Workspace` API remains experimental until v0.10
- First crates.io publish under `ontocore-*` names (prior releases used `ontoindex-*`)

## [0.8.0] - 2026-06-26

### Added

- **`ontoindex-refactor` crate** — workspace-wide usage index; rename IRI, namespace migration, move entity, extract module with preview/apply
- CLI **`ontoindex refactor`** subcommands: `usages`, `rename`, `migrate-namespace`, `move`, `extract`
- LSP refactoring: `ontoindex/findUsages`, `ontoindex/previewRefactor`, `ontoindex/applyRefactor`
- Standard LSP **`textDocument/references`**, **`textDocument/rename`** (with `prepareRename`)
- VS Code refactor commands and **Refactor Preview** React panel
- Inspector **Find Usages** and **Rename IRI** actions
- Full Manchester catalog extensions: **disjoint classes** (author + patch), **domain/range** and **property chains** (view in axiom catalog)
- Patch ops: `add_disjoint_class`, `remove_disjoint_class`
- **React Query Workbench** and **React Manchester Editor** panels (legacy HTML webviews removed)
- Fixture: `fixtures/disjoint-classes.ttl`

### Changed

- Workspace and extension version **0.8.0**
- Axiom catalog groups axioms by kind in React inspector
- Manchester editor supports `disjoint_class` axiom kind with validation UI

### Fixed

- Query Workbench dropped successful results (runId stale-guard never updated)
- Namespace migration overwrote per-IRI renames when updating `@prefix` declarations
- Multi-entity extract module used stale byte offsets in the same file
- LSP rename/references: prefixed rename targets, error reporting, and reference range width
- Explorer refreshed before refactor apply; disjoint axiom edit now passes `other_iri`
- Manchester editor: restored data property/datatype pickers; panel CSS for v0.8 React panels
- EL `classify` false negatives on unsatisfiable ontologies (reasoner ontology merge via triple bridge)
- Extract/move to new files failed path validation when target file did not exist yet
- LSP axiom patch uses atomic disk writes; buffer updated before disk
- Refactor rollback errors propagated when restore fails mid-apply
- Reasoner panel runId synchronization between host and webview
- RL/RDFS profiles report unsatisfiable classes (EL post-check)
- Catalog indexes orphan LSP buffer overrides not returned by workspace scan
- LSP patch/refactor require indexed catalog; `APPLIED_NOT_INDEXED` when reindex fails after apply
- SPARQL update guard bypass after `PREFIX` or comment lines
- Capped file reads in parser, catalog semantics, and refactor preview/backup paths

[0.9.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.9.0
[0.8.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.8.0

## [0.7.0] - 2026-06-25

### Added

- **React webview foundation** (`extension/webview-ui/`) — Vite + React, typed message protocol, CSP panel host
- **Graph visualization** — class, property, import, and neighborhood graphs via LSP `ontoindex/getGraph` and React `@xyflow/react` panels
- **React Entity Inspector** — migrated from legacy HTML webview with edit/patch parity
- **OBO format** — `.obo` scanning, parsing, `obo_id` in catalog/SQL, explorer labels
- **`ontoindex-robot` crate** — ROBOT CLI wrappers; CLI `ontoindex robot validate|merge|report`; LSP `ontoindex/runRobot`
- Extension commands: `openClassGraph`, `openPropertyGraph`, `openImportGraph`, `openNeighborhoodGraph`, `openGraph`
- OBO TextMate grammar, `ontocode.robotPath` setting, `examples/obo-workflow/`
- Docs: [webview-protocol.md](docs/webview-protocol.md)

### Changed

- Workspace and extension version **0.7.0**
- Entity inspector and graph panels use React bundle in VSIX

### Fixed

- Webview ready/init races (graph, reasoner, inspector panels buffer messages until `ready`)
- LSP `effective_index_root()` for consistent reindex and patch paths
- `applyAxiomPatch` result contract (`reindex_warning`, disk-before-buffer write, diagnostics on partial apply)
- Reasoner vs index worker serialization; `getExplanation` content-hash cache staleness
- Patch engine: safer removal, subject-boundary entity detection, batch preview on failure
- Manchester: unclosed IRI errors, unknown-prefix validation
- OBO files no longer marked editable in catalog; OBO file size cap enforced
- Diagnostics: orphan import roots, undefined-prefix false positives
- SQL/SPARQL: reject top-level bare `WHERE` columns and SPARQL UPDATE forms
- Extension: multi-root folder picker, `obo`/`json-ld` document selectors, diagnostic UTF-16 columns, hierarchy `hasChildren`

[0.7.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.7.0

## [0.6.0] - 2026-06-24

### Added

- **`ontoindex-reasoner` crate** — thin OntoLogos 0.9.0 facade (`el`, `rl`, `rdfs` adapters; `dl`/`auto` stubbed until OntoLogos 1.0)
- CLI **`ontocore classify`** and **`ontocore explain`**
- LSP **`ontoindex/runReasoner`** and **`ontoindex/getExplanation`**
- VS Code **Reasoner Results** panel, **Explanation** panel, hierarchy mode toggle (`asserted` / `inferred` / `combined`)
- Settings: `ontocode.reasoner.default`, `ontocode.reasoner.autoProfile`, `ontocode.hierarchy.mode`
- Fixtures: `fixtures/reasoner-el.ttl`, `fixtures/reasoner-unsat.ttl`
- User guide: [docs/guides/reasoner.md](docs/guides/reasoner.md)
- **Enterprise documentation pack** — production-readiness, enterprise-deployment, performance-sizing, LGPL compliance guides
- **Documentation adoption** — concepts, best-practices, Protégé coexistence, Rust library guide, migration index, crate READMEs
- **`read_file_capped`** / **`parse_boolean_literal`** helpers in `ontoindex-core`

### Changed

- `CatalogSnapshot` includes optional `reasoner` metadata after classification
- Workspace crates bumped to **0.6.0**
- SQL JSON export uses column-ordered row arrays
- Inspector webview loads entity data via `postMessage` (no inline `<script>` embedding)

### Fixed

- **Turtle patch write-back:** multi-statement subject blocks, CRLF byte spans, predicate-object removal (not line deletion), literal-safe separator cleanup
- **Resource limits:** capped file reads (scanner, LSP disk fallback, patch apply); `MAX_ENTITIES` fail-fast during catalog build; filesystem walk entry cap; index worker job coalescing
- **`ontology_id` mismatch** between entities and axioms/annotations when `owl:Ontology` is declared
- **SQL:** row cap during iteration (not after full materialization); `SELECT col AS alias` projection
- **`owl:deprecated` false positives** from substring `.contains("true")`
- **Extension:** inspector/query/reasoner stale-async guards; stricter LSP protocol validation

[0.6.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.6.0

## [0.5.0] - 2026-06-24

### Added

- **Query workbench (VS Code):** SQL and SPARQL modes, result table, CSV/JSON export, saved queries, query history, starter templates
- LSP **`ontoindex/query`** and **`ontoindex/sparql`** — tabular results against indexed workspace catalog
- **Manchester MVP editor (VS Code):** complex `SubClassOf` and `EquivalentClasses` authoring with validate, expression tree, Turtle preview
- LSP **`ontoindex/parseManchester`** — parse/validate Manchester expressions with catalog-based completion lists
- **`ontoindex-owl` Manchester module** — parse, serialize, Turtle fragment generation, expression tree JSON
- New patch ops: `add_complex_sub_class_of`, `remove_complex_sub_class_of`, `add_equivalent_class`, `remove_equivalent_class`, `set_equivalent_class`
- **`EntityDetail.axioms`** — structured `EntityAxiomSummary` rows (kind, display, manchester, parent_iri, editable)
- `fixtures/complex-classes.ttl` for Manchester and consistency tests
- Extension integration tests for SQL, SPARQL, Manchester parse, and structured axioms

### Changed

- Inspector shows **Edit in Manchester** for complex axioms and **Add Manchester axiom**
- README capability table: SQL + SPARQL in VS Code → **Yes**

### Fixed

- **Turtle span/patch engine:** structural subject/block detection; complex axiom removal via blank-node spans; transactional patch apply; Turtle literal escaping
- **Manchester:** `and` / `or` serialization emits full operand lists; SubClassOf edit uses remove+add (no duplicate axioms)
- **LSP:** patches apply to open document buffer before disk write; `APPLIED_NOT_INDEXED` when reindex fails after apply
- **Query layer:** SPARQL truncates at row cap instead of hard-failing; SQL filter errors propagate; correct `truncated` flag at exactly 100k rows
- **Extension:** query result table uses safe DOM APIs (XSS); validate/run sequence IDs ignore stale responses; `@prefix` fallback when catalog namespaces are unavailable

[0.5.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.5.0

## [0.4.0] - 2026-06-24

### Added

- **Simple write-back (v0.4a):** create/edit/delete entities; labels, comments, simple `SubClassOf` in Turtle
- New **`ontoindex-owl`** crate — Horned-OWL facade for axiom modeling and patch write-back
- LSP **`ontoindex/applyAxiomPatch`** — preview and apply patch operations
- CLI **`ontocore patch`** — apply patches from JSON
- Editable **Entity Inspector** and explorer create/delete commands in VS Code
- **`EntityDetail.editable`** and `document_path` for authoring UI
- Oxigraph ↔ Horned-OWL **consistency tests** and `examples/protege-roundtrip/` fixtures
- [docs/authoring.md](docs/authoring.md)

### Changed

- Turtle catalog entities/axioms sourced from Horned-OWL when parse succeeds (dual stack per ADR-0013)
- Workspace MSRV bumped to **1.88** (Horned-OWL 1.4)
- Label strings in catalog normalized (no extra RDF literal quotes from Horned-OWL bridge)
- Read the Docs site, first-success tutorial, errors reference, and enterprise evaluation guide

[0.4.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.4.0

## [0.3.0] - 2026-06-23

### Added

- **Ontology diagnostics (v0.3):** parse errors, broken imports, undefined prefixes, duplicate/missing labels, orphan classes
- New `ontoindex-diagnostics` crate with catalog lint rules
- `diagnostics` SQL virtual table (`SELECT * FROM diagnostics`)
- LSP `textDocument/publishDiagnostics` after workspace reindex (VS Code Problems panel)
- Diagnostics explorer tree grouped by severity
- `ontocore validate` prints all diagnostics; non-zero exit on errors
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

- **OntoCore foundation** — Rust workspace for local-first ontology indexing
- `ontoindex-core` — workspace scanner, shared types, content hashing
- `ontoindex-parser` — RDF/OWL parsing and entity extraction via Oxigraph
- `ontoindex-catalog` — semantic catalog and triple store
- `ontoindex-query` — SQL-like virtual tables and SPARQL queries
- `ontoindex-cli` — `ontoindex` binary with `index`, `query`, `sparql`, `validate`, and `inspect` commands
- Fixture ontology and integration/golden snapshot tests
- CI and crates.io release workflows

[0.1.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.1.0
