# What ships today (v0.19.0 — latest tagged)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.
>
> **Latest tagged release: v0.19.0** (Marketplace, crates.io, GitHub Releases). Documentation on `main` may describe unreleased **v0.20** work — see [Unreleased on main](#unreleased-on-main-v020-not-tagged) below. Pin installs: `cargo install ontocore-cli --locked --version 0.19.0`.

**Latest tagged: v0.19.0** · [v0.19 migration](migration/v0.19.md) · **Docs branch:** v0.20 in progress · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) · Draft [migration/v0.20.md](migration/v0.20.md)

## Products

| Product | What it is |
|---------|------------|
| **OntoCode** | VS Code IDE — explorer, React inspector, graphs (asserted/inferred modes), Query Workbench, Manchester editor, refactor preview, reasoner, explanation panel, plugin commands/views/preferences/context actions |
| **OntoCore** | Rust semantic workspace engine — `ontocore` façade, `ontocore-*` crates, `ontocore` CLI, `ontocore-lsp`, plugin host |

## Capability matrix (v0.19.0 tagged)

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
| Merge entities / replace entity references | Yes (preview + apply) | — (IDE only; not `ontocore refactor`) |
| New ontology scaffold / export (ROBOT convert or copy) | Yes | `ontocore new` / export LSP |
| Prefix manager / ontology metadata patches | Yes | `ontocore patch` |
| Active ontology selector | Yes | LSP `setActiveOntology` |
| Menus / toolbars / keybindings / perspectives | Yes | — |
| SQL-like queries | Query Workbench (React) + schema browser | `ontocore query` |
| SPARQL | Query Workbench (React) | `ontocore sparql` |
| Graph visualization (class, property, import, neighborhood) | Yes (React; asserted/inferred/combined; export JSON/CSV; expand depth) | LSP `ontocore/getGraph` |
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
| Plugin host (manifest + runtime) | Plugin commands, dockable views, inspector cards, preferences pages, context actions, plugin Problems diagnostics | `ontocore plugins` / `ontocore workflow` |
| Plugin permissions (`api_version = "1"`) | Enforced on plugin load/run | Enforced on CLI/LSP plugin host |
| Reference plugins (naming, Markdown export, SHACL scaffold) | Via validate + plugins | `ontocore plugins run` |

## Format support

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML (`.rdf`, `.xml`) | OWL/XML (`.owl`, `.owx`) | JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|--------------------------|--------------------------|---------------------------|
| Index / query | Yes | Yes | Yes (Horned catalog) | Yes (Horned catalog) | Yes |
| Write-back (inspector, patches) | Yes | Yes | Read-only | Read-only | Read-only |
| Refactor apply | Yes | — | — | — | — |
| Rich OBO metadata (synonyms, defs, xrefs) | — | Yes | — | — | — |

> **OBO versioning:** patch engine write-back since **v0.12**; Entity Inspector write-back since **v0.13**.

## New in v0.19.0 (latest tagged)

| Capability | Status |
|------------|--------|
| `ontocore-edit` semantic transactions (`compose`, `validate`, `invert`, serde) | Yes |
| Turtle / OBO LSP + CLI apply via `Transaction` (legacy patch JSON still accepted) | Yes |
| Protégé parity program baseline (manifest + CI validator) | Yes |

Full user-facing delta: [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md#0190---2026-07-13).

## Unreleased on main (v0.20 — not tagged)

These items are implemented on the `v0.20` branch but **not** in Marketplace / crates.io **0.19.0** yet:

| Capability | Status |
|------------|--------|
| Workspace runtime (registry, dirty/save, transactions, session persistence) | On branch |
| Turtle patch matching for lang-tagged / typed literals | On branch |
| Turtle patch IRI removes across CURIE and `<IRI>` forms | On branch |
| Token-aware type / characteristic detection (ignores comment substrings) | On branch |
| `SetOntologyIri` rewrites `rdf:type owl:Ontology` in place | On branch |

Draft migration: [migration/v0.20.md](migration/v0.20.md).

## Release history

Detailed notes for v0.9–v0.18 are in the [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md). This page lists **what is available in the latest tagged release**, not every past milestone.

## Manchester scope (v0.8+)

**Shipped:** named classes; `and` / `or`; `some` / `only`; `min` / `max` / `exact` cardinality; nested restrictions; `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` via Manchester editor or patch JSON; domain/range editing; **property chain editing** (v0.12) via inspector and patch JSON.

**Not shipped:** full DL axiom catalog for all formats, inline Manchester autocomplete in the text buffer. See [Protégé parity](design/PROTEGE_PARITY.md) for the v1.0 target.

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | **All folders indexed** (v0.10+), including peer folders added after open. Manual **Index Workspace** may prompt when multiple roots are open |
| Write-back | **Turtle (`.ttl`) and OBO (`.obo`)**; RDF/XML, OWL/XML, JSON-LD, N-Triples read-only |
| Refactoring | **Turtle (`.ttl`) only**; extract module uses direct-reference closure |
| Class hierarchy tree | Named-parent edges; **inferred/combined** after reasoner run |
| Reasoning | **EL / RL / RDFS / DL / auto** via Ontologos 1.0 (HermiT parity) |
| CLI release binaries | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| Scale | See [workspace limits](workspace-limits.md) (includes walk entry cap) |

## What's next

Forward milestones: workspace runtime tag (**v0.20**), RDF/XML + OWL/XML write-back (**v0.21**), full Protégé parity path (**v0.21–v0.25**), Protégé-competitive release (**1.0**). See **[Platform roadmap](roadmap.md)** · **[Known limitations](known-limitations.md)**.

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
