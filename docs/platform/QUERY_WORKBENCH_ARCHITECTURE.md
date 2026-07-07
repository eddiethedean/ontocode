# Query workbench architecture

> **Status:** Partial (v0.12) — SQL/SPARQL editor shipped; schema browser planned v0.13

## Scope

Query Workspace: editor → LSP execution → results table → history/saved queries.

## Flow

```text
User edits SQL or SPARQL
    ↓
ontocore/query or ontocore/sparql (LSP)
    ↓
Result rows + truncated flag
    ↓
Results table + export
    ↓
QueryExecuted event → WorkspaceStore
```

## Current implementation

- `extension/webview-ui/src/panels/QueryWorkbench.tsx`
- LSP methods in [lsp-api.md](../lsp-api.md)
- SQL subset: [sql-reference.md](../sql-reference.md)

## Store shape (planned)

```ts
interface QueryState {
  language: "sql" | "sparql"
  text: string
  lastResult: QueryResult | null
  history: QueryHistoryEntry[]
  saved: SavedQuery[]
  schemaBrowserExpanded: boolean
}
```

## Schema browser (planned v0.13)

- Virtual table list from catalog snapshot
- Column names and types for SQL tables
- Insert into editor on click

## Links

- [ui/QUERY_WORKBENCH.md](../ui/QUERY_WORKBENCH.md)
- [ontocode/query-workbench.md](../ontocode/query-workbench.md)
- [cursor-prompts/08-improve-query-workbench.md](../cursor-prompts/08-improve-query-workbench.md)

## Evolution

UX spec: [ui/QUERY_WORKBENCH.md](../ui/QUERY_WORKBENCH.md).
