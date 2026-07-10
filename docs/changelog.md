# Changelog

Canonical source: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) (update both on release).

Migration guides: [Migration index](migration/README.md)

## [0.17.0] - 2026-07-09

**v0.17.0** — Plugin preferences pages + context actions wired in the extension, plugin command execution via `ontocore/runPlugin`, imports reload + layout reset. Includes OBO idspace IRI normalization ([#111](https://github.com/eddiethedean/ontocode/issues/111)) and OBO patch newline/token validation ([#112](https://github.com/eddiethedean/ontocode/issues/112)). See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.16.md](migration/v0.16.md).

## [0.15.0] - 2026-07-08

**v0.15.0** — Plugin API permissions and UI views, explanation alternatives with staleness metadata, graph asserted/inferred modes, multi-root index fix. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.15.md](migration/v0.15.md).

## [0.14.0] - 2026-07-09

**v0.14.0** — Plugin host MVP: workspace manifests, reference naming validator and Markdown exporter, SHACL scaffold, `ontocore plugins list/run`, validate/docs hooks, LSP `listPlugins`/`runPlugin`, OntoCode workflow scaffold, capability registry inspector cards. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.14.md](migration/v0.14.md).

## [0.13.0] - 2026-07-08

**v0.13.0** — OntoUI platform (WorkspaceStore, focus relay, design tokens), schema browser, Horned-OWL axiom SQL tables, `ontocore diff --pr-summary`, configurable diagnostics, LSP semantic tokens, docs hierarchy export. Includes high-severity bug fixes for refactor rename, axiom patch sync, Manchester bootstrap, worktree diff, reasoner buffer overrides, OBO patch matching, and property chain editing. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.13.md](migration/v0.13.md).

## [0.12.0] - 2026-07-06

**v0.12.0** — Authoring parity — Turtle domain/range/chains/annotations, OBO write-back, OWL/XML read-only catalog, DL explanations, Protégé round-trip tests. See [migration/v0.12.md](migration/v0.12.md).

## [0.11.3] - 2026-07-06

**v0.11.3** — Patch — fixes Entity Inspector navigation when switching entities with a panel already open; adds VS Code e2e tests for inspector switching and workspace commands. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md).

## [0.11.2] - 2026-07-06

**v0.11.2** — Patch — fixes React webview panel routing when VS Code pre-populates `location.search` (Entity Inspector no longer stuck on Smoke panel); documentation adoption audit. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md).

## [0.11.1] - 2026-07-06

**v0.11.1** — Patch — initial React webview `?panel=` bootstrap; webview regression tests; Open VSX updates. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md).

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
