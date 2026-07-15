# Find your path (optional)

> **Most users:** go straight to **[First success (~10 min)](guides/first-success.md)**.  
> **Install (extension + optional CLI):** **[Install](install.md)**.  
> **Evaluators:** **[What ships today](SHIPPED.md)**.

This page is a **secondary chooser** kept for legacy links. Prefer the Home CTAs or the **Get started** tab.

## Which artifact?

| I want to… | Start here |
|------------|------------|
| Edit ontologies in VS Code / Cursor | [First success](guides/first-success.md) |
| Install options (extension, VSIX, CLI) | [Install](install.md) |
| Validate / query in CI | Linux x64 tarball → [CI integration](ci-integration.md); else [Install CLI & CI (detail)](install-cli-ci.md) |
| Embed in Rust | [Rust library guide](guides/rust-library.md) |
| Compare products in detail | [Which artifact](guides/which-artifact.md) |

Read [Known limitations](known-limitations.md) before a large evaluation.

## When not to use OntoCode (today)

- You need **byte-identical OWL/XML or RDF/XML layout** after save — OntoCode re-serializes (write-back ships in v0.21); see [OWL/XML write-back](guides/owl-xml-workflow.md).
- You need **full SQL** (JOINs, `ORDER BY`, `LIKE`) — use catalog SQL (subset) or SPARQL instead.
- You need a **curated plugin marketplace** or **production owlmake** without accepting the subprocess SDK — Plugin **SDK 1.0** freezes the wire today; marketplace/owlmake remain product **1.0** — [Plugin policy](guides/plugin-policy.md).

Canonical matrix: [What ships today](SHIPPED.md) · [Known limitations](known-limitations.md).

## Recommended sequence (IDE users)

1. [First success (~10 min)](guides/first-success.md)
2. [Supported formats](supported-formats.md)
3. [Query Workbench](ontocode/query-workbench.md) · [DL Query honesty](guides/dl-query.md) · [Reasoner](guides/reasoner.md)

CLI / scripting detail (not required for the IDE tutorial): [Install CLI & CI (detail)](install-cli-ci.md).
