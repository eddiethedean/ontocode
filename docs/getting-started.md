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

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or a [release VSIX](https://github.com/eddiethedean/ontocode/releases).
2. **File → Open Folder…** and choose a directory with `.ttl`, `.owl`, or other ontology files.
3. **Trust** the workspace when prompted.
4. Click the **OntoCode** icon in the Activity Bar.
5. Expand **Classes** and click an entity to open the **Entity Inspector**.
6. For `.ttl` files, use the edit section to change labels or parents.

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

1. Open [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) for the latest v0.4.x tag.
2. Download `ontoindex-*.tar.gz` (CLI, Linux x64) or `ontoindex-lsp-*.tar.gz` for your platform.
3. Verify with `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
4. Extract and run:

```bash
tar xzf ontoindex-*.tar.gz
./ontoindex query /path/to/ontologies "SELECT * FROM classes"
./ontoindex validate /path/to/ontologies
```

For VS Code, install the `ontocode-*.vsix` from the same release.

## Next steps

| Goal | Document |
|------|----------|
| SQL queries | [sql-reference.md](sql-reference.md) · [query cookbook](https://github.com/eddiethedean/ontocode/blob/main/examples/queries.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| Edit Turtle files | [authoring.md](authoring.md) · [patch-reference.md](patch-reference.md) |
| CI validation | [ci-integration.md](ci-integration.md) |
| LSP integration | [lsp-api.md](lsp-api.md) |
| Troubleshooting | [faq.md](faq.md) |
