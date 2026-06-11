# Contributing to OntoCode / OntoIndex

Thank you for contributing. This repository contains:

- **OntoIndex** — Rust crates under `crates/` (`ontoindex` CLI, `ontoindex-lsp`)
- **OntoCode** — VS Code extension under `extension/`
- **Specs** — product and architecture docs under `docs/design/`
- **User guides** — install, SQL, and LSP API under `docs/`

The root Cargo package `ontocode` is unpublished and hosts workspace integration tests (`tests/`).

## Prerequisites

- Rust **1.86+** (see `rust-version` in `Cargo.toml`)
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

## Pull requests

1. Fork and branch from `main`.
2. Keep changes focused; match existing style and naming.
3. Run Rust and extension tests locally (CI runs both).
4. Update docs when behavior or public API changes (`README.md`, `docs/`, `CHANGELOG.md` as appropriate).
5. For spec changes, label whether they describe **implemented** vs **planned** behavior (see `docs/lsp-api.md` as a model).

## Documentation conventions

| Audience | Where to write |
|----------|----------------|
| New users (install, SQL, LSP) | `docs/` |
| Product vision and milestones | `docs/design/` |
| Architecture decisions | `docs/design/adr/` only (do not add `adrs/`) |
| Extension settings and commands | `extension/README.md` |

## Code of conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](SECURITY.md) — please do not open public issues for security reports.
