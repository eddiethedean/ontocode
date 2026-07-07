# Cursor prompt: Implement Current Focus and FocusChanged

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)`
- [ ] [0003-current-focus-central-ux.md](../adr/0003-current-focus-central-ux.md)
- [ ] `ui/STATE_MANAGEMENT.md`

## Non-goals

- Cross-webview sync (separate webviews)
- AI context builder

## Current state

- WorkspaceStore from prompt 02
- EntityInspector loads entity by IRI locally

## Tasks

1. Add CurrentFocus type and setFocus action to workspaceStore
2. Create extension/webview-ui/src/store/events.ts with FocusChanged type
3. Add subscribe/emit helper (simple pub/sub in store module)
4. Update EntityInspector to call setFocus when entity loads
5. Add test: setFocus updates store and notifies subscriber

## Acceptance criteria

- [ ] FocusChanged fires on entity selection
- [ ] Tests pass

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not implement cross-webview sync without extension host relay

## References

- [Cursor prompts index](README.md)
