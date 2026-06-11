# OntoCode VS Code Extension

Browse ontology repositories in VS Code using the OntoIndex language server.

**Install:** [docs/vscode-install.md](../docs/vscode-install.md) (release VSIX, dev build, troubleshooting).

## Features (v0.2.2)

- Ontology explorer sidebar (ontologies, classes, properties, individuals)
- Entity inspector with IRI, labels, comments, parents, children, and axioms
- Jump to source in Turtle/RDF files
- Workspace indexing on open
- Hover, document symbols, workspace symbols, and go-to-definition for ontology files

## Platform support

Release VSIX packages bundle `ontoindex-lsp` for:

| Platform | Bundled path |
|----------|----------------|
| Linux x64 | `server/linux-x64/` |
| Linux arm64 | `server/linux-arm64/` |
| macOS Apple Silicon | `server/darwin-arm64/` |
| macOS Intel | `server/darwin-x64/` |
| Windows x64 | `server/win32-x64/ontoindex-lsp.exe` |

If no bundled binary matches your machine:

```bash
cargo install ontoindex-lsp
```

Or set `ontocode.lspPath` to the absolute path of your `ontoindex-lsp` binary.

## Development

From the repository root:

```bash
./scripts/package-extension.sh
```

Then in VS Code: **Run Extension** (F5) with the `extension/` folder, or install the generated `.vsix`.

To build a VSIX with all platform binaries locally, run `./scripts/package-extension.sh` on each target OS (or copy release artifacts into `extension/server/` before `vsce package`).

## Testing

```bash
# From repo root: build LSP and unit / spawn e2e tests
cargo build -p ontoindex-lsp --bins
cd extension && npm ci && npm test

# VS Code integration tests (downloads VS Code under extension/.vscode-test/)
./scripts/prepare-extension-server.sh "$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m | sed 's/x86_64/x64/;s/aarch64/arm64/')"
cd extension && npm run compile && npm run compile:vscode-test && npm run test:vscode

# Pin a VS Code version (matches extension engines minimum)
VSCODE_VERSION=1.85.0 npm run test:vscode
```

CI runs the VS Code matrix in [`.github/workflows/extension-vscode-e2e.yml`](../.github/workflows/extension-vscode-e2e.yml): Ubuntu (x64 + arm64), macOS, and Windows against VS Code **1.85.0** (minimum) and **stable**. Each job strips the bundled LSP execute bit before launch to catch Marketplace install regressions.

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | `""` | Path to `ontoindex-lsp` binary |
| `ontocode.autoIndexOnOpen` | `true` | Index workspace when the extension activates |

## Commands

- **OntoCode: Index Workspace** — rebuild the ontology catalog
- **OntoCode: Refresh Explorer** — refresh tree views from the catalog
- **OntoCode: Show Entity Inspector** — open inspector for an IRI
- **OntoCode: Jump to Source** — navigate to entity declaration
