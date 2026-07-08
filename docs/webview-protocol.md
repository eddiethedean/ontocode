# Webview message protocol (v0.13)

Typed messages between the VS Code extension host (`extension/src/webviews/`) and the React app (`extension/webview-ui/`).

## Panel selection

Webviews load `webview-ui/dist` with query param `?panel=`:

`inspector` | `graph` | `smoke` | `refactorPreview` | `queryWorkbench` | `manchesterEditor` | `semanticDiff` | `imports`

## Host → React (v0.13 focus relay)

Cross-panel synchronization via `FocusRelayService` in the extension host. All webviews that call `useFocusSync` accept:

| type | payload |
|------|---------|
| `focusState` | `{ focus: CurrentFocus }` — broadcast when explorer, inspector, or graph changes selection |
| `reasoningState` | `{ reasoning: ReasoningStatePayload }` — last reasoner run summary for graph/inspector |

`CurrentFocus`:

| field | type | description |
|-------|------|-------------|
| `kind` | string | `entity` \| `axiom` \| `query` \| `diagnostic` \| `graphNode` \| `documentation` \| `review` |
| `id` | string | IRI or stable object id |
| `source` | string | Originating panel id (e.g. `explorer`, `inspector`, `graph`) |
| `timestamp` | number | Unix epoch ms |

`ReasoningStatePayload`:

| field | type | description |
|-------|------|-------------|
| `profile` | string | Reasoner profile used (`el`, `rl`, `dl`, `auto`, …) |
| `unsatisfiable` | string[] | Unsatisfiable class IRIs from last run |
| `hierarchyMode` | string? | `asserted` \| `inferred` \| `combined` when applicable |
| `lastRunAt` | number | Unix epoch ms |

Query Workbench `queryInit` may also include `sqlSchema` — array of `{ name, columns: [{ name, type }] }` from LSP `ontocore/listSqlSchema`.

## Host → React (shared)

| type | payload |
|------|---------|
| `init` | `{ panel }` |
| `error` | `{ message }` |

### Inspector / graph

| type | payload |
|------|---------|
| `loadEntity` | `{ detail, classOptions }` |
| `graphData` | `{ graph }` |
| `preview` | `{ text }` |

### Query Workbench

| type | payload |
|------|---------|
| `queryInit` | `{ saved, history, sqlTables }` |
| `queryResult` | `{ runId, result?, error? }` |

### Manchester editor

| type | payload |
|------|---------|
| `manchesterInit` | `{ entityIri, axiomKind, expression, completions }` |
| `manchesterValidation` | `{ seq, result?, error? }` |

### Refactor preview

| type | payload |
|------|---------|
| `loadRefactorPlan` | `{ plan }` |

### Semantic diff (v0.10+)

| type | payload |
|------|---------|
| `loading` | — |
| `semanticDiffData` | `{ diff }` — axiom/entity changes, breaking-change flags |

Host loads diff via LSP `ontocore/semanticDiff` on panel open. See [Semantic diff guide](ontocode/semantic-diff.md).

### Manage Imports (v0.11+)

Host → React:

| type | payload shape |
|------|----------------|
| `loadImports` | `{ payload: ImportsDocumentPayload }` |
| `preview` | `{ text: string }` — Turtle preview after import patch |

`ImportsDocumentPayload`:

| field | type | description |
|-------|------|-------------|
| `path` | string | Indexed `.ttl` path |
| `ontology_iri` | string? | Ontology header IRI when known |
| `imports_editable` | boolean | `false` for non-Turtle or read-only docs |
| `error` | string? | Load failure message (panel still renders) |
| `imports` | string[] | Current `owl:imports` IRIs |
| `options` | `{ iri, path, label }[]` | Workspace ontologies available to add |

React → Host: `applyPatch` with `add_import` / `remove_import` ops and explicit `previewOnly: boolean` (required; host rejects messages without it).

Example host message:

```json
{
  "type": "loadImports",
  "payload": {
    "path": "/workspace/fixtures/example.ttl",
    "ontology_iri": "http://example.org/people",
    "imports_editable": true,
    "imports": [],
    "options": [
      {
        "iri": "http://example.org/org",
        "path": "/workspace/fixtures/organization.owl",
        "label": "organization.owl"
      }
    ]
  }
}
```

See [Manage Imports guide](ontocode/manage-imports.md).

## React → Host

| type | payload |
|------|---------|
| `ready` | `{ panel }` |
| `applyPatch` | `{ patches, previewOnly }` |
| `jumpToSource` | — |
| `openManchester` | `{ axiom }` |
| `addManchesterAxiom` | — |
| `findUsages` | — |
| `renameIri` | — |
| `requestGraph` | `{ graphKind, rootIri?, depth?, includeInferred?, filters? }` |
| `selectNode` | `{ iri }` |
| `openGraph` | `{ rootIri? }` |
| `runQuery` | `{ mode, text, runId }` |
| `saveQuery` | `{ mode, text, name }` |
| `exportQueryResult` | `{ format }` |
| `validateManchester` | `{ expression, axiomKind, seq }` |
| `applyManchester` | `{ expression, axiomKind, previewOnly }` |
| `applyRefactor` | — |
| `cancelRefactor` | — |
| `copyMarkdown` | — (semantic diff panel — copies breaking changes to clipboard) |
| `setFocus` | `{ focus: CurrentFocus }` — request focus change from a webview (relayed to other panels) |
| `showNotification` | `{ message, level? }` — toast in VS Code host |

Ontology operations use LSP from the host only ([ADR-0007](design/adr/0007-language-server-boundary.md)).
