# OntoIndex + OntoCode Documentation Package

This package contains **product vision and specification** documents. For **user-facing guides** (install, SQL, implemented LSP API), see [`docs/`](../) at the repository root.

| Package | Audience |
|---------|----------|
| [`docs/`](../) | New users and integrators (install, SQL reference, LSP API v0.2) |
| `docs/design/` (this folder) | Contributors and planners (roadmap, target architecture, ADRs) |

Two related products:

1. **OntoIndex** — Rust ontology index/query engine (`ontoindex-*` crates).
2. **OntoCode** — VS Code extension (OntoCode Explorer in v0.2).

## Document status

Many specs describe **target** behavior. Check the banner at the top of each doc, or:

- **Implemented v0.2 LSP:** [docs/lsp-api.md](../lsp-api.md)
- **Implemented SQL tables:** [docs/sql-reference.md](../sql-reference.md)
- **ADRs (canonical):** [adr/README.md](adr/README.md)

## Documents

- `PLAN.md` — combined product plan
- `SPEC.md` — combined technical specification
- `ARCHITECTURE.md` — target architecture (partial v0.2 implementation)
- `ROADMAP.md` — milestone roadmap from v0.1 to v1.0
- `PLUGIN_SPEC.md` — plugin system design (planned)
- `LSP_SPEC.md` — target language server design (see `docs/lsp-api.md` for v0.2)
- `REASONER_SPEC.md` — reasoner adapter design (planned)
- `SEMANTIC_DIFF_SPEC.md` — semantic ontology diff design (planned)
- `UI_WIREFRAMES.md` — text-based VS Code UI wireframes
- `MVP_BACKLOG.md` — v0.1/v0.2 implementation backlog
- `v1.0_BACKLOG.md` — full v1.0 implementation backlog
- `adr/` — architecture decision records (canonical; `adrs/` was merged here)
