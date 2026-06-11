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

**Ontology-as-code for Git and VS Code — v0.2 ships today.**

Browse OWL/RDF in VS Code, query and validate in CI, and index workspaces locally with a Rust engine. Editing, diagnostics, reasoning, and semantic diffs are on the [roadmap](#roadmap).

> **Naming:** **OntoCode** is the VS Code extension (product UI). **OntoIndex** is the Rust engine (`ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp`). This repo contains both.

## Choose your path

### Use the CLI (OntoIndex)

```bash
cargo install ontoindex-cli
ontoindex query ./fixtures "SELECT * FROM classes"
ontoindex validate .
```

From a clone, `cargo run --` runs the `ontoindex` binary (workspace default-member is `ontoindex-cli`).

### Use VS Code (OntoCode Explorer)

1. Download the latest VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) (Marketplace coming later).
2. **Extensions → Install from VSIX…**
3. Open a folder with `.ttl`, `.owl`, `.rdf`, or other supported ontology files.
4. Use the **OntoCode** activity bar to browse entities and open the inspector.

Full install and troubleshooting: [docs/vscode-install.md](docs/vscode-install.md).

![OntoCode Explorer preview](docs/media/explorer-preview.svg)

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

Protégé is strong for traditional ontology editing, but most engineering teams live in Git, pull requests, and VS Code. OntoCode targets that workflow:

| Shipped in v0.2 | Planned |
|-----------------|---------|
| Browse ontologies in VS Code | Inline diagnostics (v0.3) |
| Entity inspector and jump-to-source | Authoring and write-back (v0.4) |
| `ontoindex validate` for CI | Query workbench in VS Code (v0.5+) |
| SQL-like and SPARQL queries via CLI | Semantic Git diff, reasoning (v0.6+) |
| Local-first indexing | |

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

### Install the extension

See [docs/vscode-install.md](docs/vscode-install.md) (release VSIX, dev build, LSP troubleshooting) and [extension/README.md](extension/README.md).

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
docs/                   # user guides (install, SQL, LSP API)
ontocode_ontoindex_docs/  # product specs, ADRs, wireframes, backlog
examples/               # Rust examples and query cookbook
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

Column schemas, SQL limits, and examples: [docs/sql-reference.md](docs/sql-reference.md). LSP methods: [docs/lsp-api.md](docs/lsp-api.md).

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md). Quick checks:

```bash
cargo build -p ontoindex-lsp --bins
cargo test --workspace
cd extension && npm ci && npm test
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

The [release workflow](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml) verifies packages, runs tests, publishes workspace crates to [crates.io](https://crates.io/) in dependency order, and creates a GitHub Release with the `ontoindex` Linux binary, per-platform `ontoindex-lsp` archives, and a **multi-platform VSIX** (Linux, macOS, Windows). Requires the `CARGO_REGISTRY_TOKEN` repository secret.

See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) for release notes. Verify downloads: [docs/release-integrity.md](docs/release-integrity.md). Security: [SECURITY.md](SECURITY.md).

## License

MIT OR Apache-2.0
