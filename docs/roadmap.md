# OntoCore & OntoCode Roadmap

## Vision

Build the modern open-source platform for ontology engineering.

**OntoCore** is the semantic workspace engine.

**OntoCode** is the flagship IDE powered by OntoCore.

Full mission and principles: [Vision](vision.md). Ecosystem layers: [Architecture](architecture.md).

## Guiding Principle

**OntoCode 1.0 has one primary objective: become a production-ready replacement for Protégé.**

Every feature before 1.0 should answer one question:

> Does this make it easier for ontology engineers to adopt OntoCode instead of Protégé?

After 1.0, the roadmap shifts from parity to modernization.

---

## Shipped (v0.1–v0.9)

| Version | Highlights |
|---------|------------|
| v0.1–v0.4 | OntoCore foundation — scanner, catalog, SQL/SPARQL, diagnostics, Turtle write-back, LSP |
| v0.5–v0.6 | Manchester MVP, Query Workbench, OntoLogos EL/RL/RDFS reasoning |
| v0.7 | React inspector + graphs, OBO index, ROBOT CLI wrappers |
| v0.8 | Refactoring engine, full Manchester catalog, React Query Workbench + Manchester editor |
| **v0.9** | OntoCore identity — `ontocore-*` crate rename, `ontocore` façade, `ontocore` CLI, `ontocore-lsp`, `ontocore/*` LSP methods; OntoLogos 1.0 DL/auto classification |
| **v0.10** | Semantic workspace — incremental index, multi-root, stable `Workspace` API, semantic diff, optional disk cache |
| **v0.11** (current) | Editor depth — Turtle completion, diagnostic quick fixes, `ontocore docs`, Manage Imports, Open VSX, OBO `fastobo` read |

**Capability matrix:** [What ships today](SHIPPED.md) · **Engineering milestone detail:** [Milestones (shipped)](design/ROADMAP.md)

---

## v0.11 — Editor depth & distribution (shipped)

- Open VSX publishing (Cursor marketplace)
- LSP `textDocument/completion` for Turtle
- Diagnostic quick fixes via `textDocument/codeAction`
- `ontocore docs` documentation export (Markdown / HTML)
- Turtle imports management UI + `add_import` / `remove_import` patch ops
- OBO `fastobo` read path + ADR for v1.0 write-back

## v0.10 — Semantic Workspace (shipped)

- Multi-root workspaces
- Incremental indexing (content-hash reuse)
- Import graph, namespace registry, entity catalog, symbol graph
- Persistent optional cache (`.ontocore/cache/`)
- Stable `ontocore::Workspace` API
- Semantic diff (CLI, LSP, VS Code panel)

## v0.12 — Diagnostics & Quality

- Broken imports
- Circular imports
- Missing labels/comments
- Deprecated usage
- Ontology smells
- Metrics
- HTML / Markdown / JSON reports

## v0.13 — SQL Workspace Engine

DuckDB-style ontology analytics.

Virtual tables:

- ontology.classes
- ontology.properties
- ontology.individuals
- ontology.annotations
- ontology.imports
- ontology.axioms
- ontology.restrictions
- ontology.metrics
- ontology.diagnostics

## v0.14 — Refactoring

- Rename
- Namespace migration
- Merge ontology
- Extract ontology
- Safe delete
- Batch updates

## v0.15 — Ontologos Integration

- Classification
- Consistency
- Explanations
- Incremental reasoning
- Inferred hierarchy
- Reasoning-aware diagnostics

## v0.16 — Plugin Platform

