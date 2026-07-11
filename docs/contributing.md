# Contributing to OntoCode / OntoCore

Thank you for contributing. This repository contains:

- **OntoCore** — Rust semantic workspace engine under `crates/` (`ontocore` façade, `ontocore-*` implementation, `ontocore` CLI, `ontocore-lsp`)
- **OntoCode** — VS Code extension under `extension/`
- **User guides** — install, SQL, and LSP API under `docs/`

### Canonical documentation paths

| Topic | GitHub (root) | Read the Docs (`docs/`) |
|-------|---------------|-------------------------|
| Vision | [VISION.md](https://github.com/eddiethedean/ontocode/blob/main/VISION.md) | [vision.md](vision.md) |
| Architecture | [ARCHITECTURE.md](https://github.com/eddiethedean/ontocode/blob/main/ARCHITECTURE.md) | [architecture.md](architecture.md) |
| Platform roadmap | [ROADMAP.md](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) | [roadmap.md](roadmap.md) |
| Engineering specs | — | [design/README.md](design/README.md) |
| Platform planning (v0.14+) | — | [platform/OVERVIEW.md](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/OVERVIEW.md) |
| Product ADRs | — | [adr/README.md](adr/README.md) |
| Engineering ADRs | — | [design/adr/README.md](design/adr/README.md) |
| Documentation map | — | [documentation-index.md](documentation-index.md) |

Root `VISION.md`, `ARCHITECTURE.md`, and `ROADMAP.md` are mirrored under `docs/` for Read the Docs. **Edit both** when changing platform-facing content, or run `./scripts/check-doc-versions.sh` to catch drift.

- **Specs** — product and architecture docs under `docs/design/` ([DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md) for external crates)

The root Cargo package `ontocode` is unpublished and hosts workspace integration tests (`tests/`).

**New contributors:** start with [internals.md](internals.md) for role-based paths (Rust, extension, docs, LSP).

### First PR in ~60 minutes

1. Install **Rust 1.88+** and **Node 20+**.
2. `git clone https://github.com/eddiethedean/ontocode.git && cd ontocode`
3. `cargo test -p ontocore-core --lib` (fast smoke) or `cargo test --workspace` (full).
4. For extension work: `cargo build -p ontocore-lsp --bins` then `cd extension && npm ci && ONTOCORE_LSP_BIN=../target/debug/ontocore-lsp npm test`.
5. Make a small, focused change; update docs if behavior changes.
6. Before opening a PR: `./scripts/run-ci-local.sh` (or at least `./scripts/check-doc-versions.sh` for docs-only PRs).

### Plugin contributors (v0.14+)

| Task | Start here |
|------|------------|
| Author a workspace plugin (manifest, validator, exporter) | [guides/plugins.md](guides/plugins.md) |
| Understand host traits and manifest schema | [ontocore/plugin-model.md](ontocore/plugin-model.md), [PLUGIN_SPEC.md](design/PLUGIN_SPEC.md) |
| Reference implementation | `crates/ontocore-plugin-naming/`, `examples/plugin-workspace/` |
| Wire LSP / CLI integration | [lsp-api.md](lsp-api.md), `crates/ontocore-lsp/` |
| OntoUI inspector plugin cards | [platform/CAPABILITY_PROVIDERS.md](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/CAPABILITY_PROVIDERS.md), `extension/webview-ui/src/capabilities/` |

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

> **Canonical copy:** Root [`CONTRIBUTING.md` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CONTRIBUTING.md) is the source of truth. Keep this page in sync when you edit contributor docs.

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

See [Debugging guide](debugging.md) for LSP, extension host, and webview workflows.

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

See [Examples index](examples/index.md) for all runnable assets.

### Documentation site (MkDocs / Read the Docs)

```bash
pip install -r docs/requirements.txt   # Python 3.12 in CI
mkdocs serve
mkdocs build --strict   # CI uses this; must pass with no warnings
./scripts/check-doc-versions.sh   # README / RTD / extension version sync
```

Open http://127.0.0.1:8000. Configuration: [`mkdocs.yml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/mkdocs.yml), [`.readthedocs.yaml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.readthedocs.yaml).

### Full local CI (recommended before PR)

Mirrors [`.github/workflows/ci.yml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/ci.yml) plus VS Code e2e for your platform:

```bash
./scripts/run-ci-local.sh
```

This runs rustfmt, `./scripts/check-doc-versions.sh`, clippy, workspace tests, MSRV (1.88), CLI smoke, release build, LSP smoke tests, webview-ui tests, extension build/tests, `cargo audit`, crate packaging dry-run, mkdocs strict build, and VS Code extension e2e. Expect 30+ minutes on a cold build.

## Pull requests

1. Fork and branch from `main`.
2. Keep changes focused; match existing style and naming.
3. Run Rust and extension tests locally (CI runs both).
4. Update docs when behavior or public API changes (`README.md`, `docs/`, `CHANGELOG.md` as appropriate). On release, follow the checklist in [releasing.md](releasing.md).
5. For spec changes, label whether they describe **implemented** vs **planned** behavior (see [lsp-api.md](lsp-api.md) as a model).

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

1. Check [DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md) — prefer listed crates over new alternatives.
2. Follow [ADR-0016](design/adr/0016-dependency-first-implementation.md) — `ontocore-*` crates are facades, not reimplementations.
3. Update DEPENDENCY_MATRIX and [LICENSES.md](design/LICENSES.md) if the crate is new or uses a non-MIT/Apache license.
4. Pin in workspace `[workspace.dependencies]` in root `Cargo.toml`.

## Code of conduct

See [CODE_OF_CONDUCT.md](https://github.com/eddiethedean/ontocode/blob/main/CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — please do not open public issues for security reports.
