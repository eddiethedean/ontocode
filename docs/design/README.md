# OntoIndex + OntoCode Documentation Package

This package contains **product vision and specification** documents. For **user-facing guides** (install, SQL, implemented LSP API), see [`docs/`](../) at the repository root.

| Package | Audience |
|---------|----------|
| [`docs/`](../) | New users and integrators (install, SQL reference, LSP API v0.2) |
| `docs/design/` (this folder) | Contributors and planners (roadmap, target architecture, ADRs) |

Two related products:

1. **OntoIndex** — Rust ontology index/query engine (`ontoindex-*` crates).
2. **OntoCode** — VS Code extension (OntoCode Explorer in v0.2; full workbench at v1.0).

**Sibling project:** [OntoLogos](https://github.com/eddiethedean/ontologos) — Rust ontology reasoner. OntoCode delegates all reasoning to OntoLogos crates per [ADR-0015](adr/0015-adopt-ontologos-reasoner.md) (0.9.0 at v0.6, 1.0.0 at v1.0).

## v1.0 exit bar

**[PROTEGE_PARITY.md](PROTEGE_PARITY.md)** — canonical P0/P1/P2 checklist for Protégé-competitive release.

## Document status

Many specs describe **target** behavior. Check the banner at the top of each doc, or:

- **Implemented v0.2 LSP:** [docs/lsp-api.md](../lsp-api.md)
- **Implemented SQL tables:** [docs/sql-reference.md](../sql-reference.md)
- **ADRs (canonical):** [adr/README.md](adr/README.md)

## Documents

### Product & roadmap

- [PLAN.md](PLAN.md) — combined product plan
- [ROADMAP.md](ROADMAP.md) — milestone roadmap v0.1 → v1.0
- [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — **v1.0 compete checklist**
- [v1.0_BACKLOG.md](v1.0_BACKLOG.md) — implementation backlog

### Technical specs

- [SPEC.md](SPEC.md) — combined technical specification
- [ARCHITECTURE.md](ARCHITECTURE.md) — target architecture
- [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) — hybrid forms + Manchester authoring
- [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) — OBO format + ROBOT interop
- [REASONER_SPEC.md](REASONER_SPEC.md) — OntoLogos-backed reasoners (0.9.0 → 1.0.0)
- [SHACL_SPEC.md](SHACL_SPEC.md) — SHACL validation (P1)
- [SEMANTIC_DIFF_SPEC.md](SEMANTIC_DIFF_SPEC.md) — semantic ontology diff
- [LSP_SPEC.md](LSP_SPEC.md) — target language server (v1.0 methods)
- [PLUGIN_SPEC.md](PLUGIN_SPEC.md) — plugin system

### UX

- [UI_WIREFRAMES.md](UI_WIREFRAMES.md) — text-based VS Code wireframes

### Historical / backlog

- [MVP_BACKLOG.md](MVP_BACKLOG.md) — v0.1/v0.2 backlog
- [adr/](adr/) — architecture decision records