OntoCore hosts **external** plugins through stable APIs — it does not embed workflow engines. The platform exposes extension points; reference integrations (such as [owlmake](https://github.com/INCATools/owlmake)) demonstrate how ontology build, validation, and release tools plug in without becoming core dependencies.

- Diagnostics plugins
- Query plugins
- Refactoring plugins
- Visualization plugins
- **Build plugins** — compile, merge, and materialize ontologies via external tools
- **Validation plugins** — SHACL, profile checks, and custom QC rules
- **Documentation plugins** — generate human-readable ontology docs and reports
- **Workflow plugins** — orchestrate multi-step ROBOT/ODK-style pipelines
- **owlmake** as the first reference workflow plugin — shows how external ontology workflow tools integrate with OntoCore workspace, index, and diagnostics APIs

## v0.17 — Language Bindings

- Python
- TypeScript
- Stable APIs

## v0.18 — AI Platform

- MCP server
- Semantic context
- Documentation generation
- Ontology review
- Modeling suggestions

## OntoCode 1.0 — Modern Protégé Replacement

### Editing

- Complete ontology editing
- Manchester syntax
- Turtle editing
- OBO editing
- Annotation editing
- Import management

### IDE

- Explorer
- Search
- Diagnostics
- Refactoring
- Query workbench
- Visualization
- Reasoning panel

### Ontology Toolchain Integration

OntoCode 1.0 integrates with the existing ontology toolchain through OntoCore — **not** by reimplementing ROBOT, ODK, or owlmake inside the engine.

- **owlmake integration** — first-class workflow plugin; OntoCode surfaces build/release actions in the IDE
- **ROBOT-compatible operations** where practical (merge, reason, convert, validate via existing ROBOT semantics)
- **ODK project layout support** — recognize standard `src/ontology/`, catalog files, and import structure
- **ODK quality workflow support** — run and surface QC checks familiar to ODK users
- **ODK release workflow support** — tag, version, and release artifacts through integrated tooling
- **Import existing ODK projects** — open and index ODK repos without manual reconfiguration
- **Import existing ROBOT/owlmake workflows** where practical — reuse Makefile, GitHub Actions, and owlmake configs
- **Protégé migration support** — import projects, preserve IRIs, and guide users off desktop-only workflows

Ontologos provides **reasoning** (classification, consistency, explanations). OntoCore provides the **workspace platform** (index, query, diagnostics, refactoring) and, at v1.0, **plugin hosting**. **owlmake** and similar tools will provide **workflow automation** — OntoCode presents all three in one IDE.

### Success Criteria

Teams can replace Protégé without losing essential workflows.

**Exit bar:** [Protégé parity matrix](design/PROTEGE_PARITY.md)

## v1.2 — Ontology Toolchain Platform

Post-1.0 milestone to mature external workflow integration beyond the reference owlmake plugin.

- **owlmake plugin** — production-ready workflow integration (build, release, QC)
- **Build API** — stable OntoCore API for compile/merge/materialize operations
- **Release API** — version, tag, and publish ontology artifacts
- **Validation API** — plug-in QC and profile validation pipelines
- **Documentation generation** — generate docs from workspace state via plugins
- **GitHub Actions** — official actions for CI/CD ontology pipelines
- **QC reports** — HTML/Markdown/JSON quality reports in IDE and CI
- **Plugin discovery** — find and install workflow plugins from a registry
- **Plugin versioning** — semver-compatible plugin contracts and compatibility checks

## Post-1.0 — Modernize the Ecosystem

### OntoCore

- Semantic workspace APIs
- Plugin marketplace
- Advanced graph analytics
- Persistent semantic databases

### OntoCode

- AI-assisted ontology engineering
- Live collaboration
- Ontology review in pull requests
- Advanced visualization

### Ecosystem

- Python and TypeScript SDKs
- **owlmake** and third-party workflow plugins
- Documentation generators (via plugin APIs)
- GitHub Actions for ontology CI/CD
- Enterprise governance
- Knowledge graph tooling

**Strategic framing:** OntoCore provides the platform. owlmake (and peers) provide workflow, build, and release automation. OntoCode surfaces both through the UI. The goal is ecosystem collaboration — not absorbing or replacing every tool in the stack.

## Long-Term Goal

OntoCore becomes the foundation for modern ontology tooling.

OntoCode becomes the flagship IDE.

Ontologos becomes the flagship Rust reasoning engine.
