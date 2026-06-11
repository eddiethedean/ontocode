# SPEC.md — OntoIndex and OntoCode Technical Specification

## 1. System Overview

The system consists of four major layers:

1. **File layer** — ontology files in a local workspace.
2. **OntoIndex layer** — Rust indexing, parsing, cataloging, diagnostics, query, diff, docs.
3. **Language server layer** — editor protocol services.
4. **VS Code extension layer** — UI panels, commands, graph views, editing workflows.

## 2. OntoIndex Crate Layout

```text
ontoindex/
├── crates/
│   ├── ontoindex-core
│   ├── ontoindex-parser
│   ├── ontoindex-catalog
│   ├── ontoindex-query
│   ├── ontoindex-diagnostics
│   ├── ontoindex-diff
│   ├── ontoindex-docs
│   ├── ontoindex-reasoner
│   ├── ontoindex-lsp
│   └── ontoindex-cli
├── examples/
├── benches/
├── tests/
└── docs/
```

## 3. Supported Ontology Formats

Required:

- Turtle `.ttl`
- RDF/XML `.rdf`
- OWL `.owl`
- JSON-LD `.jsonld`
- N-Triples `.nt`

Desired:

- N-Quads `.nq`
- TriG `.trig`
- OBO `.obo`

## 4. Core Data Model

### OntologyDocument

Fields:

- id
- path
- format
- base_iri
- imports
- namespaces
- parse_status
- content_hash
- modified_time

### Entity

Fields:

- iri
- short_name
- kind
- ontology_id
- source_location
- labels
- comments
- annotations
- deprecated
- usages

### Axiom

Fields:

- id
- ontology_id
- subject
- predicate
- object
- axiom_kind
- source_location
- annotations

### Diagnostic

Fields:

- code
- severity
- message
- file
- range
- entity_iri
- quick_fix

## 5. Virtual Tables

Required virtual tables:

- `ontologies`
- `namespaces`
- `imports`
- `entities`
- `classes`
- `object_properties`
- `data_properties`
- `annotation_properties`
- `individuals`
- `annotations`
- `axioms`
- `subclass_axioms`
- `equivalent_class_axioms`
- `disjoint_class_axioms`
- `domain_axioms`
- `range_axioms`
- `restrictions`
- `usages`
- `diagnostics`
- `broken_imports`
- `duplicate_labels`
- `orphan_classes`
- `deprecated_usages`

## 6. Query Interfaces

### CLI

```bash
ontoindex index .
ontoindex query . "SELECT * FROM classes"
ontoindex sparql . "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
ontoindex validate .
ontoindex diff main..feature
ontoindex docs . --format markdown --out docs/ontology
```

### Rust API

```rust
let catalog = IndexBuilder::new()
    .workspace("./ontology")
    .build()?;

let rows = catalog.query("SELECT * FROM classes WHERE deprecated = true")?;
```

## 7. OntoCode Extension Components

```text
extension/
├── src/
│   ├── activate.ts
│   ├── commands/
│   ├── panels/
│   ├── treeviews/
│   ├── webviews/
│   ├── lsp/
│   └── utils/
├── media/
├── package.json
└── README.md
```

## 8. VS Code Commands

Required commands:

- `OntoCode: Index Workspace`
- `OntoCode: Validate Workspace`
- `OntoCode: Run Ontology Query`
- `OntoCode: Run SPARQL Query`
- `OntoCode: Open Ontology Explorer`
- `OntoCode: Create Class`
- `OntoCode: Create Property`
- `OntoCode: Create Individual`
- `OntoCode: Find Entity Usages`
- `OntoCode: Rename Entity IRI`
- `OntoCode: Generate Documentation`
- `OntoCode: Show Semantic Diff`
- `OntoCode: Run Reasoner`

## 9. LSP Features

Required:

- diagnostics
- hover
- completion
- go to definition
- find references
- document symbols
- workspace symbols
- rename
- code actions
- semantic tokens

## 10. Reasoner Adapter Interface

Reasoners are external integrations accessed through a stable adapter layer.

Required operations:

- classify ontology
- get inferred hierarchy
- find unsatisfiable classes
- explain inference
- validate consistency

## 11. Testing Strategy

- Rust unit tests
- Rust integration tests with ontology fixtures
- golden snapshot tests
- LSP protocol tests
- VS Code extension integration tests
- semantic diff regression tests
- parser fuzz tests
- performance benchmarks
