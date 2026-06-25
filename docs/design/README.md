# OntoIndex + OntoCode Documentation Package

> **Note:** Documents in this folder describe **product vision, target architecture, and planned features**. For **what ships in v0.6.0**, see [What ships today](../SHIPPED.md) and the [user guides](https://ontocode-vs.readthedocs.io/en/latest/).

This package contains **product vision and specification** documents. For **user-facing guides**, pick a documentation path:

| Path | Audience | Start |
|------|----------|-------|
| [VS Code extension](https://ontocode-vs.readthedocs.io/en/latest/guides/vscode-extension/) | Explorer, inspector, Query Workbench, reasoner panels | Marketplace install, no Rust required |
| [Rust & CLI](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/) | `ontoindex` CLI, crates.io libraries, CI, LSP integrators | `cargo install ontoindex-cli` |

[Documentation home](https://ontocode-vs.readthedocs.io/en/latest/) · [What ships today](../SHIPPED.md)

| Package | Audience |
|---------|----------|
| User docs (paths above) | New users and integrators |
| `docs/design/` (this folder) | Contributors and planners (roadmap, target architecture, ADRs) |

Two related products:

1. **OntoIndex** — Rust ontology index/query engine (`ontoindex-*` crates).
2. **OntoCode** — VS Code extension (explorer, diagnostics, Turtle + Manchester authoring, Query Workbench, reasoner in v0.6).

**Sibling project:** [OntoLogos](https://github.com/eddiethedean/ontologos) — Rust ontology reasoner. OntoCode delegates reasoning to OntoLogos per [ADR-0015](adr/0015-adopt-ontologos-reasoner.md).

**Dependency policy:** [ADR-0016](adr/0016-dependency-first-implementation.md) — thin `ontoindex-*` facades over mature crates. Inventory: [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md). Licenses: [LICENSES.md](LICENSES.md).

## v1.0 exit bar

**[PROTEGE_PARITY.md](PROTEGE_PARITY.md)** — canonical P0/P1/P2 checklist for Protégé-competitive release.

## Document status

Many specs describe **target** behavior. Check the banner at the top of each doc, or:

- **Implemented v0.6 LSP:** [LSP API](https://ontocode-vs.readthedocs.io/en/latest/lsp-api/) — includes `query`, `sparql`, `parseManchester`, `runReasoner`, `getExplanation`
- **Implemented reasoner:** [Reasoner guide](https://ontocode-vs.readthedocs.io/en/latest/guides/reasoner/)
- **Implemented SQL tables:** [SQL reference](https://ontocode-vs.readthedocs.io/en/latest/sql-reference/)
- **Implemented authoring:** [authoring guide](https://ontocode-vs.readthedocs.io/en/latest/authoring/)
- **ADRs (canonical):** [adr/README.md](adr/README.md)

## Documents

### Product & roadmap

- [PLAN.md](PLAN.md) — combined product plan
- [ROADMAP.md](ROADMAP.md) — milestone roadmap v0.1 → v1.0
- [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md) — **external crate inventory**
- [LICENSES.md](LICENSES.md) — third-party license summary
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
- [OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md) — **React webview migration** (v0.7a → v1.0, [ADR-0017](adr/0017-react-webview-ui.md))

### Historical / backlog

- [MVP_BACKLOG.md](MVP_BACKLOG.md) — v0.1/v0.2 backlog
- [adr/README.md](adr/README.md) — architecture decision records
