# First success: install, browse, and edit (~10 min)

This is the **canonical tutorial** for new OntoCode users. You do not need to clone this repository.

**Prerequisites:** VS Code **1.85+**; network access to download tutorial files (step 2). New to OWL/RDF? Skim [Ontology concepts](../concepts.md).

!!! warning "Write-back formats"
    Inspector write-back: **`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`**. XML is **semantic re-serialize** (not byte-identical to Protégé). JSON-LD / TriG / N-Triples are read-only — [Supported formats](../supported-formats.md).

## Core path (~10 minutes)

### 1. Install OntoCode

**VS Code:** Extensions → search **OntoCode** (`ontocode.ontocode`) → **Install** → reload if prompted.

**Cursor:** install from [Open VSX](https://open-vsx.org/extension/ontocode/ontocode).

For offline VSIX installs, see [Install VS Code](../vscode-install.md).

### 2. Open a folder

**Canonical (online):** download the three sample files below, then **File → Open Folder…**.

=== "macOS / Linux"

    ```bash
    mkdir ontocode-tutorial && cd ontocode-tutorial
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.26.0/fixtures/example.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.26.0/fixtures/complex-classes.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/ontocode/v0.26.0/examples/obo-workflow/demo.obo
    ```

=== "Windows (PowerShell)"

    ```powershell
    mkdir ontocode-tutorial; cd ontocode-tutorial
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/v0.26.0/fixtures/example.ttl -OutFile example.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/v0.26.0/fixtures/complex-classes.ttl -OutFile complex-classes.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/ontocode/v0.26.0/examples/obo-workflow/demo.obo -OutFile demo.obo
    ```

**Offline / optional pack:** If the [v0.26.0 GitHub Release](https://github.com/eddiethedean/ontocode/releases/tag/v0.26.0) includes `ontocode-tutorial.zip`, unzip it and open that folder (also download `ontocode-v0.26.0.vsix` for offline extension install). Built with `./scripts/package-tutorial-zip.sh`. **If the zip is missing**, use the curl/PowerShell commands above or open `fixtures/` from a clone.

Or browse [v0.26.0 fixtures](https://github.com/eddiethedean/ontocode/tree/v0.26.0/fixtures).

!!! note "Workspace Trust"
    The **bundled** language server works in Restricted Mode. **Trust** the folder only if you set custom `ontocode.lspPath` or `ontocode.robotPath`.

### 3. Browse the explorer

![OntoCode explorer with Classes and Entity Inspector](../assets/screenshots/explorer-inspector.png)

1. Click the **OntoCode** Activity Bar icon (ontology / book-like icon on the left sidebar; or run **OntoCode: Index Workspace**).
2. Expand **Ontologies**, then **Classes** / **Properties** / **Individuals**.
3. Click **`Person`** to open the **Entity Inspector**.

!!! success "Success looks like"
    - **Classes** contains `Person` (from `example.ttl`).
    - **Ontologies** lists `example.ttl`, `complex-classes.ttl`, and `demo.obo` with no parse errors.

If trees stay empty: run **OntoCode: Index Workspace**, then check **View → Output → OntoCore Language Server**.

### 4. Edit a Turtle entity

1. With `Person` selected, in the Inspector **Edit** section change a **label** or **comment**, or add a **parent**.
2. Confirm `example.ttl` updates on disk.

**You are done with the core path.** Optional follow-ups below.

---

## Explore next (optional)

| Next | Link |
|------|------|
| Query Workbench | Run `SELECT short_name FROM classes` — [Query Workbench](../ontocode/query-workbench.md) |
| Manchester axioms | [Manchester editor](../ontocode/manchester-editor.md) |
| Reason / realize / SWRL | [Reasoner](reasoner.md) · [Realize](../examples/realize.md) · [SWRL](../examples/swrl.md) |
| Refactor / graphs / OBO / XML | [Feature tour](../ontocode/feature-tour.md) · [OBO authoring](../ontocode/obo-authoring.md) · [OWL/XML write-back](owl-xml-workflow.md) |
| CLI / CI | [Install CLI & CI (detail)](../install-cli-ci.md) · [CI integration](../ci-integration.md) |
| Fit check | [Known limitations](../known-limitations.md) · [What ships today](../SHIPPED.md) |

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| Empty explorer | Index Workspace; Output → OntoCore Language Server |
| Cannot edit | Confirm writable format (`.ttl`/`.obo`/`.owl`/`.rdf`/`.owx`); see [Supported formats](../supported-formats.md) |
| Custom LSP path ignored | Trust the workspace |

Full help: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).
