# Installing OntoCode in VS Code

[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)
[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **OntoCode: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## Prerequisites

- **VS Code 1.85+** (see [platform compatibility](guides/platform-compatibility.md))
- OntoCode’s **bundled** language server runs in trusted and Restricted Mode. **Trust the workspace** only if you set custom `ontocode.lspPath` or `ontocode.robotPath` — those settings are ignored when the folder is untrusted.

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

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) (v0.18.0+).
2. **File → Open Folder…** and choose a directory with ontology files.
3. OntoCode’s **bundled** language server runs without Trust; **Trust** only if you need custom `ontocode.lspPath` / `ontocode.robotPath`.
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
./scripts/package-extension.sh   # builds LSP + compiles extension (does not emit VSIX)
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

[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)

[Cursor](https://cursor.com/) uses the [Open VSX](https://open-vsx.org/) registry instead of the Microsoft VS Code Marketplace.

1. Open **Extensions** in Cursor and search for **OntoCode** (publisher `ontocode`).
2. Install **OntoCode** (`ontocode.ontocode`).
3. Open a folder with ontology files. OntoCode’s **bundled** language server works in trusted and Restricted Mode — **Trust** only if you set custom `ontocode.lspPath` or `ontocode.robotPath`.

If OntoCode does not appear in search (before v0.11 or if Open VSX sync is delayed):

1. Download `ontocode-*.vsix` from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases).
2. **Cmd+Shift+P** / **Ctrl+Shift+P** → **Extensions: Install from VSIX…**

Release tags from v0.11.3 onward publish automatically to Open VSX.

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

Click an entity name to open the **Entity Inspector**. For `.ttl` and `.obo` files, use the edit section to change labels, parents, or delete entities. See [authoring.md](authoring.md).

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | `""` | **Trusted workspaces only.** Path to `ontocore-lsp`; ignored in Restricted Mode. Empty uses bundled binary |
| `ontocode.robotPath` | `""` | **Trusted workspaces only.** Path to ROBOT JAR or launcher; ignored in Restricted Mode |
| `ontocode.indexCache` | `false` | Persist parse snapshots under `.ontocore/cache/` (add to `.gitignore`) |
| `ontocode.queryHistoryLimit` | `20` | Max entries in Query Workbench history |
| `ontocode.reasoner.default` | `el` | Default profile for Run Reasoner (`el`, `rl`, `rdfs`, `dl`, `auto`) |
| `ontocode.reasoner.autoProfile` | `true` | Profile-detection warnings when running reasoner |
| `ontocode.hierarchy.mode` | `asserted` | Explorer hierarchy: `asserted`, `inferred`, or `combined` |
| `ontocode.diagnostics.rules` | `{}` | Per-rule severity overrides (see [diagnostics config](workspace-limits.md)) |

Indexing runs on workspace open. `ontocode.autoIndexOnOpen` is a legacy setting (no-op); kept for compatibility.

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

### End users (Marketplace / Open VSX install)

| Symptom | Fix |
|---------|-----|
| Extension does not activate | Open a supported ontology file or the **OntoCode → Ontologies** view |
| `failed to start language server` | Check **Output → OntoCore Language Server**; uninstall duplicate OntoCode versions; if using custom `ontocode.lspPath`, **Trust** the workspace |
| `spawn ... ontocore-lsp EACCES` (macOS/Linux) | Upgrade to OntoCode ≥ 0.4.0. Manual: `chmod +x` on the bundled binary path from the error |
| `couldn't create connection to server` | Check **Output → OntoCore Language Server**. Reinstall the extension or download a fresh VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) |
| Empty explorer after open | Run **OntoCode: Index Workspace**; check **Output → OntoCore Language Server**; Trust only if using custom `lspPath`/`robotPath` |
| Inspector has no edit controls | Entity must be in a **Turtle (`.ttl`) or OBO (`.obo`)** file; RDF/XML, OWL/XML, and JSON-LD are read-only in the inspector |

### Developers (building from source)

| Symptom | Fix |
|---------|-----|
| Bundled LSP missing in dev host | `cargo build -p ontocore-lsp --bins` then set `ontocode.lspPath` or run `./scripts/package-extension.sh` |
| Extension tests fail to spawn LSP | `export ONTOCORE_LSP_BIN="$(pwd)/target/debug/ontocore-lsp"` before `npm test` |

See [Debugging guide](debugging.md) for F5, webview-ui, and E2E workflows.

See also [troubleshooting.md](troubleshooting.md), [faq.md](faq.md), and [First success in 10 minutes](guides/first-success.md).
