# Protégé coexistence

Interim guide for teams using Protégé today and evaluating OntoCode **v0.10**. A full migration guide is a **v1.0 deliverable**.

Canonical capability matrix: [What ships today](../SHIPPED.md). Decision matrix: [Protégé vs OntoCode](protege-decision.md). Gap analysis: [Protégé parity matrix](../design/PROTEGE_PARITY.md).

## Use OntoCode for (v0.10)

| Workflow | Status |
|----------|--------|
| Browse ontologies in VS Code | Shipped |
| Edit labels, comments, parents in Turtle | Shipped |
| Complex `SubClassOf` / `EquivalentClasses` / disjoint (Manchester) | Shipped |
| Workspace refactoring (rename, migrate namespace, move, extract) | Shipped (Turtle; preview + apply) |
| SQL/SPARQL queries over workspace | Shipped |
| Graph visualization (class, property, import, neighborhood) | Shipped |
| CI lint (`ontocore validate`) | Shipped — suitable for production CI |
| EL/RL/RDFS/DL classification | Shipped |
| OWL 2 DL classification (`dl` / `auto` profiles) | Shipped (OntoLogos 1.0.0) |
| Inferred hierarchy toggle | Shipped (after reasoner run) |
| OBO format index + `obo_id` in explorer | Shipped (write-back: Turtle only in VS Code) |
| ROBOT CLI in CI (`ontocore robot`) | Shipped (requires Java + `robot` on PATH) |
| Multi-root VS Code workspaces | Shipped (all folders indexed) |
| Semantic diff (CLI / LSP / panel) | Shipped |

## Keep Protégé for (today)

| Workflow | Why |
|----------|-----|
| Property chain **editing** | View-only in OntoCode axiom catalog until v1.0 |
| Full OBO **write-back** in VS Code | OBO is indexed/read-only in inspector; Turtle write-back only |
| Full OWL 2 DL axiom catalog | Partial Manchester + patches; see [Protégé parity](../design/PROTEGE_PARITY.md) |
| Editing RDF/XML or OWL/XML in place | OntoCode write-back is **Turtle only** |
| Workflows that depend on Protégé-specific plugins | Not replicated in OntoCode |

## Practical split workflow

1. **Author** routine Turtle changes in VS Code (inspector, Manchester editor, refactoring, patches)
2. **Validate** in CI with `ontocore validate` and optionally `ontocore classify --profile el`
3. **Run ROBOT** in CI when needed — [ROBOT interop](robot-interop.md)
4. **Review** DL-heavy axioms, property chains, or OBO-specific edits in Protégé when required
5. **Commit** `.ttl` changes through Git pull requests

## File format notes

- OntoCode indexes RDF/XML, JSON-LD, OBO, and N-Triples but **writes Turtle only**
- Prefer Turtle in Git for shared authoring; use Protégé round-trip when teams still maintain OWL/XML
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
