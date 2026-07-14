# First success: install, browse, and edit (~10 min core path)

This is the **canonical tutorial** for new OntoCode users. You do not need to clone this repository.

**Prerequisites:** VS Code **1.85+**; network access to download tutorial files (step 2); optional [Ontology concepts](../concepts.md) skim if you are new to OWL/RDF.

!!! warning "Write-back formats"
    Entity Inspector write-back applies to **`.ttl`, `.obo`, `.owl`/`.rdf` (RDF/XML), and `.owx` (OWL/XML)**. XML saves are **semantic re-serialize** (not byte-identical to Protégé). JSON-LD / TriG / N-Triples stay read-only — see [Supported formats](../supported-formats.md) and [Known limitations](../known-limitations.md).

!!! tip "Corpus is OWL/XML or RDF/XML?"
    You can edit Protégé-style `.owl` / `.owx` files in the inspector (v0.21+). For caveats (re-serialize, limited ops vs Turtle), read [OWL/XML and RDF/XML write-back](owl-xml-workflow.md).

New to OWL/RDF? Skim [Ontology concepts](../concepts.md) first (IRIs, Turtle, classes).

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **OntoCode: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## Core path (~10 minutes)

Complete these four steps before exploring optional features below.

### 1. Install OntoCode

**VS Code:**

1. Open **Extensions** (`Ctrl+Shift+X` / `Cmd+Shift+X`).
2. Search for **OntoCode**.
3. Click **Install** on the extension by OntoCode.
4. Reload the window if prompted.

**Cursor (Open VSX):**

1. Open **Extensions**.
2. Search for **OntoCode** or install from [Open VSX](https://open-vsx.org/extension/ontocode/ontocode).

Or install from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode):

[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)
[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)

For offline or air-gapped environments, use a release VSIX instead — see [Install VS Code](../vscode-install.md).

### 2. Open a folder

Download a minimal tutorial pack if you do not already have ontology files.

**Offline / air-gapped:** download `ontocode-v0.21.0.vsix` (pattern: `ontocode-v<version>.vsix`) and `ontocode-tutorial.zip` from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) for the **same tagged version** (e.g. **v0.21.0**). Prefer [Versions and channels](versions-and-channels.md) if Marketplace and GitHub disagree. If the zip is missing for an older tag, use the curl commands below on a connected machine, or clone the repo and open `fixtures/`.

**Online (curl from the tagged release):**

=== "macOS / Linux"

    ```bash
    mkdir ontocode-tutorial && cd ontocode-tutorial
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.21.0/fixtures/example.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.21.0/fixtures/complex-classes.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.21.0/examples/obo-workflow/demo.obo
    ```

=== "Windows (PowerShell)"

    ```powershell
    mkdir ontocode-tutorial; cd ontocode-tutorial
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/v0.21.0/fixtures/example.ttl -OutFile example.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/v0.21.0/fixtures/complex-classes.ttl -OutFile complex-classes.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/v0.21.0/examples/obo-workflow/demo.obo -OutFile demo.obo
    ```

Or download the files from the [v0.21.0 fixtures](https://github.com/eddiethedean/ontocode/tree/v0.21.0/fixtures) and [obo-workflow](https://github.com/eddiethedean/ontocode/tree/v0.21.0/examples/obo-workflow) trees in your browser.

Then **File → Open Folder…** and select `ontocode-tutorial` (or your own ontology folder).

!!! tip "Corpus is mostly `.owl` / RDF/XML?"
    Write-back works (v0.21+). Prefer Turtle when you need byte-stable diffs, full Manchester, or refactor apply — [OWL/XML and RDF/XML write-back](owl-xml-workflow.md).
OntoCode’s **bundled** language server works in trusted and Restricted Mode.
Use **Workspace Trust** only when you set custom `ontocode.lspPath` or `ontocode.robotPath` — those settings are ignored when the folder is untrusted.

### 3. Browse the explorer

1. Click the **OntoCode** icon in the **Activity Bar** (left edge of the window — look for the OntoCode logo; or run **OntoCode: Show Entity Inspector** / **OntoCode: Index Workspace** from the Command Palette if the icon is hidden).
2. Wait for indexing to finish.
3. Expand **Ontologies** to see indexed files and parse status.
4. Expand **Classes**, **Properties**, or **Individuals** to browse entities.
5. **Click an entity name** (e.g. `Person`) to open the **Entity Inspector**.

For the tutorial pack (~3 files), indexing usually takes a few seconds. To confirm progress, open **View → Output**, select **OntoCore Language Server**, and look for index completion messages. If trees stay empty after 30 seconds, run **OntoCode: Index Workspace**.

!!! success "Success looks like"
    - **Classes** contains `Person` (from `example.ttl`).
    - **Ontologies** lists `example.ttl`, `complex-classes.ttl`, and `demo.obo` with no parse errors.

If views stay empty:

1. Run **OntoCode: Index Workspace** from the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).
2. Open **View → Output**, choose **OntoCore Language Server** in the dropdown (top-right of the Output panel), and look for errors.
3. If you set a custom `ontocode.lspPath` or `ontocode.robotPath`, **Trust** the workspace so those settings apply.

