# OntoCore architecture

> **Status:** OntoCore is the platform identity for the Rust engine shipped since v0.2. Implementation crates retain `ontocore-*` names until v1.0. See [ADR-0018](../design/adr/0018-ontocore-platform-identity.md).

## High-level diagram

```text
+---------------------------+
|        OntoCode UI        |
| VS Code trees + commands  |
| React webviews            |
+-------------+-------------+
              |
              v
+---------------------------+
|   OntoCore LSP            |
|   (ontocore-lsp)         |
+-------------+-------------+
              |
              v
+---------------------------+
|       OntoCore            |
| ontocore façade + crates  |
| catalog/query/diagnostics |
| refactor/reasoner/robot   |
+-------------+-------------+
              |
      +-------+-------+-------+
      v       v               v
+-----------+ +-----------+ +------------------+
| Oxigraph  | | HornedOWL | | OntoLogos        |
| RDF/SPARQL| | OWL axioms| | reasoners        |
+-----+-----+ +-----+-----+ +--------+---------+
      |               |              |
      +───────┬───────┘              |
              v                      |
+---------------------------+        |
|     Workspace Files       |◄───────┘
| owl/rdf/ttl/jsonld/obo    |
+---------------------------+
```

## Responsibility split

| Owner | Responsibilities |
|-------|------------------|
| **OntoCore** | Workspace discovery, indexing, RDF/OWL/OBO parsing, entity catalog, symbol graph, import graph, SQL/SPARQL, diagnostics, refactoring, reasoning integration, patch write-back, CLI, LSP, future Python/TypeScript bindings, future MCP server |
| **OntoCode** | VS Code activity bar, explorer UI, React webviews, inspector, Query Workbench UI, Manchester editor UI, graph panels, extension commands, marketplace packaging, user onboarding |
| **OntoLogos** | OWL reasoning, classification, consistency, explanations, inference profiles |

## Façade and implementation

| Layer | Crates |
|-------|--------|
| Public façade | `ontocore` — re-exports and `Workspace` API |
| Core types | `ontocore-core` |
| Parsing | `ontocore-parser`, `ontocore-owl` |
| Catalog | `ontocore-catalog` |
| Query | `ontocore-query` |
| Diagnostics | `ontocore-diagnostics` |
| Reasoning | `ontocore-reasoner` → OntoLogos |
| Refactoring | `ontocore-refactor` |
| ROBOT | `ontocore-robot` (external Java CLI) |
| CLI | `ontocore-cli` |
| LSP | `ontocore-lsp` |

See [crate map](crate-map.md) for details.

## Key design rules

- **Dual stack ([ADR-0013](../design/adr/0013-dual-stack-oxigraph-horned-owl.md)):** Catalog entities/axioms from Horned-OWL; triple counts and SPARQL from Oxigraph.
- **Reasoner adapters ([ADR-0008](../design/adr/0008-reasoner-adapters-not-built-in-reasoner.md), [ADR-0015](../design/adr/0015-adopt-ontologos-reasoner.md)):** OntoCore delegates reasoning to OntoLogos; no JVM reasoners.
- **Local-first ([ADR-0005](../design/adr/0005-local-first-by-default.md)):** Indexing and queries run on the user's machine by default.
- **LSP boundary ([ADR-0007](../design/adr/0007-language-server-boundary.md)):** OntoCode extension talks to OntoCore only via LSP — no Rust logic in TypeScript.

Target architecture details: [design/ARCHITECTURE.md](../design/ARCHITECTURE.md).
