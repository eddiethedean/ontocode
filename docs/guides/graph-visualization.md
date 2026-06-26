# Graph visualization

OntoCode v0.8.0 provides **React graph panels** for exploring ontology structure. Graphs are built by OntoIndex and delivered over LSP (`ontoindex/getGraph`).

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Open a graph

From the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`):

| Command | Graph |
|---------|-------|
| **OntoCode: Open Class Graph** | Class hierarchy (`subClassOf` edges) |
| **OntoCode: Open Property Graph** | Property domain/range relationships |
| **OntoCode: Open Import Graph** | Ontology import dependencies |
| **OntoCode: Open Neighborhood Graph** | BFS neighborhood around a selected class |

For **neighborhood** graphs, select a class in the explorer first, then run **OntoCode: Open Neighborhood Graph**.

You can also run **OntoCode: Open Graph** and pick the graph kind from the quick pick.

## Using the graph panel

- **Pan and zoom** — standard React Flow controls.
- **Click a node** — opens the **Entity Inspector** for that IRI.
- **Include inferred** — when a reasoner snapshot exists, toggle inferred edges (class and neighborhood graphs).
- **Filter** — optional ontology IRI filter and hide deprecated terms.

If the graph hits server limits, the panel shows a truncation notice. Limits: **2,000 nodes** and **5,000 edges** ([workspace limits](../workspace-limits.md)).

## CLI and LSP

Integrators can call `ontoindex/getGraph` directly — see [LSP API](../lsp-api.md).

There is no standalone `ontoindex graph` CLI command; use the VS Code panel or LSP.

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Graph command disabled | Index workspace first (**OntoCode: Index Workspace**) |
| Empty graph | Confirm entities exist for the selected graph kind |
| Neighborhood graph fails | Select a class in the explorer before opening |
| Truncated graph | Narrow with ontology filter or open a smaller neighborhood depth |

More: [Troubleshooting](../troubleshooting.md) · [Webview protocol](../webview-protocol.md)
