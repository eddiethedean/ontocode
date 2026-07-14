# DL Query vs Query Workbench

!!! warning "Not Protégé DL Query"
    OntoCode’s **Query Workbench is not** Protégé’s **DL Query** tab. There is no Manchester class-expression workflow with Instances / Subclasses / Superclasses / Equivalent classes result tabs.

## What ships today

| Need | Use today |
|------|-----------|
| Catalog questions (`SELECT … FROM classes`) | Query Workbench **SQL** mode — [SQL reference](../sql-reference.md) |
| Graph patterns | Query Workbench **SPARQL** mode — [SPARQL reference](../sparql-reference.md) |
| Inferred types / instance checks | `ontocore realize` / `ontocore check-instance` or LSP `ontocore/checkInstance` — [realize cookbook](../examples/realize.md) |
| Unsatisfiable classes | Reasoner panel / `ontocore classify` — [Reasoner guide](reasoner.md) |

## What is planned

Dedicated **DL Query** UI (Protégé-style class expressions over the reasoner) is planned for **v0.24** — see [Platform roadmap](../roadmap.md).

## Protégé coexistence

Keep Protégé open for DL Query during pilots. See [Protégé coexistence](protege-coexistence.md) and [Migrating from Protégé](protege-migration.md).

## Related

- [Query Workbench](../ontocode/query-workbench.md)
- [Known limitations](../known-limitations.md)
- [FAQ](../faq.md)
