# DL Query

!!! note "Not Protégé DL Query for SQL/SPARQL modes"
    Query Workbench **SQL** and **SPARQL** modes are still not Protégé’s DL Query tab. Use the dedicated **DL** mode (or CLI / LSP) for Manchester class-expression queries with Instances / Subclasses / Superclasses / Equivalents.

OntoCode ships **Protégé-style DL Query** in Query Workbench **DL** mode (**v0.24+**): Manchester class expressions with **Instances** / **Subclasses** / **Superclasses** / **Equivalents** result tabs (asserted or inferred).

## What ships

| Surface | Detail |
|---------|--------|
| Query Workbench **DL** mode | Manchester class expression → Instances / Subclasses / Superclasses / Equivalents |
| CLI | `ontocore dl-query` |
| LSP | `ontocore/dlQuery` |

Related query surfaces:

| Need | Use today |
|------|-----------|
| Catalog questions (`SELECT … FROM classes`) | Query Workbench **SQL** mode — [SQL reference](../sql-reference.md) |
| Graph patterns | Query Workbench **SPARQL** mode — [SPARQL reference](../sparql-reference.md) |
| Inferred types / instance checks | `ontocore realize` / `ontocore check-instance` or LSP `ontocore/checkInstance` — [realize cookbook](../examples/realize.md) |
| Unsatisfiable classes | Reasoner panel / `ontocore classify` — [Reasoner guide](reasoner.md) |

## Open DL Query in VS Code

1. **Command Palette** → **OntoCode: Open Query Workbench**
2. Set **Mode** to **DL Query**
3. Enter a Manchester class expression (e.g. `Person and hasPet some Dog`)
4. Choose asserted or inferred, then run — results appear in the four tabs

**Asserted mode:** named-class instances come from asserted class assertions (including asserted subclasses). Anonymous expressions still need **inferred** mode for instances. Saved DL queries remember the asserted/inferred toggle.

## CLI

```bash
ontocore dl-query /path/to/ontologies "Person and hasPet some Dog" --profile dl
```

See [CLI reference](../cli-reference.md) and [v0.24 migration](../migration/v0.24.md).

## Protégé coexistence

Keep Protégé when you need HermiT-identical behavior or other gaps in [Known limitations](../known-limitations.md). See [Protégé coexistence](protege-coexistence.md) and [Migrating from Protégé](protege-migration.md).

## Related

- [Query Workbench](../ontocode/query-workbench.md)
- [Known limitations](../known-limitations.md)
- [FAQ](../faq.md)
