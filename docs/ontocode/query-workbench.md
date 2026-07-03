# Query Workbench

The **Query Workbench** is an OntoCode React panel for running **SQL** and **SPARQL** queries against your indexed workspace. Queries execute in **OntoCore** via LSP (`ontocore/query`, `ontocore/sparql`).

## Open the workbench

1. **Command Palette** (`Ctrl+Shift+P` / `Cmd+Shift+P`) → **OntoCode: Open Query Workbench**
2. Or complete step 5 in [First success in 10 minutes](../guides/first-success.md)

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
