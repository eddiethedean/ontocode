# ARCHITECTURE.md

> **Document status: target architecture (v0.2 partial implementation)**
>
> **Implemented today:** workspace scanner, Oxigraph-based parsing, in-memory catalog and triple store,
> SQL-like queries via `sqlparser`, SPARQL, CLI, LSP explorer integration.
> **v0.4b+ target:** Horned-OWL layer per [ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md).
> See [docs/lsp-api.md](../lsp-api.md) and [adr/README.md](adr/README.md) for current decisions.

## 1. Architecture Goals

The architecture must support:

- Fast local indexing
- **Incremental updates (v0.9 requirement)**
- Multiple ontology syntaxes (including OBO at v0.7b)
- Queryable semantic catalog
- Editor integration
- Reasoner integration
- Safe write-back to source files
- Semantic diffing
- Plugin ecosystem (v1.0 API + reference plugins)

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
| diff/docs/reasoner/robot  |
+-------------+-------------+
              |
      +-------+-------+
      v               v
+-----------+   +-----------+
| Oxigraph  |   | HornedOWL |
| RDF/SPARQL|   | OWL axioms|
+-----+-----+   +-----+-----+
      |               |
      +───────┬───────┘
              v
+---------------------------+
|     Workspace Files       |
| owl/rdf/ttl/jsonld/obo    |
+---------------------------+
```

**Sync rule ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md)):** Catalog entities/axioms for edit and diff come from Horned-OWL; triple counts and SPARQL from Oxigraph; CI consistency tests detect drift.

## 3. OntoIndex Crate Layout

| Crate | Status | Role |
|-------|--------|------|
| `ontoindex-core` | v0.2 | Types, scanner, limits, path jail |
| `ontoindex-parser` | v0.2 | RDF parse via Oxigraph |
| `ontoindex-owl` | planned v0.4b | Horned-OWL facade |
| `ontoindex-catalog` | v0.2 | Index builder, entity API |
| `ontoindex-query` | v0.2 | SQL virtual tables, SPARQL |
| `ontoindex-diagnostics` | planned v0.3 | Lint rules, quick fixes |
| `ontoindex-diff` | planned v0.9 | Semantic diff |
| `ontoindex-docs` | planned v0.9 | Markdown/HTML export |
| `ontoindex-reasoner` | planned v0.6 | Rust adapters: whelk-rs, reasonable, in-tree DL |
| `ontoindex-robot` | planned v0.7b | ROBOT CLI wrappers |
| `ontoindex-lsp` | v0.2 | Language server |
| `ontoindex-cli` | v0.2 | `ontoindex` binary |

## 4. OntoIndex Internal Modules

### 4.1 Workspace Scanner
- Recursive discovery, ignore rules, format detection, content hashing, dependency tracking, change detection

### 4.2 Parser Layer (dual)
- **Oxigraph:** RDF parse, triple store, SPARQL
- **Horned-OWL (v0.4b+):** OWL 2 axiom model, Manchester, round-trip editing

### 4.3 Catalog Layer
- Ontologies, entities, axioms (from Horned-OWL), annotations, imports, diagnostics

### 4.4 Query Layer
- **v0.2:** `ontoindex-query` — virtual tables + SPARQL
- **v1.0:** joins, aggregations, ontology helper functions

### 4.5 Diagnostics Layer (v0.3+)
- Syntax, semantic, and quality rules; quick fixes; Problems panel

### 4.6 Diff Layer (v0.9+)
- Axiom-level semantic diff, Git integration, PR summaries

### 4.7 Docs Layer (v0.9+)
- Markdown/HTML export, entity pages

### 4.8 Reasoner Layer (v0.6+)
- In-process Rust adapters; explanation cache — see [REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0014](adr/0014-rust-native-reasoners-only.md)

## 5. OntoCode Internal Modules

### 5.1 Extension Host
Commands, views, webviews, language client, settings.

### 5.2 Tree Views
Explorer (asserted/inferred toggle), diagnostics, query history.

### 5.3 Webviews
Graph visualization, query workbench, semantic diff, Manchester axiom editor, explanation panel.

### 5.4 Language Client
LSP lifecycle; protocol guards; v1.0 authoring methods.

## 6. Index Lifecycle

1. User opens workspace → LSP starts
2. Scanner discovers files → parse via Oxigraph + Horned-OWL
3. Catalog updated → diagnostics published
4. UI refreshes → queries/reasoner/refactor use catalog
5. **v0.9:** incremental update on content hash change only

## 7. Write-Back Architecture

Per [ADR-0006](adr/0006-patch-based-write-back.md) and [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md):

- Patches from Horned-OWL axiom objects
- Preserve comments and formatting
- Preview for multi-file refactors
- VS Code undo stack

## 8. Performance Strategy

- Incremental index by content hash (**v0.9 required**)
- Parallel parse independent files
- In-memory catalog; optional disk cache
- Lazy-load inferred views after reasoner run
- Non-blocking extension host

## 9. Security Model

- Local-first by default ([ADR-0005](adr/0005-local-first-by-default.md))
- No telemetry by default
- Workspace trust for `lspPath`, `ontocode.reasoner.default`, `robotPath`
- See [SECURITY.md](../SECURITY.md)
