# Internals (design targets)

> **These pages describe target architecture and contributor specs — not necessarily what ships in the current release.**
>
> For product capabilities, use **[What ships today](SHIPPED.md)**. For a 10-minute tutorial, use **[First success](guides/first-success.md)**.

Design documents, ADRs, and backlogs live under [design/](design/README.md). **Platform implementation architecture:** [platform/OVERVIEW.md](platform/OVERVIEW.md). Product UX specs: [ui/README.md](ui/README.md) — mapped to releases in [ROADMAP_MAPPING.md](ui/ROADMAP_MAPPING.md). **Product ADRs:** [adr/README.md](adr/README.md). **Cursor prompts:** [cursor-prompts/README.md](cursor-prompts/README.md). Always cross-check [SHIPPED.md](SHIPPED.md). See also [Roadmap hub](roadmap-hub.md) and [Glossary](glossary.md).

## Contributor paths by role

### Rust-only (OntoCore crates)

1. [Contributing guide](contributing.md) — build, test, MSRV
2. [OntoCore crate map](ontocore/crate-map.md) — façade vs `ontocore-*` layout
3. [Design architecture](design/ARCHITECTURE.md) — crate responsibilities
4. Run `./scripts/run-ci-local.sh` before opening a PR

### Extension-only (VS Code + OntoUI)

1. [Extension development](guides/extension-development.md) — `extension/` layout, F5, tests
2. [Debugging guide](debugging.md) — LSP output, webview devtools
3. [Webview protocol](webview-protocol.md) — host ↔ React messages
4. `cargo build -p ontocore-lsp --bins` then `cd extension && npm run compile && npm test`

### Docs-only

1. [Contributing guide](contributing.md) — mirror rules for `VISION.md` / `ARCHITECTURE.md` / `ROADMAP.md`
2. `./scripts/check-doc-versions.sh` and `mkdocs build --strict`
3. [Documentation index](documentation-index.md) — find the right page to edit

### LSP / custom editor integrators

1. [LSP hello world](guides/lsp-hello-world.md) — minimal stdio client
2. [LSP API reference](lsp-api.md) — shipped methods through v0.17
3. Do **not** use [design/LSP_SPEC.md](design/LSP_SPEC.md) for current behavior (future target)

## Start here for contributors

- [Documentation index](documentation-index.md)
- [Glossary](glossary.md)
- [Platform architecture](platform/OVERVIEW.md)
- [Contributing guide](contributing.md)
- [Extension development](guides/extension-development.md)
- [Debugging guide](debugging.md)
- [Cursor implementation prompts](cursor-prompts/README.md)
- [Design overview](design/README.md)
- [Product design / UI specs](ui/README.md)
- [UI roadmap mapping](ui/ROADMAP_MAPPING.md)
- [OntoCore implementation architecture](design/ARCHITECTURE.md)
- [Product ADRs](adr/README.md)
- [Engineering ADRs](design/adr/README.md)
- [Releasing](releasing.md)

## Plugin model

The plugin host **shipped in v0.14** as an MVP (`ontocore-plugin` discovery + `PluginHost` runtime). The API is **not stable** until v1.0. See [plugin model](ontocore/plugin-model.md), [Plugin authoring guide](guides/plugins.md), and [PLUGIN_SPEC.md](design/PLUGIN_SPEC.md).
