# What ships today (v0.8.0)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.

**Current release:** v0.8.0 · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)

## Products

| Product | What it is |
|---------|------------|
| **OntoCode** | VS Code extension — explorer, React inspector, graphs, Query Workbench, Manchester editor, refactor preview, reasoner |
| **OntoIndex** | Rust engine — `ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp` |

## Capability matrix

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl` only; `.obo` read-only in inspector) | Yes (React inspector) | `ontoindex patch` (Turtle) |
| Create / delete entities (`.ttl`) | Yes | `ontoindex patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes | `ontoindex patch` |
| Disjoint classes (author + view) | Yes (inspector + Manchester) | `ontoindex patch` |
| Domain / range / property chains (view) | Yes (axiom catalog) | via SQL / inspect |
| Find usages / rename IRI / namespace migration / move / extract module | Yes (preview + apply) | `ontoindex refactor` |
| SQL-like queries | Query Workbench (React) | `ontoindex query` |
| SPARQL | Query Workbench (React) | `ontoindex sparql` |
| Graph visualization (class, property, import, neighborhood) | Yes (React) | LSP `ontoindex/getGraph` |
| OWL EL classification (`el` profile) | Reasoner panel + hierarchy toggle | `ontoindex classify` |
| RL / RDFS classification | Reasoner panel | `ontoindex classify --profile rl\|rdfs` |
| EL explanations (where available) | Explanation panel | `ontoindex explain` |
| OBO format index + `obo_id` in explorer | Yes | `ontoindex inspect` |
| ROBOT interop | — | `ontoindex robot validate\|merge\|report` |
| Diagnostics / lint | Problems panel | `ontoindex validate` |
| Hover, go-to-definition, symbols, find references, rename | Yes | — |
| Patch preview | Inspector / Manchester editor / refactor preview | `ontoindex patch --preview` |
| React webview UI | Inspector, graphs, Query Workbench, Manchester editor, refactor preview | — |

## Format support

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML, JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|-----------------------------------|
| Index / query | Yes | Yes | Yes |
| Write-back (inspector, patches, refactor) | Yes | Read-only in VS Code | Read-only in VS Code |

## Manchester scope (v0.8)

**Shipped:** named classes; `and` / `or`; `some` / `only`; `min` / `max` / `exact` cardinality; nested restrictions; `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` via Manchester editor or patch JSON; domain/range and property chains in axiom catalog (chains view-only).

**Not shipped:** property chain editing, full DL axiom catalog, inline Manchester autocomplete in the text buffer. See [Protégé parity](design/PROTEGE_PARITY.md) for the v1.0 target.

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | Only the **first** folder is indexed |
| Write-back | **Turtle only** |
| Refactoring | **Turtle only**; extract module uses direct-reference closure |
| Class hierarchy tree | Named-parent edges; **inferred/combined** after reasoner run |
| Reasoning | **EL / RL / RDFS** via OntoLogos 0.9; **DL/auto** stubbed until OntoLogos 1.0 |
| CLI release binaries | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| Scale | See [workspace limits](workspace-limits.md) (includes walk entry cap) |

## Where to learn more

| Topic | Guide |
|-------|-------|
| VS Code onboarding | [First success in 10 minutes](guides/first-success.md) |
| Query Workbench | [Query Workbench guide](guides/query-workbench.md) |
| Reasoner | [Reasoner guide](guides/reasoner.md) |
| Manchester editor | [Manchester editor guide](guides/manchester-editor.md) |
| Turtle editing & patches | [Authoring](authoring.md) · [Patch reference](patch-reference.md) · [Refactoring](guides/refactoring.md) |
| CLI & CI | [Getting started](getting-started.md) · [CI integration](ci-integration.md) |
| Graph visualization | [Graph visualization guide](guides/graph-visualization.md) |
| OBO workflows | [OBO workflow guide](guides/obo-workflow.md) |
| ROBOT interop | [ROBOT interop guide](guides/robot-interop.md) |
| LSP integrators | [LSP API](lsp-api.md) · [Webview protocol](webview-protocol.md) |
| Enterprise evaluation | [Enterprise evaluation](guides/enterprise-eval.md) |
