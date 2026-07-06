# Webview message protocol (v0.11)

Typed messages between the VS Code extension host (`extension/src/webviews/`) and the React app (`extension/webview-ui/`).

## Panel selection

Webviews load `webview-ui/dist` with query param `?panel=`:

`inspector` | `graph` | `smoke` | `refactorPreview` | `queryWorkbench` | `manchesterEditor` | `semanticDiff` | `imports`

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

| type | payload |
|------|---------|
| `loadImports` | `{ path, ontology_iri?, imports_editable, error?, imports[], options[] }` |
| `preview` | `{ text }` — Turtle preview after import patch |

`options` lists workspace ontologies available to add as `owl:imports`. Import changes use `applyPatch` with `add_import` / `remove_import` ops.

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

Ontology operations use LSP from the host only ([ADR-0007](design/adr/0007-language-server-boundary.md)).
