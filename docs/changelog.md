# Changelog

Canonical source: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) (update both on release).

Migration guides: [Migration index](migration/README.md)

## [0.11.0] - 2026-07-05

**v0.11.0** — Editor depth & distribution — LSP completion and code actions, `ontocore docs`, imports UI, OBO `fastobo` read path, Open VSX. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.11.md](migration/v0.11.md).

## [0.10.0] - 2026-07-04

**v0.10.0** — Semantic Workspace release — incremental indexing, multi-root workspaces, stable `Workspace` API, semantic diff (CLI/LSP/panel), optional disk cache. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.10.md](migration/v0.10.md).

## [0.9.0] - 2026-07-03

### Added

- `ontocore` façade crate with `Workspace` API; OntoCore / OntoCode documentation; ADR-0018; platform architecture docs
- OntoLogos 1.0.0 DL/auto classification (`dl`, `auto` profiles) via `ontocore-reasoner`
- Plugin platform design with owlmake as reference external workflow plugin
- OBO/ROBOT spec owlmake integration path and ODK workflow goals

### Changed

- **Breaking (v0.9.0):** `ontoindex-*` → `ontocore-*`; CLI `ontocore`; LSP `ontocore-lsp` and `ontocore/*` methods — see [v0.9 migration](migration/v0.9.md)
- OntoLogos workspace dependencies 0.9.0 → 1.0.0; enterprise docs updated for shipped DL/auto classification
- Extension marketplace metadata and `ontocore` crate README as public façade API

### Fixed

- LSP reasoner test for DL/auto profiles; MkDocs strict-mode link fixes
- Release packaging for first `ontocore-*` crates.io publish (licenses, crate READMEs, leaf-crate dry-run)
