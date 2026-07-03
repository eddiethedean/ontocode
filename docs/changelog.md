# Changelog

Canonical source: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) (update both on release).

Migration guides: [Migration index](migration/README.md)

## [0.9.0] - 2026-07-03

### Added

- `ontocore` façade crate with `Workspace` API; OntoCore / OntoCode documentation; ADR-0018
- OntoLogos 1.0.0 DL/auto classification (`dl`, `auto` profiles) via `ontocore-reasoner`

### Changed

- **Breaking (v0.9.0):** `ontoindex-*` → `ontocore-*`; CLI `ontocore`; LSP `ontocore-lsp` and `ontocore/*` methods — see [v0.9 migration](migration/v0.9.md)
- OntoLogos workspace dependencies 0.9.0 → 1.0.0; enterprise docs updated for shipped DL/auto classification
