# Query Workbench

The **Query Workbench** is an OntoCode React panel for running **SQL-like** catalog queries and **SPARQL** against your indexed workspace. Queries execute in **OntoCore** via LSP (`ontocore/query`, `ontocore/sparql`).

!!! warning "Not Protégé DL Query"
    This is **not** Protégé’s DL Query tab. There is no Manchester class-expression “Instances / Subclasses / Superclasses” workflow here. Use SQL virtual tables or SPARQL today; dedicated DL Query UI is planned for **v0.24** — details: [DL Query vs Query Workbench](../guides/dl-query.md).

!!! warning "SQL-like, not full SQL"
    The SQL mode uses **virtual tables** with a small subset of SQL: single-table `SELECT`, limited `WHERE` (`=`, `!=`, `AND`/`OR`), no `JOIN`, `ORDER BY`, `GROUP BY`, `LIKE`, or functions.
    Full details: [SQL reference](../sql-reference.md).

## Open the workbench

1. **Command Palette** (`Ctrl+Shift+P` / `Cmd+Shift+P`) → **OntoCode: Open Query Workbench**
2. Or follow the optional **Query the workspace** section in [First success in 10 minutes](../guides/first-success.md)

## Modes

| Mode | Engine | Use for |
|------|--------|---------|
| **SQL** | OntoCore virtual tables | Catalog queries (`classes`, `properties`, `diagnostics`, …) |
| **SPARQL** | Oxigraph over indexed triples | RDF graph patterns |

Toggle **Mode** at the top of the panel. Starter templates load when you switch modes.

### SQL quick start

```sql
SELECT short_name, labels FROM classes
```

Virtual table schemas: [OntoCore SQL views](../ontocore/sql-views.md) · [SQL reference](../sql-reference.md).

### SPARQL quick start

```sparql
SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10
```

More examples: [SPARQL reference](../sparql-reference.md) · [Query cookbook](../examples/queries.md).

## Schema browser (v0.13)

When the workspace is indexed, the Query Workbench shows a collapsible **Schema** sidebar (SQL mode only):

1. Expand a table name to see columns and types.
2. Click a **column** to insert its name into the query editor.
3. Click **Insert table query** for `SELECT * FROM <table>`.

Schema metadata comes from LSP `ontocore/listSqlSchema`, including Horned-OWL axiom tables (`domain_axioms`, `range_axioms`, `restrictions`, …). See [SQL reference](../sql-reference.md#horned-owl-axiom-projections-v013).

## Cross-panel focus (v0.13)

Selecting an entity in the explorer updates **Current Focus** across open React panels (Inspector, Graph) via the extension-host focus relay. You do not need to re-select the same entity in each panel.

## Results

- Results appear in a table below the query editor.
- If the server row cap (100,000 rows) is hit, a banner shows **Results truncated at server row limit.**
- **Export CSV** or **Export JSON** copies the current result to the clipboard.

## Saved queries and history

- **Save Query** — name and store the current query in workspace state.
- **Saved** / **History** dropdowns reload prior queries.
- History length is controlled by the `ontocode.queryHistoryLimit` setting (default 20).

## CLI equivalent

```bash
ontocore query . "SELECT short_name, labels FROM classes"
ontocore sparql . "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

See [Rust & CLI guide](../guides/rust-crates.md).
