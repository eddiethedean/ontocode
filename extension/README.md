# OntoCode VS Code Extension

Browse ontology repositories in VS Code using the OntoIndex language server.

**Install:** [docs/vscode-install.md](../docs/vscode-install.md) (release VSIX, dev build, troubleshooting).

## Features (v0.2.1)

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
