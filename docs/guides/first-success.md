# First success in 10 minutes

This is the **canonical tutorial** for new OntoCode users. You do not need to clone this repository.

## What you will do

1. Install OntoCode in VS Code
2. Open a folder with a Turtle ontology
3. Browse classes in the sidebar
4. Edit an entity in the Entity Inspector
5. (Optional) Validate the workspace from the CLI

## 1. Install OntoCode

Install from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode).

For offline or air-gapped environments, use a release VSIX instead — see [Install VS Code](../vscode-install.md).

## 2. Open an ontology folder

1. **File → Open Folder…** and choose a project that contains ontology files (`.ttl`, `.owl`, `.rdf`, etc.).
2. If VS Code asks, **Trust** the workspace (required for the bundled language server).

> **Multi-root workspaces:** Only the **first** folder is indexed. Open your primary ontology project as a single-root folder, or put it first in a multi-root workspace.

## 3. Browse the explorer

1. Click the **OntoCode** icon in the **Activity Bar** (left edge).
2. Wait for indexing to finish (check **Output → OntoIndex Language Server** if views stay empty).
3. Expand **Ontologies** to see indexed files and parse status.
4. Expand **Classes**, **Properties**, or **Individuals** to browse entities.
5. **Click an entity name** (e.g. `Person`) to open the **Entity Inspector**.

If views are empty, run **OntoCode: Index Workspace** from the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).

## 4. Edit a Turtle entity

Write-back works in **Turtle (`.ttl`) files only** in v0.4. RDF/XML and other formats are read-only in the inspector.

1. Select a class from a `.ttl` file in the explorer.
2. In the Entity Inspector **Edit** section:
   - Add or change a **label** or **comment**
   - Add a **parent class** (named parent IRI)
   - Or use **Delete entity** to remove the entity from the file
3. Changes are written back to the `.ttl` file on disk.

You can also right-click in the explorer to **Create Class**, **Create Property**, or **Create Individual** in a Turtle file.

Full editing reference: [Authoring guide](../authoring.md).

## 5. (Optional) Validate from the CLI

To catch lint and parse errors in CI or locally:

```bash
cargo install ontoindex-cli --locked
ontoindex validate /path/to/your/ontology/folder
```

Exit code **0** means no diagnostic **errors** (warnings are allowed). See [CI integration](../ci-integration.md).

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Sidebar says to index workspace | Run **OntoCode: Index Workspace** |
| No edit controls in inspector | Entity must be in a **`.ttl`** file |
| Language server failed to start | See [Install VS Code](../vscode-install.md#troubleshooting) |
| Empty **Classes** after indexing | Check **Output → OntoIndex Language Server**; re-run **Index Workspace** |

More help: [FAQ](../faq.md).

## Next steps

| Goal | Document |
|------|----------|
| Install options (VSIX, offline) | [vscode-install.md](../vscode-install.md) |
| Patch JSON automation | [patch-reference.md](../patch-reference.md) |
| SQL / SPARQL queries | [sql-reference.md](../sql-reference.md) · [sparql-reference.md](../sparql-reference.md) |
| CI validation | [ci-integration.md](../ci-integration.md) |
