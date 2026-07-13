# Implementation architecture

> **Audience:** Contributors only — crate layout, dependency rules, and internal diagrams.
>
> **Which architecture doc?**
>
> | Read this | When |
> |-----------|------|
> | [Platform architecture](../architecture.md) | Evaluators and adopters — ecosystem responsibilities |
> | **This page** (`design/ARCHITECTURE.md`) | Crate layout, internal modules, dependency rules |
> | [OntoCore architecture](../ontocore/architecture.md) | Short stack summary (links here for crate detail) |
>
> **Do not use this page to evaluate shipped product features.** See [What ships today](../SHIPPED.md) and [Platform architecture](../architecture.md).
>
> **Ecosystem architecture (canonical for users):** [Platform architecture](../architecture.md).
>
> **Shipped through v0.19:** workspace scanner, incremental indexing, multi-root workspaces, SQL/SPARQL queries (including Horned-OWL axiom virtual tables), diagnostics (`.ontocore/diagnostics.toml`), CLI, LSP (semantic tokens, `listSqlSchema`), Turtle and OBO write-back, Query Workbench + schema browser, Manchester editor, React webviews with WorkspaceStore + focus relay (inspector, graphs, workbench, refactor preview, semantic diff, imports, reasoner, explanation), EL–DL reasoning (OntoLogos 1.0), OBO index + OBO write-back, OWL/XML read-only catalog, ROBOT CLI wrappers, semantic diff (`--pr-summary`), `ontocore docs` export, Turtle completion, diagnostic quick fixes, property chain editing, DL unsatisfiability explanations (alternatives + stale detection), plugin host (permissions, UI views/commands/preferences/context actions), Protégé-shell menus/perspectives, reasoner cancel + distinct lifecycle, layout reopen-with-context. See [What ships today](../SHIPPED.md).
>
> **Shipped v0.14–v0.17:** plugin host MVP → permissions/views → preferences/context actions → Protégé-shell. **Planned v1.0:** full workflow plugin integration, OWL/XML write-back, full DL axiom catalog for all formats. See [Platform roadmap](../roadmap.md).
>
> **Reference:** [lsp-api.md](../lsp-api.md), [adr/README.md](adr/README.md), [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

## 1. Architecture Goals

The architecture must support:

- Fast local indexing
- **Incremental updates (v0.9 requirement)**
- Multiple ontology syntaxes (including OBO at v0.7b)
- Queryable semantic catalog
- Editor integration
- Reasoner integration
- Safe write-back to source files
- Semantic diffing
- Plugin ecosystem (v0.14 MVP; v0.15 permissions/views; full workflow integration v1.0)

## 2. High-Level Architecture

```text
External Workflow Plugins (not core)     owlmake · ROBOT/ODK adapters
          │
          ▼
+---------------------------+
|        OntoCode UI        |
| VS Code trees + commands  |
| React webviews (v0.7a+)   |
+-------------+-------------+
              | ontocore-lsp
              v
+---------------------------+
|   OntoCore Language       |
|        Server             |
+-------------+-------------+
              |
              v
+---------------------------+
|       OntoCore Core       |
| catalog/query/diagnostics |
| diff/docs/reasoner/robot  |
| plugin platform (v0.14 MVP; v0.15 API extensions) |
+------+--------+-----------+
       |        |           |
       v        v           v
+-----------+ +-----------+ +------------------+
| Oxigraph  | | HornedOWL | | OntoLogos 1.0    |
| RDF/SPARQL| | OWL axioms| | reasoning        |
+-----+-----+ +-----+-----+ +--------+---------+
      |               |              |
      +───────┬───────┘              |
              v                      |
+---------------------------+        |
|     Workspace Files       |◄───────┘
| owl/rdf/ttl/jsonld/obo    |
+---------------------------+
```

**Reasoning ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md)):** `ontocore-reasoner` delegates to [OntoLogos](https://github.com/eddiethedean/ontologos) crates (`ontologos-el`, `ontologos-rl`, `ontologos-dl`, etc.). OntoLogos **1.0.0** ships in OntoCore v0.9 for DL/auto profiles.

**Workflow plugins:** [owlmake](https://github.com/INCATools/owlmake) is the reference external workflow plugin — see [PLUGIN_SPEC.md](PLUGIN_SPEC.md), [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md). OntoCore does not embed workflow engines.

**Sync rule ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md)):** Catalog entities/axioms for edit and diff come from Horned-OWL; triple counts and SPARQL from Oxigraph; CI consistency tests detect drift.

