OntoCore & OntoCode Roadmap

Vision

Build the modern open-source platform for ontology engineering.

OntoCore provides the semantic workspace engine.

OntoCode provides the flagship IDE experience powered by OntoCore.

Together they modernize ontology development in the same way LLVM, rust-analyzer, and DuckDB modernized software development.

⸻

Platform Architecture

                 Applications
──────────────────────────────────────────────
OntoCode (VS Code)
Python SDK
TypeScript SDK
CLI
GitHub Actions
MCP Server
Documentation
Desktop
Web
Future IDEs
──────────────────────────────────────────────
OntoCore
Workspace Engine
Semantic Index
Parser
Query Engine
Diagnostics
Navigation
Refactoring
Reasoning Integration
LSP
SQL
SPARQL
Plugin System
Persistent Cache
──────────────────────────────────────────────
Ontologos
Reasoning
Classification
Inference
Explanation
Consistency
──────────────────────────────────────────────
OWL RDF SHACL Turtle OBO

⸻

v0.10 — OntoCore Foundation

Goal

Transform OntoIndex into the OntoCore platform.

Platform

* OntoCore branding
* Public API
* Umbrella crate
* Stable workspace model
* Unified documentation
* Cargo cleanup
* Better crate organization

Workspace

* Multi-root workspaces
* Import graph
* Namespace registry
* Prefix resolution
* Workspace metadata
* Configuration files

Parser

* OWL
* RDF/XML
* Turtle
* N-Triples
* JSON-LD
* OBO
* Manchester syntax

Query

* Workspace search
* Symbol search
* Label search
* Annotation search
* IRI search

⸻

v0.11 — Semantic Workspace

Goal

Become the ontology equivalent of rust-analyzer.

Navigation

* Go to Definition
* Find References
* Find Implementations
* Rename
* Hover
* Workspace Symbols
* Semantic Tokens

Semantic Graph

* Class hierarchy
* Property hierarchy
* Individual graph
* Import graph
* Dependency graph
* Usage graph

Search

* Full text
* Semantic search
* Regex search
* Metadata search

⸻

v0.12 — Diagnostics & Quality

Goal

Become the ontology quality platform.

Diagnostics

* Broken imports
* Circular imports
* Undefined prefixes
* Duplicate labels
* Missing labels
* Missing comments
* Missing domains
* Missing ranges
* Deprecated usage
* Annotation validation
* Ontology smells

Metrics

* Complexity
* Coverage
* Documentation %
* Class count
* Property count
* Restriction count
* Ontology size
* Dependency metrics

Reports

* HTML
* Markdown
* JSON

⸻

v0.13 — SQL Workspace Engine

Goal

DuckDB for ontology engineering.

Virtual Tables

* ontology.classes
* ontology.properties
* ontology.individuals
* ontology.annotations
* ontology.imports
* ontology.axioms
* ontology.restrictions
* ontology.metrics
* ontology.diagnostics

Analytics

* SQL joins
* Aggregation
* Reporting
* Statistics

Export

* CSV
* Parquet
* Arrow

⸻

v0.14 — Refactoring Engine

Goal

Modern IDE refactoring.

Refactoring

* Rename
* Move
* Namespace migration
* Merge ontology
* Extract ontology
* Safe delete
* Preview changes
* Batch updates

⸻

v0.15 — Ontologos Integration

Goal

Deep semantic reasoning.

Reasoning

* Classification
* Consistency
* Explanation
* Unsatisfiable classes
* Inferred hierarchy
* Incremental reasoning
* Reasoning cache

Semantic Views

* inferred_classes
* inferred_properties
* inferred_relationships

⸻

v0.16 — AI Platform

Goal

Become AI-native.

MCP

* Workspace API
* Search API
* Query API
* Diagnostics API
* Refactoring API

AI

* Semantic context
* Ontology explanations
* Documentation generation
* Competency questions
* Ontology review
* Modeling suggestions

⸻

v0.17 — Plugin Platform

Goal

Everything is extensible.

Plugins

* Diagnostics
* Queries
* Reports
* Refactoring
* Commands
* Visualization
* Validation

SDK

* Rust
* Python
* TypeScript

⸻

v0.18 — Python SDK

Goal

First-class Python experience.

Features

* Workspace API
* Query API
* Diagnostics
* SQL
* Search
* Refactoring
* Reasoning
* DataFrame export

⸻

v0.19 — JavaScript SDK

Goal

Power modern web applications.

Features

* Node bindings
* Browser bindings
* React hooks
* WebAssembly support
* LSP integration

⸻

v0.20 — OntoCode Modern IDE

Goal

The best ontology IDE available.

Explorer

* Workspace Explorer
* Symbol Explorer
* Import Explorer
* Graph Explorer

Editors

* Turtle
* Manchester
* OWL
* OBO

Panels

* Diagnostics
* Search
* SQL Query
* SPARQL Query
* Metrics
* Reasoning
* Documentation

Visualization

* Class graph
* Import graph
* Dependency graph
* Ontology metrics
* Interactive hierarchy

⸻

v0.21 — OBO / ROBOT / owlmake

Goal

Become the best OBO development environment.

Integration

* owlmake
* ROBOT compatibility
* ODK workflows
* Release automation
* Validation
* Imports
* Templates

⸻

v0.22 — Enterprise

Goal

Production-ready platform.

Features

* Workspace cache
* Incremental indexing
* Large ontology support
* Multi-million triple workspaces
* Parallel indexing
* Benchmark suite
* Performance profiler

⸻

v1.0

OntoCore

* Stable public API
* Stable plugin API
* Long-term support
* Comprehensive documentation
* Cross-platform
* Python
* JavaScript
* Rust
* CLI
* LSP

OntoCode

* Full-featured VS Code IDE
* Protégé migration tools
* AI assistant
* Complete visualization suite
* Enterprise-ready
* Production stable

⸻

Long-Term Vision

OntoCore becomes the semantic workspace engine used by:

* OntoCode
* AI assistants
* MCP servers
* Documentation generators
* GitHub Actions
* Enterprise governance tools
* Data quality platforms
* Knowledge graph tooling
* Future JetBrains plugins
* Future desktop applications
* Future web applications

The success of OntoCore will be measured by the number of tools that build upon it.

The success of OntoCode will be measured by making ontology engineering as productive, modern, and enjoyable as software engineering.