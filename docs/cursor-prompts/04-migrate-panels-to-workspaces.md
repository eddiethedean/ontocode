# Cursor prompt: Migrate App.tsx to workspace routing

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[ONTOUI.md](../platform/ONTOUI.md)`
- [ ] [0002-workspace-over-panel-model.md](../adr/0002-workspace-over-panel-model.md)

## Non-goals

- Remove legacy HTML panels
- Multi-tab layout

## Current state

- App.tsx panelFromQuery() + switch
- WorkspaceRegistry from prompt 03

## Tasks

1. Update App.tsx to resolve workspace id from ?panel= via mapping table
2. Render workspace.component from registry
3. Keep backward compatible ?panel= values
4. Update App.test.tsx

## Acceptance criteria

- [ ] All existing panel kinds still render
- [ ] App tests pass

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not change extension host webview registration URLs

## References

- [Cursor prompts index](README.md)
