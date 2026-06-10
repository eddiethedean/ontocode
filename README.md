# OntoCode

[![CI](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/ontoindex-core)](https://github.com/eddiethedean/ontocode/blob/main/LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.86+-orange)](https://github.com/eddiethedean/ontocode/blob/main/Cargo.toml)
[![Rust edition](https://img.shields.io/badge/edition-2021-red)](https://www.rust-lang.org)

[![crates](https://img.shields.io/badge/crates-lightgrey?style=flat-square&logo=rust)](https://crates.io/search?q=ontoindex)
[![core](https://img.shields.io/crates/v/ontoindex-core?label=core)](https://crates.io/crates/ontoindex-core)
[![parser](https://img.shields.io/crates/v/ontoindex-parser?label=parser)](https://crates.io/crates/ontoindex-parser)
[![catalog](https://img.shields.io/crates/v/ontoindex-catalog?label=catalog)](https://crates.io/crates/ontoindex-catalog)
[![query](https://img.shields.io/crates/v/ontoindex-query?label=query)](https://crates.io/crates/ontoindex-query)
[![cli](https://img.shields.io/crates/v/ontoindex-cli?label=cli)](https://crates.io/crates/ontoindex-cli)
[![lsp](https://img.shields.io/crates/v/ontoindex-lsp?label=lsp)](https://crates.io/crates/ontoindex-lsp)
[![downloads](https://img.shields.io/crates/d/ontoindex-cli?label=downloads)](https://crates.io/crates/ontoindex-cli)

**A planned VS Code extension for ontology-as-code — powered by a Rust backend.**

OntoCode aims to become a full ontology engineering workbench inside VS Code: browse classes and properties, edit OWL/RDF, run queries, validate in CI, review semantic diffs in pull requests, and work the way modern software teams already work with Git and editors.

> Build, query, validate, refactor, reason over, and document OWL/RDF ontologies directly in VS Code.

**Status:** v0.2 ships the **OntoCode Explorer** VS Code extension (dev/VSIX install) plus **OntoIndex** — the Rust engine and language server that power it. Marketplace publication is planned; install from a GitHub Release VSIX or local dev build for now.

## Two-layer architecture

OntoCode is designed as two products that ship together:

| Layer | What it is | Status in v0.2.0 |
|-------|------------|-------------------|
| **OntoCode** | VS Code extension (explorer, entity inspector, jump-to-source) | **Explorer shipping** — install VSIX or run from `extension/` |
| **OntoIndex** | Rust library + CLI + LSP (scan, parse, catalog, query, validate) | **Shipping now** |

```text
┌─────────────────────────────────────┐
│  OntoCode (v0.2)                    │
│  VS Code extension + explorer UI    │
└─────────────────┬───────────────────┘
                  │ ontoindex-lsp (stdio)
┌─────────────────▼───────────────────┐
│  OntoIndex (v0.2.0)                 │
│  Rust index, catalog, query, CLI, LSP │
└─────────────────┬───────────────────┘
                  │ Oxigraph / RDF parsers
┌─────────────────▼───────────────────┐
│  Your ontology repo                 │
│  .ttl .owl .rdf .jsonld …           │
└─────────────────────────────────────┘
```

OntoIndex is useful on its own today (CLI, CI, local analysis). The extension will call into the same engine via a language server rather than reimplementing ontology logic in TypeScript.

## Why OntoCode?

Protégé is strong for traditional ontology editing, but most engineering teams live in Git, pull requests, and VS Code. OntoCode is being built for that workflow:

- Git-native semantic diffs and review
- CI-friendly validation (`ontoindex validate`)
- Editor-native navigation and refactoring
- SQL-like and SPARQL querying over a workspace index
- Local-first indexing — no upload by default

Long-term goal: **routine ontology work in VS Code without opening Protégé.**

## What's in v0.2.0 (OntoCode Explorer)

v0.2 adds the VS Code extension described in the [v0.2 roadmap](https://github.com/eddiethedean/ontocode/blob/main/ontocode_ontoindex_docs/ROADMAP.md):

- **VS Code extension** — OntoCode activity bar with ontology tree views
- **Entity inspector** — IRI, labels, comments, parents, children, axioms
- **Jump to source** — open Turtle/RDF files at entity declarations
- **`ontoindex-lsp`** — language server with custom catalog methods
- **LSP browsing** — hover, document/workspace symbols, go-to-definition

Exit criterion (works today):

1. Open a repo with `.ttl` files in VS Code with the extension loaded
2. Browse ontologies, classes, properties, and individuals in the sidebar
3. Click an entity to inspect it and jump to its source

### Install the extension (dev)

```bash
./scripts/package-extension.sh
cd extension && npx vsce package --no-dependencies
```

Install the generated `.vsix` via **Extensions: Install from VSIX**. Or press **F5** in VS Code with the `extension/` folder open.

See [extension/README.md](https://github.com/eddiethedean/ontocode/blob/main/extension/README.md) for commands and settings.

## What's in v0.1.0 (OntoIndex foundation)

This release delivers the Rust backend described in the [v0.1 roadmap](https://github.com/eddiethedean/ontocode/blob/main/ontocode_ontoindex_docs/ROADMAP.md):

- **Workspace scanner** — recursive discovery, `.gitignore` support, content hashing
- **RDF/OWL parsing** — Turtle, RDF/XML, OWL, JSON-LD, N-Triples, N-Quads, TriG via [Oxigraph](https://github.com/oxigraph/oxigraph)
- **Semantic catalog** — ontologies, classes, properties, individuals, annotations, axioms, namespaces, imports
- **SQL-like queries** — `SELECT`, `FROM`, `WHERE`, projections, CSV/JSON export
- **SPARQL** — query indexed triples directly
- **CLI** — `ontoindex index`, `query`, `sparql`, `validate`, `inspect`

Exit criterion (works today):

```bash
cargo run -- query ./fixtures "SELECT * FROM classes"
```

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

# Validate (non-zero exit on parse errors — CI-friendly)
cargo run -- validate fixtures

# JSON output
cargo run -- query fixtures "SELECT * FROM classes" --format json
```

Install the CLI from crates.io (binary name: `ontoindex`):

```bash
cargo install ontoindex-cli
ontoindex query ./fixtures "SELECT * FROM classes"
```

Or build from source after cloning this repository.

## Planned VS Code experience (v0.3+)

Specs and wireframes live in [ontocode_ontoindex_docs/](https://github.com/eddiethedean/ontocode/tree/main/ontocode_ontoindex_docs). Upcoming OntoCode UI includes:

- Inline diagnostics and validation (v0.3)
- Class/property/individual authoring (v0.4)
- SPARQL and SQL query panels (v0.5+)
- Reasoner integration and graph visualization
- Semantic Git diff viewer

The extension is a thin TypeScript shell over **ontoindex-lsp** and the OntoIndex crates — not a second ontology stack.

## Roadmap

| Version | Deliverable |
|---------|-------------|
| v0.1 | OntoIndex: scanner, parser, catalog, CLI |
| **v0.2** (current) | VS Code extension, explorer, entity inspector, LSP |
| v0.3 | Diagnostics and Problems panel integration |
| v0.4 | Editing and patch-based write-back |
| v0.5 | Query workbench |
| v0.6–v0.9 | Reasoning, graphs, refactoring, semantic diff, docs |
| v1.0 | Protégé-replacement release for daily ontology engineering |

See [ROADMAP.md](https://github.com/eddiethedean/ontocode/blob/main/ontocode_ontoindex_docs/ROADMAP.md) and [PLAN.md](https://github.com/eddiethedean/ontocode/blob/main/ontocode_ontoindex_docs/PLAN.md) for the full product plan.

## Repository layout

```text
crates/
├── ontoindex-core      # types, workspace scanner
├── ontoindex-parser    # RDF parsing and entity extraction
├── ontoindex-catalog   # index builder and semantic catalog
├── ontoindex-query     # SQL-like and SPARQL engines
├── ontoindex-cli       # `ontoindex` binary
└── ontoindex-lsp       # language server for OntoCode
extension/              # VS Code extension (OntoCode Explorer)
fixtures/               # sample ontology for tests
scripts/                # extension packaging helpers
ontocode_ontoindex_docs/  # specs, ADRs, wireframes, backlog
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
cargo build -p ontoindex-lsp --bins
cargo test --workspace
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

Update golden snapshots:

```bash
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes
```

## Releasing

Published crates (v0.2.0):

| Crate | crates.io |
|-------|-----------|
| `ontoindex-core` | https://crates.io/crates/ontoindex-core |
| `ontoindex-parser` | https://crates.io/crates/ontoindex-parser |
| `ontoindex-catalog` | https://crates.io/crates/ontoindex-catalog |
| `ontoindex-query` | https://crates.io/crates/ontoindex-query |
| `ontoindex-lsp` | https://crates.io/crates/ontoindex-lsp |
| `ontoindex-cli` | https://crates.io/crates/ontoindex-cli |

Push a tag matching `[workspace.package].version` in `Cargo.toml` (e.g. `v0.2.0`):

```bash
git tag v0.2.0
git push origin v0.2.0
```

The [release workflow](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml) verifies packages, runs tests, publishes workspace crates to [crates.io](https://crates.io/) in dependency order, and creates a GitHub Release with the `ontoindex` and `ontoindex-lsp` Linux binaries plus an extension VSIX. Requires the `CARGO_REGISTRY_TOKEN` repository secret.

See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for release notes.

## License

MIT OR Apache-2.0
