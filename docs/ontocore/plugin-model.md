# OntoCore plugin model

> **Status:** Design target for v1.0. Not shipped in v0.9.

The plugin system will allow users and organizations to extend **OntoCore** and **OntoCode** without modifying the core project.

Full specification: [PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md).

## Plugin types (planned)

| Type | Purpose | Examples |
|------|---------|----------|
| Validator | Custom diagnostics | Naming conventions, required labels |
| Exporter | Output formats | Markdown, HTML, CSV, SHACL shapes |
| Reasoner | Native reasoner integration | Custom Rust/WASM reasoners ([ADR-0014](../design/adr/0014-rust-native-reasoners-only.md)) |
| Query function | SQL extensions | `descendants(iri)`, `ontology_depth(iri)` |
| UI | VS Code views | Custom inspectors (OntoCode layer) |

Built-in reasoner adapters (`el`, `rl`, `rdfs`, `dl`, `auto`) ship in `ontoindex-reasoner` as thin wrappers over [OntoLogos](https://github.com/eddiethedean/ontologos).

## OntoCore vs OntoCode plugins

| Layer | Plugin scope |
|-------|--------------|
| **OntoCore** | Validators, exporters, reasoners, query functions — run in CLI/LSP/Rust library |
| **OntoCode** | UI plugins — VS Code views, webview panels |

## Manifest (sketch)

```toml
[plugin]
id = "com.example.ontology-rules"
name = "Acme Ontology Rules"
version = "1.0.0"
api_version = "1"
kind = "validator"
entry = "libacme_rules.so"
```

## Timeline

- **v0.11:** Plugin/extension point design, MCP server design
- **v1.0:** Stable plugin API + reference plugins

See [OntoCore roadmap](roadmap.md).
