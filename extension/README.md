# OntoCode

[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)
[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)

**Ontology IDE for VS Code** — powered by **OntoCore** (`ontocore-lsp` language server).

**Current release: v0.17.0** — see [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) and [migration v0.17](https://ontocode-vs.readthedocs.io/en/latest/migration/v0.17/).

> **New here?** [First success (~10 min)](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) · [Migrating from Protégé?](https://ontocode-vs.readthedocs.io/en/latest/guides/protege-migration/) · [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [Migration v0.14](https://ontocode-vs.readthedocs.io/en/latest/migration/v0.14/) · [Full extension docs](https://ontocode-vs.readthedocs.io/en/latest/ontocode/vscode-extension/) · [FAQ](https://ontocode-vs.readthedocs.io/en/latest/faq/)

> **CLI or Rust crates?** See the [Rust & CLI documentation path](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/).

---

## Quick start

1. **Install** OntoCode from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor).
2. **File → Open Folder…** and choose a project that contains ontology files.
3. OntoCode’s **bundled** language server runs in trusted and Restricted Mode. **Trust the workspace** only if you set custom `ontocode.lspPath` or `ontocode.robotPath` — those settings are ignored when the folder is untrusted.
4. Click the **OntoCode** icon in the **Activity Bar** (left edge of the window).
5. Open **Classes**, **Properties**, or **Individuals** and **click an entity name** to open the Entity Inspector.

Edit **Turtle (`.ttl`)** and **OBO (`.obo`)** in the Entity Inspector. RDF/XML and OWL/XML are indexed for browse/query; write-back support differs by format — see [Supported formats](https://ontocode-vs.readthedocs.io/en/latest/supported-formats/).

New to OntoCode? Follow the [first success tutorial](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) on Read the Docs.

The language server indexes supported files automatically when the workspace opens.

---

## Supported files

OntoCode activates when your workspace contains any of:

| Extension | Format |
|-----------|--------|
| `.ttl` | Turtle (editable) |
| `.owl`, `.rdf` | RDF/XML |
| `.jsonld`, `.json-ld` | JSON-LD |
| `.nt`, `.nq` | N-Triples / N-Quads |
| `.trig` | TriG |
| `.obo` | OBO Format (index, syntax highlighting, and inspector write-back since v0.13) |

Write-back in the inspector is **Turtle (`.ttl`) and OBO (`.obo`)**.

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

The Entity Inspector panel opens with:

- IRI and kind (class, object property, individual, …)
- Labels and comments
- Parent and child classes
- Axioms (e.g. `SubClassOf`)
- **Edit section** (`.ttl` and `.obo` files) — add labels, comments, parents; edit OBO terms; delete entity
- **Jump to Source** — opens the `.ttl` / `.owl` file at the declaration

**Right-click** an entity for **Jump to Source** or **Create Class/Property/Individual** in the context menu.

**Command Palette:** **OntoCode: Show Entity Inspector** — paste an entity IRI if you know it.

Editing guide: [authoring guide](https://ontocode-vs.readthedocs.io/en/latest/authoring/).

---

## In the editor

Open a `.ttl` (or other supported) file and use standard VS Code navigation:

| Action | Shortcut (macOS) | Shortcut (Windows/Linux) |
|--------|------------------|--------------------------|
| Hover summary on an IRI | hover | hover |
| Go to definition | `F12` | `F12` |
| Rename symbol | `F2` | `F2` |
| Find references | `Shift+F12` | `Shift+F12` |
| Turtle completion (prefix, QName, IRI) | `Ctrl+Space` | `Ctrl+Space` |
| Diagnostic quick fixes | lightbulb / `Cmd+.` | lightbulb / `Ctrl+.` |
| Document outline (symbols) | `Cmd+Shift+O` | `Ctrl+Shift+O` |
| Workspace symbol search | `Cmd+T` | `Ctrl+T` |

---

## Command Palette

| Command | When to use |
|---------|-------------|
| **OntoCode: Index Workspace** | After adding/changing ontology files |
| **OntoCode: Refresh Explorer** | Reload tree views from the catalog |
| **OntoCode: Open Query Workbench** | Run SQL/SPARQL against indexed workspace |
| **OntoCode: Open Manchester Editor** | Edit complex axiom on selected entity |
| **OntoCode: Add Manchester Axiom** | Add complex subclass or equivalent |
| **OntoCode: Show Entity Inspector** | Open inspector by IRI |
| **OntoCode: Jump to Source** | Go to declaration by IRI |
| **OntoCode: Create Class** | Create a new class in a Turtle file |
| **OntoCode: Create Property** | Create a new property in a Turtle file |
| **OntoCode: Create Individual** | Create a new individual in a Turtle file |
| **OntoCode: Manage Imports** | Add or remove `owl:imports` in a Turtle file |
| **OntoCode: Open Class Graph** | Class hierarchy visualization |
| **OntoCode: Open Property Graph** | Property relationship graph |
| **OntoCode: Open Import Graph** | Ontology import dependencies |
| **OntoCode: Open Neighborhood Graph** | Entity neighborhood graph |
| **OntoCode: Run Reasoner** | Classify workspace (EL/RL/RDFS/DL/auto) and open Results panel |
| **OntoCode: Show Explanation** | Open explanation for an unsatisfiable class |
| **OntoCode: Set Hierarchy Mode** | Toggle asserted / inferred / combined class tree |
| **OntoCode: Find Entity Usages** | List all references to an entity IRI |
| **OntoCode: Rename Entity IRI** | Rename an entity across Turtle files (preview + apply) |
| **OntoCode: Migrate Namespace** | Replace a namespace base IRI workspace-wide |
| **OntoCode: Move Entity** | Move an entity block to another `.ttl` file |
| **OntoCode: Extract Module** | Extract entities into a new module file |
| **OntoCode: Semantic Diff…** | Compare versions, directories, or workspace snapshots (breaking-change heuristics) |

---

## Settings

Open **Settings** and search `ontocode`:

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | *(empty)* | Path to `ontocore-lsp` binary. **Trusted workspaces only.** Leave empty to use the bundled server |
| `ontocode.queryHistoryLimit` | `20` | Max entries in Query Workbench history |
| `ontocode.reasoner.default` | `el` | Default profile for Run Reasoner |
| `ontocode.reasoner.autoProfile` | `true` | Profile-detection warnings |
| `ontocode.hierarchy.mode` | `asserted` | Class tree: asserted / inferred / combined |
| `ontocode.indexCache` | `false` | Persist parse snapshots under `.ontocore/cache/` (add to `.gitignore`) |

Indexing is driven by the language server on startup; `ontocode.autoIndexOnOpen` is a legacy no-op kept for compatibility.

---

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Sidebar says *“Index workspace to browse ontologies”* | Run **OntoCode: Index Workspace**; confirm the folder contains `.ttl`, `.owl`, etc. |
| Extension never activates | Open a supported ontology file, or click **OntoCode → Ontologies** |
| `failed to start language server` | Check **View → Output → OntoCore Language Server**. Uninstall older OntoCode versions. Set `ontocode.lspPath` or `cargo install ontocore-lsp` |
| Empty **Classes** after indexing | **Output → OntoCore Language Server** for errors; run **Index Workspace** again |
| No items under **Diagnostics** | Index must complete first; check **Problems** panel for the same issues |
| Cannot edit in inspector | Write-back is **Turtle (`.ttl`) and OBO (`.obo`)**; OWL/XML and RDF/XML are read-only in the inspector |
| Workspace is Restricted | Bundled LSP still runs; **Trust** only if you need custom `ontocode.lspPath` / `ontocode.robotPath` (ignored when untrusted) |
| Multi-root workspace | All workspace folders are indexed; use **Index Workspace** after adding folders |

More detail: [Installation & troubleshooting](https://ontocode-vs.readthedocs.io/en/latest/vscode-install/) · [FAQ](https://ontocode-vs.readthedocs.io/en/latest/faq/)

---

## What's included in v0.17.0

**New in v0.17:** Protégé-shell menus, toolbars, and dialogs; named perspectives and layout persistence; command registry / workspace UI state via LSP.

**Also in recent minors:** plugin preferences pages and context actions (v0.16); plugin `ui.commands` via LSP `ontocore/runPlugin`; **Reload Imports** and **Reset Layout**.

**Shipped:** explorer; **React** entity inspector (panel routing fix), graph panels, Query Workbench, Manchester editor, Refactor Preview, **Manage Imports**, and **Semantic Diff** panel; workspace refactor (rename IRI, migrate namespace, move, extract); EL/RL/RDFS/DL/auto reasoner (OntoLogos 1.x); **Turtle and OBO write-back** (engine v0.12; inspector v0.13); diagnostics with **quick fixes**; Turtle **completion**; LSP navigation (hover, go-to-definition, find references, rename); multi-root workspaces; optional index disk cache; **plugin host** (`ontocore plugins`, workflow scaffold, inspector plugin cards, dockable views); [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) for Cursor.

**Planned:** stable plugin ecosystem API and full owlmake integration (**v1.0**). Full Protégé parity (OWL/XML write-back) is a **v1.0** goal — see [Protégé parity matrix](https://ontocode-vs.readthedocs.io/en/latest/design/PROTEGE_PARITY/).

---

## Platform support

Release builds bundle `ontocore-lsp` for Linux (x64, arm64), macOS (Apple Silicon, Intel), and Windows (x64). No extra install needed on those platforms.

---

## Links

- [OntoCode extension documentation](https://ontocode-vs.readthedocs.io/en/latest/ontocode/vscode-extension/) — full extension guide on Read the Docs
- **Getting started:** [First success](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/) · [VS Code install & troubleshooting](https://ontocode-vs.readthedocs.io/en/latest/vscode-install/) · [Feature tour](https://ontocode-vs.readthedocs.io/en/latest/ontocode/feature-tour/)
- **Guides:** [Reasoner](https://ontocode-vs.readthedocs.io/en/latest/guides/reasoner/) · [Query Workbench](https://ontocode-vs.readthedocs.io/en/latest/ontocode/query-workbench/) · [Manchester editor](https://ontocode-vs.readthedocs.io/en/latest/ontocode/manchester-editor/) · [Authoring & patches](https://ontocode-vs.readthedocs.io/en/latest/authoring/)
- **Reference:** [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) · [Patch reference](https://ontocode-vs.readthedocs.io/en/latest/patch-reference/) · [SQL](https://ontocode-vs.readthedocs.io/en/latest/sql-reference/) · [SPARQL](https://ontocode-vs.readthedocs.io/en/latest/sparql-reference/) · [LSP API](https://ontocode-vs.readthedocs.io/en/latest/lsp-api/)
- **Enterprise:** [Evaluation guide](https://ontocode-vs.readthedocs.io/en/latest/guides/enterprise-eval/) · [Production readiness](https://ontocode-vs.readthedocs.io/en/latest/guides/production-readiness/)
- [GitHub repository](https://github.com/eddiethedean/ontocode)
- [Report an issue](https://github.com/eddiethedean/ontocode/issues)
- [Changelog](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) · [Docs changelog](https://ontocode-vs.readthedocs.io/en/latest/changelog/)

**Contributing / building from source:** see the [repo README](https://github.com/eddiethedean/ontocode#development).
