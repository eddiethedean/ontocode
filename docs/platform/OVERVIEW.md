# Platform overview

> **v0.17 foundation shipped.** For evaluator-facing capabilities, read [What ships today](../SHIPPED.md) first.
>
> **Status:** OntoUI runtime + focus relay **shipped v0.13**; plugin host MVP **shipped v0.14**; plugin permissions/views **shipped v0.15** · **Terms:** [Glossary](../glossary.md)

## Scope

This document is the **implementation architecture hub** for the Ontologos platform: OntoCore, OntoUI, OntoCode, and future OntoStudio. User-facing ecosystem summary: [architecture.md](../architecture.md).

## Layer model

```text
Applications (hosts)
├── OntoCode (VS Code)          — Implemented (WorkspaceStore + focus relay v0.13)
├── OntoStudio (desktop)        — Planned
└── Future web client           — Proposed

OntoUI (shared React platform)  — v0.13 foundation shipped
├── Workspace runtime (Zustand store, event bus, registry)
├── Design tokens + shared primitives
├── Workspace surfaces (Entity, Graph, Query, …)
└── Host adapter (WorkspaceHost) + extension-host focus relay

OntoCore (semantic engine)      — Implemented
├── Index, query, diagnostics (configurable rules)
├── Reasoning (Ontologos)
├── Refactoring, diff (--pr-summary), docs export
├── LSP + CLI (semantic tokens, listSqlSchema)
└── Plugin runtime (shipped v0.14–v0.15)

Storage / integration
├── File system, Git
└── External tools (ROBOT, …)
```

## Responsibilities

| Layer | Owns | Does not own |
|-------|------|--------------|
| **OntoCore** | Semantic truth, indexes, LSP methods, patch apply | VS Code APIs, React components |
| **OntoUI** | Global UI state, workspaces, components, host abstraction | Ontology parsing, reasoning algorithms |
| **OntoCode** | Extension activation, commands, tree views, webview lifecycle, focus relay | Duplicate ontology logic in TypeScript |
| **OntoStudio** | Native shell, windows, marketplace (planned) | Separate React component tree |

## v0.13 shipped (OntoUI foundation)

| Component | Location | Notes |
|-----------|----------|-------|
| `WorkspaceHost` | `extension/webview-ui/src/host/` | VS Code adapter via `HostContext` |
| `WorkspaceStore` | `extension/webview-ui/src/store/` | Zustand; focus, query, reasoning, refactor slices |
| Event bus | `extension/webview-ui/src/store/events.ts` | `FocusChanged`, `QueryExecuted`, … |
| `WorkspaceRegistry` | `extension/webview-ui/src/workspaces/` | Panel → workspace routing |
| Focus relay | `extension/src/focus/focusRelay.ts` | Cross-webview `focusState` / `reasoningState` |
| Design tokens | `extension/webview-ui/src/tokens/` | `--oc-*` CSS variables |
| Schema browser | `extension/webview-ui/src/components/SchemaBrowser.tsx` | LSP `ontocore/listSqlSchema` |

**Deferred to v1.0:** persistent tabs, bottom dock, full component migration for every panel.

## Key documents

| Topic | Document |
|-------|----------|
| OntoUI package | [ONTOUI.md](ONTOUI.md) |
| WorkspaceStore, hosts, events | [WORKSPACE_RUNTIME.md](WORKSPACE_RUNTIME.md) |
| Plugin capabilities | [CAPABILITY_PROVIDERS.md](CAPABILITY_PROVIDERS.md) |
| AI safe apply | [AI_ORCHESTRATION.md](AI_ORCHESTRATION.md) |
| Refactoring transactions | [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) |
| Reasoning pipeline | [REASONING_COMPILER.md](REASONING_COMPILER.md) |
| Graph stack | [GRAPH_ARCHITECTURE.md](GRAPH_ARCHITECTURE.md) |
| Query workbench | [QUERY_WORKBENCH_ARCHITECTURE.md](QUERY_WORKBENCH_ARCHITECTURE.md) |
| OntoStudio reuse | [ONTOSTUDIO_REUSE.md](ONTOSTUDIO_REUSE.md) |
| Milestone template | [MILESTONE_TEMPLATE.md](MILESTONE_TEMPLATE.md) |

## Product ADRs

Platform decisions: [adr/README.md](../adr/README.md). Engineering ADRs: [design/adr/README.md](../design/adr/README.md).

## Evolution

Previously described in [ui/PLATFORM_ARCHITECTURE.md](../ui/PLATFORM_ARCHITECTURE.md) and [architecture.md](../architecture.md). This hub consolidates **implementation** detail; those pages remain for ecosystem and UX context.
