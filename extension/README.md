# OntoCode

**Browse and edit OWL/RDF ontologies in VS Code** — index a workspace, explore classes and properties in the sidebar, inspect and edit entities in Turtle, and jump to source.

![OntoCode Explorer — sidebar views and entity inspector](media/explorer-preview.png)

---

## Quick start

1. **Install** OntoCode from the Marketplace (you are here).
2. **File → Open Folder…** and choose a project that contains ontology files.
3. If VS Code asks, **Trust** the workspace (required for the bundled language server).
4. Click the **OntoCode** icon in the **Activity Bar** (left edge of the window).
5. Open **Classes**, **Properties**, or **Individuals** and **click an entity name** to open the Entity Inspector.

The language server indexes supported files automatically when the workspace opens.

---

## Supported files

OntoCode activates when your workspace contains any of:

| Extension | Format |
|-----------|--------|
| `.ttl` | Turtle (editable in v0.4) |
| `.owl`, `.rdf` | RDF/XML |
| `.jsonld`, `.json-ld` | JSON-LD |
| `.nt`, `.nq` | N-Triples / N-Quads |
| `.trig` | TriG |

You can also open the **OntoCode → Ontologies** view to force activation.

---

## Using the sidebar

After indexing, the **OntoCode** activity bar shows five views:

| View | What you see |
|------|----------------|
| **Ontologies** | Indexed `.ttl` / `.owl` / … files and parse status |
| **Classes** | Class hierarchy (subclasses nested under parents) |
| **Properties** | Object, data, and annotation properties (grouped) |
| **Individuals** | Named individuals |
| **Diagnostics** | Lint issues grouped by severity (Problems panel + sidebar); click to open source |

**Refresh** — click the ↻ icon on any view title, or run **OntoCode: Refresh Explorer**.

**Re-index** — run **OntoCode: Index Workspace** from the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) after you add or change ontology files.

---

## Entity Inspector

To inspect a class, property, or individual:

1. Expand **Classes** (or **Properties** / **Individuals**).
2. **Click the entity name** (e.g. `Person`).

The **Entity Inspector** panel opens with:

- IRI and kind (class, object property, individual, …)
- Labels and comments
- Parent and child classes
- Axioms (e.g. `SubClassOf`)
- **Edit section** (`.ttl` files only) — add labels, comments, parents; delete entity
- **Jump to Source** — opens the `.ttl` / `.owl` file at the declaration

**Right-click** an entity for **Jump to Source** or **Create Class/Property/Individual** in the context menu.

**Command Palette:** **OntoCode: Show Entity Inspector** — paste an entity IRI if you know it.

Editing guide: [docs/authoring.md](https://github.com/eddiethedean/ontocode/blob/main/docs/authoring.md).

---

## In the editor

Open a `.ttl` (or other supported) file and use standard VS Code navigation:

| Action | Shortcut (macOS) | Shortcut (Windows/Linux) |
|--------|------------------|--------------------------|
| Hover summary on an IRI | hover | hover |
| Go to definition | `F12` | `F12` |
| Document outline (symbols) | `Cmd+Shift+O` | `Ctrl+Shift+O` |
| Workspace symbol search | `Cmd+T` | `Ctrl+T` |

---

## Command Palette

| Command | When to use |
|---------|-------------|
| **OntoCode: Index Workspace** | After adding/changing ontology files |
| **OntoCode: Refresh Explorer** | Reload tree views from the catalog |
| **OntoCode: Show Entity Inspector** | Open inspector by IRI |
| **OntoCode: Jump to Source** | Go to declaration by IRI |
| **OntoCode: Create Class** | Create a new class in a Turtle file |
| **OntoCode: Create Property** | Create a new property in a Turtle file |
| **OntoCode: Create Individual** | Create a new individual in a Turtle file |

---

## Settings

Open **Settings** and search `ontocode`:

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | *(empty)* | Path to `ontoindex-lsp` binary. **Trusted workspaces only.** Leave empty to use the bundled server |

Indexing is driven by the language server on startup; `ontocode.autoIndexOnOpen` is a legacy no-op kept for compatibility.

---

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Sidebar says *“Index workspace to browse ontologies”* | Run **OntoCode: Index Workspace**; confirm the folder contains `.ttl`, `.owl`, etc. |
| Extension never activates | Open a supported ontology file, or click **OntoCode → Ontologies** |
| `failed to start language server` | Check **View → Output → OntoIndex Language Server**. Uninstall older OntoCode versions. Set `ontocode.lspPath` or `cargo install ontoindex-lsp` |
| Empty **Classes** after indexing | **Output → OntoIndex Language Server** for errors; run **Index Workspace** again |
| No items under **Diagnostics** | Index must complete first; check **Problems** panel for the same issues |
| Cannot edit in inspector | Write-back is **Turtle (`.ttl`) only** in v0.4; other formats are read-only |
| Workspace is Restricted | **Trust** the folder — `ontocode.lspPath` is ignored in Restricted Mode |
| Multi-root workspace | Only the **first** folder is indexed — use a single-root folder or open the primary ontology project |

More detail: [Installation & troubleshooting](https://github.com/eddiethedean/ontocode/blob/main/docs/vscode-install.md) · [FAQ](https://github.com/eddiethedean/ontocode/blob/main/docs/faq.md)

---

## What's included in v0.4.0

**Shipped:** explorer, inspector, **editing** (labels, comments, parents, create/delete in `.ttl`), diagnostics, jump-to-source, hover, symbols, go-to-definition.

**Planned (v0.5+):** Manchester editor, query workbench, reasoners — [roadmap](https://github.com/eddiethedean/ontocode/blob/main/docs/design/ROADMAP.md).

---

## Platform support

Release builds bundle `ontoindex-lsp` for Linux (x64, arm64), macOS (Apple Silicon, Intel), and Windows (x64). No extra install needed on those platforms.

---

## Links

- [GitHub repository](https://github.com/eddiethedean/ontocode)
- [Documentation](https://onto-code.readthedocs.io/)
- [Report an issue](https://github.com/eddiethedean/ontocode/issues)
- [Changelog](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)

**Contributing / building from source:** see the [repo README](https://github.com/eddiethedean/ontocode#development).
