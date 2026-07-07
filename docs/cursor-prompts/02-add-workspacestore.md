# Cursor prompt: Add WorkspaceStore module

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)`
- [ ] [0004-workspacestore-ui-source-of-truth.md](../adr/0004-workspacestore-ui-source-of-truth.md)

## Non-goals

- WorkspaceRegistry
- Migrate all panels
- Persist state to disk

## Current state

- No centralized store; panels use local useState
- messages.ts defines LSP payload types

## Tasks

1. Add zustand (or use existing state lib if present) to webview-ui package.json if needed
2. Create extension/webview-ui/src/store/types.ts with WorkspaceStore slice types from WORKSPACE_RUNTIME.md
3. Create extension/webview-ui/src/store/workspaceStore.ts with initial state and selectors
4. Create extension/webview-ui/src/store/index.ts exports
5. Add store/workspaceStore.test.ts with focus/selection update tests

## Acceptance criteria

- [ ] Store initializes with null focus
- [ ] Tests cover setFocus and setSelection
- [ ] Types match platform doc

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not wire every panel yet
- Do not add event bus until prompt 05

## References

- [Cursor prompts index](README.md)
