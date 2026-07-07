# OntoCore & OntoCode Roadmap

## Vision

Build the modern open-source platform for ontology engineering.

**OntoCore** is the semantic workspace engine.

**OntoCode** is the flagship IDE powered by OntoCore.

Full mission and principles: [Vision](vision.md). Ecosystem layers: [Architecture](architecture.md).

## Guiding principle

**OntoCode 1.0 has one primary objective: become a production-ready replacement for Protégé.**

Every feature before 1.0 should answer one question:

> Does this make it easier for ontology engineers to adopt OntoCode instead of Protégé?

After 1.0, the roadmap shifts from parity to modernization.

---

## How to read this document

| Document | Role |
|----------|------|
| [What ships today](SHIPPED.md) | **Canonical capability matrix** — what is available in the current release |
| [UI roadmap mapping](ui/ROADMAP_MAPPING.md) | **UI specs ↔ releases** — master checklist for all Product Roadmap 2.0 items |
| [Milestones (shipped)](design/ROADMAP.md) | Per-crate engineering detail — Shipped (v0.1–v0.9) and later milestones |
| [Protégé parity matrix](design/PROTEGE_PARITY.md) | **v1.0 exit bar** — P0 / P1 / P2 parity tiers |
| [v1.0 backlog](design/v1.0_BACKLOG.md) | Implementation checklist toward v1.0 |
| [Product design (UI)](ui/README.md) | UX, design system, workspace model, OntoStudio target |

**Current release:** v0.12.0

---

## Release timeline

```text
SHIPPED ─────────────────────────────────────────────────────────────►
v0.1   v0.2   v0.3   v0.4   v0.5   v0.6   v0.7   v0.8   v0.9  v0.10  v0.11
  │      │      │      │      │      │      │      │      │      │      │
Foundation Explorer Diag Write Query Reason Viz  Refactor Identity Workspace Editor depth

PLANNED ──────────────────────────────────────────────────────────────►
v0.12        v0.13           v0.14              v1.0         v1.1    v1.2+
Authoring    Platform        Plugin host       Protégé      SDKs &   Toolchain
parity       hardening       MVP               release      AI       platform
```

| Phase | Version | Status | Theme |
|-------|---------|--------|-------|
| 1 | v0.1 | Shipped | OntoCore foundation |
| 2 | v0.2 | Shipped | VS Code explorer |
| 3 | v0.3 | Shipped | Diagnostics |
| 4 | v0.4 | Shipped | Turtle write-back + Horned-OWL |
| 5 | v0.5 | Shipped | Query workbench + Manchester MVP |
| 6 | v0.6 | Shipped | Ontologos reasoning (EL/RL/RDFS) |
| 7 | v0.7 | Shipped | React UI, graphs, OBO + ROBOT |
| 8 | v0.8 | Shipped | Refactoring + full Manchester |
| 9 | v0.9 | Shipped | OntoCore platform identity |
| 10 | v0.10 | Shipped | Semantic workspace |
| 11 | v0.11 | Shipped | Editor depth & distribution |
| 12 | v0.12 | Planned | Authoring parity |
| 13 | v0.13 | Planned | Platform hardening |
| 14 | v0.14 | Planned | Plugin host MVP |
| 15 | v1.0 | Planned | Protégé-competitive release |
| 16 | v1.1 | Planned | Language bindings & AI primitives |
| 17 | v1.2+ | Planned | Ontology toolchain platform & ecosystem modernization |

> **Note on v0.12–v0.18 (retired labels):** Earlier drafts used v0.12–v0.18 for capabilities that **shipped in v0.3–v0.11** (diagnostics, SQL virtual tables, refactoring, Ontologos reasoning, semantic diff, docs export). Those labels are retired. Forward work from v0.12 onward is defined in the phases below.

---

## Shipped phases (v0.1–v0.11)

### v0.1 — OntoCore foundation (shipped)

