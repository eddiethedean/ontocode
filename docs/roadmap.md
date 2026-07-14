# OntoCore & OntoCode Roadmap

> **Multiple roadmap documents?** See [Roadmap hub](roadmap-hub.md) to pick the right page. For **what ships today**, use [SHIPPED.md](SHIPPED.md).

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

!!! warning "Not a Protégé replacement today"
    **v0.20** supports pilot and coexistence workflows — not org-wide Protégé retirement. See [What ships today](SHIPPED.md) and [Known limitations](known-limitations.md) before planning format or IDE migration.

---

## How to read this document

| Document | Role |
|----------|------|
| [What ships today](SHIPPED.md) | **Canonical capability matrix** — what is available in the current release |
| [Protégé parity program](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/README.md) | **1.0 engineering program** — scope, blockers, release gates |
| [Pre-1.0 release phases](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) | **v0.19–v0.25 → 1.0.0** versioned parity plan |
| [UI roadmap mapping](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ROADMAP_MAPPING.md) | **UI specs ↔ releases** — master checklist for all Product Roadmap 2.0 items |
| [Milestones (shipped)](design/ROADMAP.md) | Per-crate engineering detail for **shipped** v0.1–v0.11 milestones |
| [Protégé parity matrix](design/PROTEGE_PARITY.md) | Historical v0.18 P0/P1/P2 checklist (superseded for planning) |
| [v1.0 backlog](design/v1.0_BACKLOG.md) | Implementation checklist toward v1.0 |
| [Platform overview](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/OVERVIEW.md) | OntoUI / WorkspaceStore architecture (foundation shipped v0.13) |
| [Product Roadmap 2.0](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md) | UI phases with milestone acceptance criteria |
| [Product design (UI)](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/README.md) | Product design specification pack (UX, design system, OntoStudio target) |

**Current release:** v0.22.0

---

## Release phases at a glance

### Timeline

```text
SHIPPED (v0.1–v0.21) ─────────────────────────────────────────────────►
v0.1–v0.4          v0.5–v0.8              v0.9–v0.12           v0.13–v0.21
Engine foundation    IDE depth                Platform & authoring   OntoUI → formats write-back

PLANNED (v0.22–1.0) ─────────────────────────────────────────────────►
v0.22–v0.23        v0.24                  v0.25                1.0.0-rc → 1.0.0
OWL 2 + reason         Refactor + DL Query      Verify + polish      Protégé replacement
```

Full timeline: [ROADMAP.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md). Pre-1.0 phases: [PRE_1_0_PHASES.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md).

### Phase index

| Era | Versions | Status | North-star |
|-----|----------|--------|------------|
| **A — Engine foundation** | v0.1–v0.4 | Shipped | Index, browse, diagnose, edit Turtle |
| **B — IDE depth** | v0.5–v0.8 | Shipped | Query, reason, visualize, refactor |
| **C — Platform & authoring** | v0.9–v0.12 | Shipped | OntoCore identity, semantic workspace, authoring parity |
| **D — OntoUI platform** | v0.13–v0.14 | Shipped | v0.13: WorkspaceStore, focus relay; v0.14: plugin host MVP |
| **E — Desktop UX shell gate** | v0.15–v0.18 | Shipped | Menus, layouts, workflows, migration readiness (not full parity) |
| **F — Full Protégé parity path** | v0.19–v0.25 | In progress (v0.19–v0.21 shipped) | Semantic core → formats → OWL 2 → reason/SWRL → services → verify |
| **G — Protégé replacement** | 1.0.0 | Planned | Daily OWL/OBO engineering without Protégé |
| **H — Ecosystem** | v1.1–v1.2+ | Planned | SDKs, AI, toolchain & collaboration |

