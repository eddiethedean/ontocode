# MVP_BACKLOG.md

## v0.1 OntoIndex MVP Backlog

### Project Setup
- [ ] Create Rust workspace
- [ ] Configure CI
- [ ] Configure clippy/rustfmt
- [ ] Add fixture ontology repository
- [ ] Add golden snapshot test harness

### Scanner
- [ ] Recursively scan workspace
- [ ] Respect `.gitignore`
- [ ] Detect ontology file extensions
- [ ] Compute content hashes
- [ ] Track modified files

### Parser
- [ ] Parse Turtle
- [ ] Parse RDF/XML
- [ ] Parse OWL
- [ ] Parse JSON-LD
- [ ] Extract namespaces
- [ ] Extract imports
- [ ] Capture source locations where possible

### Catalog
- [ ] Store ontologies
- [ ] Store classes
- [ ] Store object properties
- [ ] Store data properties
- [ ] Store individuals
- [ ] Store annotations
- [ ] Store axioms

### Query
- [ ] Implement `SELECT * FROM classes`
- [ ] Implement filters
- [ ] Implement projections
- [ ] Implement CSV export
- [ ] Implement JSON export

### CLI
- [ ] `ontoindex index`
- [ ] `ontoindex query`
- [ ] `ontoindex validate`
- [ ] `ontoindex inspect`

## v0.2 OntoCode MVP Backlog

### Extension Setup
- [ ] Create VS Code extension
- [ ] Register activity bar icon
- [ ] Start language server
- [ ] Add configuration section

### Explorer
- [ ] Ontologies tree
- [ ] Classes tree
- [ ] Properties tree
- [ ] Individuals tree
- [ ] Refresh command

### Inspector
- [ ] Open entity inspector
- [ ] Show IRI
- [ ] Show labels/comments
- [ ] Show parents/children
- [ ] Jump to source

### LSP
- [ ] Publish diagnostics
- [ ] Hover support
- [ ] document symbols
- [ ] workspace symbols

### Packaging
- [ ] Build extension locally
- [ ] Add README
- [ ] Add screenshots
