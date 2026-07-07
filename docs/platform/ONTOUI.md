# OntoUI architecture

> **Status:** Partial (v0.12) — per-panel React webviews; **Target:** v0.13 shared platform · **ADR:** [0001-ontoui-shared-react-platform.md](../adr/0001-ontoui-shared-react-platform.md)

## Scope

**OntoUI** is the shared React UI platform for OntoCode and future OntoStudio. Host-agnostic components and state; hosts provide file I/O, notifications, and shell integration via **WorkspaceHost**.

## Current implementation (v0.12)

| Path | Role |
|------|------|
| `extension/webview-ui/` | Vite + React + TypeScript bundle |
| `extension/webview-ui/src/App.tsx` | Routes by `?panel=` query param to panel components |
| `extension/webview-ui/src/messages.ts` | Typed postMessage protocol |
| `extension/webview-ui/src/vscodeApi.ts` | VS Code webview API wrapper |
| `extension/webview-ui/src/panels/*` | EntityInspector, GraphPanel, QueryWorkbench, … |
| `extension/webview-ui/src/components/ui.tsx` | Shared UI primitives (partial) |
| `extension/src/webview/` | Extension host: webview lifecycle, CSP, message bridge |

**Gap:** No shared WorkspaceStore; each panel manages local state and talks to LSP independently.

## Target architecture (v0.13+)

```text
WorkspaceHost (VS Code | OntoStudio)
    ↕ host adapter API
OntoUI shell
├── WorkspaceStore (global state)
├── WorkspaceRegistry → Entity | Graph | Query | … workspaces
├── Design tokens + component library
└── LSP client facade (typed, shared)
```

## WorkspaceHost interface (planned)

```ts
interface WorkspaceHost {
  postToCore(message: HostToLspMessage): Promise<unknown>
  showNotification(level: "info" | "warn" | "error", message: string): void
  getTheme(): ThemeTokens
  readFile(uri: string): Promise<string>
  writeFile(uri: string, content: string): Promise<void>
  executeCommand(id: string, args?: unknown): Promise<void>
}
```

VS Code implementation lives in the extension host; OntoStudio will implement the same interface without `vscode.*` APIs.

## Panel → workspace migration

| Current panel (`?panel=`) | Target workspace |
|----------------------------|------------------|
| `inspector` | Entity Workspace |
| `graph` | Graph Workspace |
| `queryWorkbench` | Query Workspace |
| `manchesterEditor` | Entity Workspace (axiom editor) |
| `refactorPreview` | Refactoring Workspace |
| `semanticDiff` | Diff Workspace |
| `imports` | Import Workspace |

Migration: register workspaces in **WorkspaceRegistry**; App.tsx delegates to active workspace type instead of switch on panel kind.

## Build and test

```bash
cd extension/webview-ui && npm test
cd extension && npm run build:webview
```

## Links

- [WORKSPACE_RUNTIME.md](WORKSPACE_RUNTIME.md)
- [design/adr/0017-react-webview-ui.md](../design/adr/0017-react-webview-ui.md)
- [ui/COMPONENT_LIBRARY.md](../ui/COMPONENT_LIBRARY.md)
- [cursor-prompts/01-build-ontoui-workspace-platform.md](../cursor-prompts/01-build-ontoui-workspace-platform.md)

## Evolution

Extends [design/adr/0017](../design/adr/0017-react-webview-ui.md) (React webviews) to a named **OntoUI** platform. UX detail: [ui/COMPONENT_LIBRARY.md](../ui/COMPONENT_LIBRARY.md).
