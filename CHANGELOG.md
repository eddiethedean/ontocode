# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-10

### Added

- **OntoIndex foundation** — Rust workspace for local-first ontology indexing
- `ontoindex-core` — workspace scanner, shared types, content hashing
- `ontoindex-parser` — RDF/OWL parsing and entity extraction via Oxigraph
- `ontoindex-catalog` — semantic catalog and triple store
- `ontoindex-query` — SQL-like virtual tables and SPARQL queries
- `ontoindex-cli` — `ontoindex` binary with `index`, `query`, `sparql`, `validate`, and `inspect` commands
- Fixture ontology and integration/golden snapshot tests
- CI and crates.io release workflows

[0.1.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.1.0
