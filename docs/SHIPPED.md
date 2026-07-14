# What ships today (v0.22.0 — latest tagged)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.
>
> **Format write-back truth:** this page and [Supported formats](supported-formats.md) are the source of truth. Tier-1 user docs (README, Home, First success, FAQ, Evaluate pack, LSP/patch/CLI refs) must match them — see [Releasing — Tier-1 capability truth](releasing.md#documentation-sync-checklist-every-release).
>
> **Latest tagged release: v0.22.0** (crates.io, GitHub Releases; Marketplace/Open VSX may lag — see [Versions & channels](guides/versions-and-channels.md)). Pin installs: `cargo install ontocore-cli --locked --version 0.22.0`.

**Latest tagged: v0.22.0** · [v0.22 migration](migration/v0.22.md) · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)

## Products

| Product | What it is |
|---------|------------|
| **OntoCode** | VS Code IDE — explorer, React inspector, graphs (asserted/inferred modes), Query Workbench, Manchester editor, refactor preview, reasoner, explanation panel, plugin commands/views/preferences/context actions |
| **OntoCore** | Rust semantic workspace engine — `ontocore` façade, `ontocore-*` crates, `ontocore` CLI, `ontocore-lsp`, plugin host |

## Capability matrix (v0.22.0 tagged)

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`) | Yes (React inspector) | `ontocore patch` |
| Create / delete entities (`.ttl`, XML required formats) | Yes | `ontocore patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes (Turtle) | `ontocore patch` |
| Disjoint classes (author + view) | Yes (inspector + Manchester) | `ontocore patch` |
| Domain / range / characteristics / property chains | Yes (inspector + patch; Turtle) | `ontocore patch` |
| Individual assertions (class/object/data) | Yes (Turtle; class assertion on XML) | `ontocore patch` |
| Generic annotation assertions | Yes | `ontocore patch` |
| OBO term edit (name, synonym, def, is_a, …) | Yes (inspector) | `ontocore patch` |
| Find usages / rename IRI / namespace migration / move / extract module | Yes (preview + apply) | `ontocore refactor` |
| Merge entities / replace entity references | Yes (preview + apply) | — (IDE only; not `ontocore refactor`) |
| New ontology scaffold / export (ROBOT convert or copy) | Yes | `ontocore new` / export LSP |
| Prefix manager / ontology metadata patches | Yes | `ontocore patch` |
| Active ontology selector | Yes | LSP `setActiveOntology` |
| Workspace runtime (registry, dirty/save, transactions, session) | Yes | — |
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

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML (`.rdf`, `.owl`) | OWL/XML (`.owx`) | JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|--------------------------|------------------|---------------------------|
| Index / query | Yes | Yes | Yes (Horned catalog) | Yes (Horned catalog) | Yes |
| Write-back (inspector, patches) | Yes | Yes | Yes (Horned re-serialize) | Yes (Horned re-serialize) | Read-only |
| Refactor apply | Yes | — | — | — | — |
| Rich OBO metadata (synonyms, defs, xrefs) | — | Yes | — | — | — |

> **OBO versioning:** patch engine write-back since **v0.12**; Entity Inspector write-back since **v0.13**.  
> **XML write-back:** semantic fidelity (ADR-0021); not byte-identical to Protégé saves.  
> Deeper capability grid (Manchester, refactor, XML re-serialize): [Capabilities by format](guides/capabilities-by-format.md).

## New in v0.22.0 (latest tagged)

| Capability | Status |
|------------|--------|
| HasKey, DisjointUnion, inverse / equivalent / disjoint properties, sub-property hierarchy | Yes (Turtle + XML mutate) |
| Negative assertions; SameIndividual / DifferentIndividuals | Yes |
| Datatype entities + datatype definitions; axiom annotations | Yes (Turtle + XML; shared Manchester DataRange for facets / oneOf / and/or/not) |
| Catalog / EntityDetail listing for v0.22 axiom families (incl. nested axiom annotations) | Yes |
| Manchester `not`, `value`, `Self`, OneOf, data restrictions | Yes |
| Inspector UI for HasKey, DisjointUnion, Inverse, Same/Different, NegativeOPA (+ remove from listed cards) | Yes |

## Previously in v0.21.0

| Capability | Status |
|------------|--------|
| RDF/XML (`.owl` / `.rdf`) write-back via Horned serializers | Yes |
| OWL/XML (`.owx`) write-back via Horned serializers | Yes |
| Cross-format semantic comparator + Protégé edit/save/reload fixtures | Yes |
| Editable gates lifted for CLI / LSP / catalog / extension inspector | Yes |
| Session/TM, OBO/XML, focus, SQL, sameAs/prefix, plugin, Windows-path bug-fix cluster | Yes |

## Previously in v0.20.0

| Capability | Status |
|------------|--------|
| Workspace runtime (registry, dirty/save, transactions, session persistence, external-change recovery) | Yes |
| Turtle patch matching for lang-tagged / typed literals and `<IRI>` object forms | Yes |
| Token-aware type / characteristic detection (ignores comment substrings) | Yes |
| `SetOntologyIri` rewrites `rdf:type owl:Ontology` in place | Yes |

Full user-facing delta: [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md#0220---2026-07-14).

## Release history

Detailed notes for v0.9–v0.21 are in the [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md). This page lists **what is available in the latest tagged release**, not every past milestone.

## Manchester scope (v0.22)

**Shipped:** named classes; `and` / `or` / `not`; `some` / `only` / `value` / `Self`; OneOf `{…}`; `min` / `max` / `exact` cardinality; nested restrictions; data restrictions on xsd types; `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` via Manchester editor or patch JSON; domain/range; property chains; HasKey and remaining RBox/ABox ops via patch JSON / inspector.

**Not shipped:** inline Manchester autocomplete in the text buffer; SWRL. Remaining 1.0 targets: [known limitations](known-limitations.md) · [Protégé vs OntoCode](guides/protege-decision.md).

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | **All folders indexed** (v0.10+), including peer folders added after open. Manual **Index Workspace** may prompt when multiple roots are open |
| Write-back | **Turtle, OBO, RDF/XML, OWL/XML**; JSON-LD, N-Triples, TriG read-only. XML is semantic re-serialize (not byte-identical). See [Capabilities by format](guides/capabilities-by-format.md) |
| Refactoring | **Turtle (`.ttl`) only**; extract module uses direct-reference closure |
| Class hierarchy tree | Named-parent edges; **inferred/combined** after reasoner run |
| Reasoning | **EL / RL / RDFS / DL / auto** via Ontologos 1.0 (HermiT parity) |
| CLI release binaries | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| Scale | See [workspace limits](workspace-limits.md) (includes walk entry cap) |

## What's next

Forward milestones: reasoning/SWRL parity (**v0.23**), semantic services (**v0.24**), Protégé-competitive release (**1.0**). See **[Platform roadmap](roadmap.md)** · **[Known limitations](known-limitations.md)**.

## Where to learn more

| Topic | Guide |
|-------|-------|
| First success | [First success](guides/first-success.md) |
| Authoring | [Authoring](authoring.md) |
| OWL/XML & RDF/XML write-back | [OWL/XML workflow](guides/owl-xml-workflow.md) |
| OBO | [OBO authoring](ontocode/obo-authoring.md) |
| Versions | [Versions & channels](guides/versions-and-channels.md) |
