# Start here

Pick one of two documentation paths, then follow the next step for your task.

## Documentation paths

| Path | When to use | Start |
|------|-------------|-------|
| **VS Code extension** | Browse, edit Turtle, Query Workbench, reasoner panels — no Rust install | [VS Code extension docs](vscode-extension.md) |
| **Rust & CLI** | `cargo install ontocore-cli`, embed crates, CI validation | [Rust & CLI docs](rust-crates.md) |

New to OWL/RDF vocabulary? Read [Ontology concepts](../concepts.md) first.

## When not to use OntoCode (v0.9)

- You need **full OBO write-back** in the IDE (OBO is index/browse only; edit Turtle or use ROBOT).
- You rely on **multi-root** VS Code workspaces (only the **first** folder is indexed).
- You need **full SQL** (JOINs, `ORDER BY`, `LIKE`) — use SQL-like virtual tables or SPARQL instead.
- You need a **plugin host** or owlmake integration (planned v1.0; not installable yet).

Canonical matrix: [What ships today](../SHIPPED.md).

---

## VS Code extension tasks

[First success in 10 minutes](first-success.md) — install, sample ontologies, browse, edit `.ttl`.

**Then:** [Install options](../vscode-install.md) · [Authoring guide](../authoring.md) · [Query Workbench](query-workbench.md) · [Reasoner](reasoner.md)

## Rust & CLI tasks

[CLI getting started](../getting-started.md) — `cargo install ontocore-cli` or clone the repo.

**Then:** [CLI reference](../cli-reference.md) · [CI integration](../ci-integration.md) · [Rust library guide](rust-library.md)

## Shared topics

| Topic | Link |
|-------|------|
| Complex axioms (Manchester) | [Manchester editor guide](manchester-editor.md) |
| Workspace refactoring | [Refactoring guide](refactoring.md) |
| Reasoning (EL / RL / RDFS) | [Reasoner guide](reasoner.md) |
| Patch JSON | [Patch reference](../patch-reference.md) |
| Custom editor via LSP | [LSP API](../lsp-api.md) |
| Team evaluation | [Enterprise evaluation](enterprise-eval.md) |

## Help

[FAQ](../faq.md) · [Troubleshooting](../troubleshooting.md) · [Best practices](best-practices.md) · [What ships today](../SHIPPED.md)

## Full documentation map

Return to the [documentation home](../index.md#documentation-map) for the complete table of contents.
