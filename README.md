# OntoCode

**Ontology editing in VS Code, powered by a Rust engine.**

**OntoCode** is a VS Code extension for browsing and editing ontologies in your workspace. **OntoCore** is the Rust semantic workspace engine behind it (CLI + language server). Browse OWL/RDF/OBO in VS Code, edit Turtle and OBO in the Entity Inspector, run EL–DL reasoning, query or validate in CI — without Protégé.

**Current release: v0.16.0** · [Changelog](CHANGELOG.md) · [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [What's new in v0.16](docs/migration/v0.16.md)

[![CI](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](https://github.com/eddiethedean/ontocode/blob/main/LICENSE-MIT)
[![Docs](https://readthedocs.org/projects/ontocode-vs/badge/?version=latest)](https://ontocode-vs.readthedocs.io/en/latest/)
[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)
[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)
[![crates.io](https://img.shields.io/crates/v/ontocore?logo=rust)](https://crates.io/crates/ontocore)

## Start here

| I want to… | Start here |
|------------|------------|
| Edit ontologies in VS Code | [Install extension](https://ontocode-vs.readthedocs.io/en/latest/vscode-install/) → [10‑min tutorial](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) |
| Validate or query in CI | `cargo install ontocore-cli --locked` → [CI guide](https://ontocode-vs.readthedocs.io/en/latest/ci-integration/) |
| Embed in Rust | [Rust library guide](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-library/) |
| Compare ontology versions | [Semantic diff](https://ontocode-vs.readthedocs.io/en/latest/ontocode/semantic-diff/) |
| Navigate all documentation | [Documentation map](https://ontocode-vs.readthedocs.io/en/latest/documentation-index/) · [GitHub docs entrypoint](docs/README.md) · [Glossary](https://ontocode-vs.readthedocs.io/en/latest/glossary/) |

Full documentation: **[Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/)**. You do not need to clone this repo to use the extension or installed CLI.

| Install | Command / link |
|---------|----------------|
| **VS Code extension** | [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor), or [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) VSIX |
| **CLI** | `cargo install ontocore-cli --locked` then `ontocore validate /path/to/ontologies` |
| **Crates** | [`ontocore`](https://crates.io/crates/ontocore) and `ontocore-*` on [crates.io](https://crates.io/search?q=ontocore) — see [OntoCore overview](https://ontocode-vs.readthedocs.io/en/latest/ontocore/) |

Release CLI tarballs are **Linux x64 only**; macOS/Windows use `cargo install` or the language server bundled in the VSIX.

| Product | Role |
|---------|------|
| **OntoCode** | VS Code IDE — explorer, inspector, Query Workbench, Manchester editor, Manage Imports, semantic diff, reasoner |
| **OntoCore** | Rust engine — index, query, diagnostics, refactoring, diff, docs export, CLI, LSP |
| **OntoUI** | Shared React UI in `extension/webview-ui/` — design tokens, WorkspaceStore, focus relay (powers OntoCode webviews) |
| **Ontologos** | External reasoner — classification, consistency, explanations |

> **Naming:** **OntoCode** = VS Code extension. **OntoCore** = `ontocore` crate, `ontocore-*` crates, `ontocore` CLI, `ontocore-lsp`. This repo is named `ontocode` on GitHub — install the CLI with **`cargo install ontocore-cli`**, not `ontocode`.

## See it in action

[Feature tour](https://ontocode-vs.readthedocs.io/en/latest/ontocode/feature-tour/) (panel walkthrough) · [First success tutorial](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) (~10 min, no clone required)

## Quick start

**VS Code:** Install [OntoCode](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) → open a folder with ontology files → **Trust** workspace → **OntoCode** activity bar.

**CLI (install):**

```bash
cargo install ontocore-cli --locked
ontocore query /path/to/ontologies "SELECT * FROM classes"
ontocore validate /path/to/ontologies
# Requires a git repository (run from your ontology repo root):
ontocore diff HEAD..WORKTREE
ontocore docs /path/to/ontologies --format markdown --output ./docs-out
```

**CLI (clone):**

```bash
git clone https://github.com/eddiethedean/ontocode.git && cd ontocode
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
```

## Architecture

```text
┌──────────────────────────────────────────────────────────────┐
│  Shipped v0.16: plugin preferences/actions, imports/layout,   │
│  graph modes · Planned v1.0+: stable plugin API + Protégé exit │
└────────────────────────────┬─────────────────────────────────┘
                             │ ontocore-lsp (stdio)
┌────────────────────────────▼─────────────────────────────────┐
│  OntoCode — VS Code extension (React webviews)               │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│  OntoCore — semantic workspace engine (ships today)          │
│  index · query · diagnostics · refactor · diff · CLI · LSP   │
└──────────────┬─────────────────────────────┬───────────────────┘
               ▼                             ▼
        ┌─────────────┐              ┌──────────────────┐
        │  Ontologos  │              │  Oxigraph /      │
        │  reasoning  │              │  Horned-OWL      │
        └─────────────┘              └──────────────────┘
               ▼
        Your ontology repo (.ttl .owl .obo .rdf …)
```

Platform docs: [Vision](https://ontocode-vs.readthedocs.io/en/latest/vision/) · [Architecture](ARCHITECTURE.md) · [Roadmap](ROADMAP.md) · [Protégé parity](https://ontocode-vs.readthedocs.io/en/latest/design/PROTEGE_PARITY/)

**OntoCode 1.0** targets a Protégé-competitive OWL + OBO IDE in VS Code, with CLI gates for CI. **v0.15** extends the v0.14 plugin host with permissions, UI views, explanation alternatives, and graph asserted/inferred modes. See [SHIPPED matrix](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) and [What's new in v0.15](docs/migration/v0.15.md).

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md). Quick checks:

```bash
cargo test --workspace
cargo build -p ontocore-lsp --bins
cd extension && npm ci && ONTOCORE_LSP_BIN=../target/debug/ontocore-lsp npm test
cd extension/webview-ui && npm ci && npm test
cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings
```

**Full CI parity** (rustfmt, doc versions, MSRV, mkdocs strict, cargo audit, extension e2e):

```bash
./scripts/run-ci-local.sh
```

## License

MIT OR Apache-2.0. Third-party licenses: [LICENSES](https://ontocode-vs.readthedocs.io/en/latest/design/LICENSES/). Security: [security policy](https://ontocode-vs.readthedocs.io/en/latest/security/).
