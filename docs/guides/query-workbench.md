# Query Workbench

Run **SQL** and **SPARQL** queries against your indexed workspace from VS Code.

## Open the workbench

1. **Command Palette** (`Ctrl+Shift+P` / `Cmd+Shift+P`) → **OntoCode: Open Query Workbench**
2. Or complete step 5 in [First success in 10 minutes](first-success.md)

The workbench opens in a side panel beside your editor.

## Modes

| Mode | Engine | Use for |
|------|--------|---------|
| **SQL** | OntoIndex virtual tables | Catalog queries (`classes`, `properties`, `diagnostics`, …) |
| **SPARQL** | Oxigraph over indexed triples | RDF graph patterns |

Toggle **Mode** at the top of the panel. Starter templates load when you switch modes.

### SQL quick start

```sql
SELECT short_name, labels FROM classes
```

Use the **Table** dropdown to insert `SELECT * FROM <table>`. Virtual table schemas: [SQL reference](../sql-reference.md).

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

The workbench calls the same LSP methods as the CLI:

```bash
ontoindex query /path/to/ontologies "SELECT short_name FROM classes"
ontoindex sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

## Limitations (v0.5)

- No SQL/SPARQL autocomplete in the workbench editor (planned v0.8+).
- Queries run against the **indexed catalog** — run **OntoCode: Index Workspace** after file changes.
- SQL subset only: no `JOIN`, `GROUP BY`, `ORDER BY`, or `LIMIT`. See [SQL reference](../sql-reference.md).

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Empty results | Re-index workspace; confirm ontology files parse cleanly |
| Query error | Check **Output → OntoIndex Language Server**; see [errors reference](../errors.md) |
| Stale results after edit | **Index Workspace** then re-run query |

More help: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).
