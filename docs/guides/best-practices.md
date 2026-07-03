# Best practices

Patterns for using OntoCode and OntoCore in daily ontology engineering.

## Repository layout

- Keep ontology source files in a dedicated directory (e.g. `ontologies/` or `src/ontology/`)
- Prefer **Turtle (`.ttl`)** for files you edit in VS Code — write-back is Turtle-only
- Use a single-root VS Code workspace for the ontology project (multi-root indexes only the **first** folder)

## When to use SQL vs SPARQL vs classify

| Tool | Best for |
|------|----------|
| **SQL** (`ontocore query`) | Catalog tables — classes, properties, diagnostics, axioms metadata |
| **SPARQL** (`ontocore sparql`) | RDF graph patterns over indexed triples |
| **`validate`** | CI gate for parse errors and lint |
| **`classify`** | CI gate for unsatisfiable classes (EL/RL/RDFS) |

SQL and SPARQL results truncate silently at 100,000 rows — narrow queries in CI. See [workspace limits](../workspace-limits.md).

## CI recipe picker

| Goal | Command |
|------|---------|
| Lint + parse | `ontocore validate .` |
| Unsatisfiable classes | `ontocore classify . --profile el --format json` |
| Inspect errors | `ontocore query . "SELECT code, message FROM diagnostics WHERE severity = 'error'"` |

Examples: [CI integration](../ci-integration.md)

## VS Code workflow

1. Open ontology folder (single-root)
2. Trust workspace
3. Edit in Entity Inspector or Manchester editor (`.ttl` only)
4. Run **Index Workspace** after bulk file changes
5. Use **Query Workbench** for ad-hoc catalog queries

## Protégé teams

Use OntoCode for Git-native Turtle editing, CI validation, and EL classification. Keep Protégé for full DL editing until v1.0 — see [Protégé coexistence](protege-coexistence.md).

## Rust embedding

Embed OntoCore in tools or pipelines via the published crates — see [Rust library guide](rust-library.md).

## Related

- [VS Code extension docs](vscode-extension.md)
- [Rust & CLI docs](rust-crates.md)
- [Enterprise evaluation](enterprise-eval.md)
- [FAQ](../faq.md)
