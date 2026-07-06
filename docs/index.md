---
hide:
  - navigation
  - toc
---

<div class="oc-hero" markdown>

<p class="oc-hero-kicker">OntoCode documentation · v0.11.2</p>

<p class="oc-hero-title">Ontology IDE for VS Code</p>

<p class="oc-hero-lead">
OntoCode is a modern ontology IDE for VS Code, powered by <strong>OntoCore</strong> — the Rust semantic workspace engine for indexing, queries, diagnostics, and reasoning.
</p>

<div class="oc-hero-actions" markdown>
[First success (~10 min core path)](guides/first-success.md){ .md-button .md-button--primary }
[Install the extension](vscode-install.md){ .md-button }
[Install CLI](getting-started.md){ .md-button }
[What ships today](SHIPPED.md){ .md-button }
[Vision](vision.md){ .md-button }
</div>

</div>

<div class="oc-callout" markdown>

**What's new in v0.11?** Turtle completion, diagnostic quick fixes, **Manage Imports**, `ontocore docs` export, Open VSX for Cursor — [Migration v0.10 → v0.11](migration/v0.11.md).

**Not sure where to begin?** Pick a path:

- **[First success (~10 min core path)](guides/first-success.md)** — install the extension, open sample ontologies, browse and edit.
- **[Feature tour](ontocode/feature-tour.md)** — visual overview of explorer, inspector, and React panels.
- **[VS Code extension](ontocode/vscode-extension.md)** — [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor); explorer, inspector, Query Workbench (no Rust required).
- **[CLI / Rust](ontocore/index.md)** — `ontocore` CLI, crates, CI and embedding.

You do **not** need to clone this repo to use the extension or `cargo install ontocore-cli`.

> **Names in 30 seconds:** **OntoCode** = VS Code extension. **OntoCore** = Rust engine (CLI + language server). **Ontologos** = external reasoner. Elsewhere, “extension”, “CLI”, and “language server” are enough.

</div>

## Choose your path

<div class="grid cards" markdown>

-   :material-microsoft-visual-studio-code:{ .lg .middle } **VS Code extension**

    ---

    Browse, edit Turtle, run queries and the reasoner from the OntoCode activity bar.

    [:octicons-arrow-right-24: First success tutorial](guides/first-success.md)

-   :material-console:{ .lg .middle } **Rust & CLI**

    ---

    `cargo install ontocore-cli`, embed `ontocore` / `ontocore-*` crates, validate and classify in CI.

    [:octicons-arrow-right-24: OntoCore docs](ontocore/index.md)

-   :material-brain:{ .lg .middle } **Reasoner**

    ---

    Run EL/RL/RDFS classification, toggle inferred hierarchy, and open EL explanations.

    [:octicons-arrow-right-24: Reasoner guide](guides/reasoner.md)

-   :material-pencil-outline:{ .lg .middle } **Authoring**

    ---

    Edit labels, parents, and entities in `.ttl` files — in VS Code or via patch JSON.

    [:octicons-arrow-right-24: Authoring guide](authoring.md)

-   :material-database-search:{ .lg .middle } **Query Workbench**

    ---

    Run SQL and SPARQL against your indexed workspace from VS Code.

    [:octicons-arrow-right-24: Query Workbench](ontocode/query-workbench.md)

-   :material-code-braces:{ .lg .middle } **Manchester editor**

    ---

    Edit complex SubClassOf and EquivalentClasses axioms in Manchester syntax.

    [:octicons-arrow-right-24: Manchester guide](ontocode/manchester-editor.md)

-   :material-graph-outline:{ .lg .middle } **Graph visualization**

    ---

    Open class, property, import, and neighborhood graphs from the explorer.

    [:octicons-arrow-right-24: Graph guide](ontocode/graph-view.md)

-   :material-file-document-outline:{ .lg .middle } **OBO workflows**

    ---

    Index `.obo` files, browse `obo_id` in the explorer, and run ROBOT from the CLI.

    [:octicons-arrow-right-24: OBO workflow guide](guides/obo-workflow.md)

