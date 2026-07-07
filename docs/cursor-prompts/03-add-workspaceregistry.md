# Cursor prompt: Add WorkspaceRegistry

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)`
- [ ] [0002-workspace-over-panel-model.md](../adr/0002-workspace-over-panel-model.md)

## Non-goals

- Migrate App.tsx routing
- Entity workspace rewrite

## Current state

- App.tsx switch on PanelKind
- panels/ directory with one component per panel

## Tasks

1. Create extension/webview-ui/src/workspaces/types.ts (WorkspaceDefinition, WorkspaceProps)
2. Create extension/webview-ui/src/workspaces/registry.ts with register/get/list
3. Register stub entries for entity, graph, query mapping to existing panel components
4. Add registry.test.ts

## Acceptance criteria

- [ ] Registry returns registered workspace by id
- [ ] Existing panel components referenced not duplicated

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not remove ?panel= routing yet

## References

- [Cursor prompts index](README.md)