**Dependency policy:** [ADR-0016](adr/0016-dependency-first-implementation.md), [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

## 3. OntoCore Crate Layout

Full crate map with dependency edges: [crate-map.md](../ontocore/crate-map.md).

| Crate | Role | External dependency |
|-------|------|---------------------|
| `ontocore` | Public façade API (`Workspace`, module re-exports) | composes below |
| `ontocore-core` | Types, scanner, limits, path jail | `ignore` |
| `ontocore-parser` | RDF parse, entity extraction | `oxigraph` |
| `ontocore-owl` | OWL axiom facade, patch write-back | `horned-owl` |
| `ontocore-catalog` | Index builder, entity API | — |
| `ontocore-query` | SQL virtual tables, SPARQL | `sqlparser`, `oxigraph` |
| `ontocore-diagnostics` | Lint rules, LSP diagnostics | `regex`, `fastobo-validator` |
| `ontocore-diff` | Semantic diff, Git compare | `horned-owl`, `git2` |
| `ontocore-docs` | Markdown/HTML export | `pulldown-cmark`, `minijinja` |
| `ontocore-reasoner` | Reasoner facade | OntoLogos 1.0.0 |
| `ontocore-refactor` | Workspace refactoring | — |
| `ontocore-robot` | ROBOT CLI wrappers | ROBOT CLI (external) |
| `ontocore-lsp` | Language server + diagnostics + patch apply | `lsp-server`, `lsp-types` |
| `ontocore-cli` | `ontocore` binary | composes above |

## 4. OntoCore Internal Modules

### 4.1 Workspace Scanner
- Recursive discovery, ignore rules, format detection, content hashing, dependency tracking, change detection

### 4.2 Parser Layer (dual)
- **Oxigraph:** RDF parse, triple store, SPARQL ([ADR-0003](adr/0003-use-oxigraph.md))
- **Horned-OWL (v0.4.0+):** OWL 2 axiom model via `horned-owl` + `horned-functional` ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0016](adr/0016-dependency-first-implementation.md))
- **OBO (v0.7b+):** `fastobo`, `fastobo-owl`

### 4.3 Catalog Layer
- Ontologies, entities, axioms (from Horned-OWL), annotations, imports, diagnostics

### 4.4 Query Layer
- **v0.2:** `ontocore-query` — `sqlparser` virtual tables + Oxigraph SPARQL
- **v1.0:** joins/aggregations via extended virtual tables first; DataFusion if triggered ([ADR-0011](adr/0011-use-sqlparser-for-sql.md) amendment)

### 4.5 Diagnostics Layer (v0.3+)
- Oxigraph parse errors + in-house catalog lint rules; `diagnostics` SQL table; LSP Problems panel ([DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md))

### 4.6 Diff Layer (v0.9+)
- Horned-OWL axiom diff (in-house logic); `git2` for branch/commit inputs; PR summaries

### 4.7 Docs Layer (v0.9+)
- `pulldown-cmark` + `minijinja` templates; entity pages

### 4.8 Reasoner Layer (v0.6+)
- `ontocore-reasoner` thin facade over OntoLogos — see [REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0014](adr/0014-rust-native-reasoners-only.md), [ADR-0015](adr/0015-adopt-ontologos-reasoner.md)
- v0.6: `ontologos-*` 0.9.0 (`el`, `rl`, `rdfs`, `explain`)
- v1.0: `ontologos-*` 1.0.0 (`dl`, `facade`, full DL explanations)
- v0.9: optional `ontologos-watch` for incremental reclassify

## 5. OntoCode Internal Modules

### 5.1 Extension Host

Commands, tree views, LSP client, settings, webview lifecycle, `postMessage` bridge to React panels.

### 5.2 React Webview Application (v0.7a+)

Per [ADR-0017](adr/0017-react-webview-ui.md) and [OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md):

```text
extension/
  src/                    # extension host (TypeScript)
    extension.ts
    webviews/
      panelHost.ts
      messages.ts
  webview-ui/             # React app (Vite build → dist/)
    src/
      panels/
      components/
```

- Vite bundles all assets locally (Marketplace CSP: nonces, no CDNs).
- Typed message protocol: extension host ↔ React; ontology calls go through LSP only.
- VS Code theme variables for light/dark/high-contrast.

### 5.3 Tree Views

Explorer (asserted/inferred toggle), diagnostics, query history.

### 5.4 Webviews

| Webview | Status | UI stack |
|---------|--------|----------|
| Entity inspector | **Shipped** (v0.4) | Legacy HTML → **React** (v0.7) |
| Query workbench | **Shipped** (v0.5) | Legacy HTML → **React** (v0.8) |
| Manchester axiom editor | **Shipped** (v0.5 MVP) | Legacy HTML → **React** (v0.8) |
| Reasoner panel | **Shipped** (v0.6) | Legacy HTML (React migration planned) |
| Explanation panel | **Shipped** (v0.6 EL) | Legacy HTML (React migration planned) |
| Graph visualization | **Shipped** (v0.7 React) | Extend filters / layout |
| Semantic diff | **Shipped** (v0.10) | **React** |
| Manage Imports | **Shipped** (v0.11) | **React** |

### 5.5 Language Client

LSP lifecycle; protocol guards; v1.0 authoring methods.

## 6. Index Lifecycle

1. User opens workspace → LSP starts
2. Scanner discovers files → parse via Oxigraph + Horned-OWL
3. Catalog updated → diagnostics published
4. UI refreshes → queries/reasoner/refactor use catalog
5. **v0.9:** incremental update on content hash change only

## 7. Write-Back Architecture

Per [ADR-0006](adr/0006-patch-based-write-back.md) and [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md):

- Patches from Horned-OWL axiom objects
- Preserve comments and formatting
- Preview for multi-file refactors
- VS Code undo stack

## 8. Performance Strategy

- Incremental index by content hash (**v0.9 required**)
- Parallel parse independent files
- In-memory catalog; optional disk cache
- Lazy-load inferred views after reasoner run
- Non-blocking extension host

## 9. Security Model

- Local-first by default ([ADR-0005](adr/0005-local-first-by-default.md))
- No telemetry by default
- Workspace trust for `lspPath`, `ontocode.reasoner.default`, `robotPath`
- See [security.md](../security.md)
