# Getting started

Quick paths to success (5–30 min depending on install method) with OntoCore (CLI) and OntoCode (VS Code). VS Code tutorial: ~10 minutes — see [First success](guides/first-success.md).

!!! note "First `cargo install` or clone build"
    A cold Rust toolchain may take **15–30+ minutes** to compile OntoCore on first run. The VS Code extension bundles `ontocore-lsp` and does not require Rust.

## Install matrix (CLI)

| Method | Linux x64 | macOS | Windows | Needs Rust? |
|--------|-----------|-------|---------|-------------|
| `cargo install ontocore-cli --locked` | Yes | Yes | Yes | Yes (1.88+) |
| Release CLI tarball (`ontocore-v*-x86_64-unknown-linux-gnu`) | Yes | No | No | No |
| Git clone + `cargo run --` | Yes | Yes | Yes | Yes (1.88+) |

VS Code extension (bundled language server on Linux, macOS, and Windows): [vscode-install.md](vscode-install.md).

[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)
[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)

!!! note "SQL-like queries"
    `ontocore query` uses **SQL-like virtual tables** (single-table `SELECT`, limited `WHERE`). Not full SQL — see [SQL reference](sql-reference.md).

## Prerequisites

| Path | Requires |
|------|----------|
| Minimum VS Code | **1.85+** — see [platform compatibility guide](guides/platform-compatibility.md) |
| `cargo install` CLI | Rust 1.88+; `~/.cargo/bin` on your `PATH` |
| Git clone + `cargo run` | Rust 1.88+ |
| Release CLI binaries | No Rust; **Linux x64 only** (download from GitHub Releases) |

After `cargo install`, ensure `~/.cargo/bin` is on your `PATH`. If you see `ontocore: command not found`, run:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add that line to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) for persistence. If install fails with an MSRV error, run `rustup update stable` and confirm `rustc --version` is **1.88 or newer**.

## Path A — VS Code (recommended for browsing and editing)

Follow [First success in 10 minutes](guides/first-success.md) — [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) install, browse the explorer, edit `.ttl` in the Entity Inspector.

Details: [vscode-install.md](vscode-install.md) · [authoring.md](authoring.md)

## Path B — Git clone + CLI (recommended for development)

```bash
git clone https://github.com/eddiethedean/ontocode.git
cd ontocode

# Sample fixture ontology (debug build is fine for smoke tests)
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT short_name, labels FROM classes"
cargo run -- validate fixtures
```

The `fixtures/` directory is included in the repository for examples and tests.

## Path C — `cargo install` (no clone)

```bash
cargo install ontocore-cli --locked
```

**Version pinning:** Use `--locked` for reproducible installs from crates.io (recommended). Pin an exact release in CI with `cargo install ontocore-cli --locked --version 0.17.0` — see [API stability](guides/api-stability.md) and [release integrity](release-integrity.md).

Use **your own ontology directory** — there is no `fixtures/` folder outside a clone:

```bash
ontocore inspect /path/to/your/ontologies
ontocore query /path/to/your/ontologies "SELECT * FROM classes"
ontocore validate /path/to/your/ontologies
```

## Path D — Release binaries (no Rust)

**CLI pre-builds are Linux x64 only.** On macOS or Windows, use Path C (`cargo install ontocore-cli`) or install the VS Code extension (bundled LSP).

1. Open [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) for the latest **v0.17.x** tag.
2. **For CLI on Linux x64:** download `ontocore-v<version>-x86_64-unknown-linux-gnu.tar.gz`.
3. **For VS Code (any supported OS):** download `ontocode-<version>.vsix` — see [vscode-install.md](vscode-install.md).
4. Verify with `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
5. Extract and run (Linux example; replace `0.17.0` with your tag):

```bash
VERSION=0.17.0
ASSET="ontocore-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
BIN="ontocore-v${VERSION}-x86_64-unknown-linux-gnu"
tar xzf "${ASSET}"
chmod +x "${BIN}"
./"${BIN}" query /path/to/ontologies "SELECT * FROM classes"
./"${BIN}" validate /path/to/ontologies
```

> **Note:** The extracted binary is versioned (not plain `ontocore`). CLI release tarballs are **Linux x64 only**; macOS/Windows users should use `cargo install ontocore-cli` or the bundled LSP inside the VSIX.

For VS Code, install the `ontocode-*.vsix` from the same release.

## Next steps

| Goal | Document |
|------|----------|
| What ships today | [SHIPPED.md](SHIPPED.md) |
| Supported ontology formats | [supported-formats.md](supported-formats.md) |
| Query Workbench (VS Code) | [ontocode/query-workbench.md](ontocode/query-workbench.md) |
| Manchester editor | [ontocode/manchester-editor.md](ontocode/manchester-editor.md) |
| Reasoner | [guides/reasoner.md](guides/reasoner.md) |
| SQL queries | [sql-reference.md](sql-reference.md) · [query cookbook](examples/queries.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| Edit Turtle files | [authoring.md](authoring.md) · [patch-reference.md](patch-reference.md) |
| CI validation | [ci-integration.md](ci-integration.md) |
| LSP integration | [lsp-api.md](lsp-api.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) · [faq.md](faq.md) |
