# LSP_SPEC.md

## 1. Purpose

The OntoIndex language server provides ontology-aware editor features to OntoCode and potentially other editors.

## 2. Transport

- stdio for VS Code
- optional TCP for debugging

## 3. Supported File Types

- Turtle
- RDF/XML
- OWL/XML
- JSON-LD
- OBO
- N-Triples
- TriG

## 4. Required LSP Capabilities

### 4.1 Diagnostics

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

Completion contexts:

- prefixes
- IRIs
- known classes
- known properties
- annotation properties
- imported entities

### 4.4 Go to Definition

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

### `ontoindex/indexWorkspace`

Indexes the workspace.

### `ontoindex/query`

Runs SQL-style query.

### `ontoindex/sparql`

Runs SPARQL query.

### `ontoindex/getEntity`

Returns entity details.

### `ontoindex/getGraph`

Returns graph data for visualization.

### `ontoindex/getSemanticDiff`

Returns semantic diff between two refs or catalogs.

### `ontoindex/runReasoner`

Runs configured reasoner.

## 6. Error Handling

All custom methods must return structured errors with:

- code
- message
- recoverable
- user_action
