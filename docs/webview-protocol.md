# Webview message protocol (v0.7)

Typed messages between the VS Code extension host (`extension/src/webviews/`) and the React app (`extension/webview-ui/`).

## Panel selection

Webviews load `webview-ui/dist` with query param `?panel=inspector|graph|smoke`.

## Host → React

| type | payload |
|------|---------|
| `init` | `{ panel }` |
| `loadEntity` | `{ detail, classOptions }` |
| `graphData` | `{ graph }` |
| `preview` | `{ text }` |
| `error` | `{ message }` |

## React → Host

| type | payload |
|------|---------|
| `ready` | `{ panel }` |
| `applyPatch` | `{ patches, previewOnly }` |
| `jumpToSource` | — |
| `openManchester` | `{ axiom }` |
| `addManchesterAxiom` | — |
| `requestGraph` | `{ graphKind, rootIri?, depth?, includeInferred?, filters? }` |
| `selectNode` | `{ iri }` |
| `openGraph` | `{ rootIri? }` |

Ontology operations use LSP from the host only ([ADR-0007](design/adr/0007-language-server-boundary.md)).
