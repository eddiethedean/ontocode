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

## v0.4a — Simple write-back

Deliverables:

- create class / property / individual (basic)
- edit labels/comments
- delete entity
- patch-based write-back for annotations and simple `SubClassOf`
- source-location fidelity ([ADR-0006](adr/0006-patch-based-write-back.md))

Exit criteria:

- User can edit labels and simple subclass axioms without Protégé.

**Dependencies:** `ontoindex-owl` (planned); patch layer in-house per [ADR-0006](adr/0006-patch-based-write-back.md).

## v0.4b — Horned-OWL integration

Deliverables:

- `ontoindex-owl` crate ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md))
- Horned-OWL axiom model in catalog
- Oxigraph ↔ Horned-OWL consistency tests
- Protégé round-trip fixture suite started

Exit criteria:

- Catalog axioms for editing come from Horned-OWL, not triple grep.

**Dependencies:** `horned-owl`, `horned-functional` via `ontoindex-owl` ([ADR-0016](adr/0016-dependency-first-implementation.md) Appendix A).

## v0.5 — Query workbench + Manchester MVP

Deliverables:

- SQL query webview
- SPARQL query webview
- saved queries, result export, query history
- Manchester editor MVP: `SubClassOf` and `EquivalentClasses` complex expressions ([OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md))

Exit criteria:

- User can query ontologies in VS Code and edit complex subclass/equivalent axioms via Manchester.

**Dependencies:** `sqlparser`, `oxigraph`; `horned-functional`; optional `owl-ms-language-server` for Manchester assist.

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

## v0.7 — Visualization

Deliverables:

- class graph
- property graph
- import graph
- entity neighborhood graph
- graph filtering
- click node to inspect

Exit criteria:

- User can navigate ontology visually.

**Dependencies:** `petgraph` (graph structure export); layout/rendering in VS Code webview (TypeScript).

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

Exit criteria:

- User can safely refactor ontology repositories and author full OWL 2 DL expression sets via hybrid UI.

**Dependencies:** `horned-owl`, `horned-functional`; in-house refactor orchestration.

## v0.9 — Workflow and documentation

Deliverables:

- semantic diff ([SEMANTIC_DIFF_SPEC.md](SEMANTIC_DIFF_SPEC.md))
- Git branch comparison
- breaking change report
- **incremental workspace index** (required — [ARCHITECTURE.md](ARCHITECTURE.md))
- evaluate `ontologos-watch` for file-change → reclassify hook ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md))
- Markdown/HTML docs export
- PR summary generation

Exit criteria:

- User can use OntoCode in team development workflows at scale.

**Dependencies:** `git2`, `horned-owl`, `notify` or `ontologos-watch`, `pulldown-cmark`, `minijinja`.

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

Exit criteria:

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features in [PROTEGE_PARITY.md](PROTEGE_PARITY.md).

**External gate:** OntoLogos **1.0.0** published to crates.io with HermiT catalog parity complete ([OntoLogos ROADMAP](https://github.com/eddiethedean/ontologos/blob/main/ROADMAP.md)).

**Dependencies:** OntoLogos `1.0` (`ontologos-dl`, `ontologos-facade`); extended `sqlparser` joins or DataFusion if triggered ([ADR-0011](adr/0011-use-sqlparser-for-sql.md)); `rudof` for SHACL P1.

## Implementation sequencing

```text
v0.4a → v0.4b → v0.5 → v0.6 → v0.7 → v0.7b → v0.8 → v0.9 → v1.0
```
