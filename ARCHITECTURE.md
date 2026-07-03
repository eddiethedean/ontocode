# Ecosystem Architecture

```
External Workflow Plugins          ← not part of OntoCore; integrate via plugin APIs
├── owlmake (reference workflow plugin)
├── ROBOT / ODK workflow adapters
└── Future build, validation, doc plugins
          │
          ▼
Applications
├── OntoCode (VS Code)             ← surfaces OntoCore + plugin workflows in the IDE
├── Python SDK
├── TypeScript SDK
├── CLI
├── GitHub Actions
├── MCP Server
└── Future Desktop/Web Apps
          │
          ▼
      OntoCore
────────────────────────
Workspace Engine
Parser
Semantic Index
Query Engine
SQL/SPARQL
Diagnostics
Navigation
Refactoring
Plugin Platform                  ← hosts diagnostics, build, validation, doc, workflow plugins
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
OWL • RDF • SHACL • Turtle • OBO
```

## Responsibilities

### Ontologos

Reasoning algorithms and semantic inference. OntoCore delegates classification, consistency, and explanations to Ontologos — it does not embed a separate reasoner.

### OntoCore

Reusable semantic workspace platform: index, query, diagnostics, refactoring, and **plugin hosting**. Consumed by IDEs, CLIs, AI agents, automation, and language bindings. OntoCore is **not** a workflow engine; build, release, and QC automation live in external plugins.

### External workflow plugins (e.g. owlmake)

First-class integrations that plug into OntoCore's **Plugin Platform**. [owlmake](https://github.com/INCATools/owlmake) is the reference workflow plugin — it orchestrates ROBOT/ODK-style pipelines without becoming a core OntoCore dependency.

### OntoCode

Reference IDE demonstrating the full capabilities of OntoCore and integrated toolchain plugins. Presents editing, reasoning, diagnostics, and workflow actions (build, validate, release) in one VS Code experience.

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

For implementation-level crate layout and diagrams, see [docs/design/ARCHITECTURE.md](docs/design/ARCHITECTURE.md) on GitHub or [Implementation architecture](https://ontocode-vs.readthedocs.io/en/latest/design/ARCHITECTURE/) on Read the Docs.
