# VS Code extension (OntoCode)

**OntoCode** is the VS Code extension: explorer sidebar, Entity Inspector, Query Workbench, Manchester editor, reasoner panels, and inline diagnostics. It talks to the bundled **`ontoindex-lsp`** language server — you do **not** need Rust installed for normal use.

> **Looking for the CLI or Rust crates?** See the [Rust & CLI path](rust-crates.md).

## Quick start

1. Install [OntoCode from the Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode).
2. Open a folder with `.ttl`, `.owl`, or other ontology files and **Trust** the workspace.
3. Open the **OntoCode** activity bar → **Classes** → click an entity.

[:octicons-arrow-right-24: First success in 10 minutes](first-success.md) — full walkthrough with sample ontologies.

## Install and setup

| Topic | Guide |
|-------|-------|
| Install, trust, bundled LSP | [Install VS Code](../vscode-install.md) |
| Supported formats, activation | [Install VS Code](../vscode-install.md) · [FAQ](../faq.md) |
| Problems after install | [Troubleshooting](../troubleshooting.md) |

## Using the extension

| Feature | Guide |
|---------|-------|
| Browse classes, properties, individuals | [First success](first-success.md) |
| Edit Turtle (labels, parents, create/delete) | [Authoring](../authoring.md) |
| SQL and SPARQL in VS Code | [Query Workbench](query-workbench.md) |
| Complex axioms (Manchester) | [Manchester editor](manchester-editor.md) |
| EL / RL / RDFS classification | [Reasoner](reasoner.md) |
| Working alongside Protégé | [Protégé coexistence](protege-coexistence.md) |
| Team evaluation | [Enterprise evaluation](enterprise-eval.md) |

## Reference

| Topic | Link |
|-------|------|
| What ships in v0.6 | [What ships today](../SHIPPED.md) |
| Patch JSON (inspector / LSP) | [Patch reference](../patch-reference.md) |
| LSP methods (advanced) | [LSP API](../lsp-api.md) |
| Workspace limits | [Workspace limits](../workspace-limits.md) |
| Errors | [Errors reference](../errors.md) |

## Help

- [FAQ](../faq.md) — multi-root workspaces, trust mode, editing limits
- [Troubleshooting](../troubleshooting.md) — LSP failed to start, empty explorer
- [Best practices](best-practices.md) — repo layout, when to use SQL vs SPARQL vs reasoner

## Related

- [Rust & CLI path](rust-crates.md) — `ontoindex` binary, `cargo install`, embedding crates
- [Documentation home](../index.md)
