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
[![reasoner](https://img.shields.io/crates/v/ontoindex-reasoner?label=reasoner)](https://crates.io/crates/ontoindex-reasoner)
[![downloads](https://img.shields.io/crates/d/ontoindex-cli?label=downloads)](https://crates.io/crates/ontoindex-cli)
[![Docs](https://readthedocs.org/projects/ontocode-vs/badge/?version=latest)](https://ontocode-vs.readthedocs.io/en/latest/)

**OntoCode** brings OWL/RDF ontology editing into VS Code and Git — browse classes, edit Turtle, run EL reasoning, and validate in CI without Protégé.

**OntoIndex** is the Rust engine behind it: a local CLI and language server that indexes your workspace and exposes SQL, SPARQL, and patch APIs.

**Documentation:** [Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/) — [VS Code extension](https://ontocode-vs.readthedocs.io/en/latest/guides/vscode-extension/) · [Rust & CLI](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/) · [First success tutorial](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) · [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [`cargo install ontoindex-cli --locked`](https://crates.io/crates/ontoindex-cli). You do not need to clone this repo.

> **Naming:** **OntoCode** is the VS Code extension (product UI). **OntoIndex** is the Rust engine (`ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp`). This repo contains both.

## Choose your path

### Use the CLI (OntoIndex)

Docs: [Rust & CLI guide](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/).

```bash
cargo install ontoindex-cli --locked
ontoindex query /path/to/your/ontologies "SELECT * FROM classes"
ontoindex validate .
```

> **Note:** `./fixtures` exists only in a git clone. After `cargo install`, point at your own ontology folder (see [getting started guide](https://ontocode-vs.readthedocs.io/en/latest/getting-started/)).

From a clone, `cargo run --` runs the `ontoindex` binary (workspace default-member is `ontoindex-cli`):

```bash
cargo run -- query fixtures "SELECT * FROM classes"
```

### Use VS Code (OntoCode Explorer)

Docs: [VS Code extension guide](https://ontocode-vs.readthedocs.io/en/latest/guides/vscode-extension/).

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), or download a VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases).
2. Open a folder with `.ttl`, `.owl`, `.obo`, `.rdf`, or other supported ontology files.
3. Use the **OntoCode** activity bar to browse entities, edit in the inspector, and open diagnostics.

Full install and troubleshooting: [install guide](https://ontocode-vs.readthedocs.io/en/latest/vscode-install/). Editing guide: [authoring guide](https://ontocode-vs.readthedocs.io/en/latest/authoring/).

![OntoCode Explorer preview](docs/media/explorer-preview.png)

## Two-layer architecture

OntoCode is designed as two products that ship together:

| Layer | What it is | Status in v0.8.0 |
|-------|------------|-------------------|
| **OntoCode** | VS Code extension (explorer, entity inspector, diagnostics, **authoring**, **query workbench**, **Manchester editor**, **reasoner**) | **Shipping** |
| **OntoIndex** | Rust library + CLI + LSP (scan, parse, catalog, query, validate, diagnostics, **write-back**, **classify**) | **Shipping** |

```text
┌─────────────────────────────────────┐
│  OntoCode (v0.8.0)                  │
│  VS Code extension + explorer UI    │
└─────────────────┬───────────────────┘
                  │ ontoindex-lsp (stdio)
┌─────────────────▼───────────────────┐
│  OntoIndex (v0.8.0)                 │
│  Rust index, catalog, query, CLI, LSP │
└─────────────────┬───────────────────┘
                  │ Oxigraph / Horned-OWL / OntoLogos
┌─────────────────▼───────────────────┐
│  Your ontology repo                 │
│  .ttl .owl .rdf .jsonld …           │
└─────────────────────────────────────┘
```

OntoIndex is useful on its own today (CLI, CI, local analysis). The extension calls into the same engine via a language server rather than reimplementing ontology logic in TypeScript.

## What ships in v0.8.0

See the full capability matrix: **[What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/)** (updated each release).

Earlier releases: [Changelog](https://ontocode-vs.readthedocs.io/en/latest/changelog/) · [CHANGELOG.md](CHANGELOG.md) on GitHub.

## Why OntoCode?

Protégé is strong for traditional ontology editing, but most engineering teams live in Git, pull requests, and VS Code. OntoCode targets that workflow:

| Shipped in v0.8.0 | Planned (v1.0 target) |
|-------------------|------------------------|
| Browse and edit Turtle in VS Code (React inspector) | Full OWL 2 DL axiom catalog |
| Query Workbench (SQL + SPARQL, React) and graph visualization | Inline SQL/SPARQL autocomplete |
| Manchester editor (subclass, equivalent, disjoint) | Property chain editing |
| Safe IRI rename, find usages, namespace migration, move/extract module | Semantic Git diff |
| EL/RL/RDFS reasoning + inferred hierarchy | OWL 2 DL reasoning (`dl` / `auto`) |
| OBO index + `obo_id` in explorer; ROBOT CLI wrappers | Full OBO write-back in VS Code |
| Patch write-back for Turtle; `validate` / `classify` for CI | SHACL validation |

Long-term goal: **Protégé-competitive OWL 2 DL + OBO maintenance in VS Code** — see [Protégé parity checklist](https://ontocode-vs.readthedocs.io/en/latest/design/PROTEGE_PARITY/).

## Quick start

See the **[VS Code extension docs](https://ontocode-vs.readthedocs.io/en/latest/guides/vscode-extension/)** or **[Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)** — or [First success in 10 minutes](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) for a guided walkthrough.

```bash
# From a git clone
cargo build --release
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
```

```bash
# Installed CLI (use your ontology path, not ./fixtures)
cargo install ontoindex-cli --locked
ontoindex query /path/to/ontologies "SELECT * FROM classes"
ontoindex validate /path/to/ontologies
```

## Reasoning

- EL / RL / RDFS classification via [OntoLogos](https://github.com/eddiethedean/ontologos) 0.9.0
- CLI: `ontoindex classify`, `ontoindex explain`
- LSP: `ontoindex/runReasoner`, `ontoindex/getExplanation`
- Explorer hierarchy mode: asserted / inferred / combined

See [reasoner guide](https://ontocode-vs.readthedocs.io/en/latest/guides/reasoner/). DL and `auto` profiles require OntoLogos 1.0.

## UI architecture (v0.8.0)

The VS Code extension is a thin TypeScript shell over **ontoindex-lsp**. Inspector, graph, Query Workbench, Manchester editor, and refactor preview panels use **React + Vite** webviews with a typed message protocol ([webview protocol](https://ontocode-vs.readthedocs.io/en/latest/webview-protocol/)). Reasoner and explanation panels still use legacy HTML webviews until v0.9.

## Roadmap

| Version | Deliverable |
|---------|-------------|
| v0.1–v0.4 | OntoIndex core, VS Code extension, diagnostics, Turtle write-back |
| v0.6.0 | Reasoning — OntoLogos EL/RL/RDFS, inferred hierarchy, explanations |
| **v0.7.0** | React inspector + graphs, OBO index, ROBOT CLI wrappers |
| **v0.8.0** (current) | Refactoring engine, full Manchester catalog (disjoint + chains view), React Query Workbench + Manchester editor |
| v0.9 | Reasoner/explanation React migration; semantic diff; `ontologos-watch` hook |
| v1.0 | **Protégé-competitive OWL + OBO in VS Code** — DL via OntoLogos 1.0.0 ([parity checklist](https://ontocode-vs.readthedocs.io/en/latest/design/PROTEGE_PARITY/)) |

See [roadmap](https://ontocode-vs.readthedocs.io/en/latest/design/ROADMAP/), [product plan](https://ontocode-vs.readthedocs.io/en/latest/design/PLAN/), and [Protégé parity checklist](https://ontocode-vs.readthedocs.io/en/latest/design/PROTEGE_PARITY/) for the full product plan.

## Built on

OntoIndex delegates to mature Rust libraries — see [dependency matrix](https://ontocode-vs.readthedocs.io/en/latest/design/DEPENDENCY_MATRIX/).

| Layer | Crates | Crate |
|-------|--------|-------|
| RDF / SPARQL | [Oxigraph](https://crates.io/crates/oxigraph) | `ontoindex-parser`, `ontoindex-query` |
| SQL queries | [sqlparser](https://crates.io/crates/sqlparser) | `ontoindex-query` |
| OWL axioms / write-back | [horned-owl](https://crates.io/crates/horned-owl), [horned-functional](https://crates.io/crates/horned-functional) | `ontoindex-owl` |
| Reasoning | [OntoLogos](https://github.com/eddiethedean/ontologos) | `ontoindex-reasoner` |
| OBO index | line-based parser in `ontoindex-parser` | `ontoindex-parser`, `ontoindex-catalog` |
| ROBOT interop | external `robot` CLI (Java) | `ontoindex-robot`, `ontoindex-cli` |
| LSP | [lsp-server](https://crates.io/crates/lsp-server), [lsp-types](https://crates.io/crates/lsp-types) | `ontoindex-lsp` |

Policy: [ADR-0016](https://ontocode-vs.readthedocs.io/en/latest/design/adr/0016-dependency-first-implementation/). Third-party licenses (including LGPL for horned-owl): [LICENSES](https://ontocode-vs.readthedocs.io/en/latest/design/LICENSES/).

## Repository layout

```text
crates/
├── ontoindex-core      # types, workspace scanner
├── ontoindex-parser    # RDF parsing and entity extraction
├── ontoindex-owl       # Horned-OWL facade, patch write-back, Manchester
├── ontoindex-diagnostics # lint rules and diagnostic collection
├── ontoindex-catalog   # index builder and semantic catalog
├── ontoindex-query     # SQL-like and SPARQL engines
├── ontoindex-reasoner  # OntoLogos EL/RL/RDFS classification
├── ontoindex-refactor  # workspace refactoring (rename, migrate, move, extract)
├── ontoindex-robot     # ROBOT CLI wrappers
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

Column schemas: [SQL reference](https://ontocode-vs.readthedocs.io/en/latest/sql-reference/). SPARQL: [SPARQL reference](https://ontocode-vs.readthedocs.io/en/latest/sparql-reference/). LSP methods: [LSP API](https://ontocode-vs.readthedocs.io/en/latest/lsp-api/). Workspace limits: [workspace limits](https://ontocode-vs.readthedocs.io/en/latest/workspace-limits/).

## API stability (pre-1.0)

Published `ontoindex-*` crates are at **0.8.x**. Library APIs, LSP wire JSON, and SQL virtual
table columns may change between minor releases until [v1.0 stable core](https://ontocode-vs.readthedocs.io/en/latest/design/v1.0_BACKLOG/)
is complete. The CLI `validate` and `classify` exit codes are documented in
[workspace limits](https://ontocode-vs.readthedocs.io/en/latest/workspace-limits/).

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) or the [contributing guide](https://ontocode-vs.readthedocs.io/en/latest/contributing/). Quick checks:

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

Verify downloads: [release integrity](https://ontocode-vs.readthedocs.io/en/latest/release-integrity/). Maintainer release process: [releasing guide](https://ontocode-vs.readthedocs.io/en/latest/releasing/).

Workspace crates **publish to [crates.io](https://crates.io/) on each `v0.8.x` release tag**: `ontoindex-core`, `ontoindex-parser`, `ontoindex-owl`, `ontoindex-diagnostics`, `ontoindex-catalog`, `ontoindex-query`, `ontoindex-reasoner`, `ontoindex-robot`, `ontoindex-refactor`, `ontoindex-lsp`, `ontoindex-cli`.

See [CHANGELOG.md](CHANGELOG.md) for release notes. Security: [security policy](https://ontocode-vs.readthedocs.io/en/latest/security/).

## License

MIT OR Apache-2.0. Third-party licenses: [LICENSES](https://ontocode-vs.readthedocs.io/en/latest/design/LICENSES/).
