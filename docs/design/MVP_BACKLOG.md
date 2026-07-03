# MVP_BACKLOG.md

## v0.1 OntoCore MVP Backlog

### Project Setup
- [x] Create Rust workspace
- [x] Configure CI
- [x] Configure clippy/rustfmt
- [x] Add fixture ontology repository
- [x] Add golden snapshot test harness

### Scanner
- [x] Recursively scan workspace
- [x] Respect `.gitignore`
- [x] Detect ontology file extensions
- [x] Compute content hashes
- [x] Track modified files

### Parser
- [x] Parse Turtle
- [x] Parse RDF/XML
- [x] Parse OWL
- [x] Parse JSON-LD
- [x] Extract namespaces
- [x] Extract imports
- [ ] Capture source locations where possible

### Catalog
- [x] Store ontologies
- [x] Store classes
- [x] Store object properties
- [x] Store data properties
- [x] Store individuals
- [x] Store annotations
- [x] Store axioms

### Query
- [x] Implement `SELECT * FROM classes`
- [x] Implement filters
- [x] Implement projections
- [x] Implement CSV export
- [x] Implement JSON export

### CLI
- [x] `ontocore index`
- [x] `ontocore query`
- [x] `ontocore validate`
- [x] `ontocore inspect`

## v0.2 OntoCode MVP Backlog

### Extension Setup
- [x] Create VS Code extension
- [x] Register activity bar icon
- [x] Start language server
- [x] Add configuration section

### Explorer
- [x] Ontologies tree
- [x] Classes tree
- [x] Properties tree
- [x] Individuals tree
- [x] Refresh command

### Inspector
- [x] Open entity inspector
- [x] Show IRI
- [x] Show labels/comments
- [x] Show parents/children
- [x] Jump to source

### LSP
- [x] Publish diagnostics (v0.3)
- [x] Hover support
- [x] document symbols
- [x] workspace symbols

### Packaging
- [x] Build extension locally
- [x] Add README
- [ ] Add screenshots

## v0.3 Diagnostics Backlog

### Engine
- [x] `ontocore-diagnostics` crate with catalog lint rules
- [x] `diagnostics` SQL virtual table
- [x] Parse error locations from Oxigraph
- [x] Open-buffer parsing for unsaved edits

### LSP
- [x] `textDocument/publishDiagnostics` after reindex
- [x] `CatalogSnapshot.diagnostics` for explorer

### Extension
- [x] Diagnostics explorer tree grouped by severity
- [x] Problems panel via LSP (no client wiring required)

### CLI
- [x] `ontocore validate` prints all diagnostics; non-zero exit on errors
