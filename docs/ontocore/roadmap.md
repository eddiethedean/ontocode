# OntoCore roadmap

Platform evolution for OntoCore inside the OntoCode monorepo.

## v0.9.0 — OntoCore identity (current)

- **`ontocore-*` crate rename** — all implementation crates, CLI (`ontocore`), and LSP (`ontocore-lsp`)
- **`ontocore` façade** on crates.io with experimental `Workspace` API
- LSP custom methods under `ontocore/*`
- OntoCore branding and documentation restructure

## v0.10 — Public API + workflow

- Stabilize `ontocore::Workspace` and ergonomic search/query/diagnostics/refactor APIs
- Examples and docs.rs documentation for `ontocore`
- Semantic diff and Git branch comparison
- Incremental workspace index
- React migration for reasoner/explanation panels

## v0.11 — Platform layer

- Persistent workspace cache API
- Richer semantic graph API
- Plugin/extension point design
- MCP server design
- Python and TypeScript binding boundaries

## v0.12 — Binding prep

- FFI-safe stable data models
- JSON serialization contracts
- API compatibility tests
- Prepare `ontocore-python` and `@ontocore/node`

## v1.0 — OntoCore-powered OntoCode

- OntoCode depends cleanly on OntoCore APIs
- OntoCore documented as the platform; OntoCode as flagship IDE
- OWL 2 DL reasoning via OntoLogos 1.0

See also [design roadmap](../design/ROADMAP.md) and [ADR-0018](../design/adr/0018-ontocore-platform-identity.md).
