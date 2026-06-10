# ROADMAP.md

## v0.1 — OntoIndex Foundation

Deliverables:

- Rust workspace
- CLI skeleton
- recursive scanner
- file hashing
- parser adapters
- basic catalog
- `ontologies`, `classes`, `properties` tables
- basic SQL query
- basic SPARQL query

Exit criteria:

- User can run `ontoindex query ./repo "SELECT * FROM classes"`.

## v0.2 — OntoCode Explorer

Deliverables:

- VS Code extension skeleton
- language server process
- workspace indexing command
- ontology explorer
- class/property/individual trees
- entity inspector
- jump to source

Exit criteria:

- User can browse an ontology repo in VS Code.

## v0.3 — Diagnostics

Deliverables:

- parse errors
- broken imports
- undefined prefixes
- duplicate labels
- missing labels
- orphan classes
- diagnostics table
- VS Code Problems integration

Exit criteria:

- User gets useful ontology diagnostics inline.

## v0.4 — Editing

Deliverables:

- create class
- create object property
- create data property
- create annotation property
- create individual
- edit labels/comments
- delete entity
- patch-based write-back

Exit criteria:

- User can perform basic authoring without Protégé.

## v0.5 — Query Workbench

Deliverables:

- SQL query webview
- SPARQL query webview
- saved queries
- result table
- CSV/JSON export
- query history

Exit criteria:

- User can inspect and analyze ontologies interactively.

## v0.6 — Reasoning

Deliverables:

- reasoner adapter API
- ELK adapter
- HermiT adapter
- unsatisfiable classes
- inferred hierarchy view
- explanation placeholder

Exit criteria:

- User can run classification and inspect inferred hierarchy.

## v0.7 — Visualization

Deliverables:

- class graph
- property graph
- import graph
- entity neighborhood graph
- graph filtering
- click node to inspect

Exit criteria:

- User can navigate ontology visually.

## v0.8 — Refactoring

Deliverables:

- safe IRI rename
- namespace migration
- find usages
- move entity between files
- extract module
- preview changes

Exit criteria:

- User can safely refactor ontology repositories.

## v0.9 — Workflow and Documentation

Deliverables:

- semantic diff
- Git branch comparison
- breaking change report
- CI validation command
- Markdown docs export
- HTML docs export
- PR summary generation

Exit criteria:

- User can use OntoCode in team development workflows.

## v1.0.0 — Protégé Replacement Release

Deliverables:

- complete routine ontology CRUD
- axiom editing
- imports management
- query workbench
- reasoner integration
- graph visualization
- diagnostics
- semantic diff
- refactoring
- docs export
- stable CLI/API/LSP

Exit criteria:

- A daily ontology engineer can complete routine work in VS Code without opening Protégé.
