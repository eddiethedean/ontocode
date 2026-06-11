# ARCHITECTURE.md

> **Document status: target architecture (v0.2 partial implementation)**
>
> **Implemented today:** workspace scanner, Oxigraph-based parsing, in-memory catalog and triple store,
> SQL-like queries via `sqlparser`, SPARQL, CLI, LSP explorer integration.
> **Not yet implemented:** diagnostics layer, diff layer, docs export, Horned-OWL modeling, DataFusion SQL.
> See [docs/lsp-api.md](../docs/lsp-api.md) and [adr/README.md](adr/README.md) for current decisions.

## 1. Architecture Goals

The architecture must support:

- Fast local indexing
- Incremental updates
- Multiple ontology syntaxes
- Queryable semantic catalog
- Editor integration
- Reasoner integration
- Safe write-back to source files
- Semantic diffing
- Future plugin ecosystem

## 2. High-Level Architecture

```text
+---------------------------+
|        OntoCode UI        |
| VS Code panels + commands |
+-------------+-------------+
              |
              v
+---------------------------+
|   OntoIndex Language      |
|        Server             |
+-------------+-------------+
              |
              v
+---------------------------+
|       OntoIndex Core      |
| catalog/query/diagnostics |
+-------------+-------------+
              |
              v
+---------------------------+
|   Parser + RDF/OWL Layer  |
| Oxigraph (+ sqlparser)    |
+-------------+-------------+
              |
              v
+---------------------------+
|     Workspace Files       |
| owl/rdf/ttl/jsonld/obo    |
+---------------------------+
```

## 3. OntoIndex Internal Modules

### 3.1 Workspace Scanner
Responsible for:

- Recursive discovery
- Ignore rules
- format detection
- content hashing
- dependency tracking
- change detection

### 3.2 Parser Layer
Responsible for:

- RDF parsing
- OWL parsing
- namespace extraction
- source mapping
- parse error recovery

### 3.3 Catalog Layer
Responsible for normalized semantic tables:

- ontologies
- entities
- axioms
- annotations
- imports
- diagnostics

### 3.4 Query Layer

**v0.2:** implemented (`ontoindex-query` — virtual tables + SPARQL over Oxigraph store).

Responsible for:

- SQL-style queries
- SPARQL pass-through
- ontology helper functions
- result serialization

### 3.5 Diagnostics Layer

**v0.2:** not implemented (planned v0.3).

Responsible for:

- syntax diagnostics
- semantic diagnostics
- quality rules
- quick fixes

### 3.6 Diff Layer

**v0.2:** not implemented.

Responsible for:

- semantic comparison
- breaking change detection
- Git integration
- PR summaries

### 3.7 Docs Layer

**v0.2:** not implemented.

Responsible for:

- Markdown export
- HTML export
- entity pages
- diagrams
- reports

## 4. OntoCode Internal Modules

### 4.1 Extension Host
Registers commands, views, webviews, language clients, and configuration.

### 4.2 Tree Views
Render ontology explorer, import graph, diagnostics, and query history.

### 4.3 Webviews
Render graph visualization, query workbench, semantic diff, and entity inspector.

### 4.4 Language Client
Manages LSP lifecycle and communication with `ontoindex-lsp`.

## 5. Index Lifecycle

1. User opens workspace.
2. OntoCode starts language server.
3. Language server scans workspace.
4. OntoIndex parses changed ontology files.
5. Catalog is updated.
6. Diagnostics are published (planned v0.3).
7. Tree views and panels refresh.
8. Queries and refactors use the catalog.

## 6. Write-Back Architecture

Ontology editing must preserve user trust.

Rules:

- Never destructively rewrite entire files unless explicitly requested.
- Prefer source-range patches.
- Preserve comments where possible.
- Preserve formatting where possible.
- Show preview for multi-file refactors.
- All refactors must be undoable through VS Code.

## 7. Performance Strategy

- Incremental index by content hash
- Parallel parse independent files
- Store normalized catalog in memory
- Optional disk cache for large workspaces
- Lazy-load expensive inferred views
- Avoid blocking VS Code extension host

## 8. Security Model

- Local-first by default
- No telemetry by default
- No ontology upload without explicit user action
- AI features opt-in
- Workspace trust respected
