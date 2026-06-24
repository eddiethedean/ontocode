# Getting started

Five-minute paths to success with OntoIndex (CLI) and OntoCode (VS Code).

## Prerequisites

| Path | Requires |
|------|----------|
| VS Code extension | VS Code 1.85+ |
| `cargo install` CLI | Rust 1.88+ |
| Git clone + `cargo run` | Rust 1.88+ |
| Release binaries | No Rust (download from GitHub Releases) |

## Path A — VS Code (recommended for browsing and editing)

Follow [First success in 10 minutes](guides/first-success.md) — Marketplace install, browse the explorer, edit `.ttl` in the Entity Inspector.

Details: [vscode-install.md](vscode-install.md) · [authoring.md](authoring.md)

## Path B — Git clone + CLI (recommended for development)

```bash
git clone https://github.com/eddiethedean/ontocode.git
cd ontocode
cargo build --release

# Sample fixture ontology
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT short_name, labels FROM classes"
cargo run -- validate fixtures
```

The `fixtures/` directory is included in the repository for examples and tests.

## Path C — `cargo install` (no clone)

```bash
cargo install ontoindex-cli --locked
```

Use **your own ontology directory** — there is no `fixtures/` folder outside a clone:

```bash
ontoindex inspect /path/to/your/ontologies
ontoindex query /path/to/your/ontologies "SELECT * FROM classes"
ontoindex validate /path/to/your/ontologies
```

## Path D — Release binaries (no Rust)

1. Open [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) for the latest **v0.6.x** tag.
2. Download the CLI tarball for your platform (Linux x64 example below) or `ontoindex-lsp-v0.6.0-<platform>.tar.gz` / `.zip` for LSP-only use.
3. Verify with `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
4. Extract and run (replace `0.6.0` with your release tag):

```bash
VERSION=0.6.0
ASSET="ontoindex-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
BIN="ontoindex-v${VERSION}-x86_64-unknown-linux-gnu"
tar xzf "${ASSET}"
chmod +x "${BIN}"
./"${BIN}" query /path/to/ontologies "SELECT * FROM classes"
./"${BIN}" validate /path/to/ontologies
```

> **Note:** The extracted binary is versioned (not plain `ontoindex`). CLI release tarballs are **Linux x64 only**; macOS/Windows users should use `cargo install ontoindex-cli` or the bundled LSP inside the VSIX.

For VS Code, install the `ontocode-*.vsix` from the same release.

## Next steps

| Goal | Document |
|------|----------|
| What ships today | [SHIPPED.md](SHIPPED.md) |
| Query Workbench (VS Code) | [guides/query-workbench.md](guides/query-workbench.md) |
| Manchester editor | [guides/manchester-editor.md](guides/manchester-editor.md) |
| Reasoner | [guides/reasoner.md](guides/reasoner.md) |
| SQL queries | [sql-reference.md](sql-reference.md) · [query cookbook](examples/queries.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| Edit Turtle files | [authoring.md](authoring.md) · [patch-reference.md](patch-reference.md) |
| CI validation | [ci-integration.md](ci-integration.md) |
| LSP integration | [lsp-api.md](lsp-api.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) · [faq.md](faq.md) |
