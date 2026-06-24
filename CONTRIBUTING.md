# Contributing to OntoCode / OntoIndex

Thank you for contributing. This repository contains:

- **OntoIndex** — Rust crates under `crates/` (`ontoindex` CLI, `ontoindex-lsp`)
- **OntoCode** — VS Code extension under `extension/`
- **Specs** — product and architecture docs under `docs/design/` ([DEPENDENCY_MATRIX.md](docs/design/DEPENDENCY_MATRIX.md) for external crates)
- **User guides** — install, SQL, and LSP API under `docs/`

The root Cargo package `ontocode` is unpublished and hosts workspace integration tests (`tests/`).

## Prerequisites

- Rust **1.88+** (see `rust-version` in `Cargo.toml`)
- Node.js **20** (extension CI)
- `npm` (extension build)

## Build and test

### Rust (full workspace)

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Build the language server binary:

```bash
cargo build -p ontoindex-lsp --bins
```

Run CLI smoke commands against fixtures:

```bash
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
```

`cargo run --` uses the workspace default member `ontoindex-cli` (binary name: `ontoindex`).

### Extension

```bash
cd extension
npm ci
npm run compile
npm test
```

Extension tests expect a built `ontoindex-lsp` binary. From the repo root:

```bash
cargo build -p ontoindex-lsp --bins
cd extension
export ONTOINDEX_LSP_BIN="$(pwd)/../target/debug/ontoindex-lsp"
npm test
```

**F5 / Run Extension:** Open the `extension/` folder in VS Code, build LSP (`cargo build -p ontoindex-lsp --bins`), optionally set `ontocode.lspPath` to your debug binary, then launch **Run Extension**.

**LSP integration smoke test** (workspace crate):

```bash
cargo test -p ontocode --test lsp_smoke
```

**VS Code E2E matrix** (separate workflow): see `.github/workflows/extension-vscode-e2e.yml`. Run locally with `@vscode/test-electron` after packaging the extension.

Full extension packaging (bundles LSP for current platform):

```bash
./scripts/package-extension.sh
```

### Golden snapshots

Some tests compare query output to committed snapshots. To update:

```bash
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes
```

Review the diff in `tests/golden/snapshots/` before committing.

### Examples

```bash
cargo run -p ontocode --example index_and_query
```

### Documentation site (MkDocs / Read the Docs)

```bash
pip install -r docs/requirements.txt   # Python 3.12 in CI
mkdocs serve
mkdocs build --strict   # CI uses this; must pass with no warnings
```

Open http://127.0.0.1:8000. Configuration: [`mkdocs.yml`](../mkdocs.yml), [`.readthedocs.yaml`](../.readthedocs.yaml).

## Pull requests

1. Fork and branch from `main`.
2. Keep changes focused; match existing style and naming.
3. Run Rust and extension tests locally (CI runs both).
4. Update docs when behavior or public API changes (`README.md`, `docs/`, `CHANGELOG.md` as appropriate). On release, follow the checklist in [docs/releasing.md](docs/releasing.md).
5. For spec changes, label whether they describe **implemented** vs **planned** behavior (see `docs/lsp-api.md` as a model).

## Documentation conventions

| Audience | Where to write |
|----------|----------------|
| New users (install, SQL, LSP) | `docs/` |
| Product vision, roadmap, ADRs | `docs/design/` |
| Architecture decisions | `docs/design/adr/` only (do not add `adrs/`) |
| Extension settings and commands | `extension/README.md` |

### Adding dependencies

Before adding a Rust crate dependency:

1. Check [DEPENDENCY_MATRIX.md](docs/design/DEPENDENCY_MATRIX.md) — prefer listed crates over new alternatives.
2. Follow [ADR-0016](docs/design/adr/0016-dependency-first-implementation.md) — `ontoindex-*` crates are facades, not reimplementations.
3. Update DEPENDENCY_MATRIX and [LICENSES.md](docs/design/LICENSES.md) if the crate is new or uses a non-MIT/Apache license.
4. Pin in workspace `[workspace.dependencies]` in root `Cargo.toml`.

## Code of conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](SECURITY.md) — please do not open public issues for security reports.