| Phase | Version | Era | Status | UI phases | Theme |
|-------|---------|-----|--------|-----------|-------|
| 1–18 | v0.1–v0.18 | A–E | Shipped | — | See [ROADMAP.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) |
| 19 | v0.19 | F | Shipped | — | Semantic foundation + program baseline |
| 20 | v0.20 | F | Shipped | 1† | Workspace runtime |
| 21 | v0.21 | F | Shipped | — | RDF/XML + OWL/XML write-back |
| 22 | v0.22 | F | Shipped | 2† | Complete OWL 2 authoring |
| 23 | v0.23 | F | Planned | 5† | Reasoning parity + SWRL |
| 24 | v0.24 | F | Planned | 3†, 6† | Refactoring + DL Query parity |
| 25 | v0.25 | F | Planned | 4†, 8† | Viz + plugin SDK 1.0 + a11y + parity CI |
| 26 | 1.0.0-rc | F | Planned | — | Stabilize; all P0 VERIFIED |
| 27 | v1.0 | G | Planned | 1–6 exit, 9† | Protégé-competitive release |
| 28 | v1.1 | H | Planned | 7, 2†, 3†, 4†, 8†, 9† | Language bindings & AI primitives |
| 29 | v1.2+ | H | Planned | 9, 10, 11 | Ontology toolchain platform |

†Partial scope in this release (remainder in later releases). Full mapping: [ROADMAP_MAPPING.md](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ROADMAP_MAPPING.md).

### UI phase reference (Product Roadmap 2.0)

OntoUI work uses **UI phases 0–12** from [Product Roadmap 2.0](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md). They are integrated into release phases above — not a separate track.

| UI phase | Name | Primary releases |
|----------|------|------------------|
| **0** | Stabilize OntoUI | v0.13 (shipped) |
| **1** | Workspace foundation | v0.13 (core shipped); v1.0 (tabs, dock) |
| **2** | Entity workspace | v0.4–v0.12 (MVP); v1.0 (relationship/metadata views); v1.1† (AI explain) |
| **3** | Query workbench | v0.5+ (shipped); v0.13† (schema browser shipped); v1.1† (AI query) |
| **4** | Graph workspace | v0.7+ (shipped); v1.0 (layouts, filters); v1.1† (AI graph) |
| **5** | Reasoning experience | v0.9–v0.13† (store integration shipped); v1.0 (pipeline UI, history) |
| **6** | Semantic refactoring | v0.8+ (shipped); v1.0 (merge, batch, undo) |
| **7** | AI experience | v1.1 |
| **8** | Plugin platform | v0.14 (runtime shipped); v1.1† (AI provider API) |
| **9** | Collaboration | v0.10+ (diff); v0.13† (PR summary CLI shipped); v1.0 (review); v1.2 (GitHub UI) |
| **10** | OntoStudio desktop | v1.2† (marketplace); Post-1.2 (shell, native graph) |
| **11** | Ecosystem & docs | v0.11+ (guides); v1.2 (registry, templates) |
| **12** | Semantic engineering platform | Post-1.2 (browser, cloud, team workspaces) |

> **Note on v0.13–v0.18 (retired labels):** Earlier drafts used v0.13–v0.18 for capabilities that **shipped in v0.3–v0.11** (diagnostics, SQL virtual tables, refactoring, Ontologos reasoning, semantic diff, docs export). Those labels are retired. Forward work from v0.13 onward is defined in the phases below.

---

## Shipped releases (v0.1–v0.19)

### Era A — Engine foundation (v0.1–v0.4)

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

**UI phases delivered:** **1** (partial) — explorer trees, entity inspector, VS Code command palette.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | LSP process; workspace indexing command |
| **OntoCode / OntoUI** | VS Code extension skeleton; ontology explorer; class/property/individual trees; entity inspector; jump to source; hover, go-to-definition, document/workspace symbols |

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

