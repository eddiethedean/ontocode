# OntoCode VS Code Extension

Browse ontology repositories in VS Code using the OntoIndex language server.

## Features (v0.2)

- Ontology explorer sidebar (ontologies, classes, properties, individuals)
- Entity inspector with IRI, labels, comments, parents, children, and axioms
- Jump to source in Turtle/RDF files
- Workspace indexing on open
- Hover, document symbols, workspace symbols, and go-to-definition for ontology files

## Development

From the repository root:

```bash
./scripts/package-extension.sh
```

Then in VS Code: **Run Extension** (F5) with the `extension/` folder, or install the generated `.vsix`.

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `ontocode.lspPath` | `""` | Path to `ontoindex-lsp` binary |
| `ontocode.autoIndexOnOpen` | `true` | Index workspace when the extension activates |

## Commands

- **OntoCode: Index Workspace** — rebuild the ontology catalog
- **OntoCode: Refresh Explorer** — refresh tree views from the catalog
- **OntoCode: Show Entity Inspector** — open inspector for an IRI
- **OntoCode: Jump to Source** — navigate to entity declaration
