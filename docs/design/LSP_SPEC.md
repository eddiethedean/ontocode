# LSP_SPEC.md

> **Document status: target design (v0.2 partial implementation)**
>
> For what ships today, read **[docs/lsp-api.md](../lsp-api.md)** (authoritative for v0.2).
> Implemented: hover, document/workspace symbols, go-to-definition, and custom methods
> `ontoindex/indexWorkspace`, `ontoindex/getCatalogSnapshot`, `ontoindex/getEntity`.
> See [`crates/ontoindex-lsp/src/handlers.rs`](../crates/ontoindex-lsp/src/handlers.rs) and
> [`crates/ontoindex-lsp/src/protocol.rs`](../crates/ontoindex-lsp/src/protocol.rs).

## 1. Purpose

The OntoIndex language server provides ontology-aware editor features to OntoCode and potentially other editors.

## 2. Transport

- stdio for VS Code (shipped)
- optional TCP for debugging (**not implemented** — if added, must bind `127.0.0.1` only and require explicit opt-in; never expose unauthenticated LSP on a public interface; see [SECURITY.md](../SECURITY.md))

## 3. Supported File Types

- Turtle
- RDF/XML
- OWL/XML
- JSON-LD
- OBO
- N-Triples
- TriG

## 4. Required LSP Capabilities

Sections below describe the **target** capability set. Implementation status is noted where v0.2 differs.

### 4.1 Diagnostics

**v0.2:** not implemented (planned v0.3).

Diagnostics include:

- parse errors
- undefined prefixes
- broken imports
- duplicate labels
- missing labels
- missing comments
- deprecated usage
- invalid domain/range reference

### 4.2 Hover

**v0.2:** partial — basic entity information.

Hover should show:

- entity IRI
- label
- comment
- type
- source ontology
- deprecation status
- parent classes
- usages count

### 4.3 Completion

**v0.2:** not implemented.

Completion contexts:

- prefixes
- IRIs
- known classes
- known properties
- annotation properties
- imported entities

### 4.4 Go to Definition

**v0.2:** implemented.

For entity references, jump to source declaration.

### 4.5 Find References

Return all entity usages across workspace.

### 4.6 Rename

Safe IRI rename across workspace.

Requirements:

- preview edits
- update imports if needed
- update annotations and axioms
- avoid string-only false positives

### 4.7 Code Actions

Examples:

- add missing label
- add missing comment
- resolve prefix
- create missing import
- mark deprecated usage
- replace deprecated entity

### 4.8 Document Symbols

Expose ontology entities as symbols.

### 4.9 Workspace Symbols

Global entity search.

## 5. Custom LSP Methods

| Method | v0.2 status |
|--------|-------------|
| `ontoindex/indexWorkspace` | **Implemented** |
| `ontoindex/getCatalogSnapshot` | **Implemented** (not listed in early drafts; used by explorer) |
| `ontoindex/getEntity` | **Implemented** |
| `ontoindex/query` | Planned |
| `ontoindex/sparql` | Planned |
| `ontoindex/getGraph` | Planned |
| `ontoindex/getSemanticDiff` | Planned |
| `ontoindex/runReasoner` | Planned (v0.6) |
| `ontoindex/applyAxiomPatch` | Planned (v0.4a+) |
| `ontoindex/parseManchester` | Planned (v0.5) |
| `ontoindex/getExplanation` | Planned (v0.6) |
| `ontoindex/runRobot` | Planned (v0.7b) |

### `ontoindex/indexWorkspace`

Indexes the workspace.

### `ontoindex/getCatalogSnapshot`

Returns documents, entities, and class hierarchy for UI clients.

### `ontoindex/query`

Runs SQL-style query (use CLI or Rust API in v0.2).

### `ontoindex/sparql`

Runs SPARQL query (use CLI or Rust API in v0.2).

### `ontoindex/getEntity`

Returns entity details.

### `ontoindex/getGraph`

Returns graph data for visualization.

### `ontoindex/getSemanticDiff`

Returns semantic diff between two refs or catalogs.

### `ontoindex/runReasoner`

Runs configured reasoner.

### `ontoindex/applyAxiomPatch`

Apply a Horned-OWL axiom patch to a document. Used by quick forms and Manchester editor.
See [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md).

**Params:** document URI, axiom patch (JSON), preview-only flag.

### `ontoindex/parseManchester`

Parse Manchester OWL Syntax expression; return diagnostics and normalized form.

### `ontoindex/getExplanation`

Return justification chain for unsatisfiable class. See [REASONER_SPEC.md](REASONER_SPEC.md).

### `ontoindex/runRobot`

Run ROBOT subcommand (`validate`, `merge`, `report`). See [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md).

## 6. Error Handling

All custom methods must return structured errors with:

- code
- message
- recoverable
- user_action
