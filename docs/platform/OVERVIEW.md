# Platform overview

> **Planned v0.13+ — not shipped today.** For what you can install and use now, read [What ships today](../SHIPPED.md) first.
>
> **Status:** Architecture target for v0.13+ · **Shipped today:** [SHIPPED.md](../SHIPPED.md) · **Terms:** [Glossary](../glossary.md)

## Scope

This document is the **implementation architecture hub** for the Ontologos platform: OntoCore, OntoUI, OntoCode, and future OntoStudio. User-facing ecosystem summary: [architecture.md](../architecture.md).

## Layer model

```text
Applications (hosts)
├── OntoCode (VS Code)          — Implemented
├── OntoStudio (desktop)        — Planned
└── Future web client           — Proposed

OntoUI (shared React platform)  — Partial
├── Workspace runtime (store, event bus, registry)
├── Design system + components
├── Workspace surfaces (Entity, Graph, Query, …)
└── Host adapter (WorkspaceHost)

OntoCore (semantic engine)      — Implemented
├── Index, query, diagnostics
├── Reasoning (Ontologos)
├── Refactoring, diff, docs export
├── LSP + CLI
└── Plugin runtime (planned v0.14)

Storage / integration
├── File system, Git
└── External tools (ROBOT, …)
```

## Responsibilities

| Layer | Owns | Does not own |
|-------|------|--------------|
| **OntoCore** | Semantic truth, indexes, LSP methods, patch apply | VS Code APIs, React components |
| **OntoUI** | Global UI state, workspaces, components, host abstraction | Ontology parsing, reasoning algorithms |
| **OntoCode** | Extension activation, commands, tree views, webview lifecycle | Duplicate ontology logic in TypeScript |
| **OntoStudio** | Native shell, windows, marketplace (planned) | Separate React component tree |

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
