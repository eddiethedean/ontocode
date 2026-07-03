# ROADMAP.md

> v1.0 exit bar: [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — all **P0** items green.
>
> **Dependencies:** [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md) · [ADR-0016](adr/0016-dependency-first-implementation.md)

## v0.1 — OntoIndex Foundation

Deliverables:

- Rust workspace
- CLI skeleton
- recursive scanner
- file hashing
- parser adapters
- basic catalog
- `ontologies`, `classes`, `properties` tables
- basic SQL query
- basic SPARQL query

Exit criteria:

- User can run `ontoindex query ./repo "SELECT * FROM classes"`.

**Dependencies:** `oxigraph`, `sqlparser`, `ignore`, `clap`.

## v0.2 — OntoCode Explorer (shipped)

Deliverables:

- VS Code extension skeleton
- language server process
- workspace indexing command
- ontology explorer
- class/property/individual trees
- entity inspector
- jump to source

Exit criteria:

- User can browse an ontology repo in VS Code.

**Dependencies:** `lsp-server`, `lsp-types`, OntoIndex crates above.

## v0.3 — Diagnostics (shipped)

Deliverables:

- parse errors
- broken imports
- undefined prefixes
- duplicate labels
- missing labels
- orphan classes
- diagnostics table
- VS Code Problems integration

Exit criteria:

- User gets useful ontology diagnostics inline.

**Dependencies:** `oxigraph` (parse errors); in-house catalog lint rules in `ontoindex-diagnostics`. See [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

## v0.4 — Write-back + Horned-OWL (shipped as v0.4.0)

Deliverables:

- create class / property / individual (basic)
- edit labels/comments, simple `SubClassOf`, deprecated flag
- delete entity
- patch-based write-back for Turtle ([ADR-0006](adr/0006-patch-based-write-back.md))
- `ontoindex-owl` crate — Horned-OWL catalog bridge ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md))
- Oxigraph ↔ Horned-OWL consistency tests
- LSP `ontoindex/applyAxiomPatch`, CLI `ontoindex patch`
- Editable Entity Inspector in VS Code

Exit criteria:

- User can edit labels and simple subclass axioms in Turtle without Protégé.
- Catalog axioms for Turtle editing come from Horned-OWL.

**Dependencies:** `horned-owl`, `horned-functional` via `ontoindex-owl` ([ADR-0016](adr/0016-dependency-first-implementation.md)).

User docs: [docs/authoring.md](../authoring.md), [docs/patch-reference.md](../patch-reference.md).

## v0.5 — Query workbench + Manchester MVP (shipped as v0.5.0)

Deliverables:

- SQL query webview
- SPARQL query webview
- saved queries, result export, query history
- Manchester editor MVP: `SubClassOf` and `EquivalentClasses` complex expressions ([OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md))

Exit criteria:

- User can query ontologies in VS Code and edit complex subclass/equivalent axioms via Manchester.

**Dependencies:** `sqlparser`, `oxigraph`; Manchester parse/serialize in `ontoindex-owl` (catalog pickers for assist; `owl-ms-language-server` deferred).

## v0.6 — Reasoning

Deliverables:

- `ontoindex-reasoner` crate — thin facade over [OntoLogos](https://github.com/eddiethedean/ontologos) **0.9.0** ([REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0014](adr/0014-rust-native-reasoners-only.md), [ADR-0015](adr/0015-adopt-ontologos-reasoner.md))
- `el` adapter → `ontologos-el` (OWL EL classification)
- `rl` / `rdfs` adapters → `ontologos-rl` / `ontologos-rdfs` (P1)
- profile detection via `ontologos-profile`
- unsatisfiable classes (EL scope in 0.9.0)
- inferred hierarchy view (asserted / inferred / combined toggle)
- **explanation panel** — EL-first via `ontologos-explain` (DL clash traces deferred to v1.0 / OntoLogos 1.0.0)

Exit criteria:

- User can classify EL ontologies, see inferred hierarchy, and get EL explanations where available.
- `dl` adapter stubbed with clear UI until OntoLogos 1.0.0 ships on crates.io.

**Dependencies:** OntoLogos `ontologos-*` `0.9` ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md)); transitive `reasonable`, `horned-owl`, `petgraph` via OntoLogos — do not depend directly.

## v0.7a — React webview foundation

Deliverables:

- `extension/webview-ui/` — Vite + React + TypeScript ([ADR-0017](adr/0017-react-webview-ui.md), [OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md))
- Typed `postMessage` protocol between extension host and webviews
- Panel host, CSP nonce framework, bundled assets (no CDNs)
- `npm run build:webview` integrated into VSIX packaging
- First React panel shell (smoke panel or entity inspector scaffold)

Exit criteria:

- VSIX builds include React assets; at least one webview loads the React bundle with Marketplace-compliant CSP.
- Extension host ↔ React message contract documented.

**Dependencies:** `react`, `react-dom`, `vite` (extension `webview-ui` only); extension host remains TypeScript orchestration only.

## v0.7 — Visualization

