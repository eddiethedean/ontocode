# First success: install, browse, and edit (~10 min core path)

This is the **canonical tutorial** for new OntoCode users. You do not need to clone this repository.

New to OWL/RDF? Skim [Ontology concepts](../concepts.md) first (IRIs, Turtle, classes).

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **OntoCode: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## Core path (~10 minutes)

Complete these four steps before exploring optional features below.

### 1. Install OntoCode

Install from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor):

[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)
[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)

For offline or air-gapped environments, use a release VSIX instead — see [Install VS Code](../vscode-install.md).

### 2. Open a folder and trust the workspace

Download a minimal tutorial pack if you do not already have ontology files:

=== "macOS / Linux"

    ```bash
    mkdir ontocode-tutorial && cd ontocode-tutorial
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/main/fixtures/example.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/main/fixtures/complex-classes.ttl
    ```

=== "Windows (PowerShell)"

    ```powershell
    mkdir ontocode-tutorial; cd ontocode-tutorial
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/main/fixtures/example.ttl -OutFile example.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/main/fixtures/complex-classes.ttl -OutFile complex-classes.ttl
    ```

Or download the two files from [fixtures/](https://github.com/eddiethedean/ontocode/tree/main/fixtures) in your browser.

Then **File → Open Folder…** and select `ontocode-tutorial` (or your own ontology folder).

When VS Code asks, choose **Trust** the workspace. OntoCode uses its **bundled** `ontocore-lsp` binary in both trusted and Restricted Mode. In **Restricted Mode**, custom settings such as `ontocode.lspPath` and `ontocode.robotPath` are ignored. If the explorer stays empty after 30 seconds, trust the folder and run **OntoCode: Index Workspace**.

### 3. Browse the explorer

1. Click the **OntoCode** icon in the **Activity Bar** (left edge of the window).
2. Wait for indexing to finish — indexing is complete when **Classes** lists entities (e.g. `Person`) and **Ontologies** shows your `.ttl` files without parse errors. For the tutorial pack (~2 files), this usually takes a few seconds. To confirm progress, open **View → Output**, select **OntoCore Language Server**, and look for index completion messages. If trees stay empty after 30 seconds, run **OntoCode: Index Workspace** and confirm the workspace is trusted.
3. Expand **Ontologies** to see indexed files and parse status.
4. Expand **Classes**, **Properties**, or **Individuals** to browse entities.
5. **Click an entity name** (e.g. `Person`) to open the **Entity Inspector**.

If views stay empty:

1. Confirm the workspace is **trusted** (step 2).
2. Open **View → Output**, choose **OntoCore Language Server** in the dropdown (top-right of the Output panel), and look for errors.
3. Run **OntoCode: Index Workspace** from the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).

See the [feature tour](../ontocode/feature-tour.md) for a visual overview of the sidebar and inspector.

### 4. Edit a Turtle entity

Write-back works in **Turtle (`.ttl`) files only**. RDF/XML and other formats are read-only in the inspector.

1. Select a class from a `.ttl` file in the explorer.
2. In the Entity Inspector **Edit** section:
   - Add or change a **label** or **comment**
   - Add a **parent class** (named parent IRI)
   - Or use **Delete entity** to remove the entity from the file
3. Changes are written back to the `.ttl` file on disk.

You can also right-click in the explorer to **Create Class**, **Create Property**, or **Create Individual** in a Turtle file.

Full editing reference: [Authoring guide](../authoring.md).

**You are done with the core path.** The sections below are optional follow-ups when you have more time.

---

## Explore next (optional)

### Query the workspace

1. Run **OntoCode: Open Query Workbench** from the Command Palette.
2. Choose **SQL** mode and run `SELECT short_name FROM classes`.
3. Confirm rows appear (e.g. classes from your `.ttl` files).
4. Switch to **SPARQL** and run `SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10`.

Guide: [Query Workbench](../ontocode/query-workbench.md).

### Edit a complex axiom in Manchester

Requires a Turtle ontology with a complex subclass (included in the sample pack as `complex-classes.ttl`).

1. Select **Patient** (or another class with a restriction) in the explorer.
2. In the Entity Inspector, click **Edit in Manchester** on the complex axiom row.
3. Validate the expression, preview Turtle, and apply.

Guide: [Manchester editor](../ontocode/manchester-editor.md).

### Run the reasoner

1. Run **OntoCode: Run Reasoner** from the Command Palette.
2. Review the **Reasoner Results** panel (profile, consistency, unsatisfiable classes).
3. Run **OntoCode: Set Hierarchy Mode** → choose **inferred** or **combined** to update the Classes tree.

Guide: [Reasoner guide](../guides/reasoner.md).

### Refactor an entity IRI

1. Select an entity in the explorer (from a `.ttl` file).
2. Run **OntoCode: Rename Entity IRI** from the Command Palette or Entity Inspector.
3. Enter the new IRI and review the **Refactor Preview** diff.
4. Click **Apply** and confirm the `.ttl` files updated.

Guide: [Refactoring](../guides/refactoring.md).

### Open a class graph

1. Run **OntoCode: Open Class Graph** from the Command Palette.
2. Click a node to open the Entity Inspector for that class.

Guide: [Graph view](../ontocode/graph-view.md).

### Validate from the CLI

To catch lint and parse errors in CI or locally:

```bash
cargo install ontocore-cli --locked --version 0.11.1
ontocore validate /path/to/your/ontology/folder
```

Use the folder you opened in VS Code (e.g. `ontocode-tutorial`). Exit code **0** means no diagnostic **errors** (warnings are allowed). See [CI integration](../ci-integration.md).

### Semantic diff

1. Run **OntoCode: Semantic Diff…** from the Command Palette.
2. Compare `HEAD` vs `WORKTREE` (or your branch refs).
3. Review added, removed, and breaking changes in the panel.

Guide: [Semantic diff](../ontocode/semantic-diff.md).

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Sidebar says to index workspace | Run **OntoCode: Index Workspace** |
| No edit controls in inspector | Entity must be in a **`.ttl`** file |
| Language server failed to start | See [Install VS Code](../vscode-install.md#troubleshooting) |
| Empty **Classes** after indexing | Check **Output → OntoCore Language Server**; re-run **Index Workspace** |

More help: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).

## Next steps

| Goal | Document |
|------|----------|
| Visual tour of panels | [Feature tour](../ontocode/feature-tour.md) |
| Install options (VSIX, offline) | [vscode-install.md](../vscode-install.md) |
| Migrating from Protégé | [protege-migration.md](protege-migration.md) |
| Query workbench | [ontocode/query-workbench.md](../ontocode/query-workbench.md) |
| Graph visualization | [ontocode/graph-view.md](../ontocode/graph-view.md) |
| OBO workflows | [guides/obo-workflow.md](../guides/obo-workflow.md) |
| Reasoner | [guides/reasoner.md](../guides/reasoner.md) |
| Manchester editor | [ontocode/manchester-editor.md](../ontocode/manchester-editor.md) |
| Semantic diff | [ontocode/semantic-diff.md](../ontocode/semantic-diff.md) |
| Refactoring | [guides/refactoring.md](../guides/refactoring.md) |
| Patch JSON automation | [patch-reference.md](../patch-reference.md) |
| SQL / SPARQL reference | [sql-reference.md](../sql-reference.md) · [sparql-reference.md](../sparql-reference.md) |
| Authoring overview | [authoring.md](../authoring.md) |
| CI validation | [ci-integration.md](../ci-integration.md) |
