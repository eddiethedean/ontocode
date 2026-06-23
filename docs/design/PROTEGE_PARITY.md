# Protégé Parity Matrix (v1.0.0)

> **Canonical exit bar for v1.0.0.** All **P0** items must be green before release.
> See [ROADMAP.md](ROADMAP.md) v1.0 and [PLAN.md](PLAN.md) §4.

## Tier definitions

| Tier | Meaning |
|------|---------|
| **P0 — v1.0 blocker** | Must ship; v1.0 cannot release without green |
| **P1 — v1.0 target** | Expected at launch; documented known gaps if slipped |
| **P2 — post-1.0** | Explicitly out of v1.0 scope |

## P0 — v1.0 blockers

### OWL 2 DL authoring (hybrid UX)

| Item | Spec | v0.2 |
|------|------|------|
| Quick forms: labels, comments, deprecated | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | Partial (read-only inspector) |
| Quick forms: `SubClassOf`, domain, range, property characteristics | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | No |
| Manchester editor for complex class expressions | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | No |
| Axiom types: `SubClassOf`, `EquivalentClasses`, `DisjointClasses` | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | Partial (extract only) |
| Object/data property domain, range, characteristics | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | Partial |
| Class/object/data property assertions on individuals | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | Partial |
| Annotation assertions | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | Partial |
| Horned-OWL manipulation layer + round-trip tests | [ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md) | No (Oxigraph extraction only) |
| Patch-based write-back from OWL objects | [ADR-0006](adr/0006-patch-based-write-back.md) | No |

### Reasoning (Rust-native — [ADR-0014](adr/0014-rust-native-reasoners-only.md))

| Item | Spec | v0.2 |
|------|------|------|
| `el` adapter (OWL EL) | [REASONER_SPEC.md](REASONER_SPEC.md) | No |
| `dl` adapter (OWL 2 DL classification + consistency) | [REASONER_SPEC.md](REASONER_SPEC.md) | No — requires [OntoLogos 1.0.0](https://github.com/eddiethedean/ontologos) |
| Unsatisfiable class reporting | [REASONER_SPEC.md](REASONER_SPEC.md) | No |
| **Real** unsatisfiability explanations (clash-trace justification) | [REASONER_SPEC.md](REASONER_SPEC.md) | No |
| Asserted / inferred / combined hierarchy toggle | [REASONER_SPEC.md](REASONER_SPEC.md) | Asserted only |
| Consistency check | [REASONER_SPEC.md](REASONER_SPEC.md) | No |
| Zero JVM / Java dependency for reasoning | [ADR-0014](adr/0014-rust-native-reasoners-only.md) | Yes (no reasoner yet) |

### Editor & LSP

| Item | Spec | v0.2 |
|------|------|------|
| Diagnostics → Problems panel | [SPEC.md](SPEC.md) §9 | No |
| Completion | [SPEC.md](SPEC.md) §9 | No |
| Rename (safe IRI) | [SPEC.md](SPEC.md) §9 | No |
| Find references | [SPEC.md](SPEC.md) §9 | No |
| Code actions | [SPEC.md](SPEC.md) §9 | No |
| Semantic tokens | [SPEC.md](SPEC.md) §9 | No |
| Hover, go-to-definition, symbols | [docs/lsp-api.md](../lsp-api.md) | Yes |

### Workflow & platform

| Item | Spec | v0.2 |
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

| Item | Spec | v0.2 |
|------|------|------|
| OBO format read/write | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | No |
| ROBOT interop (`validate`, `merge`, `report`) | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | No |
| OBO id rendering in explorer / Manchester autocomplete | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | No |

## P1 — v1.0 targets

| Item | Spec |
|------|------|
| SHACL validation via adapter | [SHACL_SPEC.md](SHACL_SPEC.md) |
| SWRL rule **viewing** (authoring is P2) | [PROTEGE_PARITY.md](PROTEGE_PARITY.md) |
| `rl` / `rdfs` adapters (OWL 2 RL / RDFS) | [REASONER_SPEC.md](REASONER_SPEC.md) |
| Instance checking | [REASONER_SPEC.md](REASONER_SPEC.md) |
| Plugin API stable + 3 reference plugins | [PLUGIN_SPEC.md](PLUGIN_SPEC.md) |
| SQL joins and aggregations | [SPEC.md](SPEC.md) |
| Incremental workspace index | [ARCHITECTURE.md](ARCHITECTURE.md) |
| Performance benchmarks (large ontology targets) | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) |

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
