# What ships today (v0.15.0)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.

**Current release:** v0.15.0 · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) · [Migration from v0.14](migration/v0.15.md)

## Products

| Product | What it is |
|---------|------------|
| **OntoCode** | VS Code IDE — explorer, React inspector, graphs (asserted/inferred modes), Query Workbench, Manchester editor, refactor preview, reasoner, explanation panel, plugin commands and views |
| **OntoCore** | Rust semantic workspace engine — `ontocore` façade, `ontocore-*` crates, `ontocore` CLI, `ontocore-lsp`, plugin host |

## Capability matrix

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl` and `.obo`) | Yes (React inspector) | `ontocore patch` |
| Create / delete entities (`.ttl`) | Yes | `ontocore patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes | `ontocore patch` |
| Disjoint classes (author + view) | Yes (inspector + Manchester) | `ontocore patch` |
| Domain / range / characteristics / property chains | Yes (inspector + patch) | `ontocore patch` |
| Individual assertions (class/object/data) | Yes (Turtle) | `ontocore patch` |
| Generic annotation assertions | Yes (Turtle) | `ontocore patch` |
| OBO term edit (name, synonym, def, is_a, …) | Yes (inspector) | `ontocore patch` |
| Find usages / rename IRI / namespace migration / move / extract module | Yes (preview + apply) | `ontocore refactor` |
| SQL-like queries | Query Workbench (React) + schema browser | `ontocore query` |
| SPARQL | Query Workbench (React) | `ontocore sparql` |
| Graph visualization (class, property, import, neighborhood) | Yes (React; asserted/inferred/combined modes, layouts, search) | LSP `ontocore/getGraph` |
| OWL EL classification (`el` profile) | Reasoner panel + hierarchy toggle | `ontocore classify` |
| RL / RDFS classification | Reasoner panel | `ontocore classify --profile rl\|rdfs` |
| OWL 2 DL classification (`dl` profile) | Reasoner panel + hierarchy toggle | `ontocore classify --profile dl` |
| Auto profile routing (`auto`) | Reasoner panel | `ontocore classify --profile auto` |
| EL / DL explanations (where available) | Explanation panel (multiple alternatives, staleness detection) | `ontocore explain` |
| OBO format index + `obo_id` in explorer | Yes | `ontocore inspect` |
| ROBOT interop | — | `ontocore robot validate\|merge\|report` |
| Diagnostics / lint | Problems panel | `ontocore validate` |
| Hover, go-to-definition, symbols, find references, rename | Yes | — |
| Turtle completion (prefix, QName, IRI) | Yes (LSP) | — |
| Diagnostic quick fixes (code actions) | Yes | — |
| Turtle imports add/remove | Yes (Manage Imports panel) | `ontocore patch` (`add_import`, `remove_import`) |
| Documentation export (Markdown / HTML) | — | `ontocore docs` |
| Patch preview | Inspector / Manchester editor / refactor preview / imports panel | `ontocore patch --preview` |
| Semantic diff (versions / workspace compare) | Semantic Diff panel (React) | `ontocore diff` / `--pr-summary` |
| Cross-panel focus sync | Explorer → Inspector + Graph (relay) | — |
| LSP semantic tokens (Turtle, OBO) | Editor highlighting | — |
| Configurable diagnostics | Problems panel + `.ontocore/diagnostics.toml` | `ontocore validate` |
| React webview UI | Inspector, graphs, Query Workbench, Manchester editor, refactor preview, semantic diff, imports | — |
| Plugin host (manifest + runtime) | Plugin commands, dockable views, inspector cards, plugin Problems diagnostics | `ontocore plugins` / `ontocore workflow` |
| Plugin permissions (`api_version = "1"`) | Enforced on plugin load/run | Enforced on CLI/LSP plugin host |
| Reference plugins (naming, Markdown export, SHACL scaffold) | Via validate + plugins | `ontocore plugins run` |

## Format support

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML (`.owl`), OWL/XML (`.owx`) | JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|-------------------------------------|---------------------------|
| Index / query | Yes | Yes | Yes (Horned catalog) | Yes |
| Write-back (inspector, patches, refactor) | Yes | Yes | Read-only | Read-only |
| Rich OBO metadata (synonyms, defs, xrefs) | — | Yes | — | — |

## New in v0.15.0

| Capability | Status |
|------------|--------|
| Plugin permissions enforcement (`workspace.read`, `workspace.write`, `external_process`) | Yes |
| Versioned plugin API (`api_version = "1"`) | Yes |
| Plugin UI views (dockable webview panels) | Yes |
| Plugin UI commands (command palette) | Yes |
| LSP `ontocore/runPlugin` `ui_view` action | Yes |
| Explanation alternatives (multiple justifications) | Yes |
| Explanation staleness metadata (`indexed_at`, `content_hash`) | Yes |
| Graph asserted / inferred / combined modes | Yes |
| Graph layouts (grid, circle, stack) and search | Yes |
| Subprocess plugin path-jail hardening | Yes |
| Multi-root index workspace root selection fix | Yes |

**Schema only (manifest defined, extension not wired):** `preferences_pages`, `context_actions` — see [Plugin authoring](guides/plugins.md).

## New in v0.14.0

