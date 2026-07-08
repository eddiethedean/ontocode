# Protégé coexistence

Guide for teams using Protégé today and evaluating OntoCode **v0.14**. A [first-week migration guide](protege-migration.md) ships today; extended OWL/XML-heavy playbooks remain **v1.0 targets**.

Canonical capability matrix: [What ships today](../SHIPPED.md). Decision matrix: [Protégé vs OntoCode](protege-decision.md). Gap analysis: [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## Use OntoCode for (v0.14)

| Workflow | Status |
|----------|--------|
| Browse ontologies in VS Code | Shipped |
| Edit labels, comments, parents in Turtle | Shipped |
| Edit OBO terms (name, synonyms, defs, is_a, …) | Shipped (engine v0.12; inspector v0.13) |
| Complex `SubClassOf` / `EquivalentClasses` / disjoint (Manchester) | Shipped |
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
| Full OWL 2 DL axiom catalog for all formats | Partial Manchester + patches; OWL/XML write-back not shipped — see [Protégé parity](../design/PROTEGE_PARITY.md) |
| Editing RDF/XML or OWL/XML in place | OntoCode write-back is **Turtle and OBO only** |
| Workflows that depend on Protégé-specific plugins | Not replicated in OntoCode |

## Practical split workflow

1. **Author** routine Turtle and OBO changes in VS Code (inspector, Manchester editor, refactoring, patches)
2. **Validate** in CI with `ontocore validate` and optionally `ontocore classify --profile el`
3. **Run ROBOT** in CI when needed — [ROBOT interop](robot-interop.md)
4. **Review** OWL/XML-heavy modules or Protégé-specific plugins in Protégé when required
5. **Share** `.ttl` and `.obo` changes through pull requests when your team uses version control

## File format notes

- OntoCode indexes RDF/XML, JSON-LD, OBO, and N-Triples; **writes Turtle and OBO**
- Prefer Turtle or OBO for shared authoring; use Protégé round-trip when teams still maintain OWL/XML
- Example round-trip fixtures: `examples/protege-roundtrip/` in the repository

## Expectations on reasoning

OntoCore uses separate Oxigraph/Horned-OWL and OntoLogos models. Results **may differ from Protégé** on partial OWL mappings — check profile warnings in the Reasoner Results panel and run a pilot comparison on your corpus — [Production evidence protocol](production-evidence.md).

## Evaluation checklist

1. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
2. Run the [production evidence protocol](production-evidence.md) on your ontology corpus
3. Run CI validation — [CI integration](../ci-integration.md)
4. Compare [Protégé decision matrix](protege-decision.md) and [Protégé parity matrix](../design/PROTEGE_PARITY.md) against your requirements
5. Review [enterprise evaluation](enterprise-eval.md) with platform/security teams

## Related

- [Protégé vs OntoCode decision matrix](protege-decision.md)
- [FAQ](../faq.md) — Protégé comparison
- [Reasoner guide](reasoner.md)
- [Refactoring guide](refactoring.md)
- [Authoring](../authoring.md)
