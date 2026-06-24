# Changelog

Canonical source: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) (update both on release).

Migration guides: [Migration index](migration/README.md)

## [0.6.0] - 2026-06-24

### Added

- **`ontoindex-reasoner` crate** — thin OntoLogos 0.9.0 facade (`el`, `rl`, `rdfs` adapters; `dl`/`auto` stubbed until OntoLogos 1.0)
- CLI **`ontoindex classify`** and **`ontoindex explain`**
- LSP **`ontoindex/runReasoner`** and **`ontoindex/getExplanation`**
- VS Code **Reasoner Results** panel, **Explanation** panel, hierarchy mode toggle (`asserted` / `inferred` / `combined`)
- Settings: `ontocode.reasoner.default`, `ontocode.reasoner.autoProfile`, `ontocode.hierarchy.mode`
- User guide: [Reasoner guide](guides/reasoner.md)

### Changed

- `CatalogSnapshot` includes optional `reasoner` metadata after classification
- Workspace crates bumped to **0.6.0**

## [0.5.0] - 2026-06-24

### Added

- **Query workbench (VS Code):** SQL and SPARQL modes, result table, CSV/JSON export, saved queries, query history
- LSP **`ontoindex/query`**, **`ontoindex/sparql`**, **`ontoindex/parseManchester`**
- **Manchester MVP editor** and complex Turtle patch operations
- **`EntityDetail.axioms`** structured axiom rows for inspector and Manchester editor

See [GitHub CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for v0.4.0 and earlier.