**Theme:** Prove the semantic workspace engine — index, catalog, and query ontology files from the CLI.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Rust workspace; CLI skeleton; recursive scanner; file hashing; parser adapters; basic catalog (`ontologies`, `classes`, `properties` tables); SQL and SPARQL query |
| **OntoCode** | — |

**Exit criteria:** `ontocore query ./repo "SELECT * FROM classes"` returns indexed classes.

**Dependencies:** `oxigraph`, `sqlparser`, `ignore`, `clap`

---

### v0.2 — OntoCode explorer (shipped)

**Theme:** Browse ontologies in VS Code via a language server.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | LSP process; workspace indexing command |
| **OntoCode** | VS Code extension skeleton; ontology explorer; class/property/individual trees; entity inspector; jump to source; hover, go-to-definition, document/workspace symbols |

**Exit criteria:** User can browse an ontology repo in VS Code.

**Dependencies:** `lsp-server`, `lsp-types`, OntoCore crates

---

### v0.3 — Diagnostics (shipped)

**Theme:** Surface ontology quality issues inline.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Parse errors; broken imports; undefined prefixes; duplicate labels; missing labels; orphan classes; `diagnostics` SQL virtual table |
| **OntoCode** | Problems panel integration |

**Exit criteria:** User gets useful ontology diagnostics inline.

**Dependencies:** `oxigraph` (parse errors); in-house lint rules in `ontocore-diagnostics`

---

### v0.4 — Write-back + Horned-OWL (shipped)

**Theme:** Edit Turtle ontologies without Protégé.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-owl` crate (Horned-OWL catalog bridge); patch-based write-back; create/edit/delete classes, properties, individuals; edit labels, comments, simple `SubClassOf`, deprecated flag; Oxigraph ↔ Horned-OWL consistency tests; CLI `ontocore patch`; LSP `ontocore/applyAxiomPatch` |
| **OntoCode** | Editable entity inspector |

**Exit criteria:** User can edit labels and simple subclass axioms in Turtle; catalog axioms for editing come from Horned-OWL.

**Dependencies:** `horned-owl`, `horned-functional` via `ontocore-owl` ([ADR-0013](design/adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0006](design/adr/0006-patch-based-write-back.md))

---

### v0.5 — Query workbench + Manchester MVP (shipped)

**Theme:** Query and author complex class expressions.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Manchester parse/serialize for `SubClassOf` and `EquivalentClasses`; LSP `ontocore/parseManchester`, `ontocore/query`, `ontocore/sparql` |
| **OntoCode** | SQL and SPARQL query webviews; saved queries, result export, query history; Manchester editor MVP |

**Exit criteria:** User can query ontologies in VS Code and edit complex subclass/equivalent axioms via Manchester.

**Dependencies:** `sqlparser`, `oxigraph`; Manchester in `ontocore-owl`

---

### v0.6 — Reasoning (shipped)

**Theme:** Rust-native OWL classification and inferred hierarchy.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-reasoner` crate (Ontologos facade); `el`, `rl`, `rdfs` adapters; profile detection; unsatisfiable classes; classification result cache |
| **OntoCode** | Reasoner panel; asserted/inferred/combined hierarchy toggle; explanation panel (EL-first) |

**Exit criteria:** User can classify EL ontologies, see inferred hierarchy, and get EL explanations where available.

**Dependencies:** Ontologos `ontologos-*` ([ADR-0015](design/adr/0015-adopt-ontologos-reasoner.md))

---

### v0.7 — Visualization, React UI, OBO & ROBOT (shipped)

**Theme:** Rich IDE panels and biomedical toolchain interop.

