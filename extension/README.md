# OntoCode

[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)
[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)

**Ontology IDE for VS Code** — browse, edit Turtle/OBO/RDF/XML/OWL/XML, query, reason (EL–DL), validate, and diff OWL/RDF/OBO ontologies.

**Current release: v0.23.0**

## Start here

1. Install from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor).
2. Follow **[First success (~10 min)](https://ontocode-vs.readthedocs.io/en/latest/guides/first-success/)** on Read the Docs.

## Docs

| Topic | Link |
|-------|------|
| What ships today | [SHIPPED](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/) |
| Known limitations | [Known limitations](https://ontocode-vs.readthedocs.io/en/latest/known-limitations/) |
| Install options (VSIX, offline) | [Install VS Code](https://ontocode-vs.readthedocs.io/en/latest/vscode-install/) |
| CLI / CI / Rust crates | [Install CLI & CI](https://ontocode-vs.readthedocs.io/en/latest/getting-started/) |
| Full documentation | [Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/) |
| Extension overview | [VS Code extension docs](https://ontocode-vs.readthedocs.io/en/latest/ontocode/vscode-extension/) |

> **Editable today:** Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`). XML is semantic re-serialize (not Protégé byte-identical). JSON-LD / TriG / N-Triples remain read-only.

> **Names:** **OntoCode** = this extension. **OntoCore** = Rust engine (`ontocore-cli`, `ontocore-lsp`). Install the CLI with `cargo install ontocore-cli`, not `ontocode`.

## Features (summary)

Explorer, Entity Inspector, Query Workbench (SQL subset + SPARQL), Manchester editor, graphs, reasoner (EL–DL, realize / instance check), SWRL Rule Browser/Editor, semantic diff, Manage Imports, refactoring preview, plugin host MVP.

Details: [Feature tour](https://ontocode-vs.readthedocs.io/en/latest/ontocode/feature-tour/) · [Supported formats](https://ontocode-vs.readthedocs.io/en/latest/supported-formats/)

## Development

See [Contributing](https://github.com/eddiethedean/ontocode/blob/main/CONTRIBUTING.md) and [Extension development](https://ontocode-vs.readthedocs.io/en/latest/guides/extension-development/).

License: MIT (extension). OntoCore engine: MIT OR Apache-2.0.