| Capability | Status |
|------------|--------|
| Plugin manifest discovery (`.ontocore/plugins/*.toml`) | Yes |
| Plugin host runtime (in-process + subprocess) | Yes |
| Reference naming validator plugin | Yes |
| Reference Markdown exporter plugin | Yes |
| SHACL validator scaffold | Yes |
| `ontocore plugins list/run`, `ontocore workflow` | Yes |
| LSP `ontocore/listPlugins`, `ontocore/runPlugin` | Yes |
| OntoUI capability registry + WorkspaceStore plugins slice | Yes |
| owlmake workflow scaffold command | Yes |

## New in v0.13.0

| Capability | Status |
|------------|--------|
| WorkspaceStore + focus relay across webviews | Yes |
| Schema browser (LSP `listSqlSchema`) | Yes |
| Horned-OWL axiom SQL virtual tables | Yes |
| `ontocore diff --pr-summary` | Yes |
| `.ontocore/diagnostics.toml` rule config | Yes |
| LSP semantic tokens (Turtle, OBO) | Yes |
| Docs class hierarchy + property index | Yes |
| Design tokens + shared UI primitives | Yes |

## New in v0.12.0

| Capability | Status |
|------------|--------|
| Turtle domain/range/characteristics/chain/annotation patch ops | Yes |
| OBO write-back (`ontocore-obo`) | Yes |
| OWL/XML (`.owx`) and RDF/XML Horned catalog | Yes (read-only inspector) |
| DL unsatisfiability explanations | Yes (with EL/RL fallback) |
| Protégé round-trip golden tests | Yes (`cargo test protege_roundtrip`) |
| PreviewApplyBar on all inspector edits | Yes |

## New in v0.11.3

| Capability | Status |
|------------|--------|
| Entity Inspector reuses panel when navigating to another entity | Yes |
| VS Code e2e tests for inspector navigation and workspace commands | Yes |

## New in v0.11.2

| Capability | Status |
|------------|--------|
| Webview panel routing with pre-existing query params (Cursor/VS Code) | Yes |
| Entity Inspector panel recovery when webview stuck on Smoke fallback | Yes |

## New in v0.11.1

| Capability | Status |
|------------|--------|
| React webview `?panel=` bootstrap before React loads | Yes |

## New in v0.11.0

| Capability | Status |
|------------|--------|
| Open VSX publish (Cursor marketplace) | Yes |
| LSP `textDocument/completion` (Turtle) | Yes |
| LSP `textDocument/codeAction` (diagnostic quick fixes) | Yes |
| `ontocore docs` + `ontocore-docs` crate | Yes |
| Import patch ops + Manage Imports UI | Yes |
| OBO indexed via `fastobo` (read path) | Yes |
| OBO write-back (CLI/LSP patches) | v0.11 read-only in editor; **engine shipped v0.12**; **inspector write-back v0.13** — [OBO authoring](ontocode/obo-authoring.md) |

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
| **Ontologos 1.0 DL/auto classification** (`dl`, `auto` profiles) | Yes |
| OntoCore / OntoCode documentation trees | Yes |

## Manchester scope (v0.8+)

**Shipped:** named classes; `and` / `or`; `some` / `only`; `min` / `max` / `exact` cardinality; nested restrictions; `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` via Manchester editor or patch JSON; domain/range editing; **property chain editing** (v0.12) via inspector and patch JSON.

**Not shipped:** full DL axiom catalog for all formats, inline Manchester autocomplete in the text buffer. See [Protégé parity](design/PROTEGE_PARITY.md) for the v1.0 target.

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | **All folders indexed** (v0.10+). Manual **Index Workspace** may prompt when multiple roots are open |
| Write-back | **Turtle (`.ttl`) and OBO (`.obo`)**; RDF/XML, OWL/XML, JSON-LD, N-Triples read-only |
| Refactoring | **Turtle (`.ttl`) only**; extract module uses direct-reference closure |
| Class hierarchy tree | Named-parent edges; **inferred/combined** after reasoner run |
| Reasoning | **EL / RL / RDFS / DL / auto** via Ontologos 1.0 (HermiT parity) |
| CLI release binaries | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| Scale | See [workspace limits](workspace-limits.md) (includes walk entry cap) |

## What's next

Forward milestones (v0.15 plugin API → v1.0 Protégé replacement): **[Platform roadmap](roadmap.md)**.

## Where to learn more

| Topic | Guide |
|-------|-------|
| VS Code onboarding | [First success in 10 minutes](guides/first-success.md) |
| Query workbench | [Query Workbench](ontocode/query-workbench.md) |
| Reasoner & explanations | [Reasoner guide](guides/reasoner.md) |
| Plugin authoring | [Plugin authoring](guides/plugins.md) |
| Manchester editor | [Manchester editor](ontocode/manchester-editor.md) |
| Semantic diff | [Semantic diff guide](ontocode/semantic-diff.md) |
| Turtle editing & patches | [Authoring](authoring.md) · [Patch reference](patch-reference.md) · [Refactoring](guides/refactoring.md) |
| CLI & CI | [Getting started](getting-started.md) · [CI integration](ci-integration.md) |
| Graph visualization | [Graph view](ontocode/graph-view.md) |
| OBO workflows | [OBO workflow guide](guides/obo-workflow.md) |
| OWL/XML & RDF/XML (read-only) | [OWL/XML workflow](guides/owl-xml-workflow.md) |
| ROBOT interop | [ROBOT interop guide](guides/robot-interop.md) |
| LSP integrators | [LSP API](lsp-api.md) · [Webview protocol](webview-protocol.md) |
| Enterprise evaluation | [Enterprise evaluation](guides/enterprise-eval.md) |
