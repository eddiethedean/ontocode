# Ecosystem Architecture

> **Audience:** Evaluators, adopters, and contributors — high-level product architecture.
>
> **Which architecture doc?**
>
> | Read this | When |
> |-----------|------|
> | **This page** (`ARCHITECTURE.md`) | Product/ecosystem overview — Ontologos, OntoCore, OntoCode |
> | [Implementation architecture](docs/design/ARCHITECTURE.md) | Contributor crate layout and internal modules |
> | [Product design / UI platform](docs/ui/PLATFORM_ARCHITECTURE.md) | Shared React UI, OntoStudio target, design system |
> | [OntoCore architecture](docs/ontocore/architecture.md) | Short OntoCore stack summary (links to design doc for detail) |
>
> **Contributor crate layout:** [Implementation architecture](docs/design/ARCHITECTURE.md) (internal modules, not a user capability list).
>
> **v0.14 ships today:** OntoCode (VS Code), OntoCore (CLI/LSP/library), Turtle + OBO write-back, property chain editing, OWL/XML read-only catalog, DL explanations, semantic diff, incremental indexing, Ontologos reasoning, ROBOT CLI wrappers, **plugin host MVP** (manifests, reference validators/exporters, CLI/LSP hooks, owlmake workflow scaffold).
> **Planned v1.0 (not installable yet):** stable plugin API, language SDKs, MCP server, full owlmake integration.
> **Planned post-1.0:** OntoStudio desktop, AI-native workflows — [UI roadmap mapping](docs/ui/ROADMAP_MAPPING.md).
> Canonical capability matrix: [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/).

```
External Workflow Plugins (v0.14 MVP)  ← subprocess workflow plugins; stable API v1.0
├── owlmake (reference design)
├── ROBOT / ODK workflow adapters
└── Future build, validation, doc plugins
          │
          ▼
Applications
├── OntoCode (VS Code)             ← ships today
├── OntoStudio (desktop)           ← planned post v1.0 ([UI spec](docs/ui/ONTOSTUDIO_DESKTOP.md))
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

Reusable semantic workspace platform: index, query, diagnostics, refactoring, and semantic diff. Consumed by the VS Code extension, CLI, and Rust library. **Plugin hosting is a v1.0 design** — not installable yet. OntoCore is **not** a workflow engine; build, release, and QC automation will live in external plugins when the host ships.

### External workflow plugins (e.g. owlmake)

**Planned v1.0.** [owlmake](https://github.com/INCATools/owlmake) is the reference workflow plugin design — ROBOT/ODK-style pipelines without becoming a core OntoCore dependency. Today, ROBOT interop is the `ontocore robot` CLI wrapper only.

### OntoCode

Reference IDE on top of OntoCore. Presents editing, reasoning, and diagnostics in VS Code. Workflow actions (build, release) via plugins are a v1.0 target.

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
