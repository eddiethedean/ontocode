# Protégé Parity Matrix (v1.0.0)

> **Canonical exit bar for v1.0.0.** All **P0** items must be green before release.
> See [ROADMAP.md](ROADMAP.md) v1.0 and [PLAN.md](PLAN.md) §4.
>
> **Dependencies:** [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md)

## Tier definitions

| Tier | Meaning |
|------|---------|
| **P0 — v1.0 blocker** | Must ship; v1.0 cannot release without green |
| **P1 — v1.0 target** | Expected at launch; documented known gaps if slipped |
| **P2 — post-1.0** | Explicitly out of v1.0 scope |

## P0 — v1.0 blockers

### OWL 2 DL authoring (hybrid UX)

| Item | Spec | Dependency | v0.3 |
|------|------|------------|------|
| Quick forms: labels, comments, deprecated | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | Partial (read-only inspector) |
| Quick forms: `SubClassOf`, domain, range, property characteristics | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | No |
| Manchester editor for complex class expressions | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-functional` | No |
| Axiom types: `SubClassOf`, `EquivalentClasses`, `DisjointClasses` | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | Partial (extract only) |
| Object/data property domain, range, characteristics | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | Partial |
| Class/object/data property assertions on individuals | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | Partial |
| Annotation assertions | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | Partial |
| Horned-OWL manipulation layer + round-trip tests | [ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md) | `horned-owl`, `horned-functional` | No (Oxigraph extraction only) |
| Patch-based write-back from OWL objects | [ADR-0006](adr/0006-patch-based-write-back.md) | in-house patches | No |

### Reasoning (Rust-native — [ADR-0014](adr/0014-rust-native-reasoners-only.md))

| Item | Spec | Dependency | v0.3 |
|------|------|------------|------|
| `el` adapter (OWL EL) | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-el` | No |
| `dl` adapter (OWL 2 DL classification + consistency) | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-dl` 1.0.0 | No |
| Unsatisfiable class reporting | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos | No |
| **Real** unsatisfiability explanations (clash-trace justification) | [REASONER_SPEC.md](REASONER_SPEC.md) | `ontologos-explain` + `ontologos-dl` | No |
| Asserted / inferred / combined hierarchy toggle | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos | Asserted only |
| Consistency check | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos | No |
| Zero JVM / Java dependency for reasoning | [ADR-0014](adr/0014-rust-native-reasoners-only.md) | — | Yes (no reasoner yet) |

### Editor & LSP

| Item | Spec | v0.3 |
|------|------|------|
| Diagnostics → Problems panel | [SPEC.md](SPEC.md) §9 | Yes |
| Completion | [SPEC.md](SPEC.md) §9 | No |
| Rename (safe IRI) | [SPEC.md](SPEC.md) §9 | No |
| Find references | [SPEC.md](SPEC.md) §9 | No |
| Code actions | [SPEC.md](SPEC.md) §9 | No |
| Semantic tokens | [SPEC.md](SPEC.md) §9 | No |
| Hover, go-to-definition, symbols | [docs/lsp-api.md](../lsp-api.md) | Yes |

### Workflow & platform

| Item | Spec | v0.3 |
|------|------|------|
| Imports management UI | [SPEC.md](SPEC.md) | No |
| SQL + SPARQL query workbench | [SPEC.md](SPEC.md) | CLI only |
| Semantic diff + Git branch compare | [SEMANTIC_DIFF_SPEC.md](SEMANTIC_DIFF_SPEC.md) | No |
| Safe IRI rename across workspace | [ROADMAP.md](ROADMAP.md) v0.8 | No |
| Graph visualization (class, property, import, neighborhood) | [ROADMAP.md](ROADMAP.md) v0.7 | No |
| Documentation export (Markdown + HTML) | [ROADMAP.md](ROADMAP.md) v0.9 | No |
| CI validation command | CLI `validate` | Yes |
| VS Code Marketplace publish | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) | No (VSIX from Releases) |
| Migration guide from Protégé | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) | No |

### OBO & biomedical

| Item | Spec | Dependency | v0.3 |
|------|------|------------|------|
| OBO format read/write | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | `fastobo`, `fastobo-owl` | No |
| ROBOT interop (`validate`, `merge`, `report`) | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | ROBOT CLI | No |
| OBO id rendering in explorer / Manchester autocomplete | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | `fastobo` | No |

## P1 — v1.0 targets

| Item | Spec | Dependency |
|------|------|------------|
| SHACL validation via adapter | [SHACL_SPEC.md](SHACL_SPEC.md) | `rudof` |
| SWRL rule **viewing** (authoring is P2) | [PROTEGE_PARITY.md](PROTEGE_PARITY.md) | in-house |
| `rl` / `rdfs` adapters (OWL 2 RL / RDFS) | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-rl`, `ontologos-rdfs` |
| Instance checking | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-abox` (1.0+) |
| Plugin API stable + 3 reference plugins | [PLUGIN_SPEC.md](PLUGIN_SPEC.md) | — |
| SQL joins and aggregations | [SPEC.md](SPEC.md) | `sqlparser` extend; DataFusion TBD |
| Incremental workspace index | [ARCHITECTURE.md](ARCHITECTURE.md) | `notify` / `ontologos-watch` |
| Performance benchmarks (large ontology targets) | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) | — |

## P2 — post-1.0

| Item | Notes |
|------|-------|
| WebProtégé / collaborative editing | [PLAN.md](PLAN.md) §9 non-goal |
| Full SWRL authoring | |
| Protégé plugin compatibility | |
| Protégé-scale plugin marketplace | v1.0 ships API + reference plugins only |
| Reimplementing ROBOT | Interop only per [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) |

## Honest scope statement

**“Compete with Protégé” at v1.0 means:**

- Primary IDE for **Git-native ontology engineering** (general OWL 2 DL + OBO maintenance).
- **Hybrid authoring** (forms + Manchester) and **real reasoning** match Protégé’s core loop.
- **Git semantic diff, CI, and VS Code integration** exceed Protégé.

**It does not mean:**

- Every Protégé tutorial works unchanged.
- SWRL authoring, WebProtégé, or the full Protégé plugin catalog.
- Bit-for-bit identical results to HermiT on every ontology (Rust DL engine is test-validated, not JVM-cross-checked).

## v1.0 exit criterion

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features listed above.

Track implementation in [v1.0_BACKLOG.md](v1.0_BACKLOG.md).