**UI phases delivered:** **2** (partial) — editable entity inspector, simple axiom forms.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-owl` crate (Horned-OWL catalog bridge); patch-based write-back; create/edit/delete classes, properties, individuals; edit labels, comments, simple `SubClassOf`, deprecated flag; Oxigraph ↔ Horned-OWL consistency tests; CLI `ontocore patch`; LSP `ontocore/applyAxiomPatch` |
| **OntoCode / OntoUI** | Editable entity inspector |

**Exit criteria:** User can edit labels and simple subclass axioms in Turtle; catalog axioms for editing come from Horned-OWL.

**Dependencies:** `horned-owl`, `horned-functional` via `ontocore-owl` ([ADR-0013](design/adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0006](design/adr/0006-patch-based-write-back.md))

---

### Era B — IDE depth (v0.5–v0.8)

### v0.5 — Query workbench + Manchester MVP (shipped)

**Theme:** Query and author complex class expressions.

**UI phases delivered:** **3** — SQL/SPARQL editor, results table, query history, saved queries; Manchester editor MVP.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Manchester parse/serialize for `SubClassOf` and `EquivalentClasses`; LSP `ontocore/parseManchester`, `ontocore/query`, `ontocore/sparql` |
| **OntoCode / OntoUI** | SQL and SPARQL query webviews; saved queries, result export, query history; Manchester editor MVP |

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

**UI phases delivered:** **2** (partial), **4** (partial) — React entity inspector; class/property/import/neighborhood graphs.

Sub-phases: **v0.7a** (React foundation) → **v0.7** (graphs + inspector) → **v0.7b** (OBO + ROBOT).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-robot` wrappers (`validate`, `merge`, `report`); OBO format index; graph structure export via `petgraph` |
| **OntoCode / OntoUI** | `extension/webview-ui/` — Vite + React + TypeScript; typed `postMessage` protocol; CSP-compliant panel host; class/property/import/neighborhood graphs; entity inspector on React; OBO syntax highlighting and id rendering in explorer |

**Exit criteria:** User can navigate ontologies visually; biomedical maintainer can index OBO and run ROBOT in CI.

**Dependencies:** `react`, `vite` (extension); `fastobo`, `fastobo-owl`, `fastobo-validator`; ROBOT CLI ([ADR-0017](design/adr/0017-react-webview-ui.md))

---

### v0.8 — Refactoring + full Manchester (shipped)

**Theme:** Safe large-scale ontology maintenance and full OWL 2 DL expression authoring.

**UI phases delivered:** **6** — refactor preview panel; rename/find references in explorer.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Safe IRI rename, namespace migration, find usages, move entity, extract module; preview/apply refactor plan; full Manchester axiom catalog (restrictions, disjoint, property chains view) |
| **OntoCode / OntoUI** | Refactor preview panel; Query Workbench and Manchester editor migrated to React; LSP rename and find references |

**Exit criteria:** User can safely refactor ontology repositories and author full OWL 2 DL expression sets via hybrid UI.

**Dependencies:** `horned-owl`, `horned-functional`; React webview UI

---

### Era C — Platform & authoring (v0.9–v0.13)

### v0.9 — OntoCore platform identity (shipped)

**Theme:** Unified naming, public API, and DL reasoning.

