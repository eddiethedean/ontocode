# OntoCode

**Edit OWL/RDF/OBO ontologies in VS Code — with a Rust engine for CI.**

Install the [VS Code extension](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), open a folder of `.ttl` / `.obo` files, and use the **OntoCode** activity bar to browse and edit. For CI gates, pin `cargo install ontocore-cli --locked --version 0.20.0` (first compile can take 15–30+ minutes) then `ontocore validate ./ontologies`.

**Editable today:** Turtle (`.ttl`) and OBO (`.obo`). Other formats index and query as read-only — see [Known limitations](https://ontocode-vs.readthedocs.io/en/latest/known-limitations/).
**Catalog SQL (subset):** not full SQL — prefer SPARQL for graph patterns.

**Current release: v0.20.0** · [10-minute tutorial](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) · [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [Changelog](CHANGELOG.md) · [Docs](https://ontocode-vs.readthedocs.io/en/latest/)

[![CI](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/ontocode/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](https://github.com/eddiethedean/ontocode/blob/main/LICENSE-MIT)
[![Docs](https://readthedocs.org/projects/ontocode-vs/badge/?version=latest)](https://ontocode-vs.readthedocs.io/en/latest/)
[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)
[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)
[![crates.io](https://img.shields.io/crates/v/ontocore?logo=rust)](https://crates.io/crates/ontocore)

![OntoCode product tour](docs/assets/screenshots/product-tour.gif)

## Start here

| I want to… | Start here |
|------------|------------|
| **Edit ontologies in VS Code** | **[First success (~10 min)](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/)** |
| Validate or query in CI | `cargo install ontocore-cli --locked --version 0.20.0` → [CI guide](https://ontocode-vs.readthedocs.io/en/latest/ci-integration/) (first compile: 15–30+ min) |
| Decide if it fits | [Known limitations](https://ontocode-vs.readthedocs.io/en/latest/known-limitations/) · [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [Versions & channels](https://ontocode-vs.readthedocs.io/en/latest/guides/versions-and-channels/) |
| Embed in Rust | [Rust library guide](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-library/) |
| Contribute | [CONTRIBUTING.md](CONTRIBUTING.md) |

Full documentation: **[Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/)**. You do not need to clone this repo to use the extension or installed CLI.

| Install | Command / link |
|---------|----------------|
| **VS Code extension** | [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor), or [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) VSIX |
| **CLI** | `cargo install ontocore-cli --locked` then `ontocore validate /path/to/ontologies` |
| **Crates** | [`ontocore`](https://crates.io/crates/ontocore) on [crates.io](https://crates.io/search?q=ontocore) |

Release CLI tarballs are **Linux x64 only**; macOS/Windows use `cargo install` (Rust 1.88+) or the language server bundled in the VSIX.

> **Names:** **OntoCode** = VS Code extension. **OntoCore** = Rust engine (`ontocore` CLI + `ontocore-lsp`). **Ontologos** = external reasoner. This GitHub repo is `ontocode` — install the CLI with **`cargo install ontocore-cli`**, not `ontocode`.

## See it in action

[Feature tour](https://ontocode-vs.readthedocs.io/en/latest/ontocode/feature-tour/) · [First success](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) (~10 min, no clone)

<p>
<img src="docs/assets/screenshots/explorer-inspector.png" alt="Explorer and Entity Inspector" width="48%" />
<img src="docs/assets/screenshots/query-workbench.png" alt="Query Workbench" width="48%" />
</p>

## Quick start

**VS Code:** Install [OntoCode](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) → open a folder with ontology files → click the **OntoCode** activity bar. Edit **Turtle (`.ttl`)** and **OBO (`.obo`)** in the Entity Inspector. RDF/XML and OWL/XML are indexed for browse/query only — [Supported formats](https://ontocode-vs.readthedocs.io/en/latest/supported-formats/). OntoCode’s **bundled** language server runs in trusted and Restricted Mode; **Trust** only if you set custom `ontocode.lspPath` or `ontocode.robotPath`.

**CLI (install):** First `cargo install` compiles dependencies — expect **15–30+ minutes** on a cold machine (Rust 1.88+). Pin releases in CI.

```bash
cargo install ontocore-cli --locked --version 0.20.0
# Catalog SQL (subset) — not full SQL; see docs
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
│  OntoCode (VS Code) ──ontocore-lsp──► OntoCore (Rust engine) │
│  index · query · diagnostics · refactor · diff · CLI · LSP   │
└──────────────┬─────────────────────────────┬───────────────────┘
               ▼                             ▼
        ┌─────────────┐              ┌──────────────────┐
        │  Ontologos  │              │  Oxigraph /      │
        │  reasoning  │              │  Horned-OWL      │
        └─────────────┘              └──────────────────┘
```

Platform docs: [Vision](https://ontocode-vs.readthedocs.io/en/latest/vision/) · [Architecture](ARCHITECTURE.md) · [Roadmap hub](https://ontocode-vs.readthedocs.io/en/latest/roadmap-hub/) · [Protégé vs OntoCode](https://ontocode-vs.readthedocs.io/en/latest/guides/protege-decision/)

**v0.20.0** adds workspace runtime and Turtle patch matching hardening for Protégé/ROBOT-style files. See [SHIPPED](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/), [v0.20 migration](docs/migration/v0.20.md), and [What's new in v0.19](docs/migration/v0.19.md).

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md). Quick checks:

```bash
cargo test --workspace
cargo build -p ontocore-lsp --bins
cd extension && npm ci && ONTOCORE_LSP_BIN=../target/debug/ontocore-lsp npm test
cd extension/webview-ui && npm ci && npm test
cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings
```

**Full CI parity:** `./scripts/run-ci-local.sh`

## License

MIT OR Apache-2.0. Third-party licenses: [LICENSES](https://ontocode-vs.readthedocs.io/en/latest/design/LICENSES/). Security: [security policy](https://ontocode-vs.readthedocs.io/en/latest/security/).
