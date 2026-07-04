# OntoCode VS Code extension

**OntoCode** is the VS Code ontology IDE powered by **OntoCore**. It provides the explorer sidebar, Entity Inspector, Query Workbench, Manchester editor, graph panels, reasoner views, and inline diagnostics.

The extension talks to the bundled **OntoCore LSP** (`ontocore-lsp`) — you do **not** need Rust installed for normal use.

> **Looking for the CLI or Rust library?** See [OntoCore overview](../ontocore/index.md) and [Rust & CLI guide](../guides/rust-crates.md).

## Quick start

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode).
2. Open a folder with `.ttl`, `.owl`, or other ontology files and **Trust** the workspace.
3. Open the **OntoCode** activity bar → **Classes** → click an entity.

[:octicons-arrow-right-24: First success in 10 minutes](../guides/first-success.md)

## Install and setup

| Topic | Guide |
|-------|-------|
| Install, trust, bundled LSP | [Install VS Code](../vscode-install.md) |
| Supported formats, activation | [Install VS Code](../vscode-install.md) · [FAQ](../faq.md) |
| Problems after install | [Troubleshooting](../troubleshooting.md) |

## OntoCode features

| Feature | Guide |
|---------|-------|
| Browse classes, properties, individuals | [First success](../guides/first-success.md) |
| Entity Inspector | [Inspector](inspector.md) |
| Edit Turtle (labels, parents, create/delete) | [Authoring](../authoring.md) |
| Workspace refactoring | [Refactoring](../guides/refactoring.md) |
| SQL and SPARQL | [Query Workbench](query-workbench.md) |
| Complex axioms (Manchester) | [Manchester editor](manchester-editor.md) |
| Class/property/import graphs | [Graph view](graph-view.md) |
| Semantic diff (git / workspace) | [Semantic diff](semantic-diff.md) |
| EL / RL / RDFS / DL classification | [Reasoner](../guides/reasoner.md) |
| Working alongside Protégé | [Protégé coexistence](../guides/protege-coexistence.md) |

## Architecture

```text
OntoCode (TypeScript + React webviews)
        │ stdio LSP
OntoCore LSP (ontocore-lsp)
        │
OntoCore engine (ontocore / ontocore-*)
```

OntoCode owns UI and marketplace packaging. OntoCore owns indexing, queries, diagnostics, and write-back logic.

## Reference

| Topic | Link |
|-------|------|
| What ships today | [SHIPPED](../SHIPPED.md) |
| Webview protocol | [Webview protocol](../webview-protocol.md) |
| LSP API (OntoCore) | [OntoCore LSP](../ontocore/lsp.md) |