**UI phases delivered:** **5** (partial) — reasoner panel; asserted/inferred/combined hierarchy; EL explanations.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Rename `ontoindex-*` → `ontocore-*`; CLI `ontocore`; LSP `ontocore-lsp` with `ontocore/*` methods; `ontocore` façade crate with `Workspace` API; Ontologos 1.0.0 integration (`dl` and `auto` adapters); plugin platform design ([PLUGIN_SPEC.md](https://github.com/eddiethedean/ontocode/blob/main/docs/design/PLUGIN_SPEC.md)) |
| **OntoCode / OntoUI** | Reasoner + explanation panels on React; legacy HTML webviews removed; OntoCore branding |

**Exit criteria:** Contributors and users distinguish OntoCore (engine) from OntoCode (IDE); DL/auto classification enabled.

**Dependencies:** Ontologos 1.0.0; breaking release for v0.8 integrators ([ADR-0018](design/adr/0018-ontocore-platform-identity.md))

---

### v0.10 — Semantic workspace (shipped)

**Theme:** Team-scale development workflows.

**UI phases delivered:** **9** (partial) — semantic diff React panel.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Incremental indexing (content-hash reuse); multi-root workspaces; stable `ontocore::Workspace` API; `ontocore-diff` crate; `ontocore diff` CLI; git ref compare; breaking-change detection; optional disk cache (`.ontocore/cache/`) |
| **OntoCode / OntoUI** | Semantic diff React panel; LSP `ontocore/semanticDiff`; multi-root folder support |

**Exit criteria:** User can diff ontology versions, work in multi-root workspaces, and reindex incrementally at scale.

**Dependencies:** `git2`, `horned-owl`, `pulldown-cmark`, `minijinja`

---

### v0.11 — Editor depth & distribution (shipped)

**Theme:** Close editor gaps and expand distribution.

**UI phases delivered:** **5** (quick fixes), **7** (partial — `ontocore docs` export), **11** (partial — enterprise adoption guides).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | LSP `textDocument/completion` (Turtle prefix, QName, IRI); diagnostic quick fixes (`undefined_prefix`, `missing_label`, `broken_import`); `ontocore-docs` crate; `ontocore docs` CLI (Markdown/HTML); `add_import` / `remove_import` patch ops; OBO read path via `fastobo` (synonyms, defs, xrefs); ADR for v1.0 OBO write-back ([ADR-0019](design/adr/0019-obo-write-back.md)) |
| **OntoCode / OntoUI** | Manage Imports panel; Open VSX publishing (Cursor); diagnostic code actions; entity inspector panel reuse on navigation; VS Code e2e tests |

**Exit criteria:** Daily Turtle editing, import management, and docs export work without leaving VS Code; extension available on VS Code Marketplace and Open VSX.

**Dependencies:** `fastobo`; `minijinja`, `pulldown-cmark`

---

### v0.12 — Authoring parity (shipped)

**Released:** v0.12.0 (2026-07-06)

**Theme:** Close remaining **P0** OWL and OBO authoring gaps from [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md).

**UI phases delivered:** **2** (P0 exit) — domain/range/characteristics forms, property chain editor, OBO edit forms, preview-before-apply on all axiom types.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Property domain, range, and characteristics authoring (patch ops); individual class/property assertions; expanded annotation assertion editing; property chain editing; full OBO write-back ([OBO_ROBOT_SPEC.md](design/OBO_ROBOT_SPEC.md), [ADR-0019](design/adr/0019-obo-write-back.md)); OWL/XML read support; Horned-OWL → Ontologos bridge improvements; axiom round-trip golden tests (Protégé fixtures) |
| **OntoCode / OntoUI** | Inspector forms for domain/range/characteristics; property chain editor; OBO write-back in inspector; DL clash-trace explanations via `ontologos-explain` + `ontologos-dl`; Turtle preview before apply for all axiom types |

**Exit criteria:** All **P0 — OWL 2 DL authoring** and **OBO & biomedical** rows in [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) are green.

**Dependencies:** `horned-owl`, `horned-functional`, `fastobo`, `fastobo-owl`; Ontologos `ontologos-explain`

---

### Era D — OntoUI platform (v0.13)

### v0.13 — Platform hardening (shipped)

**Released:** v0.13.0 (2026-07-08)

**Theme:** OntoUI platform foundation + OntoCore hardening for plugins (v0.14) and Protégé polish (v1.0).

**UI phases delivered:** **0**, **1**, partial **3** (schema browser), partial **5** (reasoning store integration), partial **9** (PR summary CLI). Checklist: [ROADMAP_MAPPING.md § v0.13](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ROADMAP_MAPPING.md)

| Area | Deliverables |
|------|--------------|
| **OntoUI** | `WorkspaceHost` adapter; Zustand `WorkspaceStore` + event bus; Current Focus + `FocusChanged`; `WorkspaceRegistry`; design tokens + shared primitives; extension-host `FocusRelayService`; Entity Inspector, Graph, Query Workbench, and Refactor Preview on store; schema browser in Query Workbench; refactor + reasoning store slices |
| **OntoCore** | Horned-OWL SQL virtual tables (`restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`, `domain_axioms`, `range_axioms`); LSP `ontocore/listSqlSchema`; `ontocore diff --pr-summary`; `.ontocore/diagnostics.toml`; LSP semantic tokens (Turtle/OBO); `ontocore docs` class hierarchy + property index; API stability policy; benchmark smoke tests (`tests/bench_index.rs`) |
| **OntoCode** | Cross-panel focus sync (explorer → inspector + graph); Vitest + extension integration tests for focus relay, schema browser, and store slices; UX audit ([UX_AUDIT_v0.13.md](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/UX_AUDIT_v0.13.md)) |

**Exit criteria (met):**

- [x] **Focus sync** — explorer selection relays to inspector and graph via extension-host focus relay
- [x] **Store ownership** — Entity, Graph, Query, and Refactor Preview workspaces consume WorkspaceStore
- [x] **Design system** — design tokens + shared primitives on Entity Inspector and Query Workbench
- [x] **Schema browser** — browse virtual tables/columns; insert snippets into Query Workbench editor
- [x] **Team workflow** — `ontocore diff A..B --pr-summary` emits PR-ready Markdown; documented and tested
- [x] **Performance** — benchmark fixtures + sizing guide update
- [x] **Quality** — 161 webview-ui Vitest tests + extension integration tests; accessibility pass on migrated panels
- [x] **API policy** — public `ontocore` API stability documented on path to 1.0

**Deferred to later releases:** persistent tabs + bottom dock, full panel component migration, SQL `JOIN`/`GROUP BY`, PR summary UI panel, validation report panel, Reasoner/Semantic Diff full store migration → **v1.0** / **v1.2** per original scope boundaries.

**Dependencies:** `sqlparser` + `horned-owl` ([ADR-0011](design/adr/0011-use-sqlparser-for-sql.md), [ADR-0013](design/adr/0013-dual-stack-oxigraph-horned-owl.md)); platform ADRs [0002–0004](adr/README.md)

---

### v0.14 — Plugin host MVP (shipped)

**Released:** v0.14.0 (2026-07-09)

**Theme:** External extensibility without embedding workflow engines in core.

**UI phases delivered:** **8** (plugin platform MVP). Milestone: [Product Roadmap 2.0 phase 8](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `PluginHost` runtime; manifest discovery from `.ontocore/plugins/`; reference plugins (naming validator, Markdown exporter, SHACL scaffold); CLI `plugins list/run`, `validate`/`docs` hooks; LSP `listPlugins`/`runPlugin`; subprocess workflow runner |
| **OntoUI** | Capability provider registry; inspector plugin cards; `WorkspaceStore` plugins slice |
| **OntoCode** | Plugin commands; owlmake workflow scaffold (`workflow run --plugin owlmake`); workflow output panel |
| **Ecosystem** | `examples/plugin-workspace/` fixture; [Plugin authoring guide](guides/plugins.md) |

**Exit criteria (met):**

- [x] Third party can ship a validation or export plugin without forking OntoCore (reference plugins + manifest schema)
- [x] owlmake can be invoked from OntoCode as an external workflow (subprocess scaffold)

**Dependencies:** `ontocore-plugin` crate; reference plugin binaries; [PLUGIN_SPEC.md](https://github.com/eddiethedean/ontocode/blob/main/docs/design/PLUGIN_SPEC.md)

---

### v0.15 — Plugin API + visualization parity + explanation workspace (shipped, partial)

**Released:** v0.15.0 (2026-07-08)

**Theme:** Extend the v0.14 plugin host with permissions, UI views, explanation alternatives, and graph asserted/inferred modes.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Plugin permissions + `api_version = "1"`; subprocess path-jail hardening; explanation alternatives + staleness metadata in LSP/CLI |
| **OntoUI** | Graph asserted/inferred/combined modes, layouts, search; explanation panel with multiple justifications |
| **OntoCode** | Plugin UI views (dockable panels); plugin commands; explanation staleness warnings |
| **Ecosystem** | `demo-ui-view.toml` fixture; updated [Plugin authoring guide](guides/plugins.md) |

**Exit criteria (partial):** dockable plugin views + commands shipped; graph modes + explanation alternatives shipped; preferences/context actions **shipped in v0.16**.

See [migration/v0.15.md](migration/v0.15.md) and [SHIPPED.md](SHIPPED.md).

---

### v0.16 — Workspace layouts + preferences + imports polish (shipped, partial)

**Released:** v0.16.0 (2026-07-09)

**Theme:** Close the “desktop shell” parity gap: plugin preferences, context actions, and imports/layout polish in the VS Code extension.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Plugin command dispatch via LSP `ontocore/runPlugin` (validator/export/workflow) |
| **OntoUI** | Plugin preferences pages and context actions surfaced from manifest contributions |
| **OntoCode** | **Plugins: Open Preferences…**, **Plugins: Run Context Action…**, **Reload Imports**, **Reset Layout** commands |
| **Ecosystem** | [v0.16 scope](design/v0.16_SCOPE.md); [migration/v0.16.md](migration/v0.16.md) |

**Exit criteria (partial):** P0 preferences/context actions/imports reload shipped; full layout persistence and workspace perspectives deferred to v0.17+.

See [migration/v0.16.md](migration/v0.16.md) and [SHIPPED.md](SHIPPED.md).

---

### v0.17 — Menus/toolbars/dialog parity (shipped, partial)

**Released:** v0.17.0 (2026-07-10)

See [v0.17 scope](design/v0.17_SCOPE.md), [migration/v0.17.md](migration/v0.17.md), and root [ROADMAP.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md).

---

### v0.18 — Desktop UX shell gate + migration readiness (shipped)

**Released:** v0.18.0 (2026-07-11); patches **v0.18.1** (2026-07-12), **v0.18.2** (2026-07-13)

**Theme:** Close the desktop UX shell gate (menus, layouts, workflows, migration docs). **Not** full functional Protégé parity — see [PRE_1_0_PHASES.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) for v0.19–v0.25.

**Scope (docs/audit-first):** [v0.18_SCOPE.md](design/v0.18_SCOPE.md) · [0.18 parity assessment](https://github.com/eddiethedean/ontocode/blob/main/docs/PROTEGE_REVERSE_ENGINEERING/ONTOCODE_PARITY/ONTOCODE_0.18_PROTEGE_PARITY_ASSESSMENT.md)

Canonical detail: root [ROADMAP.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) § v0.18. See [migration/v0.18.md](migration/v0.18.md).

---

### Era F — Full Protégé parity path (v0.19–v0.25) — in progress

**Canonical plan:** [PRE_1_0_PHASES.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md)

### v0.19 — Semantic foundation + program baseline (shipped)

**Released:** v0.19.0 (2026-07-13)

**Theme:** Freeze parity scope; route Turtle/OBO edits through semantic transactions (`ontocore-edit`).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `Transaction` / `SemanticChange`; LSP + CLI apply via adapters; invert/compose/validate |
| **Parity program** | Frozen scope; `parity/protege-desktop-parity.yaml` + CI validator; epics EPIC-001…011 |

See [migration/v0.19.md](migration/v0.19.md) · [SHIPPED.md](SHIPPED.md) · full detail in [ROADMAP.md § v0.19](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md).

---

### v0.20 — Workspace runtime (shipped)

**Status:** **Shipped** as tagged **v0.20.0**.

**Theme:** Workspace as central runtime for ontology state and transactions.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.20.md](migration/v0.20.md) · full detail in [ROADMAP.md § v0.20](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md).

---

### v0.21 — Required format write-back (shipped)

**Status:** **Shipped** as tagged **v0.21.0**.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.21.md](migration/v0.21.md).

---

### v0.22 — Complete OWL 2 authoring (shipped)

**Status:** **Shipped** as tagged **v0.22.0**.

**Theme:** Every P0 OWL 2 construct authorable across required formats.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.22.md](migration/v0.22.md) · [OWL2_AUTHORING_GAPS.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/06_SUBSYSTEMS/OWL2_AUTHORING_GAPS.md).

---

## Planned releases (v0.23 → v1.2+)

**Pre-1.0 parity phases remaining:** [PRE_1_0_PHASES.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) (v0.23–v0.25 → 1.0.0-rc → 1.0.0). Per-release detail: [ROADMAP.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) § Era F.

---

### Era G — Protégé replacement (v1.0)

### v1.0 — Protégé-competitive release (planned)

**Theme:** Production-ready Protégé replacement in VS Code.

**UI phases:** **1–6** exit polish, partial **9** (review workspace). Milestones: [Product Roadmap 2.0 phases 2–6](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md). Wireframes: [WORKSPACE_WIREFRAMES.md](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/WORKSPACE_WIREFRAMES.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | All [protege-parity P0 requirements](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/03_PARITY/PARITY_RELEASE_GATE.md) green; all **P1** items green or documented known gaps; stable CLI/API/LSP semver 1.0; `examples/protege-roundtrip/` ontology set with workflow doc; performance benchmarks published |
| **OntoUI** | **[1]** Persistent tabs + bottom dock ([WORKSPACE_WIREFRAMES](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/WORKSPACE_WIREFRAMES.md)). **[2]** Relationship cards, references view, metadata view; entity workspace diagnostics integration ([ENTITY_EDITOR_SPEC](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ENTITY_EDITOR_SPEC.md)). **[4]** Graph saved layouts, filters, reasoning overlays ([GRAPH_WORKSPACE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/GRAPH_WORKSPACE.md)). **[5]** Semantic build pipeline UI; entity-level reasoning cards; reasoning history; Problems ↔ reasoning integration ([REASONING_EXPERIENCE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/REASONING_EXPERIENCE.md)). **[6]** Merge classes; batch label normalization; undo/redo on refactor and patch writes ([SEMANTIC_REFACTORING](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/SEMANTIC_REFACTORING.md)). **[9]** Review workspace MVP ([COLLABORATION](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/COLLABORATION.md)). Supporting: HIG + keyboard shortcuts ([HUMAN_INTERFACE_GUIDELINES](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/HUMAN_INTERFACE_GUIDELINES.md), [KEYBOARD_SHORTCUTS](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/KEYBOARD_SHORTCUTS.md)) |
| **OntoCode** | Complete hybrid authoring loop (forms + Manchester + Turtle/OBO); full IDE surface (explorer, search, diagnostics, refactoring, query workbench, visualization, reasoning); React webview hardening; VS Code Marketplace + Open VSX publish as 1.0 |
| **Toolchain** | ODK project layout recognition (`src/ontology/`, catalog files, import structure); ODK QC and release workflow surfacing; ROBOT-compatible operations where practical; import existing ODK/ROBOT/owlmake workflows (Makefile, GitHub Actions); Protégé migration guide with honest parity table |
| **Ecosystem** | Ontologos 1.0.0 reasoner gate satisfied; published `ontocore` + `ontocore-*` 1.0.0 on crates.io |

**Already shipped (v0.5–v0.15):** entity editor MVP, query workbench, graph canvas (with asserted/inferred modes in v0.15), reasoner panel, refactoring preview, semantic diff, WorkspaceStore + focus relay, schema browser, plugin host MVP + plugin permissions/views — see [ROADMAP_MAPPING.md](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ROADMAP_MAPPING.md).

**Exit criteria:**

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features in [PARITY_SCOPE.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/PARITY_SCOPE.md).

Track implementation: [PRE_1_0_PHASES.md](https://github.com/eddiethedean/ontocode/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) · [v1.0_BACKLOG.md](design/v1.0_BACKLOG.md)

**Dependencies:** Ontologos 1.0.0; `react` / `vite` (extension `webview-ui`); [cursor-prompts/](https://github.com/eddiethedean/ontocode/blob/main/docs/cursor-prompts/README.md) 06–07, 11–12

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

### Era H — Ecosystem expansion (v1.1+)

### v1.1 — Language bindings & AI primitives (planned)

**Theme:** Cross-language integration and AI-native tooling foundations.

**UI phases:** **7** (primary), deferred AI from **2**, **3**, **4**, **8**, **9**. Milestone: [Product Roadmap 2.0 phase 7](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md). [ADR-0010](design/adr/0010-ai-features-opt-in.md).

Former roadmap labels **v0.17 (Language Bindings)** and **v0.18 (AI Platform)** are consolidated here.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Python SDK (workspace index, query, validate, diff); TypeScript SDK (LSP client helpers, webview protocol types); MCP server exposing workspace context (entities, axioms, diagnostics, query results) |
| **OntoUI** | **[7]** AI sidebar; inline suggestions; review ontology; repair diagnostics; project-wide AI tasks; MCP context bridge ([AI_EXPERIENCE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/AI_EXPERIENCE.md), [AI_ORCHESTRATION_ARCHITECTURE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/AI_ORCHESTRATION_ARCHITECTURE.md)). **[2†]** AI explain entity. **[3†]** AI query generation. **[4†]** AI graph explanations. **[8†]** AI provider API ([PLUGIN_API_SPEC](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLUGIN_API_SPEC.md)). **[9†]** AI review ([COLLABORATION](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/COLLABORATION.md)). All AI: read-only suggestions with preview/approval |
| **OntoCode** | MCP-driven semantic context for external AI tools; documentation generation hooks (extends v0.11 `ontocore docs`) |
| **Ecosystem** | Published SDK packages; MCP server installable via `cargo install` or pip |

**Exit criteria:** Python and TypeScript consumers can index and query ontologies without shelling to CLI; MCP clients can retrieve structured ontology context from an open workspace.

**Dependencies:** MCP protocol; PyO3 or subprocess bridge TBD in ADR; [cursor-prompts/09-add-ai-action-lifecycle.md](https://github.com/eddiethedean/ontocode/blob/main/docs/cursor-prompts/09-add-ai-action-lifecycle.md)

---

### v1.2 — Ontology Toolchain Platform (planned)

**Theme:** Mature external workflow integration beyond the reference owlmake plugin.

**UI phases:** **9**, **10**, **11**. Milestones: [Product Roadmap 2.0 phases 9 + 11](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Build API (compile/merge/materialize); Release API (version, tag, publish artifacts); Validation API (plug-in QC pipelines); plugin discovery and install from registry; semver-compatible plugin contracts |
| **OntoUI** | **[9]** GitHub integration; semantic PR summaries (UI panel); merge checks ([COLLABORATION](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/COLLABORATION.md)). **[10, 11]** Public plugin registry + marketplace UI; sample domain plugins ([PLUGIN_PLATFORM](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLUGIN_PLATFORM.md), [PLUGIN_SPEC](https://github.com/eddiethedean/ontocode/blob/main/docs/design/PLUGIN_SPEC.md)). **[11]** Community templates. Workflow / QC status dashboard |
| **OntoCode** | Production-ready owlmake plugin integration; QC reports (HTML/Markdown/JSON) in IDE |
| **Ecosystem** | Official GitHub Actions for ontology CI/CD; plugin marketplace; third-party workflow plugins |

**Exit criteria:** ODK-style release pipeline runnable end-to-end from OntoCode with discoverable, versioned plugins.

---

### Post-1.2 — Ecosystem modernization (planned)

**Theme:** Shift from Protégé parity to ecosystem leadership.

**UI phases:** **10**, **12**, plus collaboration items from **9**. Milestones: [Product Roadmap 2.0 phases 10–12](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Semantic workspace APIs (persistent semantic databases); plugin marketplace maturity; advanced graph analytics |
| **OntoUI** | **[10]** OntoStudio Tauri app shell; shared React UI (OntoCode + OntoStudio); native graph performance; local AI support; enterprise packaging ([ONTOSTUDIO_DESKTOP](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ONTOSTUDIO_DESKTOP.md), [GRAPH_RENDERING_ARCHITECTURE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/GRAPH_RENDERING_ARCHITECTURE.md), [COMPONENT_LIBRARY](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/COMPONENT_LIBRARY.md)). **[12]** Browser client; cloud sync; team workspaces; distributed reasoning; shared semantic canvases; governance workflows ([PLATFORM_ARCHITECTURE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLATFORM_ARCHITECTURE.md), [COLLABORATION](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/COLLABORATION.md), [GRAPH_WORKSPACE](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/GRAPH_WORKSPACE.md), [governance](guides/governance.md)). **[9, 12]** Live collaboration; ontology PR review; advanced visualization (large-graph layout, temporal diff) |
| **OntoCode** | AI-assisted ontology engineering (modeling suggestions, axiom completion); live collaboration; ontology review in pull requests |
| **Ecosystem** | Enterprise governance tooling; knowledge graph tooling integrations; documentation generators via plugin APIs |

**Strategic framing:** OntoCore provides the platform. owlmake (and peers) provide workflow, build, and release automation. OntoCode surfaces both through the UI. The goal is ecosystem collaboration — not absorbing or replacing every tool in the stack.

---

## Long-term goal

OntoCore becomes the foundation for modern ontology tooling.

OntoCode becomes the flagship IDE.

Ontologos becomes the flagship Rust reasoning engine.
