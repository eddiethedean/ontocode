# Protégé coexistence

Guide for teams using Protégé today and evaluating OntoCode **v0.22**. A [first-week migration guide](protege-migration.md) ships today. RDF/XML and OWL/XML write-back **shipped in v0.21** (semantic re-serialize, not byte-identical); v0.22 completes OWL 2 authoring depth — see [OWL/XML write-back](owl-xml-workflow.md), [What ships today](../SHIPPED.md), and [roadmap](../roadmap.md).

Canonical capability matrix: [What ships today](../SHIPPED.md). Decision matrix: [Protégé vs OntoCode](protege-decision.md). Gap analysis for adopters: [Known limitations](../known-limitations.md).

## Use OntoCode for (v0.22)

| Workflow | Status |
|----------|--------|
| Browse ontologies in VS Code | Shipped |
| Edit labels, comments, parents in Turtle | Shipped |
| Edit OBO terms (name, synonyms, defs, is_a, …) | Shipped (engine v0.12; inspector v0.13) |
| Edit RDF/XML / OWL/XML (core inspector + patch ops) | Shipped (v0.21; semantic re-serialize) |
| Complex `SubClassOf` / `EquivalentClasses` / disjoint (Manchester) | Shipped (richest on Turtle) |
| Property chain editing | Shipped (v0.12) |
| Workspace refactoring (rename, migrate namespace, move, extract) | Shipped (Turtle; preview + apply) |
| SQL/SPARQL queries over workspace | Shipped |
| Graph visualization (class, property, import, neighborhood) | Shipped |
| CI lint (`ontocore validate`) | Shipped — suitable for production CI |
| EL/RL/RDFS/DL classification | Shipped |
| OWL 2 DL classification (`dl` / `auto` profiles) | Shipped (OntoLogos 1.x) |
| Inferred hierarchy toggle | Shipped (after reasoner run) |
| OBO format index + `obo_id` in explorer | Shipped |
| ROBOT CLI in CI (`ontocore robot`) | Shipped (requires Java + `robot` on PATH) |
| Multi-root VS Code workspaces | Shipped (all folders indexed) |
| Semantic diff (CLI / LSP / panel) | Shipped |

## Keep Protégé for (today)

| Workflow | Why |
|----------|-----|
| Full OWL 2 DL axiom catalog for all formats | Partial Manchester + patches — see [SHIPPED](../SHIPPED.md) and [known limitations](../known-limitations.md) |
| Byte-identical OWL/XML or RDF/XML layout | OntoCode re-serializes for semantic fidelity ([ADR-0021](../design/adr/0021-deterministic-xml-serializers.md)) |
| Workflows that depend on Protégé-specific plugins | Not replicated in OntoCode |

## Practical split workflow

1. **Author** routine Turtle, OBO, and light XML changes in VS Code (inspector, Manchester editor, refactoring, patches)
2. **Validate** in CI with `ontocore validate` and optionally `ontocore classify --profile el`
3. **Run ROBOT** in CI when needed — [ROBOT interop](robot-interop.md)
4. **Keep Protégé** for Protégé-only plugins, byte-identical XML, or uncovered axiom types
5. Prefer Turtle when you need byte-stable diffs or refactor apply

## Round-trip tips

- After Protégé saves XML, re-index in OntoCode; expect layout differences, not semantic loss for edited entities/labels/parents/imports.
- Prefer Turtle for shared team authoring when Git diffs must stay readable.
- See [OWL/XML write-back](owl-xml-workflow.md) for supported XML patch ops.

## Related

- [Protégé migration (first week)](protege-migration.md)
- [Production readiness](production-readiness.md)
- [What ships today](../SHIPPED.md)
