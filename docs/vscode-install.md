# Installing OntoCode in VS Code

> **Multi-root workspaces:** Only the **first** folder is indexed. Open your ontology project as a **single-root** folder, or put it first in a multi-root workspace.

## Option A — VS Code Marketplace (recommended)

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) (v0.7.0+).
2. **File → Open Folder…** and choose a directory with ontology files.
3. **Trust** the workspace when prompted.
4. Open the **OntoCode** activity bar and browse ontologies, classes, properties, individuals, and **Diagnostics**.

For a full walkthrough, see [First success in 10 minutes](guides/first-success.md).

> **Multi-root workspaces:** Only the **first** workspace folder is indexed. Use a single-root folder or open the primary ontology project as the first folder.

## Option B — GitHub Release VSIX (offline / air-gapped)

1. Open [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) and download the latest `ontocode-*.vsix`.
2. In VS Code: **Extensions** → **…** menu → **Install from VSIX…**
3. Verify against `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
4. Open a folder containing ontology files (`.ttl`, `.obo`, `.owl`, `.rdf`, `.jsonld`, `.nt`, `.nq`, `.trig`).

Release VSIX packages bundle `ontoindex-lsp` for Linux, macOS, and Windows.

## Option C — Build from source

**Prerequisites:** Rust **1.88+**, Node **20+**, npm (see [contributing.md](contributing.md)).

From the repository root:

```bash
./scripts/package-extension.sh
cd extension && npx vsce package --no-dependencies
```

Install the generated `.vsix` via **Install from VSIX…**, or press **F5** in VS Code with the `extension/` folder open (Run Extension).

## Option D — Language server on PATH

Use this only when the bundled server is missing or you are developing the LSP:

```bash
cargo install ontoindex-lsp --locked
```

Set **OntoCode: Lsp Path** (`ontocode.lspPath`) to the absolute path of your `ontoindex-lsp` binary. **Trusted workspaces only** — ignored in Restricted Mode.

## Using the sidebar

After indexing, the **OntoCode** activity bar shows five views:

| View | What you see |
|------|----------------|
| **Ontologies** | Indexed files and parse status |
| **Classes** | Class hierarchy |
| **Properties** | Object, data, and annotation properties |
| **Individuals** | Named individuals |
| **Diagnostics** | Lint issues; click to open source |

**Refresh** — click ↻ on a view title, or run **OntoCode: Refresh Explorer**.

**Re-index** — run **OntoCode: Index Workspace** after adding or changing ontology files.

Click an entity name to open the **Entity Inspector**. For `.ttl` files, use the edit section to change labels, parents, or delete entities. See [authoring.md](authoring.md).

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | `""` | **Trusted workspaces only.** Path to `ontoindex-lsp`; ignored in Restricted Mode. Empty uses bundled binary |
| `ontocode.queryHistoryLimit` | `20` | Max entries in Query Workbench history |
| `ontocode.reasoner.default` | `el` | Default profile for Run Reasoner (`dl`/`auto` require OntoLogos 1.0) |
| `ontocode.reasoner.autoProfile` | `true` | Profile-detection warnings when running reasoner |
| `ontocode.hierarchy.mode` | `asserted` | Explorer hierarchy: `asserted`, `inferred`, or `combined` |

Indexing runs on workspace open. `ontocode.autoIndexOnOpen` is a legacy setting (no-op).

## Commands

- **OntoCode: Index Workspace** — rebuild catalog
- **OntoCode: Refresh Explorer** — refresh tree views (including diagnostics)
- **OntoCode: Open Query Workbench** — SQL and SPARQL against indexed workspace ([guide](guides/query-workbench.md))
- **OntoCode: Open Manchester Editor** / **Add Manchester Axiom** — complex class expressions ([guide](guides/manchester-editor.md))
- **OntoCode: Run Reasoner** — EL/RL/RDFS classification ([guide](guides/reasoner.md))
- **OntoCode: Show Explanation** — justification for unsatisfiable class
- **OntoCode: Set Hierarchy Mode** — asserted / inferred / combined class tree
- **OntoCode: Open Class Graph** / **Property Graph** / **Import Graph** / **Neighborhood Graph** — visualization ([guide](guides/graph-visualization.md))
- **OntoCode: Create Class / Property / Individual** — authoring in `.ttl` files
- **Problems panel** — inline diagnostics from `ontoindex-lsp` after indexing
- **OntoCode: Show Entity Inspector** / **Jump to Source** — from explorer context menu

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| Extension does not activate | Open a supported ontology file or the **OntoCode → Ontologies** view |
| `failed to start language server` | Run `./scripts/package-extension.sh`, set `ontocode.lspPath`, or `cargo install ontoindex-lsp` |
| `spawn ... ontoindex-lsp EACCES` (macOS/Linux) | Upgrade to OntoCode ≥ 0.4.0. Manual: `chmod +x` on the bundled binary path from the error |
| `couldn't create connection to server` | Check **Output → OntoIndex Language Server**. Uninstall older OntoCode versions. Try `cargo install ontoindex-lsp` and set `ontocode.lspPath` |
| Empty explorer after open | Run **OntoCode: Index Workspace**; check **Output → OntoIndex Language Server** |
| Inspector has no edit controls | Entity must be in a **Turtle (`.ttl`)** file; other formats are read-only |

See also [troubleshooting.md](troubleshooting.md), [faq.md](faq.md), and [First success in 10 minutes](guides/first-success.md).
