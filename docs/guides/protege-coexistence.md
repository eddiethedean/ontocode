# Protégé coexistence

Interim guide for teams using Protégé today and evaluating OntoCode **v0.6**. A full migration guide is a v1.0 deliverable.

Canonical capability matrix: [What ships today](../SHIPPED.md). Gap analysis: [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## Use OntoCode for

| Workflow | v0.6 support |
|----------|--------------|
| Browse ontologies in VS Code | Yes |
| Edit labels, comments, simple parents in Turtle | Yes |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester MVP) | Yes |
| SQL/SPARQL queries over workspace | Yes |
| CI lint (`ontoindex validate`) | Yes |
| EL/RL/RDFS classification | Yes |
| Inferred hierarchy toggle | Yes (after reasoner run) |

## Keep Protégé for (today)

| Workflow | Why |
|----------|-----|
| Full OWL 2 DL reasoning (`dl` / `auto`) | OntoLogos 1.0 not shipped |
| Disjoint axioms, property chains, full axiom catalog | v0.8–v1.0 target |
| OBO id workflows and ROBOT interop | v0.7b target |
| Editing RDF/XML or OWL/XML in place | OntoCode write-back is Turtle-only |

## Practical split workflow

1. **Author** routine Turtle changes in VS Code (inspector, Manchester editor, patches)
2. **Validate** in CI with `ontoindex validate` and optionally `ontoindex classify --profile el`
3. **Review** complex DL axioms or OBO-specific edits in Protégé when needed
4. **Commit** `.ttl` changes through Git pull requests

## File format notes

- OntoCode indexes RDF/XML, JSON-LD, and N-Triples but **writes Turtle only**
- Round-trip through Protégé: save as Turtle for Git-native workflows when possible
- Example round-trip fixtures: `examples/protege-roundtrip/` in the repository

## Expectations on reasoning

OntoIndex maintains separate Oxigraph/Horned-OWL and OntoLogos models in v0.6. Results may differ from Protégé on partial OWL mappings — check profile warnings in the Reasoner Results panel.

## Evaluation checklist

1. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
2. Run CI validation — [CI integration](../ci-integration.md)
3. Compare [Protégé parity matrix](../design/PROTEGE_PARITY.md) against your requirements
4. Review [enterprise evaluation](enterprise-eval.md) with platform/security teams

## Related

- [FAQ](../faq.md) — Protégé comparison
- [Reasoner guide](reasoner.md)
- [Authoring](../authoring.md)
