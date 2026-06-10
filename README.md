# OntoCode 0.1.0 — OntoIndex Foundation

OntoCode is a VS Code-native ontology workbench. **v0.1.0** delivers **OntoIndex**: the Rust backend that scans, parses, catalogs, and queries ontology repositories.

This release implements the [v0.1 roadmap](ontocode_ontoindex_docs/ROADMAP.md) exit criterion:

```bash
cargo run -- query ./fixtures "SELECT * FROM classes"
```

## Features (v0.1.0)

- **Workspace scanner** — recursive discovery with `.gitignore` support and content hashing
- **RDF/OWL parsing** — Turtle, RDF/XML, OWL, JSON-LD, N-Triples, N-Quads, TriG via [Oxigraph](https://github.com/oxigraph/oxigraph)
- **Semantic catalog** — ontologies, classes, properties, individuals, annotations, axioms, namespaces, imports
- **SQL-like queries** — `SELECT`, `FROM`, `WHERE`, column projection, CSV/JSON export
- **SPARQL** — query indexed triples directly
- **CLI** — `index`, `query`, `sparql`, `validate`, `inspect`

## Quick start

```bash
# Build
cargo build --release

# Index and inspect a workspace
cargo run -- inspect fixtures

# Query classes
cargo run -- query fixtures "SELECT * FROM classes"

# Filter results
cargo run -- query fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"

# SPARQL
cargo run -- sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"

# Validate (non-zero exit on parse errors)
cargo run -- validate fixtures

# JSON output
cargo run -- query fixtures "SELECT * FROM classes" --format json
```

## Workspace layout

```text
crates/
├── ontoindex-core      # types, scanner
├── ontoindex-parser    # RDF parsing and entity extraction
├── ontoindex-catalog   # index builder and catalog
├── ontoindex-query     # SQL-like and SPARQL query engines
└── ontoindex-cli       # `ontoindex` binary
fixtures/               # sample ontology for tests
tests/                  # integration and golden snapshot tests
```

## Virtual tables

| Table | Description |
|-------|-------------|
| `ontologies` | Indexed ontology documents |
| `classes` | OWL/RDFS classes |
| `object_properties` | OWL object properties |
| `data_properties` | OWL datatype properties |
| `annotation_properties` | OWL annotation properties |
| `individuals` | OWL named individuals |
| `entities` | All extracted entities |
| `annotations` | Label/comment and other annotation triples |
| `axioms` | Extracted axioms (e.g. SubClassOf) |
| `namespaces` | Namespace prefixes |
| `imports` | Ontology imports |
| `properties` | Union of all property kinds |

## Development

```bash
cargo test
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## Releasing

Push a tag matching the workspace version in `Cargo.toml` (e.g. `v0.1.0`):

```bash
git tag v0.1.0
git push origin v0.1.0
```

The [release workflow](.github/workflows/release.yml) will verify the tag, run tests, publish workspace crates to [crates.io](https://crates.io/) (in dependency order), and create a GitHub Release with the `ontoindex` Linux binary. Requires the `CARGO_REGISTRY_TOKEN` repository secret.

Update golden snapshots:

```bash
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes
```

## Roadmap

| Version | Focus |
|---------|-------|
| **v0.1** (this release) | OntoIndex scanner, parser, catalog, CLI |
| v0.2 | OntoCode VS Code explorer and entity inspector |
| v0.3+ | Diagnostics, editing, query workbench, reasoning |

See [ontocode_ontoindex_docs/](ontocode_ontoindex_docs/) for full specifications.

## License

MIT OR Apache-2.0
