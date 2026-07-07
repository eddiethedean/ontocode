# Platform Architecture

## 1. Overview

The Ontologos platform is a layered architecture centered on OntoCore.

```text
Applications
├── OntoCode VS Code Extension
├── OntoStudio Desktop
└── Future Web Client

Shared Frontend Platform
├── React Component Library
├── Design System
├── Workspace Model
├── Plugin UI Runtime
└── AI Interaction Layer

Semantic Platform
├── OntoCore Workspace Engine
├── Parser Framework
├── Query Engine
├── Reasoning Engine
├── Refactoring Engine
├── Diagnostics Engine
├── Documentation Engine
├── Graph Engine
├── Plugin Runtime
└── AI Context Engine

Storage / Integration
├── File System
├── Git
├── Local Cache
├── Workspace Database
├── Plugin Storage
└── Remote Services
```

## 2. OntoCore Responsibilities

OntoCore owns semantic truth.

- Parse ontology formats.
- Build workspace indexes.
- Resolve symbols and references.
- Provide semantic diagnostics.
- Execute queries.
- Coordinate reasoning.
- Support semantic refactoring.
- Produce semantic diffs.
- Generate documentation metadata.
- Expose stable APIs to applications and plugins.

## 3. OntoCode Responsibilities

OntoCode is the VS Code product surface.

- Extension activation.
- VS Code commands.
- LSP integration.
- Webview hosting.
- File watching.
- Workspace discovery.
- Command palette contribution.
- VS Code settings.
- Native tree views where appropriate.
- Bridge between VS Code and shared React UI.

## 4. OntoStudio Responsibilities

OntoStudio is the dedicated desktop product.

- Native app shell.
- Multi-window layouts.
- High-performance graph rendering.
- Plugin marketplace.
- Offline-first workflows.
- Local AI integration.
- Enterprise deployment support.

## 5. Shared UI Platform

The React UI must be application-host agnostic.

Host adapters provide:

- File operations.
- Notifications.
- Theme values.
- Clipboard.
- Command execution.
- Plugin loading.
- Native shell integrations.

## 6. Capability Interfaces

All extensible systems use capability interfaces.

Examples:

```ts
interface ReasonerProvider {}
interface AIProvider {}
interface QueryLanguageProvider {}
interface RefactoringProvider {}
interface DiagnosticsProvider {}
interface DocumentationProvider {}
interface VisualizationProvider {}
interface ImportExportProvider {}
```

Plugins register capabilities. The platform orchestrates them.

## 7. API Boundaries

### Rust Boundary

Rust exposes typed APIs through:

- LSP
- JSON-RPC
- WASM where appropriate
- Native Tauri commands
- CLI
- Future bindings

### Frontend Boundary

Frontend communicates through a typed client:

```ts
const workspace = await api.workspace.open(path)
const entity = await api.entities.get(id)
const result = await api.query.run(query)
```

No component directly calls raw transport APIs.

## 8. Performance Architecture

- Incremental parsing.
- Incremental indexing.
- Virtualized UI lists.
- Background reasoning.
- Cached query plans.
- Progressive graph loading.
- Lazy plugin activation.
- Streaming AI responses.

## 9. Security Architecture

- Plugin sandboxing.
- Capability permissions.
- Signed plugin packages.
- Workspace trust model.
- AI data disclosure controls.
- Enterprise policy hooks.

## 10. Success Criteria

The platform succeeds when OntoCode, OntoStudio, plugins, AI tools, and automation systems all share the same semantic foundation without duplicating domain logic.
