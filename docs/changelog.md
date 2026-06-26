# Changelog

Canonical source: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) (update both on release).

Migration guides: [Migration index](migration/README.md)

## [0.8.0] - 2026-06-26

### Added

- `ontoindex-refactor` crate; CLI `ontoindex refactor`; LSP refactor methods and standard rename/references
- React Query Workbench and Manchester Editor; Refactor Preview panel; disjoint class authoring

### Changed

- Workspace and extension version **0.8.0**

See [GitHub CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for the full v0.8.0 list.

## [0.7.0] - 2026-06-25

### Added

- React Entity Inspector and graph visualization panels
- OBO format index, ROBOT CLI wrappers, graph LSP method

### Changed

- Workspace and extension version **0.7.0**

### Fixed

- Webview ready/init races, LSP index root consistency, patch result contract, reasoner cache staleness, patch/Manchester safety, OBO editable mismatch, diagnostics and query hardening, extension multi-root and UTF-16 diagnostics

See [GitHub CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for the full v0.7.0 list.

## [0.6.0] - 2026-06-24

### Added

- **`ontoindex-reasoner` crate** — thin OntoLogos 0.9.0 facade (`el`, `rl`, `rdfs` adapters; `dl`/`auto` stubbed until OntoLogos 1.0)
- CLI **`ontoindex classify`** and **`ontoindex explain`**
- LSP **`ontoindex/runReasoner`** and **`ontoindex/getExplanation`**
- VS Code **Reasoner Results** panel, **Explanation** panel, hierarchy mode toggle
- Enterprise and adoption documentation packs (see [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md))

### Changed

- `CatalogSnapshot` includes optional `reasoner` metadata after classification
- Workspace crates bumped to **0.6.0**

### Fixed

- Turtle patch write-back corruption, file-size limit bypasses, ontology_id joins, SQL row-cap/alias bugs, extension XSS/stale-async races

See [GitHub CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for the full list and v0.5.0 and earlier.

## [0.5.0] - 2026-06-24

### Added

- **Query workbench (VS Code):** SQL and SPARQL modes, result table, CSV/JSON export, saved queries, query history
- LSP **`ontoindex/query`**, **`ontoindex/sparql`**, **`ontoindex/parseManchester`**
- **Manchester MVP editor** and complex Turtle patch operations
- **`EntityDetail.axioms`** structured axiom rows for inspector and Manchester editor

See [GitHub CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for v0.4.0 and earlier.
