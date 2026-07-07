# Cursor prompt: Improve Graph Workspace focus sync

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[GRAPH_ARCHITECTURE.md](../platform/GRAPH_ARCHITECTURE.md)`
- [ ] `ui/GRAPH_WORKSPACE.md`

## Non-goals

- Saved layouts
- Reasoning overlays

## Current state

- extension/webview-ui/src/panels/GraphPanel.tsx
- LSP getGraph

## Tasks

1. Subscribe GraphPanel to FocusChanged; reload neighborhood when entity focus changes
2. Emit setFocus when user selects graph node
3. Add GraphPanel test with mock store

## Acceptance criteria

- [ ] Graph reacts to focus change
- [ ] Node selection updates focus in store

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not implement new layout engine

## References

- [Cursor prompts index](README.md)
