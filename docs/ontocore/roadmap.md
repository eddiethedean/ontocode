# OntoCore roadmap

Platform evolution for OntoCore inside the OntoCode monorepo.

## v0.9.0 — OntoCore Identity (current)

- OntoCore branding and documentation
- `ontocore` façade crate with experimental `Workspace` API
- Architecture diagrams and responsibility split
- `ontoindex-*` crate names unchanged
- CLI remains `ontoindex`; LSP remains `ontoindex-lsp`

## v0.10 — OntoCore Public API + workflow

- Stabilize `ontocore::Workspace` and ergonomic search/query/diagnostics/refactor APIs
- Examples and docs.rs documentation for `ontocore`
- `ontocore` CLI alias (alongside `ontoindex`)
- Semantic diff and Git branch comparison
- Incremental workspace index
- React migration for reasoner/explanation panels

## v0.11 — OntoCore Platform Layer

- Persistent workspace cache API
- Richer semantic graph API
- Plugin/extension point design
- MCP server design
- Python and TypeScript binding boundaries

## v0.12 — Binding Prep

- FFI-safe stable data models
- JSON serialization contracts
- API compatibility tests
- Prepare `ontocore-python` and `@ontocore/node`

## v1.0 — OntoCore-Powered OntoCode

- OntoCode depends cleanly on OntoCore APIs
- OntoCore documented as the platform; OntoCode as flagship IDE
- `ontocore` CLI primary; `ontoindex` compatibility alias
- Optional rename of `ontoindex-*` → `ontocore-*` crates
- OWL 2 DL reasoning via OntoLogos 1.0

## CLI branding timeline

| Version | CLI |
|---------|-----|
| v0.9 | `ontoindex` primary |
| v0.10 | `ontocore` alias added |
| v0.11 | Docs switch to `ontocore` |
| v1.0 | `ontocore` primary, `ontoindex` compatibility alias |

See also [design roadmap](../design/ROADMAP.md) and [ADR-0018](../design/adr/0018-ontocore-platform-identity.md).