Sub-phases: **v0.7a** (React foundation) → **v0.7** (graphs + inspector) → **v0.7b** (OBO + ROBOT).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-robot` wrappers (`validate`, `merge`, `report`); OBO format index; graph structure export via `petgraph` |
| **OntoCode** | `extension/webview-ui/` — Vite + React + TypeScript; typed `postMessage` protocol; CSP-compliant panel host; class/property/import/neighborhood graphs; entity inspector on React; OBO syntax highlighting and id rendering in explorer |

**Exit criteria:** User can navigate ontologies visually; biomedical maintainer can index OBO and run ROBOT in CI.

**Dependencies:** `react`, `vite` (extension); `fastobo`, `fastobo-owl`, `fastobo-validator`; ROBOT CLI ([ADR-0017](design/adr/0017-react-webview-ui.md))

---

### v0.8 — Refactoring + full Manchester (shipped)

**Theme:** Safe large-scale ontology maintenance and full OWL 2 DL expression authoring.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Safe IRI rename, namespace migration, find usages, move entity, extract module; preview/apply refactor plan; full Manchester axiom catalog (restrictions, disjoint, property chains view) |
| **OntoCode** | Refactor preview panel; Query Workbench and Manchester editor migrated to React; LSP rename and find references |

**Exit criteria:** User can safely refactor ontology repositories and author full OWL 2 DL expression sets via hybrid UI.

**Dependencies:** `horned-owl`, `horned-functional`; React webview UI

---

### v0.9 — OntoCore platform identity (shipped)

**Theme:** Unified naming, public API, and DL reasoning.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Rename `ontoindex-*` → `ontocore-*`; CLI `ontocore`; LSP `ontocore-lsp` with `ontocore/*` methods; `ontocore` façade crate with `Workspace` API; Ontologos 1.0.0 integration (`dl` and `auto` adapters); plugin platform design ([PLUGIN_SPEC.md](design/PLUGIN_SPEC.md)) |
| **OntoCode** | Reasoner + explanation panels on React; legacy HTML webviews removed; OntoCore branding |

**Exit criteria:** Contributors and users distinguish OntoCore (engine) from OntoCode (IDE); DL/auto classification enabled.

**Dependencies:** Ontologos 1.0.0; breaking release for v0.8 integrators ([ADR-0018](design/adr/0018-ontocore-platform-identity.md))

---

### v0.10 — Semantic workspace (shipped)

**Theme:** Team-scale development workflows.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Incremental indexing (content-hash reuse); multi-root workspaces; stable `ontocore::Workspace` API; `ontocore-diff` crate; `ontocore diff` CLI; git ref compare; breaking-change detection; optional disk cache (`.ontocore/cache/`) |
| **OntoCode** | Semantic diff React panel; LSP `ontocore/semanticDiff`; multi-root folder support |

**Exit criteria:** User can diff ontology versions, work in multi-root workspaces, and reindex incrementally at scale.

**Dependencies:** `git2`, `horned-owl`, `pulldown-cmark`, `minijinja`

---

### v0.11 — Editor depth & distribution (shipped)

**Theme:** Close editor gaps and expand distribution.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | LSP `textDocument/completion` (Turtle prefix, QName, IRI); diagnostic quick fixes (`undefined_prefix`, `missing_label`, `broken_import`); `ontocore-docs` crate; `ontocore docs` CLI (Markdown/HTML); `add_import` / `remove_import` patch ops; OBO read path via `fastobo` (synonyms, defs, xrefs); ADR for v1.0 OBO write-back ([ADR-0019](design/adr/0019-obo-write-back.md)) |
| **OntoCode** | Manage Imports panel; Open VSX publishing (Cursor); diagnostic code actions; entity inspector panel reuse on navigation; VS Code e2e tests |

**Exit criteria:** Daily Turtle editing, import management, and docs export work without leaving VS Code; extension available on VS Code Marketplace and Open VSX.

**Dependencies:** `fastobo`; `minijinja`, `pulldown-cmark`

---

## Planned phases (v0.12 → v1.2+)

### v0.12 — Authoring parity (shipped)

**Theme:** Close remaining **P0** OWL and OBO authoring gaps from [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Property domain, range, and characteristics authoring (patch ops); individual class/property assertions; expanded annotation assertion editing; property chain editing; full OBO write-back ([OBO_ROBOT_SPEC.md](design/OBO_ROBOT_SPEC.md), [ADR-0019](design/adr/0019-obo-write-back.md)); OWL/XML read support; Horned-OWL → Ontologos bridge improvements; axiom round-trip golden tests (Protégé fixtures) |
| **OntoCode** | Inspector forms for domain/range/characteristics; property chain editor; OBO write-back in inspector; DL clash-trace explanations via `ontologos-explain` + `ontologos-dl`; Turtle preview before apply for all axiom types |

**Exit criteria:** All **P0 — OWL 2 DL authoring** and **OBO & biomedical** rows in [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) are green.

**Dependencies:** `horned-owl`, `horned-functional`, `fastobo`, `fastobo-owl`; Ontologos `ontologos-explain`

---

### v0.13 — Platform hardening (planned)

**Theme:** Stable APIs, editor polish, and team workflow features.

**UI track:** [Product Roadmap 2.0](ui/PRODUCT_ROADMAP_2.0.md) phases **0–1** — WorkspaceStore, Current Focus model, design tokens, centralized webview bus. See [ROADMAP_MAPPING.md](ui/ROADMAP_MAPPING.md) (master checklist).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Stable semver APIs: catalog, query, diagnostics, semantic diff, docs, OWL; SQL joins and aggregations (extend `sqlparser` virtual tables); Horned-OWL virtual tables (`restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`, `domain_axioms`, `range_axioms`); configurable diagnostic rules; PR summary generation from semantic diff; performance benchmarks on large ontology targets; ontology helper functions in query layer |
| **OntoCode** | LSP semantic tokens; **WorkspaceStore + Current Focus** ([STATE_MANAGEMENT.md](ui/STATE_MANAGEMENT.md)); shared design tokens ([DESIGN_TOKENS.md](ui/DESIGN_TOKENS.md)); centralized webview event bus; component library alignment ([COMPONENT_LIBRARY.md](ui/COMPONENT_LIBRARY.md)); webview accessibility review; webview integration test hardening; validation report panel; class hierarchy and property docs in `ontocore docs` export |

#### UI deliverables (Product Roadmap 2.0)

| Deliverable | Phase | Spec |
|-------------|-------|------|
| Extension UX audit; clunky-flow fixes | 0 | [TESTING_STRATEGY.md](ui/TESTING_STRATEGY.md) |
| Centralize webview communication | 0 | [EVENT_SEQUENCE_DIAGRAMS.md](ui/EVENT_SEQUENCE_DIAGRAMS.md) |
| WorkspaceStore + Current Focus | 0, 1 | [STATE_MANAGEMENT.md](ui/STATE_MANAGEMENT.md) |
| Design tokens + reusable components | 0 | [DESIGN_TOKENS.md](ui/DESIGN_TOKENS.md), [COMPONENT_LIBRARY.md](ui/COMPONENT_LIBRARY.md) |
| Semantic navigation history | 1 | [WORKSPACE_MODEL.md](ui/WORKSPACE_MODEL.md) |
| Explorer ↔ inspector synchronization | 1 | [INFORMATION_ARCHITECTURE.md](ui/INFORMATION_ARCHITECTURE.md) |
| Core event bus | 1 | [EVENT_SEQUENCE_DIAGRAMS.md](ui/EVENT_SEQUENCE_DIAGRAMS.md) |
| Query schema browser | 3 | [QUERY_WORKBENCH.md](ui/QUERY_WORKBENCH.md) |
| Semantic PR summary (CLI/LSP) | 9 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Accessibility pass + webview integration tests | — | [ACCESSIBILITY_SPEC.md](ui/ACCESSIBILITY_SPEC.md), [TESTING_STRATEGY.md](ui/TESTING_STRATEGY.md) |
| Visual design system baseline | — | [VISUAL_DESIGN_SYSTEM.md](ui/VISUAL_DESIGN_SYSTEM.md) |

**Exit criteria:** Public `ontocore` APIs are semver-stable; large-ontology performance documented; React panels pass accessibility and integration test bar.

**Dependencies:** `sqlparser` ([ADR-0011](design/adr/0011-use-sqlparser-for-sql.md)); evaluate DataFusion only if virtual-table joins are insufficient

---

### v0.14 — Plugin host MVP (planned)

**Theme:** External extensibility without embedding workflow engines in core.

**UI track:** [Product Roadmap 2.0](ui/PRODUCT_ROADMAP_2.0.md) phase **8** — plugin manifest, inspector cards, command API. See [PLUGIN_PLATFORM.md](ui/PLUGIN_PLATFORM.md) and [ROADMAP_MAPPING.md](ui/ROADMAP_MAPPING.md).

OntoCore hosts **external** plugins through stable APIs — it does not embed ROBOT, ODK, or owlmake as core dependencies. See [PLUGIN_SPEC.md](design/PLUGIN_SPEC.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Plugin host runtime; stable plugin API (semver); plugin load/discover from workspace config; reference plugins: naming convention validator, Markdown exporter, SHACL validator via `rudof` ([SHACL_SPEC.md](design/SHACL_SPEC.md)); CLI/LSP hooks for plugin diagnostics and exports |
| **OntoCode** | Plugin-contributed diagnostics in Problems panel; plugin commands in palette; owlmake integration scaffold (invoke external workflow, surface build/QC status) |
| **Ecosystem** | `examples/obo-workflow/` fixture repo; owlmake as first reference workflow plugin (external repo) |

#### UI deliverables (Product Roadmap 2.0 — phase 8)

| Deliverable | Spec |
|-------------|------|
| Plugin manifest + runtime | [PLUGIN_API_SPEC.md](ui/PLUGIN_API_SPEC.md) |
| Command API | [PLUGIN_PLATFORM.md](ui/PLUGIN_PLATFORM.md) |
| Inspector card API | [PLUGIN_API_SPEC.md](ui/PLUGIN_API_SPEC.md) |
| Reasoner provider API | [PLUGIN_API_SPEC.md](ui/PLUGIN_API_SPEC.md) |
| Plugin SDK + reference examples | [design/PLUGIN_SPEC.md](design/PLUGIN_SPEC.md) |
| Plugin-contributed diagnostics in Problems panel | [PLUGIN_PLATFORM.md](ui/PLUGIN_PLATFORM.md) |

**Exit criteria:** Third party can ship a validation or export plugin without forking OntoCore; owlmake can be invoked from OntoCode as an external workflow.

**Dependencies:** `rudof` (SHACL P1); owlmake (external)

---

### v1.0 — Protégé-competitive release (planned)

**Theme:** Production-ready Protégé replacement in VS Code.

**UI track:** [Product Roadmap 2.0](ui/PRODUCT_ROADMAP_2.0.md) phases **2–6** exit polish — entity workspace, query/graph/reasoning/refactor UX complete per [PRODUCT_DESIGN_SPECIFICATION.md](ui/PRODUCT_DESIGN_SPECIFICATION.md). Wireframes: [WORKSPACE_WIREFRAMES.md](ui/WORKSPACE_WIREFRAMES.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | All [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) **P0** items green; all **P1** items green or documented known gaps; stable CLI/API/LSP semver 1.0; `examples/protege-roundtrip/` ontology set with workflow doc; performance benchmarks published |
| **OntoCode** | Complete hybrid authoring loop (forms + Manchester + Turtle); full IDE surface (explorer, search, diagnostics, refactoring, query workbench, visualization, reasoning); React webview hardening complete; VS Code Marketplace + Open VSX publish as 1.0 |
| **Toolchain** | ODK project layout recognition (`src/ontology/`, catalog files, import structure); ODK QC and release workflow surfacing; ROBOT-compatible operations where practical; import existing ODK/ROBOT/owlmake workflows (Makefile, GitHub Actions); Protégé migration guide with honest parity table |
| **Ecosystem** | Ontologos 1.0.0 reasoner gate satisfied; published `ontocore` + `ontocore-*` 1.0.0 on crates.io |

#### UI deliverables (Product Roadmap 2.0 — phases 1–6 exit + partial 9)

| Deliverable | Phase | Spec |
|-------------|-------|------|
| Persistent tabs + bottom dock | 1 | [WORKSPACE_WIREFRAMES.md](ui/WORKSPACE_WIREFRAMES.md) |
| Relationship cards, references view, metadata view | 2 | [ENTITY_EDITOR_SPEC.md](ui/ENTITY_EDITOR_SPEC.md) |
| Entity workspace diagnostics integration | 2 | [ENTITY_EDITOR_SPEC.md](ui/ENTITY_EDITOR_SPEC.md) |
| Graph saved layouts, filters, reasoning overlays | 4 | [GRAPH_WORKSPACE.md](ui/GRAPH_WORKSPACE.md) |
| Semantic build pipeline UI | 5 | [REASONING_EXPERIENCE.md](ui/REASONING_EXPERIENCE.md) |
| Entity-level reasoning cards + reasoning history | 5 | [REASONING_EXPERIENCE.md](ui/REASONING_EXPERIENCE.md) |
| Problems panel ↔ reasoning integration | 5 | [REASONING_EXPERIENCE.md](ui/REASONING_EXPERIENCE.md) |
| Merge classes + batch label normalization | 6 | [SEMANTIC_REFACTORING.md](ui/SEMANTIC_REFACTORING.md) |
| Undo/redo on refactor and patch writes | 6 | [SEMANTIC_REFACTORING.md](ui/SEMANTIC_REFACTORING.md) |
| Review workspace (MVP) | 9 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Human interface guidelines + keyboard shortcuts | — | [HUMAN_INTERFACE_GUIDELINES.md](ui/HUMAN_INTERFACE_GUIDELINES.md), [KEYBOARD_SHORTCUTS.md](ui/KEYBOARD_SHORTCUTS.md) |
| Wireframes validated | — | [WORKSPACE_WIREFRAMES.md](ui/WORKSPACE_WIREFRAMES.md) |

**Already shipped (v0.5–v0.12):** entity editor MVP, query workbench, graph canvas, reasoner panel, refactoring preview, semantic diff — see [ROADMAP_MAPPING.md § Master checklist](ui/ROADMAP_MAPPING.md).

**Exit criteria:**

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features in [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md).

Track implementation: [v1.0_BACKLOG.md](design/v1.0_BACKLOG.md)

**Dependencies:** Ontologos 1.0.0; `react` / `vite` (extension `webview-ui`)

#### OntoCode 1.0 — Modern Protégé replacement

<a id="ontocode-10-modern-protege-replacement"></a>

##### Editing scope

- Complete ontology editing (classes, properties, individuals, annotations)
- Manchester syntax for complex expressions
- Turtle write-back (primary authoring format)
- OBO editing (read + write)
- Import management

##### IDE scope

- Explorer, search, diagnostics, refactoring
- Query workbench (SQL + SPARQL)
- Graph visualization (class, property, import, neighborhood)
- Reasoner panel with EL/RL/RDFS/DL/auto profiles and explanations

##### Toolchain integration scope

OntoCode integrates with the existing ontology toolchain through OntoCore — **not** by reimplementing ROBOT, ODK, or owlmake inside the engine.

- **owlmake** — first-class workflow plugin; build/release actions in IDE
- **ROBOT** — merge, reason, convert, validate via existing ROBOT semantics
- **ODK** — project layout, QC workflows, release workflows, zero-config repo open
- **Protégé migration** — import projects, preserve IRIs, guide users off desktop-only workflows

Ontologos provides **reasoning**. OntoCore provides the **workspace platform** and **plugin hosting**. owlmake and peers provide **workflow automation**. OntoCode presents all three in one IDE.

---

### v1.1 — Language bindings & AI primitives (planned)

**Theme:** Cross-language integration and AI-native tooling foundations.

**UI track:** [Product Roadmap 2.0](ui/PRODUCT_ROADMAP_2.0.md) phase **7** — AI sidebar, inline suggestions, read-only review. See [AI_EXPERIENCE.md](ui/AI_EXPERIENCE.md), [AI_ORCHESTRATION_ARCHITECTURE.md](ui/AI_ORCHESTRATION_ARCHITECTURE.md), [ADR-0010](design/adr/0010-ai-features-opt-in.md).

Former roadmap labels **v0.17 (Language Bindings)** and **v0.18 (AI Platform)** are consolidated here.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Python SDK (workspace index, query, validate, diff); TypeScript SDK (LSP client helpers, webview protocol types); MCP server exposing workspace context (entities, axioms, diagnostics, query results) |
| **OntoCode** | MCP-driven semantic context for external AI tools; documentation generation hooks; ontology review assistance (read-only suggestions) |
| **Ecosystem** | Published SDK packages; MCP server installable via `cargo install` or pip |

#### UI deliverables (Product Roadmap 2.0 — phase 7 + deferred AI)

| Deliverable | Phase | Spec |
|-------------|-------|------|
| AI sidebar + inline suggestions | 7 | [AI_EXPERIENCE.md](ui/AI_EXPERIENCE.md) |
| AI explain entity | 2 | [AI_EXPERIENCE.md](ui/AI_EXPERIENCE.md) |
| AI query generation | 3 | [AI_EXPERIENCE.md](ui/AI_EXPERIENCE.md) |
| AI graph explanations | 4 | [AI_EXPERIENCE.md](ui/AI_EXPERIENCE.md) |
| Review ontology + repair diagnostics | 7 | [AI_EXPERIENCE.md](ui/AI_EXPERIENCE.md) |
| Project-wide AI tasks | 7 | [AI_ORCHESTRATION_ARCHITECTURE.md](ui/AI_ORCHESTRATION_ARCHITECTURE.md) |
| AI review (collaboration) | 9 | [COLLABORATION.md](ui/COLLABORATION.md) |
| AI provider API (plugins) | 8 | [PLUGIN_API_SPEC.md](ui/PLUGIN_API_SPEC.md) |
| MCP context bridge | 7 | [AI_ORCHESTRATION_ARCHITECTURE.md](ui/AI_ORCHESTRATION_ARCHITECTURE.md) |

All AI features: **read-only suggestions with preview/approval** — [ADR-0010](design/adr/0010-ai-features-opt-in.md).

**Exit criteria:** Python and TypeScript consumers can index and query ontologies without shelling to CLI; MCP clients can retrieve structured ontology context from an open workspace.

**Dependencies:** MCP protocol; PyO3 or subprocess bridge TBD in ADR

---

### v1.2 — Ontology Toolchain Platform (planned)

**Theme:** Mature external workflow integration beyond the reference owlmake plugin.

**UI track:** [Product Roadmap 2.0](ui/PRODUCT_ROADMAP_2.0.md) phases **9 + 11** — collaboration tooling, plugin registry, community templates. See [ROADMAP_MAPPING.md](ui/ROADMAP_MAPPING.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Build API (compile/merge/materialize); Release API (version, tag, publish artifacts); Validation API (plug-in QC pipelines); plugin discovery and install from registry; semver-compatible plugin contracts |
| **OntoCode** | Production-ready owlmake plugin integration; QC reports (HTML/Markdown/JSON) in IDE; workflow status dashboard |
| **Ecosystem** | Official GitHub Actions for ontology CI/CD; plugin marketplace; third-party workflow plugins |

#### UI deliverables (Product Roadmap 2.0 — phases 9 + 11)

| Deliverable | Phase | Spec |
|-------------|-------|------|
| GitHub integration | 9 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Semantic PR summaries (UI panel) | 9 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Merge checks | 9 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Public plugin registry + marketplace UI | 10, 11 | [PLUGIN_PLATFORM.md](ui/PLUGIN_PLATFORM.md) |
| Sample domain plugins | 11 | [design/PLUGIN_SPEC.md](design/PLUGIN_SPEC.md) |
| Community templates | 11 | — |
| Workflow / QC status dashboard | toolchain | [guides/docs-export.md](guides/docs-export.md) |

**Exit criteria:** ODK-style release pipeline runnable end-to-end from OntoCode with discoverable, versioned plugins.

---

### Post-1.2 — Ecosystem modernization (planned)

**Theme:** Shift from Protégé parity to ecosystem leadership.

**UI track:** [Product Roadmap 2.0](ui/PRODUCT_ROADMAP_2.0.md) phases **10–12** — OntoStudio desktop ([ONTOSTUDIO_DESKTOP.md](ui/ONTOSTUDIO_DESKTOP.md)), collaboration workspace ([COLLABORATION.md](ui/COLLABORATION.md)), semantic engineering platform.

#### UI deliverables (Product Roadmap 2.0 — phases 10 + 12)

| Deliverable | Phase | Spec |
|-------------|-------|------|
| OntoStudio Tauri app shell | 10 | [ONTOSTUDIO_DESKTOP.md](ui/ONTOSTUDIO_DESKTOP.md) |
| Shared React UI (OntoCode + OntoStudio) | 10 | [COMPONENT_LIBRARY.md](ui/COMPONENT_LIBRARY.md) |
| Native graph performance | 10 | [GRAPH_RENDERING_ARCHITECTURE.md](ui/GRAPH_RENDERING_ARCHITECTURE.md) |
| Local AI support | 10 | [AI_ORCHESTRATION_ARCHITECTURE.md](ui/AI_ORCHESTRATION_ARCHITECTURE.md) |
| Enterprise packaging | 10 | [ONTOSTUDIO_DESKTOP.md](ui/ONTOSTUDIO_DESKTOP.md) |
| Browser client | 12 | [PLATFORM_ARCHITECTURE.md](ui/PLATFORM_ARCHITECTURE.md) |
| Cloud sync + team workspaces | 12 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Distributed reasoning | 12 | [REASONING_EXPERIENCE.md](ui/REASONING_EXPERIENCE.md) |
| Shared semantic canvases | 12 | [GRAPH_WORKSPACE.md](ui/GRAPH_WORKSPACE.md) |
| Governance workflows | 12 | [guides/governance.md](guides/governance.md) |
| Live collaboration + ontology PR review | 9, 12 | [COLLABORATION.md](ui/COLLABORATION.md) |
| Advanced visualization (large-graph layout, temporal diff) | 4, 12 | [GRAPH_WORKSPACE.md](ui/GRAPH_WORKSPACE.md) |

#### OntoCore

- Semantic workspace APIs (persistent semantic databases)
- Plugin marketplace maturity
- Advanced graph analytics

#### OntoCode

- AI-assisted ontology engineering (modeling suggestions, axiom completion)
- Live collaboration
- Ontology review in pull requests
- Advanced visualization (large-graph layout, temporal diff)

#### Ecosystem

- Enterprise governance tooling
- Knowledge graph tooling integrations
- Documentation generators via plugin APIs

**Strategic framing:** OntoCore provides the platform. owlmake (and peers) provide workflow, build, and release automation. OntoCode surfaces both through the UI. The goal is ecosystem collaboration — not absorbing or replacing every tool in the stack.

---

## Long-term goal

OntoCore becomes the foundation for modern ontology tooling.

OntoCode becomes the flagship IDE.

Ontologos becomes the flagship Rust reasoning engine.
