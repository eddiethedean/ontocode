# Webview message protocol (v0.11)

Typed messages between the VS Code extension host (`extension/src/webviews/`) and the React app (`extension/webview-ui/`).

## Panel selection

Webviews load `webview-ui/dist` with query param `?panel=`:

`inspector` | `graph` | `smoke` | `refactorPreview` | `queryWorkbench` | `manchesterEditor`

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

Ontology operations use LSP from the host only ([ADR-0007](design/adr/0007-language-server-boundary.md)).
