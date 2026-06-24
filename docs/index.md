---
hide:
  - navigation
  - toc
---

<div class="oc-hero" markdown>

<p class="oc-hero-kicker">OntoCode documentation · v0.6.0</p>

<p class="oc-hero-title">Ontology-as-code for Git and VS Code</p>

<p class="oc-hero-lead">
Browse OWL/RDF in VS Code, edit Turtle ontologies, run EL/RL/RDFS reasoning, query and validate in CI, and index workspaces locally with the Rust <strong>OntoIndex</strong> engine.
</p>

<div class="oc-hero-badges" markdown>
<span class="oc-badge oc-badge--accent">VS Code extension</span>
<span class="oc-badge oc-badge--accent">Rust CLI</span>
<span class="oc-badge">Turtle write-back</span>
<span class="oc-badge">Query Workbench</span>
<span class="oc-badge">Manchester editor</span>
<span class="oc-badge">Reasoner (EL)</span>
<span class="oc-badge">SPARQL + SQL</span>
</div>

<div class="oc-hero-actions" markdown>
[Start here](guides/start-here.md){ .md-button .md-button--primary }
[First success (10 min)](guides/first-success.md){ .md-button }
[What ships today](SHIPPED.md){ .md-button }
</div>

</div>

<div class="oc-callout" markdown>

**Not sure where to begin?** Use [Start here](guides/start-here.md) to pick a path — VS Code explorer, CLI indexing, Turtle editing, reasoning, or CI validation. You do **not** need to clone this repo to use the Marketplace extension or `cargo install ontoindex-cli`.

> **Naming:** **OntoCode** is the VS Code UI. **OntoIndex** is the engine (`ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp`).

</div>

## Choose your path

<div class="grid cards" markdown>

-   :material-console:{ .lg .middle } **CLI & OntoIndex**

    ---

    Index ontologies, run SQL/SPARQL queries, validate and classify in CI, and apply Turtle patches.

    [:octicons-arrow-right-24: Getting started](getting-started.md)

-   :material-microsoft-visual-studio-code:{ .lg .middle } **VS Code (OntoCode)**

    ---

    Browse classes and properties, edit Turtle in the inspector, and see diagnostics inline.

    [:octicons-arrow-right-24: Install extension](vscode-install.md)

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

    [:octicons-arrow-right-24: Query Workbench](guides/query-workbench.md)

-   :material-code-braces:{ .lg .middle } **Manchester editor**

    ---

    Edit complex SubClassOf and EquivalentClasses axioms in Manchester syntax.

    [:octicons-arrow-right-24: Manchester guide](guides/manchester-editor.md)

-   :material-help-circle:{ .lg .middle } **Questions?**

    ---

    FAQ, troubleshooting, workspace limits, and security.

    [:octicons-arrow-right-24: Troubleshooting](troubleshooting.md)

</div>

## What ships in v0.6.0

See the full matrix: **[What ships today](SHIPPED.md)**.

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl`) | Yes | `ontoindex patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes | `ontoindex patch` |
| Create / delete entities (`.ttl`) | Yes | `ontoindex patch` |
| SQL-like queries | Query Workbench | `ontoindex query` |
| SPARQL | Query Workbench | `ontoindex sparql` |
| EL/RL/RDFS classification | Reasoner panel | `ontoindex classify` |
| Inferred hierarchy toggle | Explorer | via `classify` JSON |
| EL explanations (where available) | Explanation panel | `ontoindex explain` |
| Diagnostics / lint | Problems panel | `ontoindex validate` |

## Quick start

=== "VS Code"

    1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)
    2. Open a folder with `.ttl`, `.owl`, or other ontology files
    3. Open the **OntoCode** activity bar → browse **Classes** → click an entity

=== "CLI (install)"

    ```bash
    cargo install ontoindex-cli
    ontoindex query /path/to/ontologies "SELECT * FROM classes"
    ontoindex validate /path/to/ontologies
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
| **What ships today (canonical)** | [SHIPPED.md](SHIPPED.md) |
| Start here (pick a path) | [guides/start-here.md](guides/start-here.md) |
| First success tutorial | [guides/first-success.md](guides/first-success.md) |
| Reasoner | [guides/reasoner.md](guides/reasoner.md) |
| Query Workbench | [guides/query-workbench.md](guides/query-workbench.md) |
| Manchester editor | [guides/manchester-editor.md](guides/manchester-editor.md) |
| Install VS Code | [vscode-install.md](vscode-install.md) |
| Getting started (CLI) | [getting-started.md](getting-started.md) |
| CLI reference | [cli-reference.md](cli-reference.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) |
| SQL virtual tables | [sql-reference.md](sql-reference.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| LSP API | [lsp-api.md](lsp-api.md) |
| Errors reference | [errors.md](errors.md) |
| Patch JSON | [patch-reference.md](patch-reference.md) |
| CI integration | [ci-integration.md](ci-integration.md) |
| Examples | [examples/queries.md](examples/queries.md) |
| Design specs & roadmap | [design/README.md](design/README.md) |
| Contributing | [contributing.md](contributing.md) |

Release notes: [CHANGELOG on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)