Deliverables:

- class graph
- property graph
- import graph
- entity neighborhood graph
- graph filtering
- click node to inspect
- **Entity Inspector on React stack** (migrate from legacy HTML webview)

Exit criteria:

- User can navigate ontology visually.
- Entity inspector and new graph panels use the v0.7a React foundation.

**Dependencies:** `petgraph` (graph structure export); React layout/rendering in `extension/webview-ui` ([ADR-0017](adr/0017-react-webview-ui.md)).

## v0.7b — OBO & ROBOT interop

Deliverables:

- OBO format read/write ([OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md))
- `ontoindex robot validate|merge|report` wrappers
- OBO id rendering in explorer and Manchester autocomplete
- `examples/obo-workflow/` fixture repo

Exit criteria:

- Biomedical maintainer can edit OBO in VS Code and run ROBOT in CI alongside OntoCode.

**Dependencies:** `fastobo`, `fastobo-owl`, `fastobo-validator`; [ROBOT](https://github.com/ontodev/robot) CLI via `ontoindex-robot`.

## v0.8 — Refactoring + full Manchester

Deliverables:

- safe IRI rename, namespace migration, find usages, move entity, extract module
- full Manchester axiom catalog: restrictions, disjoint, property chains view
- preview changes
- **Query Workbench + Manchester Editor on React stack** (migrate from legacy HTML webviews)

Exit criteria:

- User can safely refactor ontology repositories and author full OWL 2 DL expression sets via hybrid UI.
- Query workbench and Manchester editor run in React panels.

**Dependencies:** `horned-owl`, `horned-functional`; in-house refactor orchestration; React webview UI ([ADR-0017](adr/0017-react-webview-ui.md)).

## v0.9 — OntoCore Identity

Deliverables:

- **OntoCore** platform branding and documentation (`docs/ontocore/`, `docs/ontocode/`)
- `ontocore` façade crate with experimental `Workspace` API ([ADR-0018](adr/0018-ontocore-platform-identity.md))
- Architecture diagram and responsibility split updates
- `ontoindex-*` crate names unchanged; CLI remains `ontoindex`; LSP remains `ontoindex-lsp`

Exit criteria:

- Contributors and users can distinguish OntoCore (engine) from OntoCode (IDE).
- Rust embedders can depend on `ontocore` as the public entry point.

**Dependencies:** existing `ontoindex-*` crates; no breaking API changes.

## v0.10 — OntoCore Public API + workflow

Deliverables:

- Stabilize `ontocore::Workspace` and ergonomic APIs; docs.rs for `ontocore`
- `ontocore` CLI alias (alongside `ontoindex`)
- semantic diff ([SEMANTIC_DIFF_SPEC.md](SEMANTIC_DIFF_SPEC.md))
- Git branch comparison and breaking change report
- **incremental workspace index** (required — [ARCHITECTURE.md](ARCHITECTURE.md))
- evaluate `ontologos-watch` for file-change → reclassify hook ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md))
- Markdown/HTML docs export; PR summary generation
- **Reasoner + Explanation panels on React stack**; semantic diff panel in React

Exit criteria:

- User can use OntoCode in team development workflows at scale.
- Reasoner, explanation, and semantic diff panels use the React webview stack.

**Dependencies:** `git2`, `horned-owl`, `notify` or `ontologos-watch`, `pulldown-cmark`, `minijinja`; React webview UI ([ADR-0017](adr/0017-react-webview-ui.md)).

## v1.0.0 — Protégé-competitive release

Deliverables:

- All [PROTEGE_PARITY.md](PROTEGE_PARITY.md) **P0** items green
- Bump `ontologos-*` to **1.0.0** — enable `dl` and `auto` adapters ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md))
- DL classification + clash-trace explanations via `ontologos-dl` + `ontologos-explain`
- Stable CLI/API/LSP
- VS Code Marketplace publish
- Migration guide from Protégé (honest parity table; cite OntoLogos supported constructs)
- `examples/protege-roundtrip/` ontology set
- Performance benchmarks document
- **React webview hardening** — accessibility, integration tests, legacy HTML panels removed ([OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md) phase 7)

Exit criteria:

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features in [PROTEGE_PARITY.md](PROTEGE_PARITY.md).
> All production webview panels run on the React stack with Marketplace-compliant CSP.

**External gate:** OntoLogos **1.0.0** published to crates.io with HermiT catalog parity complete ([OntoLogos ROADMAP](https://github.com/eddiethedean/ontologos/blob/main/ROADMAP.md)).

**Dependencies:** OntoLogos `1.0` (`ontologos-dl`, `ontologos-facade`); extended `sqlparser` joins or DataFusion if triggered ([ADR-0011](adr/0011-use-sqlparser-for-sql.md)); `rudof` for SHACL P1; `react` / `vite` (extension `webview-ui`).

## Implementation sequencing

```text
v0.4a → v0.4b → v0.5 → v0.6 → v0.7a → v0.7 → v0.7b → v0.8 → v0.9 → v0.10 → v1.0
```
