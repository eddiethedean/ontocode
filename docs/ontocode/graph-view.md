# Graph view

OntoCode provides **React graph panels** for exploring ontology structure. Graphs are built by **OntoCore** and delivered over LSP (`ontocore/getGraph`).

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
- **Click a node** — opens the [Entity Inspector](inspector.md) for that IRI.
- **Include inferred** — when a reasoner snapshot exists, toggle inferred edges (class and neighborhood graphs).
- **Filter** — optional ontology IRI filter and hide deprecated terms.

If the graph hits server limits, the panel shows a truncation notice. Limits: **2,000 nodes** and **5,000 edges** ([workspace limits](../workspace-limits.md)).

## Integrators

Call `ontocore/getGraph` via OntoCore LSP — see [LSP API](../lsp-api.md) and [OntoCore LSP](../ontocore/lsp.md).

There is no standalone `ontocore graph` CLI command; use the VS Code panel or LSP.

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Empty graph | Re-index workspace; check ontology has relevant axioms |
| Truncation notice | Narrow filter or open neighborhood graph from a specific class |
| Panel won't open | Check Output → OntoCode for LSP errors |

See [Graph visualization guide](../guides/graph-visualization.md) for additional notes.
