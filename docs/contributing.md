# Contributing to OntoCode / OntoCore

Thank you for contributing. This repository contains:

- **OntoCore** — Rust semantic workspace engine under `crates/` (`ontocore` façade, `ontocore-*` implementation, `ontocore` CLI, `ontocore-lsp`)
- **OntoCode** — VS Code extension under `extension/`
- **Platform docs** — [VISION.md](https://github.com/eddiethedean/ontocode/blob/main/VISION.md), [ARCHITECTURE.md](https://github.com/eddiethedean/ontocode/blob/main/ARCHITECTURE.md), [ROADMAP.md](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) (mirrored as [vision.md](vision.md), [architecture.md](architecture.md), [roadmap.md](roadmap.md))
- **Specs** — product and architecture docs under `docs/design/` ([DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md) for external crates)
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
cargo build -p ontocore-lsp --bins
```

Run CLI smoke commands against fixtures:

```bash
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
```

`cargo run --` uses the workspace default member `ontocore-cli` (binary name: `ontocore`).

### Extension

```bash
cd extension
npm ci
npm run compile
npm test
```

Extension tests expect a built `ontocore-lsp` binary. From the repo root:

```bash
cargo build -p ontocore-lsp --bins
cd extension
export ONTOCORE_LSP_BIN="$(pwd)/../target/debug/ontocore-lsp"
npm test
```

**F5 / Run Extension:** Open the `extension/` folder in VS Code, build LSP (`cargo build -p ontocore-lsp --bins`), optionally set `ontocode.lspPath` to your debug binary, then launch **Run Extension**.

**LSP integration smoke test** (workspace crate):

```bash
cargo test -p ontocode --test lsp_smoke
cargo test -p ontocode --test lsp_reasoner
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
cargo run -p ontocode --example ontocore_workspace
```

### Documentation site (MkDocs / Read the Docs)

```bash
pip install -r docs/requirements.txt   # Python 3.12 in CI
mkdocs serve
mkdocs build --strict   # CI uses this; must pass with no warnings
./scripts/check-doc-versions.sh   # README / RTD / extension version sync
```

Open http://127.0.0.1:8000. Configuration: [`mkdocs.yml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/mkdocs.yml), [`.readthedocs.yaml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.readthedocs.yaml).

## Pull requests

1. Fork and branch from `main`.
2. Keep changes focused; match existing style and naming.
3. Run Rust and extension tests locally (CI runs both).
4. Update docs when behavior or public API changes (`README.md`, `docs/`, `CHANGELOG.md` as appropriate). On release, follow the checklist in [releasing.md](releasing.md).
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

1. Check [DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md) — prefer listed crates over new alternatives.
2. Follow [ADR-0016](design/adr/0016-dependency-first-implementation.md) — `ontocore-*` crates are facades, not reimplementations.
3. Update DEPENDENCY_MATRIX and [LICENSES.md](design/LICENSES.md) if the crate is new or uses a non-MIT/Apache license.
4. Pin in workspace `[workspace.dependencies]` in root `Cargo.toml`.

## Code of conduct

See [CODE_OF_CONDUCT.md](https://github.com/eddiethedean/ontocode/blob/main/CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — please do not open public issues for security reports.
