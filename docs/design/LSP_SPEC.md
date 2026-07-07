# LSP_SPEC.md

> **Document status: future target design**
>
> **Do not use this page for integration.** For what ships today, read **[lsp-api.md](../lsp-api.md)** (authoritative for **v0.13**).
> Implemented: hover, document/workspace symbols, go-to-definition, diagnostics publishing,
> `ontocore/indexWorkspace`, `ontocore/getCatalogSnapshot`, `ontocore/getEntity`, `ontocore/applyAxiomPatch`,
> `ontocore/query`, `ontocore/sparql`, `ontocore/parseManchester`, `ontocore/runReasoner`, `ontocore/getExplanation`.
> See [`handlers.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-lsp/src/handlers.rs) and
> [`protocol.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-lsp/src/protocol.rs).

## 1. Purpose

The OntoCore language server provides ontology-aware editor features to OntoCode and potentially other editors.

## 2. Transport

- stdio for VS Code (shipped)
- optional TCP for debugging (**not implemented** — if added, must bind `127.0.0.1` only and require explicit opt-in; never expose unauthenticated LSP on a public interface; see [security.md](../security.md))

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

**v0.3 (shipped):** parse errors, broken imports, undefined prefixes, duplicate/missing labels, orphan classes — via `textDocument/publishDiagnostics` and `CatalogSnapshot.diagnostics`. Deferred to later milestones: missing comments, deprecated usage, invalid domain/range.

**Sources ([DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md)):**

- **Parse errors** — [`oxigraph`](https://crates.io/crates/oxigraph) via `ontocore-parser`
- **Import / prefix / quality rules** — in-house `ontocore-diagnostics` on catalog (duplicate labels, orphans, broken imports, undefined prefixes)
- **OBO (v0.7b)** — [`fastobo-validator`](https://crates.io/crates/fastobo-validator) violations mapped to LSP
- **SHACL (v1.0 P1)** — [`rudof`](https://crates.io/crates/rudof) via plugin adapter

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

| Method | v0.3 status |
|--------|-------------|
| `ontocore/indexWorkspace` | **Implemented** |
| `ontocore/getCatalogSnapshot` | **Implemented** (not listed in early drafts; used by explorer) |
| `ontocore/getEntity` | **Implemented** |
| `ontocore/query` | **Implemented** (v0.5) |
| `ontocore/sparql` | **Implemented** (v0.5) |
| `ontocore/getGraph` | **Implemented** (v0.7) — see [lsp-api.md](../lsp-api.md) |
| `ontocore/getSemanticDiff` | Planned |
| `ontocore/runReasoner` | **Implemented** (v0.6) — see [lsp-api.md](../lsp-api.md) |
| `ontocore/applyAxiomPatch` | **Implemented** (v0.4) |
| `ontocore/parseManchester` | **Implemented** (v0.5) |
| `ontocore/getExplanation` | **Implemented** (v0.6) — see [lsp-api.md](../lsp-api.md) |
| `ontocore/runRobot` | **Implemented** (v0.7) — see [lsp-api.md](../lsp-api.md) |

### `ontocore/indexWorkspace`

Indexes the workspace.

### `ontocore/getCatalogSnapshot`

Returns documents, entities, class hierarchy, and diagnostics for UI clients.

### `ontocore/query`

Runs SQL-style query against the indexed workspace catalog (implemented v0.5).

### `ontocore/sparql`

Runs SPARQL query against the indexed catalog (implemented v0.5).

### `ontocore/getEntity`

Returns entity details.

### `ontocore/getGraph`

Returns graph data for visualization.

### `ontocore/getSemanticDiff`

Returns semantic diff between two refs or catalogs.

### `ontocore/runReasoner`

Runs configured reasoner.

### `ontocore/applyAxiomPatch`

Apply a Horned-OWL axiom patch to a document. Used by quick forms and Manchester editor.
See [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md).

**Params:** document URI, axiom patch (JSON), preview-only flag.

### `ontocore/parseManchester`

Parse Manchester OWL Syntax expression; return diagnostics and normalized form.

### `ontocore/getExplanation`

Return justification chain for unsatisfiable class. See [REASONER_SPEC.md](REASONER_SPEC.md).

### `ontocore/runRobot`

Run ROBOT subcommand (`validate`, `merge`, `report`). See [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md).

## 6. Error Handling

All custom methods must return structured errors with:

- code
- message
- recoverable
- user_action