-   :material-help-circle:{ .lg .middle } **Questions?**

    ---

    FAQ, troubleshooting, workspace limits, and security.

    [:octicons-arrow-right-24: Troubleshooting](troubleshooting.md)

</div>

## What ships today

**v0.11.2 highlights:** Turtle completion, diagnostic quick fixes, Manage Imports, semantic diff (v0.10+), `ontocore docs` export, EL–DL reasoning (`dl` / `auto` profiles), multi-root indexing.

Full matrix: **[What ships today](SHIPPED.md)** (canonical — do not rely on this summary alone).

## Quick start

=== "VS Code"

    1. Install **OntoCode** from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor) — not the VS Code editor itself
    2. **File → Open Folder…** with `.ttl`, `.owl`, `.obo`, or other ontology files
    3. When prompted, **Trust** the workspace (required for indexing)
    4. Open the **OntoCode** activity bar → browse **Classes** → click an entity

=== "CLI (install)"

    ```bash
    cargo install ontocore-cli --locked
    ontocore query /path/to/ontologies "SELECT * FROM classes"
    ontocore validate /path/to/ontologies
    ```

=== "CLI (clone)"

    ```bash
    git clone https://github.com/eddiethedean/ontocode.git
    cd ontocode
    cargo run -- query fixtures "SELECT * FROM classes"
    ```

## Documentation map

| Topic | Link |
|-------|------|
| **Vision** | [vision.md](vision.md) |
| **Architecture** | [architecture.md](architecture.md) |
| **Roadmap** | [roadmap.md](roadmap.md) |
| **OntoCore (platform)** | [ontocore/index.md](ontocore/index.md) |
| **OntoCode extension** | [ontocode/vscode-extension.md](ontocode/vscode-extension.md) |
| **Rust & CLI** | [guides/rust-crates.md](guides/rust-crates.md) |
| **What ships today (canonical)** | [SHIPPED.md](SHIPPED.md) |
| Pick a task (all paths) | [guides/start-here.md](guides/start-here.md) |
| First success tutorial | [guides/first-success.md](guides/first-success.md) |
| Feature tour | [ontocode/feature-tour.md](ontocode/feature-tour.md) |
| Migrating from Protégé | [guides/protege-migration.md](guides/protege-migration.md) |
| Rust API reference | [ontocore/rust-api.md](ontocore/rust-api.md) |
| Reasoner | [guides/reasoner.md](guides/reasoner.md) |
| Query Workbench | [ontocode/query-workbench.md](ontocode/query-workbench.md) |
| Manchester editor | [ontocode/manchester-editor.md](ontocode/manchester-editor.md) |
| Manage Imports | [ontocode/manage-imports.md](ontocode/manage-imports.md) |
| Install VS Code | [vscode-install.md](vscode-install.md) |
| Getting started (CLI) | [getting-started.md](getting-started.md) |
| CLI reference | [cli-reference.md](cli-reference.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) |
| SQL virtual tables | [sql-reference.md](sql-reference.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| Graph visualization | [ontocode/graph-view.md](ontocode/graph-view.md) |
| Refactoring | [guides/refactoring.md](guides/refactoring.md) |
| OBO workflows | [guides/obo-workflow.md](guides/obo-workflow.md) |
| ROBOT interop | [guides/robot-interop.md](guides/robot-interop.md) |
| Webview protocol | [webview-protocol.md](webview-protocol.md) |
| LSP API | [lsp-api.md](lsp-api.md) |
| Errors reference | [errors.md](errors.md) |
| Patch JSON | [patch-reference.md](patch-reference.md) |
| CI integration | [ci-integration.md](ci-integration.md) |
| Examples | [examples/queries.md](examples/queries.md) |
| Design specs & roadmap | [design/README.md](design/README.md) |
| Contributing | [contributing.md](contributing.md) |

Release notes: [CHANGELOG on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)
