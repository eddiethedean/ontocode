# Glossary

Canonical terminology for OntoCode, OntoCore, OntoUI, and the Ontologos platform. Use these terms consistently in planning, architecture, and implementation docs.

| Term | Definition | Status |
|------|------------|--------|
| **OntoCore** | Rust semantic workspace engine — indexing, queries, diagnostics, reasoning integration, refactoring, CLI, LSP | **Implemented** (v0.12) |
| **OntoCode** | VS Code extension — host shell, commands, LSP client, webview hosting | **Implemented** (v0.12) |
| **OntoUI** | Shared React UI platform — component library, design tokens, workspace runtime target; lives in `extension/webview-ui/` today | **Partial** — per-panel React webviews; no shared WorkspaceStore yet |
| **OntoStudio** | Future standalone desktop application reusing OntoUI + OntoCore | **Planned** (post v1.0) |
| **Workspace** | Task-oriented product surface (Entity, Graph, Query, Reasoning, Refactoring, …) — **not** a VS Code workspace folder | **Partial** — implemented as isolated webview panels |
| **Current Focus** | Active semantic object (entity, axiom, query, diagnostic, graph node) that drives UI synchronization | **Planned** (v0.13) |
| **WorkspaceStore** | Single source of truth for OntoUI global state | **Planned** (v0.13) |
| **WorkspaceRegistry** | Registry mapping workspace type → factory / component | **Planned** (v0.13) |
| **WorkspaceHost** | Host adapter bridging OntoUI to a shell (VS Code webview, future Electron/Tauri) | **Partial** — VS Code extension host only |
| **Capability Provider** | Plugin-provided implementation of a platform capability (reasoning, querying, AI, refactoring, diagnostics, import/export) | **Planned** (v0.14+) |
| **Ontologos** | External OWL reasoner (EL/RL/RDFS/DL) integrated via `ontocore-reasoner` | **Implemented** (v0.9+) |

## Disambiguation

| Avoid | Use instead |
|-------|-------------|
| "VS Code workspace" when meaning product UI | **Workspace** (product) or "VS Code workspace folder" |
| "Shared React UI" / "Frontend platform" | **OntoUI** |
| "Plugin" (generic) | **Capability Provider** when referring to extensibility interfaces |
| "Panel" (long-term) | **Workspace** (product surface); "panel" OK for current webview implementation |

## See also

- [Platform overview](platform/OVERVIEW.md)
- [What ships today](SHIPPED.md)
- [Documentation index](documentation-index.md)
