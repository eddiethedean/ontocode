---
hide:
  - navigation
  - toc
---

<div class="oc-hero">

<div class="oc-hero-badges">
  <span class="oc-badge oc-badge--accent">Latest tagged v0.25.0</span>
  <span class="oc-badge">VS Code</span>
  <span class="oc-badge">CLI · LSP</span>
</div>

<p class="oc-hero-kicker">OntoCode documentation</p>

<p class="oc-hero-title">Ontology IDE for VS Code</p>

<p class="oc-hero-lead">
Index and explore OWL/RDF/OBO, run queries, refactors, and reasoning — in VS Code — powered by <strong>OntoCore</strong> (Rust workspace engine + LSP).
</p>

<p class="oc-hero-ctas">
  <a class="oc-hero-cta" href="guides/first-success/">First success (~10 min) →</a>
</p>

<p class="oc-hero-subcta"><a href="SHIPPED/">Evaluate · What ships today</a> · <a href="install/">Install</a> · <a href="guides/versions-and-channels/">Versions &amp; channels</a> · <a href="known-limitations/">Known limitations</a> · Latest tagged: <strong>v0.25.0</strong></p>

<div class="oc-hero-links">
  <a href="ontocode/">OntoCode overview</a>
  <a href="ontocode/feature-tour/">Feature tour</a>
  <a href="glossary/">Glossary</a>
</div>

</div>

<div class="oc-callout" markdown>

**Primary path:** **[First success (~10 min)](guides/first-success.md)** — install the extension, open sample ontologies, browse and edit. No clone required.

**Also:** [Install](install.md) · [Examples](examples/index.md) · [Feature tour](ontocode/feature-tour.md) · [What ships today](SHIPPED.md)

![OntoCode product tour](assets/screenshots/product-tour.gif)

<details markdown>
<summary>Formats, SQL subset, and names</summary>

!!! warning "Editable formats"
    Entity Inspector write-back applies to **`.ttl`, `.obo`, `.owl`/`.rdf` (RDF/XML), and `.owx` (OWL/XML)**. XML is **semantic re-serialize** (not Protégé byte-identical). JSON-LD / TriG / N-Triples stay read-only — [Supported formats](supported-formats.md).

!!! note "Catalog SQL (subset)"
    Query Workbench SQL mode is **not** full SQL (no `JOIN` / `ORDER BY` / `LIMIT`). Prefer SPARQL for graph patterns — [SQL reference](sql-reference.md). Query Workbench also has **DL** mode for Manchester class expressions — [DL Query](guides/dl-query.md).

!!! tip "CLI on macOS/Windows?"
    Most IDE users never need the CLI — the extension bundles `ontocore-lsp`. If you need `ontocore` for CI or scripting, see [Install](install.md).

> **Names:** **OntoCode** = VS Code extension. **OntoCore** = Rust engine (CLI + language server). **Ontologos** = external reasoner.

</details>

</div>

## Choose your path

<div class="grid cards" markdown>

-   :material-microsoft-visual-studio-code:{ .lg .middle } **VS Code extension**

    ---

    Browse, edit Turtle / OBO / RDF/XML / OWL/XML, run queries and the reasoner from the OntoCode activity bar.

    [:octicons-arrow-right-24: First success tutorial](guides/first-success.md)

-   :material-console:{ .lg .middle } **Rust & CLI**

    ---

    `cargo install ontocore-cli`, embed `ontocore` / `ontocore-*` crates, validate and classify in CI.

    [:octicons-arrow-right-24: Install](install.md)

-   :material-clipboard-check-outline:{ .lg .middle } **Evaluate adoption**

    ---

    Capability matrix, Protégé comparison, production readiness, and known limits.

    [:octicons-arrow-right-24: What ships today](SHIPPED.md)

</div>

## What ships today

**Latest tagged: v0.25.0.** Full capability matrix: **[What ships today](SHIPPED.md)**. For channel lag (Marketplace vs crates.io vs docs), see [Versions & channels](guides/versions-and-channels.md).

## Quick start

=== "VS Code"

    1. Install the **OntoCode extension** from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (for Cursor)
    2. **File → Open Folder…** with **`.ttl` / `.obo` / `.owl` / `.rdf` / `.owx`** (editable) — JSON-LD / TriG / N-Triples are browse/query only ([Supported formats](supported-formats.md))
    3. OntoCode’s **bundled** language server indexes in Restricted Mode — **Trust** only if you set custom `ontocode.lspPath` or `ontocode.robotPath`
    4. Open the **OntoCode** activity bar → browse **Classes** → click an entity

=== "CLI (Linux x64)"

    Prefer the [release tarball](https://github.com/eddiethedean/ontocode/releases/tag/v0.25.0). Full download → extract → validate steps: [CI integration](ci-integration.md).

=== "CLI (macOS / Windows — cargo)"

    Requires Rust **1.88+**. Windows: [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). macOS: Xcode Command Line Tools (`xcode-select --install`). First compile: **15–30+ minutes**. See [Install CLI](guides/install-cli.md).

    ```bash
    cargo install ontocore-cli --locked --version 0.25.0
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
| Protégé comparison | [Protégé vs OntoCode](guides/protege-decision.md) |
| CLI / CI | [Install](install.md) · [CI integration](ci-integration.md) |
| Embed in Rust | [Rust library guide](guides/rust-library.md) |
| Roadmap (pick the right doc) | [Roadmap hub](roadmap-hub.md) |
| Feature tour | [ontocode/feature-tour.md](ontocode/feature-tour.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) |
| Contributing | [contributing.md](contributing.md) |

Release notes: [CHANGELOG on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)
