# What ships today (v0.11.1)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.

**Current release:** v0.11.1 · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)

## Products

| Product | What it is |
|---------|------------|
| **OntoCode** | VS Code IDE — explorer, React inspector, graphs, Query Workbench, Manchester editor, refactor preview, reasoner |
| **OntoCore** | Rust semantic workspace engine — `ontocore` façade, `ontocore-*` crates, `ontocore` CLI, `ontocore-lsp` |

## Capability matrix

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl` only; `.obo` read-only in inspector) | Yes (React inspector) | `ontocore patch` (Turtle) |
| Create / delete entities (`.ttl`) | Yes | `ontocore patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes | `ontocore patch` |
| Disjoint classes (author + view) | Yes (inspector + Manchester) | `ontocore patch` |
| Domain / range / property chains (view) | Yes (axiom catalog) | via SQL / inspect |
| Find usages / rename IRI / namespace migration / move / extract module | Yes (preview + apply) | `ontocore refactor` |
| SQL-like queries | Query Workbench (React) | `ontocore query` |
| SPARQL | Query Workbench (React) | `ontocore sparql` |
| Graph visualization (class, property, import, neighborhood) | Yes (React) | LSP `ontocore/getGraph` |
| OWL EL classification (`el` profile) | Reasoner panel + hierarchy toggle | `ontocore classify` |
| RL / RDFS classification | Reasoner panel | `ontocore classify --profile rl\|rdfs` |
| OWL 2 DL classification (`dl` profile) | Reasoner panel + hierarchy toggle | `ontocore classify --profile dl` |
| Auto profile routing (`auto`) | Reasoner panel | `ontocore classify --profile auto` |
| EL explanations (where available) | Explanation panel | `ontocore explain` |
| OBO format index + `obo_id` in explorer | Yes | `ontocore inspect` |
| ROBOT interop | — | `ontocore robot validate\|merge\|report` |
| Diagnostics / lint | Problems panel | `ontocore validate` |
| Hover, go-to-definition, symbols, find references, rename | Yes | — |
| Turtle completion (prefix, QName, IRI) | Yes (LSP) | — |
| Diagnostic quick fixes (code actions) | Yes | — |
| Turtle imports add/remove | Yes (Manage Imports panel) | `ontocore patch` (`add_import`, `remove_import`) |
| Documentation export (Markdown / HTML) | — | `ontocore docs` |
| Patch preview | Inspector / Manchester editor / refactor preview / imports panel | `ontocore patch --preview` |
| Semantic diff (versions / workspace compare) | Semantic Diff panel (React) | `ontocore diff` |
| React webview UI | Inspector, graphs, Query Workbench, Manchester editor, refactor preview, semantic diff, imports | — |

## Format support

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML, JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|-----------------------------------|
| Index / query | Yes | Yes | Yes |
| Write-back (inspector, patches, refactor) | Yes | Read-only in VS Code | Read-only in VS Code |
| Rich OBO metadata (synonyms, defs, xrefs) | — | Yes (fastobo read) | — |

## New in v0.11.1

| Capability | Status |
|------------|--------|
| React webview panel routing (Entity Inspector, Query Workbench, etc.) | Yes |

## New in v0.11.0

| Capability | Status |
|------------|--------|
| Open VSX publish (Cursor marketplace) | Yes |
| LSP `textDocument/completion` (Turtle) | Yes |
| LSP `textDocument/codeAction` (diagnostic quick fixes) | Yes |
| `ontocore docs` + `ontocore-docs` crate | Yes |
| Import patch ops + Manage Imports UI | Yes |
| OBO indexed via `fastobo` (read path) | Yes |
| OBO write-back | Documented (ADR-0019); Turtle-only in editor |

## New in v0.10.0

| Capability | Status |
|------------|--------|
| Incremental workspace indexing (content-hash reuse) | Yes |
| Multi-root VS Code / LSP workspaces | Yes |
| Stable `ontocore::Workspace` API | Yes |
| `ontocore-diff` + `ontocore diff` CLI | Yes |
| LSP `ontocore/semanticDiff` + VS Code panel | Yes |
| Optional `.ontocore/cache/` disk index cache | Yes |

## New in v0.9.0

| Capability | Status |
|------------|--------|
| `ontocore` façade crate on crates.io | Yes |
| `Workspace::open` experimental API | Yes |
| **`ontocore-*` crate rename** (from `ontoindex-*`) | Yes |
| **OntoLogos 1.0 DL/auto classification** (`dl`, `auto` profiles) | Yes |
| OntoCore / OntoCode documentation trees | Yes |

## Manchester scope (v0.8+)

**Shipped:** named classes; `and` / `or`; `some` / `only`; `min` / `max` / `exact` cardinality; nested restrictions; `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` via Manchester editor or patch JSON; domain/range and property chains in axiom catalog (chains view-only).

**Not shipped:** property chain editing, full DL axiom catalog, inline Manchester autocomplete in the text buffer. See [Protégé parity](design/PROTEGE_PARITY.md) for the v1.0 target.

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | **All folders indexed** (v0.10+). Manual **Index Workspace** may prompt when multiple roots are open |
| Write-back | **Turtle only** |
| Refactoring | **Turtle only**; extract module uses direct-reference closure |
| Class hierarchy tree | Named-parent edges; **inferred/combined** after reasoner run |
| Reasoning | **EL / RL / RDFS / DL / auto** via OntoLogos 1.0 (HermiT parity) |
| CLI release binaries | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| Scale | See [workspace limits](workspace-limits.md) (includes walk entry cap) |

## What's next

Forward milestones (v0.12 diagnostics & quality → v1.0 Protégé replacement): **[Platform roadmap](roadmap.md)**.

## Where to learn more

| Topic | Guide |
|-------|-------|
| VS Code onboarding | [First success in 10 minutes](guides/first-success.md) |
| Query workbench | [Query Workbench](ontocode/query-workbench.md) |
| Reasoner | [Reasoner guide](guides/reasoner.md) |
| Manchester editor | [Manchester editor](ontocode/manchester-editor.md) |
| Semantic diff | [Semantic diff guide](ontocode/semantic-diff.md) |
| Turtle editing & patches | [Authoring](authoring.md) · [Patch reference](patch-reference.md) · [Refactoring](guides/refactoring.md) |
| CLI & CI | [Getting started](getting-started.md) · [CI integration](ci-integration.md) |
| Graph visualization | [Graph view](ontocode/graph-view.md) |
| OBO workflows | [OBO workflow guide](guides/obo-workflow.md) |
| ROBOT interop | [ROBOT interop guide](guides/robot-interop.md) |
| LSP integrators | [LSP API](lsp-api.md) · [Webview protocol](webview-protocol.md) |
| Enterprise evaluation | [Enterprise evaluation](guides/enterprise-eval.md) |
