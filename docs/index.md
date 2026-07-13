---
hide:
  - navigation
  - toc
---

<div class="oc-hero">

<div class="oc-hero-badges">
  <span class="oc-badge oc-badge--accent">v0.19.0</span>
  <span class="oc-badge">VS Code</span>
  <span class="oc-badge">CLI · LSP</span>
  <span class="oc-badge">Plugins</span>
</div>

<p class="oc-hero-kicker">OntoCode documentation</p>

<p class="oc-hero-title">Ontology IDE for VS Code</p>

<p class="oc-hero-lead">
Index and explore OWL/RDF/OBO, run queries, refactors, and reasoning — in VS Code — powered by <strong>OntoCore</strong> (Rust workspace engine + LSP).
</p>

<p class="oc-hero-ctas">
  <a class="oc-hero-cta" href="guides/first-success/">First success (~10 min) →</a>
  <a class="oc-hero-cta" href="getting-started/" style="margin-left:0.75rem">CLI / CI →</a>
</p>

<p class="oc-hero-subcta"><a href="SHIPPED/">What ships today</a> · <a href="known-limitations/">Known limitations</a></p>

<div class="oc-hero-links">
  <a href="ontocode/feature-tour/">Feature tour</a>
  <a href="glossary/">Glossary</a>
  <a href="vscode-install/">Install extension</a>
</div>

</div>

<div class="oc-callout" markdown>

**Primary path:** **[First success (~10 min)](guides/first-success.md)** — install the extension, open sample ontologies, browse and edit. No clone required.

**Also:** [CLI / CI](getting-started.md) · [Known limitations](known-limitations.md) · [What ships today](SHIPPED.md) · [Feature tour](ontocode/feature-tour.md)

!!! warning "Editable formats"
    Entity Inspector write-back applies to **`.ttl` and `.obo` only**. Other formats index and query as read-only.

!!! note "Catalog SQL (subset)"
    Query Workbench SQL mode is **not** full SQL (no `JOIN` / `ORDER BY` / `LIMIT`). Prefer SPARQL for graph patterns — [SQL reference](sql-reference.md).

> **Names:** **OntoCode** = VS Code extension. **OntoCore** = Rust engine (CLI + language server). **Ontologos** = external reasoner.

![OntoCode product tour](assets/screenshots/product-tour.gif)

</div>

## Choose your path

<div class="grid cards" markdown>

-   :material-microsoft-visual-studio-code:{ .lg .middle } **VS Code extension**

    ---

    Browse, edit Turtle and OBO, run queries and the reasoner from the OntoCode activity bar.

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

    FAQ, troubleshooting, support, workspace limits, and security.

    [:octicons-arrow-right-24: Troubleshooting](troubleshooting.md)

</div>

## What ships today

**v0.19.0 highlights:** Semantic transaction apply path (`ontocore-edit`) for Turtle/OBO; Protégé parity manifest and CI validator; GitHub epics for P0 blockers. Built on the v0.18 Protégé Desktop parity gate.

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

| I need… | Read |
|---------|------|
| 10-minute tutorial | [First success](guides/first-success.md) |
| Honest limits | [Known limitations](known-limitations.md) |
| Capability matrix | [SHIPPED.md](SHIPPED.md) |
| Product overview | [architecture.md](architecture.md) |
| Plugin authoring | [guides/plugins.md](guides/plugins.md) |
| Engineering specs (GitHub) | [engineering.md](engineering.md) |

| Topic | Link |
|-------|------|
| **Vision** | [vision.md](vision.md) |
| **Architecture** | [architecture.md](architecture.md) |
| **Roadmap** | [roadmap.md](roadmap.md) |
| **OntoCore** | [ontocore/index.md](ontocore/index.md) |
| **OntoCode extension** | [ontocode/vscode-extension.md](ontocode/vscode-extension.md) |
| Feature tour | [ontocode/feature-tour.md](ontocode/feature-tour.md) |
| Migrating from Protégé | [guides/protege-migration.md](guides/protege-migration.md) |
| Catalog SQL (subset) | [sql-reference.md](sql-reference.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) |
| Contributing | [contributing.md](contributing.md) |

Release notes: [CHANGELOG on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)
