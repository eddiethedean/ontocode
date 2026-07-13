# Contributing to OntoCode / OntoCore

Thank you for contributing. This repository contains:

- **OntoCore** — Rust semantic workspace engine under `crates/` (`ontocore` façade, `ontocore-*` implementation, `ontocore` CLI, `ontocore-lsp`)
- **OntoCode** — VS Code extension under `extension/`
- **User guides** — install, SQL, and LSP API under `docs/`

### Docs-only contributors (~15 minutes)

**No Rust or Node required** for documentation-only PRs. Expect **under one hour** end-to-end including review prep:

1. Edit pages under [`docs/`](docs/) (Read the Docs source). Root `README.md`, `CONTRIBUTING.md`, and `extension/README.md` when user-facing install text changes.
2. Run `./scripts/check-doc-versions.sh` (version pins and link hygiene).
3. Optional preview: `pip install -r docs/requirements.txt && ./scripts/serve-docs.sh`
4. Open a focused PR describing what confused adopters you fixed.

**Public install pins** must match the latest tagged release in [`docs/TAGGED_RELEASE`](docs/TAGGED_RELEASE) — not the unreleased workspace version in `Cargo.toml`.

Platform vision/roadmap/architecture: prefer editing [`docs/vision.md`](docs/vision.md), [`docs/roadmap.md`](docs/roadmap.md), and [`docs/architecture.md`](docs/architecture.md); keep root mirrors in sync — see [Canonical documentation paths](#canonical-documentation-paths) below.

### Canonical documentation paths

| Topic | GitHub (root) | Read the Docs (`docs/`) |
|-------|---------------|-------------------------|
| Vision | [VISION.md](VISION.md) | [docs/vision.md](docs/vision.md) |
| Architecture | [ARCHITECTURE.md](ARCHITECTURE.md) | [docs/architecture.md](docs/architecture.md) |
| Platform roadmap | [ROADMAP.md](ROADMAP.md) | [docs/roadmap.md](docs/roadmap.md) |
| Engineering specs | — | [docs/design/](docs/design/README.md) |
| Platform planning (v0.14+) | — | [docs/platform/](docs/platform/OVERVIEW.md) |
| Product ADRs | — | [docs/adr/](docs/adr/README.md) |
| Engineering ADRs | — | [docs/design/adr/](docs/design/adr/README.md) |
| Documentation map | — | [docs/documentation-index.md](docs/documentation-index.md) |

Root `VISION.md`, `ARCHITECTURE.md`, and `ROADMAP.md` are mirrored under `docs/` for Read the Docs. **Prefer editing `docs/` copies first**, then update root mirrors when platform-facing content changes, or run `./scripts/check-doc-versions.sh` to catch drift.

- **Specs** — product and architecture docs under `docs/design/` ([DEPENDENCY_MATRIX.md](docs/design/DEPENDENCY_MATRIX.md) for external crates)

The root Cargo package `ontocode` is unpublished and hosts workspace integration tests (`tests/`).

**New contributors:** start with [Architecture tour](docs/guides/architecture-tour.md) (~15 min) and [internals.md](docs/internals.md) for role-based paths (Rust, extension, docs, LSP).

### First PR paths

**Smoke PR (~15 minutes)** — docs-only or small Rust fix:

1. Install **Rust 1.88+** (Node 20 only if you touch `extension/`).
2. `git clone https://github.com/eddiethedean/ontocode.git && cd ontocode`
3. `cargo test -p ontocore-core --lib` (or your touched crate).
4. `cargo fmt --all --check` and `./scripts/check-doc-versions.sh` for docs changes.
5. Open a focused PR with a short description.

**Full contributor setup (~60+ minutes warm cache; 2–4+ hours cold)** — extension, LSP, or release-impacting changes:

1. Complete smoke steps above.
2. `cargo test --workspace` and `cargo build -p ontocore-lsp --bins`.
3. Extension: `cd extension && npm ci && ONTOCORE_LSP_BIN=../target/debug/ontocore-lsp npm test`.
4. Webview: `cd extension/webview-ui && npm ci && npm test`.
5. Before opening a PR: `./scripts/run-ci-local.sh` (30–60+ minutes; matches GitHub Actions).

See [Testing matrix](docs/guides/testing-matrix.md) for which commands to run by change type.

### Plugin contributors (v0.14+)

| Task | Start here |
|------|------------|
| Author a workspace plugin (manifest, validator, exporter) | **[docs/guides/plugins.md](docs/guides/plugins.md)** — canonical author guide |
| Reference implementation | `crates/ontocore-plugin-naming/`, `examples/plugin-workspace/` |
| Historical trait-based design (do not implement from) | [docs/design/PLUGIN_SPEC.md](docs/design/PLUGIN_SPEC.md) |
| Wire LSP / CLI integration | [docs/lsp-api.md](docs/lsp-api.md), `crates/ontocore-lsp/` |

Run `./scripts/run-ci-local.sh` before PRs that touch plugin host, reference plugins, or extension capability registry.

## Before opening a PR

**Minimum checks (most changes):**

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build -p ontocore-lsp --bins
cd extension/webview-ui && npm ci && npm test
cd extension && ONTOCORE_LSP_BIN=../target/debug/ontocore-lsp npm ci && npm run compile && npm test
./scripts/check-doc-versions.sh
```

**Full CI parity (release branches or broad changes):** `./scripts/run-ci-local.sh` (~30+ minutes). This matches GitHub Actions (rustfmt, clippy, tests, extension e2e, mkdocs, packaging).

Open an issue or discussion before large features. Follow existing commit message style in `git log`.

## Prerequisites

- Rust **1.88+** (see `rust-version` in `Cargo.toml`)
- Node.js **20** (extension CI)
- `npm` (extension build)
- **`cargo-audit`** — `cargo install cargo-audit` (required by CI; run `cargo audit` before PRs)

## Optional dependencies

- **Java 11+** and **[ROBOT](http://robot.obolibrary.org/)** on `PATH` — optional; needed only for manual `ontocore robot` / ROBOT interop development (not required for `cargo test --workspace`)
- **Python 3.12** — for MkDocs doc site (`pip install -r docs/requirements.txt`)

> **Canonical copy:** Root [`CONTRIBUTING.md`](CONTRIBUTING.md) is the source of truth for GitHub. Mirror changes to [`docs/contributing.md`](docs/contributing.md) for Read the Docs.

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

### Webview UI (React panels)

```bash
cd extension/webview-ui
npm ci
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

**VS Code E2E matrix** (separate workflow): see `.github/workflows/extension-vscode-e2e.yml`. Run locally:

```bash
cargo build -p ontocore-lsp --bins
cd extension && npm ci && npm run compile && npm run test:vscode
```

See [Debugging guide](docs/debugging.md) for LSP, extension host, and webview workflows.

Full extension packaging (bundles LSP for current platform):

```bash
./scripts/package-extension.sh
```

### Golden and fixture snapshots

Some tests compare output to committed snapshots. To update (env var names retain the legacy `ONTOINDEX_*` prefix):

```bash
# SQL/query golden snapshots (tests/golden/snapshots/)
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes

# Extension catalog fixture snapshot
ONTOINDEX_UPDATE_FIXTURE_SNAPSHOT=1 cargo test -p ontocode --test fixture_snapshot
```

Review the diffs before committing.

### Examples

```bash
cargo run -p ontocode --example index_and_query
cargo run -p ontocode --example ontocore_workspace
cargo run -p ontocode --example workspace_operations
cargo run -p ontocode --example semantic_diff
```

See [Examples index](docs/examples/index.md) for all runnable assets.

### Documentation site (MkDocs / Read the Docs)

```bash
pip install -r docs/requirements.txt   # Python 3.12 in CI
./scripts/serve-docs.sh
./scripts/build-docs.sh   # CI-equivalent; must pass with no warnings
./scripts/check-doc-versions.sh   # README / RTD / extension version sync
```

Open http://127.0.0.1:8000. Configuration: [`mkdocs.yml`](mkdocs.yml), [`.readthedocs.yaml`](.readthedocs.yaml).

### Full local CI (recommended before PR)

Mirrors [`.github/workflows/ci.yml`](.github/workflows/ci.yml) plus VS Code e2e for your platform:

```bash
./scripts/run-ci-local.sh
```

This runs rustfmt, `./scripts/check-doc-versions.sh`, clippy, workspace tests, MSRV (1.88), CLI smoke, release build, LSP smoke tests, webview-ui tests, extension build/tests, `cargo audit`, crate packaging dry-run, mkdocs strict build, and VS Code extension e2e. Expect 30+ minutes on a cold build.

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
| Product vision / platform roadmap | Root `VISION.md` / `ROADMAP.md` **and** mirrors under `docs/` (edit both) |
| Product & platform ADRs | `docs/adr/` |
| Engineering ADRs (crate/design decisions) | `docs/design/adr/` (do not add a top-level `adrs/` folder) |
| Engineering specs / dependency matrix | `docs/design/` |
| Extension settings and commands | `extension/README.md` |

**Mirror policy:** Root `VISION.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `SECURITY.md`, `CONTRIBUTING.md`, and `CHANGELOG.md` are mirrored under `docs/` for Read the Docs. When you change platform-facing content, update **both** copies (or expect `./scripts/check-doc-versions.sh` to fail).

### Adding dependencies

Before adding a Rust crate dependency:

1. Check [DEPENDENCY_MATRIX.md](docs/design/DEPENDENCY_MATRIX.md) — prefer listed crates over new alternatives.
2. Follow [ADR-0016](docs/design/adr/0016-dependency-first-implementation.md) — `ontocore-*` crates are facades, not reimplementations.
3. Update DEPENDENCY_MATRIX and [LICENSES.md](docs/design/LICENSES.md) if the crate is new or uses a non-MIT/Apache license.
4. Pin in workspace `[workspace.dependencies]` in root `Cargo.toml`.

## Code of conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](SECURITY.md) — please do not open public issues for security reports.
