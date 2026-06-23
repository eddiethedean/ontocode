# Installing OntoCode in VS Code

Install **OntoCode** from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) (v0.3.0+ recommended), or use a release VSIX / local dev build below.

## Option A — GitHub Release VSIX (recommended)

1. Open [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) and download the latest `ontocode-*.vsix` for your platform.
2. In VS Code: **Extensions** → **…** menu → **Install from VSIX…**
3. Open a folder containing ontology files (`.ttl`, `.owl`, `.rdf`, `.jsonld`, `.nt`, `.nq`, `.trig`).
4. Open the **OntoCode** activity bar and browse ontologies, classes, properties, individuals, and **Diagnostics**.

> **v0.3 note:** Only the **first** workspace folder is indexed. Use a single-root folder or run **Index Workspace** after opening the primary ontology project.

Release VSIX packages bundle `ontoindex-lsp` for Linux, macOS, and Windows.

## Option B — Build from source

From the repository root:

```bash
./scripts/package-extension.sh
cd extension && npx vsce package --no-dependencies
```

Install the generated `.vsix` via **Install from VSIX…**, or press **F5** in VS Code with the `extension/` folder open (Run Extension).

## Option C — Language server on PATH

If the bundled server is missing, install the LSP binary:

```bash
cargo install ontoindex-lsp
```

Or set **OntoCode: Lsp Path** (`ontocode.lspPath`) to the absolute path of your `ontoindex-lsp` binary.

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | `""` | **Trusted workspaces only.** Path to `ontoindex-lsp`; ignored in Restricted Mode. Empty uses bundled binary or PATH |
| `ontocode.autoIndexOnOpen` | `true` | *(Deprecated in v0.3 — server indexes on startup.)* Legacy setting; indexing is driven by the language server |

## Commands

- **OntoCode: Index Workspace** — rebuild catalog
- **OntoCode: Refresh Explorer** — refresh tree views (including diagnostics)
- **Problems panel** — inline diagnostics from `ontoindex-lsp` after indexing
- **OntoCode: Show Entity Inspector** / **Jump to Source** — from explorer context menu

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| Extension does not activate | Ensure the workspace contains a supported ontology file (see activation in `extension/package.json`) or open the OntoCode Ontologies view |
| `failed to start language server` | Run `./scripts/package-extension.sh`, set `ontocode.lspPath`, or `cargo install ontoindex-lsp` |
| `spawn ... ontoindex-lsp EACCES` (macOS/Linux) | Upgrade to OntoCode ≥ 0.3.0. Manual: `chmod +x` on the bundled binary path from the error |
| `couldn't create connection to server` | Check **Output → OntoIndex Language Server**. Uninstall older OntoCode versions (keep only latest). Try `cargo install ontoindex-lsp` and set `ontocode.lspPath` |
| Empty explorer after open | Run **OntoCode: Index Workspace**; check **Output → OntoIndex Language Server** |

See also [extension/README.md](../extension/README.md).
