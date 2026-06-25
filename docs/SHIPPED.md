# What ships today (v0.7.0)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.

**Current release:** v0.7.0 · [CHANGELOG](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)

## Products

| Product | What it is |
|---------|------------|
| **OntoCode** | VS Code extension — explorer, React inspector, graphs, Query Workbench, Manchester editor, reasoner |
| **OntoIndex** | Rust engine — `ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp` |

## Capability matrix

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl`, `.obo`) | Yes (React inspector) | `ontoindex patch` (Turtle) |
| Create / delete entities (`.ttl`) | Yes | `ontoindex patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes | `ontoindex patch` |
| SQL-like queries | Query Workbench | `ontoindex query` |
| SPARQL | Query Workbench | `ontoindex sparql` |
| Graph visualization (class, property, import, neighborhood) | Yes (React) | LSP `ontoindex/getGraph` |
| OWL EL classification (`el` profile) | Reasoner panel + hierarchy toggle | `ontoindex classify` |
| RL / RDFS classification | Reasoner panel | `ontoindex classify --profile rl\|rdfs` |
| EL explanations (where available) | Explanation panel | `ontoindex explain` |
| OBO format index + `obo_id` in explorer | Yes | `ontoindex inspect` |
| ROBOT interop | — | `ontoindex robot validate\|merge\|report` |
| Diagnostics / lint | Problems panel | `ontoindex validate` |
| Hover, go-to-definition, symbols | Yes | — |
| Patch preview | Inspector / Manchester editor | `ontoindex patch --preview` |
| React webview UI | Inspector + graphs | — |

## Format support

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML, JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|-----------------------------------|
| Index / query | Yes | Yes | Yes |
| Write-back (inspector, patches) | Yes | Read-only in VS Code | Read-only in VS Code |

## Manchester MVP scope (v0.5)

**Shipped:** named classes; `and` / `or`; `some` / `only`; `min` / `max` / `exact` cardinality; nested restrictions; `SubClassOf` and `EquivalentClasses` via Manchester editor or patch JSON.

**Not shipped:** disjoint axioms, property chains, full axiom catalog, inline Manchester autocomplete in the text buffer. See [Protégé parity](design/PROTEGE_PARITY.md) for the v1.0 target.

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | Only the **first** folder is indexed |
| Write-back | **Turtle only** |
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
| Turtle editing & patches | [Authoring](authoring.md) · [Patch reference](patch-reference.md) |
| CLI & CI | [Getting started](getting-started.md) · [CI integration](ci-integration.md) |
| LSP integrators | [LSP API](lsp-api.md) |
| Enterprise evaluation | [Enterprise evaluation](guides/enterprise-eval.md) |
