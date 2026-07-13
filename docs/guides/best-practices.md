# Best practices

Patterns for using OntoCode and OntoCore in daily ontology engineering.

## Repository layout

- Keep ontology source files in a dedicated directory (e.g. `ontologies/` or `src/ontology/`)
- Prefer **Turtle (`.ttl`) or OBO (`.obo`)** for files you edit in VS Code — those formats support write-back. RDF/XML and OWL/XML are read-only — see [Supported formats](../supported-formats.md). Refactor apply is Turtle-only today.
- Use a single-root VS Code workspace when you want the simplest onboarding (multi-root is supported since v0.10 — all folders are indexed)

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
2. Bundled language server works in Restricted Mode — **Trust** only if you set custom `ontocode.lspPath` or `ontocode.robotPath`
3. Edit in Entity Inspector or Manchester editor (`.ttl` and `.obo`)
4. Run **Index Workspace** after bulk file changes
5. Use **Query Workbench** for ad-hoc catalog queries

## Protégé teams

Use OntoCode for Turtle editing in VS Code, CI validation, and EL classification. Keep Protégé for full DL editing until v1.0 — see [Protégé coexistence](protege-coexistence.md).

## Rust embedding

Embed OntoCore in tools or pipelines via the published crates — see [Rust library guide](rust-library.md).

## Related

- [VS Code extension docs](vscode-extension.md)
- [Rust & CLI docs](rust-crates.md)
- [Enterprise evaluation](enterprise-eval.md)
- [FAQ](../faq.md)
