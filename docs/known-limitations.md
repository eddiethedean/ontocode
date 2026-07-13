# Known limitations (v0.18)

Honest limits for evaluators and new users. Capability matrix: [What ships today](SHIPPED.md). Desktop parity gate: [v0.18 assessment on GitHub](https://github.com/eddiethedean/ontocode/blob/main/docs/PROTEGE_REVERSE_ENGINEERING/ONTOCODE_PARITY/ONTOCODE_0.18_PROTEGE_PARITY_ASSESSMENT.md).

## Editable formats

| Can edit (write-back) | Index / browse / query only |
|-----------------------|-----------------------------|
| Turtle (`.ttl`) | OWL/XML (`.owl`, `.owx`) |
| OBO (`.obo`) | RDF/XML (`.rdf`, `.xml`), JSON-LD, N-Triples, N-Quads, TriG |

Entity Inspector and patch write-back apply to **`.ttl` and `.obo` only**. Opening a typical Protégé `.owl` corpus works for browse and query; convert or dual-maintain Turtle for editing. See [Supported formats](supported-formats.md) and [OWL/XML workflow](guides/owl-xml-workflow.md).

## Catalog SQL (subset)

`ontocore query` and Query Workbench **SQL mode** are **not** full SQL. Supported: single-table `SELECT`, limited `WHERE` (`=`, `!=`, `AND`, `OR`, booleans). **No** `JOIN`, `GROUP BY`, `ORDER BY`, or `LIMIT`. Prefer [SPARQL](sparql-reference.md) for graph patterns. Details: [SQL reference](sql-reference.md).

## CLI binaries

| Platform | Prebuilt CLI tarball | Recommended install |
|----------|----------------------|---------------------|
| Linux x64 | Yes (GitHub Releases) | Tarball or `cargo install ontocore-cli --locked` |
| macOS | No | `cargo install ontocore-cli --locked` (Rust **1.88+**; first build 15–30+ min) |
| Windows | No | `cargo install ontocore-cli --locked` |

Interactive editing does **not** need the CLI — use the [VS Code / Cursor extension](vscode-install.md) (bundled language server on all platforms).

## Plugins and owlmake

Plugin host **MVP shipped** (manifest, permissions, views, preferences, context actions). A **stable, semver-guaranteed plugin API** and full production [owlmake](https://github.com/INCATools/owlmake) integration are **v1.0** targets. See [Plugin authoring](guides/plugins.md) and [API stability](guides/api-stability.md).

## API stability (pre-1.0)

Published crates are **0.19.x**. Library APIs, LSP JSON, and SQL table columns may change between minor releases until v1.0. Pin in CI: `cargo install ontocore-cli --locked --version 0.19.0`.

## Reasoning

EL / RL / RDFS / DL classification ships via **Ontologos**. Explanations are **EL-first**; DL clash traces are partial. Start / Synchronize / Classify / Consistency are distinct client workflows; **Stop** cancels the in-flight client request and ignores late results (the server may still finish CPU-bound classify). See [Reasoner guide](guides/reasoner.md).

## Layout persistence

Webview **tabs** survive VS Code reload. Restored tabs offer **Reopen panel** using the last saved command + context. Full Protégé-style dock/layout serialization remains a **v1.0** IDE-shell item. Named perspectives open a fixed panel set.

## Large ontologies

Graphs may be **truncated** (badge in the Graph panel). Prefer narrower search, lower neighborhood depth, or asserted-only mode. See [workspace limits](workspace-limits.md).

## When not to use OntoCode today

- You need **in-place OWL/XML or RDF/XML write-back** — use Turtle/OBO or Protégé.
- You need **full SQL analytics** — use SPARQL or an external store.
- You need a **stable plugin marketplace API** without scaffolding — wait for v1.0 or keep Protégé plugins.
- You need **WebProtégé collaboration** — out of scope until post-1.0.

More: [Start here](start.md) · [Protégé migration](guides/protege-migration.md) · [Protégé decision](guides/protege-decision.md) · [FAQ](faq.md)
