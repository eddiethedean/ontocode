# Cursor prompt: Build OntoUI workspace platform scaffold

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[ONTOUI.md](../platform/ONTOUI.md)`
- [ ] [0001-ontoui-shared-react-platform.md](../adr/0001-ontoui-shared-react-platform.md)

## Non-goals

- OntoStudio shell
- Full WorkspaceStore
- Plugin runtime

## Current state

- extension/webview-ui/src/App.tsx routes panels via ?panel=
- extension/webview-ui/src/vscodeApi.ts wraps acquireVsCodeApi
- No extension/webview-ui/src/store/ yet

## Tasks

1. Create extension/webview-ui/src/host/types.ts with WorkspaceHost interface (postToCore, getTheme, showNotification)
2. Create extension/webview-ui/src/host/vscodeHost.ts implementing WorkspaceHost using existing vscodeApi
3. Create extension/webview-ui/src/context/HostContext.tsx to provide host to components
4. Refactor one panel (SmokePanel) to consume HostContext instead of direct vscodeApi import
5. Add Vitest test for HostContext provider

## Acceptance criteria

- [ ] HostContext renders without error
- [ ] SmokePanel still passes existing tests
- [ ] No ontology logic added to TS

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not rename all panels in one PR
- Do not add Zustand yet (prompt 02)
- Do not change LSP protocol

## References

- [Cursor prompts index](README.md)
