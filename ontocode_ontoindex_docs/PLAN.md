# PLAN.md — OntoIndex and OntoCode

## 1. Executive Summary

OntoIndex and OntoCode form a two-layer product strategy for modern ontology engineering.

**OntoIndex** is the Rust backend: a local-first embedded ontology engine that scans a directory of OWL/RDF/Turtle/JSON-LD/OBO files, builds a semantic index, exposes ontology concepts as queryable tables, validates ontology repositories, performs semantic diffs, and powers editor integrations.

**OntoCode** is the VS Code extension: a full ontology engineering workbench built on top of OntoIndex. Its long-term goal is to replace Protégé for developers, data engineers, knowledge graph engineers, semantic modelers, and organizations that prefer Git-native ontology-as-code workflows.

## 2. Product Thesis

Protégé is excellent for traditional ontology editing, but it is not designed around the way modern software teams work:

- Git repositories
- pull requests
- CI validation
- semantic diffs
- code review
- local development
- editor-native workflows
- AI-assisted engineering
- documentation pipelines
- automated quality checks

OntoCode should eventually provide the authoring depth of Protégé while adding the workflow strengths of VS Code.

## 3. Products

### 3.1 OntoIndex

OntoIndex is a Rust library and CLI.

Primary capabilities:

- Recursively scan ontology repositories
- Parse `.owl`, `.rdf`, `.ttl`, `.jsonld`, `.obo`, `.nt`, `.nq`, `.trig`
- Build incremental ontology catalog
- Expose virtual tables for ontology entities
- Run SQL-like queries over ontology structures
- Run SPARQL over RDF triples
- Detect errors, warnings, and quality issues
- Generate semantic diffs
- Generate documentation
- Provide services for a language server

### 3.2 OntoCode

OntoCode is a VS Code extension.

Primary capabilities:

- Ontology Explorer sidebar
- Class hierarchy editor
- Object/data/annotation property editor
- Individual editor
- Axiom editor
- Imports manager
- Query workbench
- Reasoner integration
- Graph visualization
- Semantic Git diff viewer
- Documentation generator
- AI-assisted ontology review

## 4. v1.0.0 Bar

v1.0.0 should be positioned as:

> A serious Protégé replacement for ontology-as-code workflows inside VS Code.

The v1.0.0 release must allow daily ontology engineering work without requiring users to open Protégé for routine tasks.

Required v1.0.0 features:

- Full workspace indexing
- Full ontology entity browsing
- Class/property/individual CRUD
- Annotation editing
- OWL axiom editing
- Imports management
- SPARQL query panel
- SQL-like ontology query panel
- Reasoner adapter support
- Inferred hierarchy display
- Unsatisfiable class reporting
- Entity usage lookup
- Safe IRI rename
- Semantic diff
- Git-aware review workflow
- Documentation export
- CI validation command

## 5. Target Users

### 5.1 Primary Users

- Ontology engineers
- Knowledge graph engineers
- Semantic web developers
- Data governance engineers
- Biomedical ontology maintainers
- Enterprise taxonomy/modeling teams
- Standards authors
- AI/LLM knowledge modeling teams

### 5.2 Secondary Users

- Data engineers
- Software engineers working with RDF/OWL assets
- Technical writers documenting domain models
- QA teams validating semantic repositories
- Researchers maintaining ontology datasets

## 6. Core Differentiators

OntoCode should not merely copy Protégé. It should beat Protégé in developer workflows.

Key differentiators:

- Git-native semantic diffs
- CI-friendly validation
- VS Code-native editing
- Queryable ontology repository model
- Repo-wide refactoring
- AI-friendly schema and docs generation
- Local-first indexing
- Workspace-aware imports
- File-aware diagnostics
- Documentation generation
- Extension/plugin architecture

## 7. Positioning

Possible tagline:

> Build, query, validate, refactor, reason over, and document OWL/RDF ontologies directly in VS Code.

Short positioning:

> OntoCode is a VS Code-native ontology workbench powered by OntoIndex, a Rust ontology index and query engine.

## 8. Roadmap Summary

- v0.1: OntoIndex scanner, parser, catalog, CLI
- v0.2: OntoCode explorer and entity inspector
- v0.3: diagnostics and validation
- v0.4: editing and write-back
- v0.5: query workbench
- v0.6: reasoner adapters
- v0.7: graph visualization
- v0.8: refactoring
- v0.9: semantic diff, docs, CI
- v1.0: Protégé replacement release

## 9. Non-Goals for Early Releases

- Hosted collaborative editing
- Multi-user permissions
- Full WebProtégé replacement
- Cloud ontology repository hosting
- Enterprise identity management
- Custom visual graph database hosting
- Reimplementing a triplestore from scratch

## 10. Strategic Implementation Guidance

Build on existing mature components:

- Use **Horned-OWL** for OWL 2 parsing/modeling where possible.
- Use **Oxigraph** for RDF/SPARQL infrastructure.
- Use **DataFusion** or equivalent for SQL-style query execution.
- Use a Rust language-server backend for editor services.
- Use TypeScript only for VS Code UI orchestration.
- Keep OntoIndex useful as a standalone CLI even without OntoCode.
