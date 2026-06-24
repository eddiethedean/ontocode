# OntoCode

[![CI](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml)
[![Extension VS Code E2E](https://github.com/eddiethedean/ontocode/actions/workflows/extension-vscode-e2e.yml/badge.svg)](https://github.com/eddiethedean/ontocode/actions/workflows/extension-vscode-e2e.yml)
[![License](https://img.shields.io/crates/l/ontoindex-core)](https://github.com/eddiethedean/ontocode/blob/main/LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.88+-orange)](https://github.com/eddiethedean/ontocode/blob/main/Cargo.toml)
[![Rust edition](https://img.shields.io/badge/edition-2021-red)](https://www.rust-lang.org)

[![crates](https://img.shields.io/badge/crates-lightgrey?style=flat-square&logo=rust)](https://crates.io/search?q=ontoindex)
[![core](https://img.shields.io/crates/v/ontoindex-core?label=core)](https://crates.io/crates/ontoindex-core)
[![parser](https://img.shields.io/crates/v/ontoindex-parser?label=parser)](https://crates.io/crates/ontoindex-parser)
[![catalog](https://img.shields.io/crates/v/ontoindex-catalog?label=catalog)](https://crates.io/crates/ontoindex-catalog)
[![query](https://img.shields.io/crates/v/ontoindex-query?label=query)](https://crates.io/crates/ontoindex-query)
[![cli](https://img.shields.io/crates/v/ontoindex-cli?label=cli)](https://crates.io/crates/ontoindex-cli)
[![lsp](https://img.shields.io/crates/v/ontoindex-lsp?label=lsp)](https://crates.io/crates/ontoindex-lsp)
[![owl](https://img.shields.io/crates/v/ontoindex-owl?label=owl)](https://crates.io/crates/ontoindex-owl)
[![downloads](https://img.shields.io/crates/d/ontoindex-cli?label=downloads)](https://crates.io/crates/ontoindex-cli)
[![Docs](https://readthedocs.org/projects/onto-code/badge/?version=latest)](https://onto-code.readthedocs.io/en/latest/)

**Ontology-as-code for Git and VS Code — v0.4.0 ships today.**

Browse OWL/RDF in VS Code, **edit Turtle ontologies**, query and validate in CI, and index workspaces locally with a Rust engine. **v0.4** adds patch-based write-back and Horned-OWL catalog integration; Manchester editing and reasoning are on the [roadmap](#roadmap).

> **Naming:** **OntoCode** is the VS Code extension (product UI). **OntoIndex** is the Rust engine (`ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp`). This repo contains both.

**Documentation:** [Read the Docs](https://onto-code.readthedocs.io/) — [Start here](https://onto-code.readthedocs.io/en/latest/guides/start-here/) · [5-minute guide](https://onto-code.readthedocs.io/en/latest/getting-started/)

## Choose your path

### Use the CLI (OntoIndex)

```bash
cargo install ontoindex-cli
ontoindex query /path/to/your/ontologies "SELECT * FROM classes"
ontoindex validate .
```

> **Note:** `./fixtures` exists only in a git clone. After `cargo install`, point at your own ontology folder (see [getting started guide](https://onto-code.readthedocs.io/en/latest/getting-started/)).

From a clone, `cargo run --` runs the `ontoindex` binary (workspace default-member is `ontoindex-cli`):

```bash
cargo run -- query fixtures "SELECT * FROM classes"
```

### Use VS Code (OntoCode Explorer)

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), or download a VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases).
2. Open a folder with `.ttl`, `.owl`, `.rdf`, or other supported ontology files.
3. Use the **OntoCode** activity bar to browse entities, edit in the inspector, and open diagnostics.

Full install and troubleshooting: [install guide](https://onto-code.readthedocs.io/en/latest/vscode-install/). Editing guide: [authoring guide](https://onto-code.readthedocs.io/en/latest/authoring/).

![OntoCode Explorer preview](docs/media/explorer-preview.png)

## Two-layer architecture

OntoCode is designed as two products that ship together:

| Layer | What it is | Status in v0.4.0 |
|-------|------------|-------------------|
| **OntoCode** | VS Code extension (explorer, entity inspector, diagnostics, **authoring**) | **Shipping** |
| **OntoIndex** | Rust library + CLI + LSP (scan, parse, catalog, query, validate, diagnostics, **write-back**) | **Shipping** |

```text
┌─────────────────────────────────────┐
│  OntoCode (v0.4.0)                  │
│  VS Code extension + explorer UI    │
└─────────────────┬───────────────────┘
                  │ ontoindex-lsp (stdio)
┌─────────────────▼───────────────────┐
│  OntoIndex (v0.4.0)                 │
│  Rust index, catalog, query, CLI, LSP │
└─────────────────┬───────────────────┘
                  │ Oxigraph / Horned-OWL
┌─────────────────▼───────────────────┐
│  Your ontology repo                 │
│  .ttl .owl .rdf .jsonld …           │
└─────────────────────────────────────┘
```

OntoIndex is useful on its own today (CLI, CI, local analysis). The extension calls into the same engine via a language server rather than reimplementing ontology logic in TypeScript.

## What ships in v0.4.0

**VS Code (OntoCode):** Browse ontologies, entity inspector with **editing**, diagnostics, jump-to-source.

**Engine (OntoIndex):** Index, SQL/SPARQL query, validate, patch write-back for Turtle.

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl`) | Yes | `ontoindex patch` |
| Create / delete entities (`.ttl`) | Yes | `ontoindex patch` |
| Diagnostics / lint | Problems panel | `ontoindex validate` |
| SPARQL | — | `ontoindex sparql` |
| SQL-like queries | — | `ontoindex query` |

Earlier releases: see [CHANGELOG.md](CHANGELOG.md).

## Why OntoCode?

Protégé is strong for traditional ontology editing, but most engineering teams live in Git, pull requests, and VS Code. OntoCode targets that workflow:

| Shipped in v0.4 | Planned (v1.0 target) |
|-----------------|-------------------------|
| Browse ontologies in VS Code | Hybrid authoring: forms + Manchester editor |
| Entity inspector with **edit labels, parents, create/delete** | Complex class expressions in Manchester |
| Patch write-back for Turtle (`.ttl`) | Multi-format write-back |
| Horned-OWL catalog for Turtle axioms | Full OWL 2 DL axiom editing |
| Inline diagnostics (Problems panel + explorer) | Query workbench, reasoners + explanations |
| `ontoindex validate` and `ontoindex patch` for CI | OBO format + ROBOT interop |
| SQL-like and SPARQL queries via CLI | Semantic Git diff, LSP completion/rename |
| Local-first indexing | SHACL validation (rudof) |

Long-term goal: **Protégé-competitive OWL 2 DL + OBO maintenance in VS Code** — see [Protégé parity checklist](https://onto-code.readthedocs.io/en/latest/design/PROTEGE_PARITY/).

## Quick start

See the [getting started guide](https://onto-code.readthedocs.io/en/latest/getting-started/) or [First success in 10 minutes](https://onto-code.readthedocs.io/en/latest/guides/first-success/) for full paths (clone, `cargo install`, release binaries, VS Code).

```bash
# From a git clone
cargo build --release
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
```

```bash
# Installed CLI (use your ontology path, not ./fixtures)
cargo install ontoindex-cli
ontoindex query /path/to/ontologies "SELECT * FROM classes"
ontoindex validate /path/to/ontologies
```

## Coming in v0.5+

Future plans (not all implemented) — specs in [design docs](https://onto-code.readthedocs.io/en/latest/design/):

- SPARQL and SQL query panels in VS Code
- Manchester syntax editor
- Reasoner integration and graph visualization
- Semantic Git diff viewer

The extension is a thin TypeScript shell over **ontoindex-lsp** and the OntoIndex crates — not a second ontology stack.

## Roadmap

| Version | Deliverable |
|---------|-------------|
| v0.1 | OntoIndex: scanner, parser, catalog, CLI |
| v0.2 | VS Code extension, explorer, entity inspector, LSP |
| v0.3 | Ontology diagnostics (Problems panel, `validate`) |
| **v0.4.0** (current) | **Write-back** — Turtle patches, Horned-OWL catalog, editable inspector |
| v0.5 | Query workbench + Manchester MVP |
| v0.6 | Reasoning via [OntoLogos](https://github.com/eddiethedean/ontologos) 0.9.0 (EL, RL, inferred hierarchy) |
| v0.7–v0.7b | Graphs + OBO/ROBOT interop |
| v0.8–v0.9 | Full Manchester, refactoring, semantic diff; `ontologos-watch` hook |
| v1.0 | **Protégé-competitive OWL + OBO in VS Code** — DL via OntoLogos 1.0.0 ([parity checklist](https://onto-code.readthedocs.io/en/latest/design/PROTEGE_PARITY/)) |

See [roadmap](https://onto-code.readthedocs.io/en/latest/design/ROADMAP/), [product plan](https://onto-code.readthedocs.io/en/latest/design/PLAN/), and [Protégé parity checklist](https://onto-code.readthedocs.io/en/latest/design/PROTEGE_PARITY/) for the full product plan.

## Built on

OntoIndex delegates to mature Rust libraries — see [dependency matrix](https://onto-code.readthedocs.io/en/latest/design/DEPENDENCY_MATRIX/).

| Layer | Crates | Crate |
|-------|--------|-------|
| RDF / SPARQL | [Oxigraph](https://crates.io/crates/oxigraph) | `ontoindex-parser`, `ontoindex-query` |
| SQL queries | [sqlparser](https://crates.io/crates/sqlparser) | `ontoindex-query` |
| OWL axioms / write-back | [horned-owl](https://crates.io/crates/horned-owl), [horned-functional](https://crates.io/crates/horned-functional) | `ontoindex-owl` |
| Reasoning (planned) | [OntoLogos](https://github.com/eddiethedean/ontologos) | `ontoindex-reasoner` (planned) |
| OBO (planned) | [fastobo](https://crates.io/crates/fastobo) | planned v0.7b |
| LSP | [lsp-server](https://crates.io/crates/lsp-server), [lsp-types](https://crates.io/crates/lsp-types) | `ontoindex-lsp` |

Policy: [ADR-0016](https://onto-code.readthedocs.io/en/latest/design/adr/0016-dependency-first-implementation/). Third-party licenses (including LGPL for horned-owl): [LICENSES](https://onto-code.readthedocs.io/en/latest/design/LICENSES/).

## Repository layout

```text
crates/
├── ontoindex-core      # types, workspace scanner
├── ontoindex-parser    # RDF parsing and entity extraction
├── ontoindex-owl       # Horned-OWL facade, patch write-back (v0.4)
├── ontoindex-catalog   # index builder and semantic catalog
├── ontoindex-diagnostics # lint rules and diagnostic collection
├── ontoindex-query     # SQL-like and SPARQL engines
├── ontoindex-cli       # `ontoindex` binary
└── ontoindex-lsp       # language server for OntoCode
extension/              # VS Code extension (OntoCode Explorer)
fixtures/               # sample ontology for tests
scripts/                # extension packaging helpers
docs/                   # user guides (install, SQL, LSP API)
docs/design/            # product specs, ADRs, wireframes, backlog
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
| `diagnostics` | Lint and parse diagnostics |
| `properties` | Union of all property kinds |

Column schemas: [SQL reference](https://onto-code.readthedocs.io/en/latest/sql-reference/). SPARQL: [SPARQL reference](https://onto-code.readthedocs.io/en/latest/sparql-reference/). LSP methods: [LSP API](https://onto-code.readthedocs.io/en/latest/lsp-api/). Workspace limits: [workspace limits](https://onto-code.readthedocs.io/en/latest/workspace-limits/).

## API stability (pre-1.0)

Published `ontoindex-*` crates are at **0.4.x**. Library APIs, LSP wire JSON, and SQL virtual
table columns may change between minor releases until [v1.0 stable core](https://onto-code.readthedocs.io/en/latest/design/v1.0_BACKLOG/)
is complete. The CLI `validate` exit code (errors fail, warnings pass) is documented in
[workspace limits](https://onto-code.readthedocs.io/en/latest/workspace-limits/).

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) or the [contributing guide](https://onto-code.readthedocs.io/en/latest/contributing/). Quick checks:

```bash
cargo build -p ontoindex-lsp --bins
cargo test --workspace
cd extension && npm ci && npm test
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Update golden snapshots:

```bash
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes
```

## Installing from releases

Pre-built artifacts on [GitHub Releases](https://github.com/eddiethedean/ontocode/releases):

- `ontoindex` CLI (Linux x64)
- `ontoindex-lsp` per platform (Linux, macOS, Windows)
- `ontocode-*.vsix` (VS Code extension)

Verify downloads: [release integrity](https://onto-code.readthedocs.io/en/latest/release-integrity/). Maintainer release process: [releasing guide](https://onto-code.readthedocs.io/en/latest/releasing/).

Workspace crates **publish to [crates.io](https://crates.io/) on each `v0.4.x` release tag** (first publish in v0.4.0): `ontoindex-core`, `ontoindex-parser`, `ontoindex-owl`, `ontoindex-diagnostics`, `ontoindex-catalog`, `ontoindex-query`, `ontoindex-lsp`, `ontoindex-cli`.

See [CHANGELOG.md](CHANGELOG.md) for release notes. Security: [security policy](https://onto-code.readthedocs.io/en/latest/security/).

## License

MIT OR Apache-2.0. Third-party licenses: [LICENSES](https://onto-code.readthedocs.io/en/latest/design/LICENSES/).