See the [feature tour](../ontocode/feature-tour.md) for a visual overview of the sidebar and inspector.

### 4. Edit a Turtle entity

Write-back works in **Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`)**. For a full matrix (index/query vs write-back), see [Supported formats](../supported-formats.md). JSON-LD and line-oriented RDF remain read-only in the inspector.

1. Select a class from a `.ttl` file in the explorer.
2. In the Entity Inspector **Edit** section:
   - Add or change a **label** or **comment**
   - Add a **parent class** (named parent IRI)
   - Or use **Delete entity** to remove the entity from the file
3. Changes are written back to the `.ttl` file on disk.

**Success looks like:**

- The `.ttl` file updates on disk (you can open `example.ttl` and see the new `rdfs:label` or axiom).
- Re-indexing is not required for this tutorial step, but the explorer/inspector should reflect the new value shortly after the write completes.

You can also right-click in the explorer to **Create Class**, **Create Property**, or **Create Individual** in a Turtle file.

Full editing reference: [Authoring guide](../authoring.md).

**You are done with the core path.** The sections below are optional follow-ups when you have more time.

---

## Explore next (optional)

### Query the workspace

1. Run **OntoCode: Open Query Workbench** from the Command Palette.
2. Choose **Catalog SQL (subset)** mode and run `SELECT short_name FROM classes`.
   !!! note
       Not full SQL — no `JOIN`, `ORDER BY`, or `LIMIT`. See [SQL reference](../sql-reference.md).
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

### Edit an OBO term (optional)

The tutorial pack includes `demo.obo`:

1. Select a term in the **Classes** explorer (from `demo.obo`).
2. In the Entity Inspector, edit **name**, **definition**, **synonyms**, or **is_a** parents.
3. Use **Preview** then **Apply** — changes write to the `.obo` file on disk.

Guide: [OBO workflow](../guides/obo-workflow.md).

### Validate from the CLI

To catch lint and parse errors in CI or locally (optional — **not part of the 10-minute core path**):

!!! note "CLI install time"
    First `cargo install ontocore-cli` compiles dependencies — expect **15–30+ minutes** on macOS/Windows. **Linux CI:** use the [prebuilt release binary](../ci-integration.md) instead of compiling every job.

=== "Linux x64 (release binary — recommended for CI)"

    ```bash
    VERSION=0.21.0
    ASSET="ontocore-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
    BIN="ontocore-v${VERSION}-x86_64-unknown-linux-gnu"
    curl -fsSL -o "${ASSET}" \
      "https://github.com/eddiethedean/ontocode/releases/download/v${VERSION}/${ASSET}"
    tar xzf "${ASSET}"
    chmod +x "${BIN}"
    ./"${BIN}" validate /path/to/your/ontology/folder
    ```

=== "macOS / Windows / dev (cargo install)"

    ```bash
    cargo install ontocore-cli --locked --version 0.21.0
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
| No edit controls in inspector | Entity must be in **`.ttl`, `.obo`, `.owl`/`.rdf`, or `.owx`**; check parse errors and [Supported formats](../supported-formats.md) |
| Language server failed to start | See [Install VS Code](../vscode-install.md#troubleshooting) |
| Empty **Classes** after indexing | Check **Output → OntoCore Language Server**; re-run **Index Workspace** |

More help: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).

## Next steps

Suggested order after first success:

1. **Query** — [Query Workbench](../ontocode/query-workbench.md) then [SQL reference](../sql-reference.md)
2. **Reason** — [Reasoner guide](../guides/reasoner.md)
3. **Refactor / CI** — [Refactoring](../guides/refactoring.md) then [CI integration](../ci-integration.md)

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
