# First success in 10 minutes

This is the **canonical tutorial** for new OntoCode users. You do not need to clone this repository.

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **OntoCode: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## What you will do

1. Install OntoCode in VS Code
2. Open a folder with Turtle ontology files and **trust** the workspace
3. Browse classes in the sidebar
4. Edit an entity in the Entity Inspector
5. (Optional) Query, Manchester editing, reasoner, and CLI validate

## 1. Install OntoCode

Install from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode).

For offline or air-gapped environments, use a release VSIX instead — see [Install VS Code](../vscode-install.md).

## 2. Open a folder and trust the workspace

Download a minimal tutorial pack if you do not already have ontology files:

```bash
mkdir ontocode-tutorial && cd ontocode-tutorial
curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/main/fixtures/example.ttl
curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/main/fixtures/complex-classes.ttl
```

Then **File → Open Folder…** and select `ontocode-tutorial` (or your own ontology folder).

When VS Code asks, choose **Yes, I trust the authors** (or **Trust**). Restricted Mode blocks the language server and the explorer will stay empty.

## 3. Browse the explorer

1. Click the **OntoCode** icon in the **Activity Bar** (left edge of the window).
2. Wait for indexing to finish.
3. Expand **Ontologies** to see indexed files and parse status.
4. Expand **Classes**, **Properties**, or **Individuals** to browse entities.
5. **Click an entity name** (e.g. `Person`) to open the **Entity Inspector**.

If views stay empty:

1. Confirm the workspace is **trusted** (step 2).
2. Open **View → Output**, choose **OntoCore Language Server** in the dropdown (top-right of the Output panel), and look for errors.
3. Run **OntoCode: Index Workspace** from the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).

## 4. Edit a Turtle entity

Write-back works in **Turtle (`.ttl`) files only**. RDF/XML and other formats are read-only in the inspector.

1. Select a class from a `.ttl` file in the explorer.
2. In the Entity Inspector **Edit** section:
   - Add or change a **label** or **comment**
   - Add a **parent class** (named parent IRI)
   - Or use **Delete entity** to remove the entity from the file
3. Changes are written back to the `.ttl` file on disk.

You can also right-click in the explorer to **Create Class**, **Create Property**, or **Create Individual** in a Turtle file.

Full editing reference: [Authoring guide](../authoring.md).

## 5. Query the workspace

1. Run **OntoCode: Open Query Workbench** from the Command Palette.
2. Choose **SQL** mode and run `SELECT short_name FROM classes`.
3. Confirm rows appear (e.g. classes from your `.ttl` files).
4. Switch to **SPARQL** and run `SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10`.

## 6. Edit a complex axiom in Manchester

Requires a Turtle ontology with a complex subclass (included in the sample pack as `complex-classes.ttl`).

1. Select **Patient** (or another class with a restriction) in the explorer.
2. In the Entity Inspector, click **Edit in Manchester** on the complex axiom row.
3. Validate the expression, preview Turtle, and apply.

## 7. (Optional) Run the reasoner

1. Run **OntoCode: Run Reasoner** from the Command Palette.
2. Review the **Reasoner Results** panel (profile, consistency, unsatisfiable classes).
3. Run **OntoCode: Set Hierarchy Mode** → choose **inferred** or **combined** to update the Classes tree.

Full guide: [Reasoner guide](../guides/reasoner.md).

## 8. (Optional) Refactor an entity IRI

1. Select an entity in the explorer (from a `.ttl` file).
2. Run **OntoCode: Rename Entity IRI** from the Command Palette or Entity Inspector.
3. Enter the new IRI and review the **Refactor Preview** diff.
4. Click **Apply** and confirm the `.ttl` files updated.

Guide: [Refactoring](../guides/refactoring.md).

## 9. (Optional) Open a class graph

1. Run **OntoCode: Open Class Graph** from the Command Palette.
2. Click a node to open the Entity Inspector for that class.

Guide: [Graph view](../ontocode/graph-view.md).

## 10. (Optional) Validate from the CLI

To catch lint and parse errors in CI or locally:

```bash
cargo install ontocore-cli --locked
ontocore validate /path/to/your/ontology/folder
```

Use the folder you opened in VS Code (e.g. `ontocode-tutorial`). Exit code **0** means no diagnostic **errors** (warnings are allowed). See [CI integration](../ci-integration.md).

## 11. (Optional) Semantic diff

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
| Install options (VSIX, offline) | [vscode-install.md](../vscode-install.md) |
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
