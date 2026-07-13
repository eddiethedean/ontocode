# Ecosystem Architecture

> **Audience:** Evaluators, adopters, and new contributors — **canonical user-facing architecture**.
>
> **Which architecture doc?**
>
> | Read this | When |
> |-----------|------|
> | **This page** (`architecture.md`) | Product/ecosystem overview — Ontologos, OntoCore, OntoCode |
> | [Implementation architecture](design/ARCHITECTURE.md) | Contributor crate layout and internal modules |
> | [Product design / UI platform](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLATFORM_ARCHITECTURE.md) | Shared **OntoUI**, OntoStudio target, design system |
> | [Platform architecture (implementation)](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/OVERVIEW.md) | OntoUI, WorkspaceStore, plugin host — **shipped v0.13–v0.17** |
> | [Plugin authoring](guides/plugins.md) | Workspace manifests, reference plugins, subprocess workflows (v0.17) |
> | [OntoCore architecture](ontocore/architecture.md) | Short OntoCore stack summary (links here for detail) |
>
> **Contributor crate layout:** [Implementation architecture](design/ARCHITECTURE.md) (internal modules only).
>
> **v0.20 in progress (unreleased):** OntoCode (VS Code), OntoCore (CLI/LSP/library), Turtle + OBO write-back, property chain editing, OWL/XML read-only catalog, DL explanations (with alternatives and staleness), semantic diff (`--pr-summary`), incremental indexing, Ontologos reasoning, ROBOT CLI wrappers, **WorkspaceStore + cross-panel focus sync**, **Query Workbench schema browser**, **configurable diagnostics**, **LSP semantic tokens** (Turtle/OBO), **plugin host** (manifests, permissions, UI views/commands/**preferences/context actions**, reference plugins, CLI/LSP hooks, owlmake scaffold), **graph asserted/inferred/combined modes**, **imports reload + layout reset**, **reasoner cancel + distinct lifecycle**, **layout reopen-with-context**, Protégé Desktop parity gate, Turtle patch matching hardening on this branch. Latest tagged: **v0.19.0**.
> **Planned v1.0:** stable plugin API, full workflow plugin integration, language SDKs, MCP server. See [Platform roadmap](roadmap.md).
> **Planned post-1.0:** OntoStudio desktop, AI-native workflows — [UI roadmap mapping](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ROADMAP_MAPPING.md).
> Canonical capability matrix: [What ships today](SHIPPED.md).

```
External Workflow Plugins (v0.17)  ← subprocess workflow plugins; API v1 (permissions, views, preferences)
├── owlmake (reference design)
├── ROBOT / ODK workflow adapters
└── Future build, validation, doc plugins
          │
          ▼
Applications
├── OntoCode (VS Code)             ← ships today
├── OntoStudio (desktop)           ← planned post v1.0 ([UI spec](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/ONTOSTUDIO_DESKTOP.md))
├── CLI                            ← ships today
├── GitHub Actions (via CLI)       ← ships today
├── Python / TypeScript SDKs       ← planned
├── MCP Server                     ← planned
└── Future Desktop/Web Apps
          │
          ▼
      OntoCore (ships today)
────────────────────────
Workspace Engine
Parser
Semantic Index
Query Engine
SQL/SPARQL (SQL-like virtual tables)
Diagnostics
Navigation
Refactoring
Plugin Platform (v1.0 target)
Persistent Cache
LSP
          │
          ▼
      Ontologos
────────────────────────
Reasoning
Classification
Consistency
Inference
Explanations
          │
          ▼
OWL • RDF • Turtle • OBO
(SHACL: planned)
```

## Responsibilities

### Ontologos

Reasoning algorithms and semantic inference. OntoCore delegates classification, consistency, and explanations to Ontologos — it does not embed a separate reasoner.

### OntoCore

Reusable semantic workspace platform: index, query, diagnostics, refactoring, and semantic diff. Consumed by the VS Code extension, CLI, and Rust library.

**Plugin platform status:**

- **Shipped (v0.14–v0.17):** plugin host MVP — workspace manifest discovery, reference plugins, CLI/LSP hooks, subprocess workflow runner, UI views/commands, and (v0.16+) preferences pages + context actions (see [Plugin authoring](guides/plugins.md)).
- **Planned (v1.0+):** semver-stable plugin API, hardened permissions/sandboxing, and marketplace/discovery.

OntoCore is **not** a workflow engine; build, release, and QC automation should live in external tools and workflow plugins rather than becoming core engine dependencies.

### External workflow plugins (e.g. owlmake)

**Host ships today; full owlmake integration is v1.0.** [owlmake](https://github.com/INCATools/owlmake) is the reference workflow plugin design — ROBOT/ODK-style pipelines without becoming a core OntoCore dependency. Today, ROBOT interop is the `ontocore robot` CLI wrapper plus the subprocess workflow scaffold.

### OntoCode

Reference IDE on top of OntoCore. Presents editing, reasoning, and diagnostics in VS Code. Plugin views, commands, preferences, and context actions ship today; marketplace-scale workflow automation remains a v1.0 target.

## Design Philosophy

Ontologos thinks.

OntoCore understands.

OntoCode presents.

Workflow plugins automate.

## Future Extensions

- Plugin marketplace and discovery
- owlmake and third-party workflow plugins
- AI assistants
- Enterprise governance
- Documentation generators (via plugin APIs)
- Visualization tools
- Collaborative editing
- JetBrains and web clients

For implementation-level crate layout and diagrams, see [Implementation architecture](https://ontocode-vs.readthedocs.io/en/latest/design/ARCHITECTURE/) (also [docs/design/ARCHITECTURE.md](https://github.com/eddiethedean/ontocode/blob/main/docs/design/ARCHITECTURE.md) on GitHub).
