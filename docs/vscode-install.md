# Installing OntoCode in VS Code

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **OntoCode: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## Install matrix

| Method | Linux | macOS | Windows | Needs Rust? |
|--------|-------|-------|---------|-------------|
| Marketplace extension (bundled language server) | Yes | Yes | Yes | No |
| Open VSX / Cursor marketplace (v0.11+) | Yes | Yes | Yes | No |
| Release VSIX (bundled language server) | Yes | Yes | Yes | No |
| `cargo install ontocore-lsp` + `ontocode.lspPath` | Yes | Yes | Yes | Yes (1.88+) |
| Build from source (`package-extension.sh`) | Yes | Yes | Yes | Yes + Node 20 |

CLI install options (separate from the extension): [getting started (CLI)](getting-started.md).

## Option A — VS Code Marketplace (recommended)

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) (v0.11.0+).
2. **File → Open Folder…** and choose a directory with ontology files.
3. **Trust** the workspace when prompted.
4. Open the **OntoCode** activity bar and browse ontologies, classes, properties, individuals, and **Diagnostics**.

For a full walkthrough, see [First success in 10 minutes](guides/first-success.md).

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **OntoCode: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## Option B — GitHub Release VSIX (offline / air-gapped)

1. Open [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) and download the latest `ontocode-*.vsix`.
2. In VS Code: **Extensions** → **…** menu → **Install from VSIX…**
3. Verify against `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
4. Open a folder containing ontology files (`.ttl`, `.obo`, `.owl`, `.rdf`, `.jsonld`, `.nt`, `.nq`, `.trig`).

Release VSIX packages bundle `ontocore-lsp` for Linux, macOS, and Windows.

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
cargo install ontocore-lsp --locked
```

Set **OntoCode: Lsp Path** (`ontocode.lspPath`) to the absolute path of your `ontocore-lsp` binary. **Trusted workspaces only** — ignored in Restricted Mode.

## Option E — Cursor / Open VSX (v0.11+)

[Cursor](https://cursor.com/) uses the [Open VSX](https://open-vsx.org/) registry instead of the Microsoft VS Code Marketplace.

1. Open **Extensions** in Cursor and search for **OntoCode** (publisher `ontocode`).
2. Install **OntoCode** (`ontocode.ontocode`).
3. Open a folder with ontology files and **Trust** the workspace.

If OntoCode does not appear in search (before v0.11 or if Open VSX sync is delayed):

1. Download `ontocode-*.vsix` from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases).
2. **Cmd+Shift+P** / **Ctrl+Shift+P** → **Extensions: Install from VSIX…**

Release tags from v0.11.0 onward publish automatically to Open VSX.

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
| `ontocode.lspPath` | `""` | **Trusted workspaces only.** Path to `ontocore-lsp`; ignored in Restricted Mode. Empty uses bundled binary |
| `ontocode.queryHistoryLimit` | `20` | Max entries in Query Workbench history |
| `ontocode.reasoner.default` | `el` | Default profile for Run Reasoner (`el`, `rl`, `rdfs`, `dl`, `auto`) |
| `ontocode.reasoner.autoProfile` | `true` | Profile-detection warnings when running reasoner |
| `ontocode.hierarchy.mode` | `asserted` | Explorer hierarchy: `asserted`, `inferred`, or `combined` |

Indexing runs on workspace open. `ontocode.autoIndexOnOpen` is a legacy setting (no-op).

## Commands

- **OntoCode: Index Workspace** — rebuild catalog
- **OntoCode: Refresh Explorer** — refresh tree views (including diagnostics)
- **OntoCode: Open Query Workbench** — SQL and SPARQL against indexed workspace ([guide](ontocode/query-workbench.md))
- **OntoCode: Open Manchester Editor** / **Add Manchester Axiom** — complex class expressions ([guide](ontocode/manchester-editor.md))
- **OntoCode: Run Reasoner** — EL/RL/RDFS classification ([guide](guides/reasoner.md))
- **OntoCode: Show Explanation** — justification for unsatisfiable class
- **OntoCode: Set Hierarchy Mode** — asserted / inferred / combined class tree
- **OntoCode: Open Class Graph** / **Property Graph** / **Import Graph** / **Neighborhood Graph** — visualization ([guide](ontocode/graph-view.md))
- **OntoCode: Create Class / Property / Individual** — authoring in `.ttl` files
- **Problems panel** — inline diagnostics from `ontocore-lsp` after indexing
- **OntoCode: Show Entity Inspector** / **Jump to Source** — from explorer context menu

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| Extension does not activate | Open a supported ontology file or the **OntoCode → Ontologies** view |
| `failed to start language server` | Run `./scripts/package-extension.sh`, set `ontocode.lspPath`, or `cargo install ontocore-lsp` |
| `spawn ... ontocore-lsp EACCES` (macOS/Linux) | Upgrade to OntoCode ≥ 0.4.0. Manual: `chmod +x` on the bundled binary path from the error |
| `couldn't create connection to server` | Check **Output → OntoCore Language Server**. Uninstall older OntoCode versions. Try `cargo install ontocore-lsp` and set `ontocode.lspPath` |
| Empty explorer after open | Run **OntoCode: Index Workspace**; check **Output → OntoCore Language Server** |
| Inspector has no edit controls | Entity must be in a **Turtle (`.ttl`)** file; other formats are read-only |

See also [troubleshooting.md](troubleshooting.md), [faq.md](faq.md), and [First success in 10 minutes](guides/first-success.md).
