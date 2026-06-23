# ARCHITECTURE.md

> **Document status: target architecture (v0.2 partial implementation)**
>
> **Implemented today:** workspace scanner, Oxigraph-based parsing, in-memory catalog and triple store,
> SQL-like queries via `sqlparser`, SPARQL, CLI, LSP explorer integration.
> **v0.4b+ target:** Horned-OWL layer per [ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md).
> See [docs/lsp-api.md](../lsp-api.md), [adr/README.md](adr/README.md), and [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md) for current decisions.

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
      +-------+-------+-------+
      v       v               v
+-----------+ +-----------+ +------------------+
| Oxigraph  | | HornedOWL | | OntoLogos        |
| RDF/SPARQL| | OWL axioms| | reasoners 0.9/1.0|
+-----+-----+ +-----+-----+ +--------+---------+
      |               |              |
      +───────┬───────┘              |
              v                      |
+---------------------------+        |
|     Workspace Files       |◄───────┘
| owl/rdf/ttl/jsonld/obo    |
+---------------------------+
```

**Reasoning ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md)):** `ontoindex-reasoner` delegates to [OntoLogos](https://github.com/eddiethedean/ontologos) crates (`ontologos-el`, `ontologos-rl`, `ontologos-dl`, etc.). Pin **0.9.0** at v0.6; bump to **1.0.0** for DL parity at OntoCode v1.0.

**Sync rule ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md)):** Catalog entities/axioms for edit and diff come from Horned-OWL; triple counts and SPARQL from Oxigraph; CI consistency tests detect drift.

**Dependency policy:** [ADR-0016](adr/0016-dependency-first-implementation.md), [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

## 3. OntoIndex Crate Layout

| Crate | Status | Role | External dependency |
|-------|--------|------|---------------------|
| `ontoindex-core` | v0.2 | Types, scanner, limits, path jail | `ignore` |
| `ontoindex-parser` | v0.2 | RDF parse, entity extraction | `oxigraph` |
| `ontoindex-owl` | planned v0.4b | OWL axiom facade, Manchester | `horned-owl`, `horned-functional` |
| `ontoindex-catalog` | v0.2 | Index builder, entity API | — |
| `ontoindex-query` | v0.2 | SQL virtual tables, SPARQL | `sqlparser`, `oxigraph` |
| `ontoindex-diagnostics` | planned v0.3 | Lint rules, LSP diagnostics | `oxigraph` (+ `fastobo-validator` v0.7b) |
| `ontoindex-diff` | planned v0.9 | Semantic diff, Git compare | `horned-owl`, `git2` |
| `ontoindex-docs` | planned v0.9 | Markdown/HTML export | `pulldown-cmark`, `minijinja` |
| `ontoindex-reasoner` | planned v0.6 | Reasoner facade | OntoLogos `0.9`→`1.0` |
| `ontoindex-robot` | planned v0.7b | ROBOT CLI wrappers | ROBOT CLI (external) |
| `ontoindex-lsp` | v0.2 | Language server | `lsp-server`, `lsp-types` |
| `ontoindex-cli` | v0.2 | `ontoindex` binary | composes above |

## 4. OntoIndex Internal Modules

### 4.1 Workspace Scanner
- Recursive discovery, ignore rules, format detection, content hashing, dependency tracking, change detection

### 4.2 Parser Layer (dual)
- **Oxigraph:** RDF parse, triple store, SPARQL ([ADR-0003](adr/0003-use-oxigraph.md))
- **Horned-OWL (v0.4b+):** OWL 2 axiom model via `horned-owl` + `horned-functional` ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0016](adr/0016-dependency-first-implementation.md))
- **OBO (v0.7b+):** `fastobo`, `fastobo-owl`

### 4.3 Catalog Layer
- Ontologies, entities, axioms (from Horned-OWL), annotations, imports, diagnostics

### 4.4 Query Layer
- **v0.2:** `ontoindex-query` — `sqlparser` virtual tables + Oxigraph SPARQL
- **v1.0:** joins/aggregations via extended virtual tables first; DataFusion if triggered ([ADR-0011](adr/0011-use-sqlparser-for-sql.md) amendment)

### 4.5 Diagnostics Layer (v0.3+)
- Oxigraph parse errors + in-house catalog lint rules; `diagnostics` SQL table; LSP Problems panel ([DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md))

### 4.6 Diff Layer (v0.9+)
- Horned-OWL axiom diff (in-house logic); `git2` for branch/commit inputs; PR summaries

### 4.7 Docs Layer (v0.9+)
- `pulldown-cmark` + `minijinja` templates; entity pages

### 4.8 Reasoner Layer (v0.6+)
- `ontoindex-reasoner` thin facade over OntoLogos — see [REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0014](adr/0014-rust-native-reasoners-only.md), [ADR-0015](adr/0015-adopt-ontologos-reasoner.md)
- v0.6: `ontologos-*` 0.9.0 (`el`, `rl`, `rdfs`, `explain`)
- v1.0: `ontologos-*` 1.0.0 (`dl`, `facade`, full DL explanations)
- v0.9: optional `ontologos-watch` for incremental reclassify

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
