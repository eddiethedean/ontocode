# FAQ

Common questions about OntoCode and OntoIndex. For step-by-step fixes, see [Troubleshooting](troubleshooting.md).

## Naming and products

**What is the difference between OntoCode and OntoIndex?**

- **OntoCode** — VS Code extension (explorer, inspector, Query Workbench, Manchester editor, diagnostics).
- **OntoIndex** — Rust engine (`ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp`).

This repository contains both.

**Is the API stable?**

Pre-1.0. Published crates are at **0.8.x**. Library APIs, LSP JSON, and SQL table columns may change between minor releases until v1.0. Pin versions in CI with `cargo install ontoindex-cli --locked --version 0.8.0`. The `validate` and `classify` exit codes are documented in [workspace-limits.md](workspace-limits.md).

**What ships in the current release?**

See [What ships today](SHIPPED.md) for the canonical capability matrix.

## Installation

**I ran `cargo install ontoindex-cli` and `ontoindex query ./fixtures` failed.**

The `fixtures/` directory exists only in a git clone. Use your own ontology path:

```bash
ontoindex query /path/to/your/ontologies "SELECT * FROM classes"
```

See [getting-started.md](getting-started.md).

**Do I need Rust to use VS Code?**

No. Install the extension from the Marketplace or a release VSIX. The language server is bundled.

**Can I install without cargo?**

Yes. Download release binaries and VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases). See [release-integrity.md](release-integrity.md).

## VS Code

**The explorer is empty.**

1. Run **OntoCode: Index Workspace** from the Command Palette.
2. Check **View → Output → OntoIndex Language Server** for errors.
3. Confirm the folder contains supported files (`.ttl`, `.owl`, etc.).

**How do I run SQL or SPARQL in VS Code?**

Command Palette → **OntoCode: Open Query Workbench**. See [Query Workbench guide](guides/query-workbench.md).

**How do I edit complex axioms?**

Select a class in a `.ttl` file → Entity Inspector → **Edit in Manchester** or **Add Manchester axiom**. See [Manchester editor guide](guides/manchester-editor.md).

**I cannot edit in the Entity Inspector.**

Write-back is **Turtle (`.ttl`) only**. RDF/XML, OWL XML, and JSON-LD files are read-only in the inspector.

**Only one folder in my multi-root workspace is indexed.**

Known limitation: only the **first** workspace folder is indexed. Open the primary ontology project as a single-root folder, or as the first folder in a multi-root workspace.

**`failed to start language server`**

- Trust the workspace.
- Uninstall duplicate OntoCode versions.
- Set `ontocode.lspPath` to a local `ontoindex-lsp` binary (`cargo install ontoindex-lsp`).
- See [vscode-install.md](vscode-install.md) and [troubleshooting.md](troubleshooting.md).

**Does `ontocode.autoIndexOnOpen` do anything?**

It is a legacy setting. Indexing is driven by the language server on workspace open.

## CLI and queries

**What SQL is supported?**

A subset: single-table `SELECT`, `FROM`, `WHERE` with `=`, `!=`, `AND`, `OR`, and boolean columns. No `JOIN`, `GROUP BY`, `ORDER BY`, or `LIMIT`. See [sql-reference.md](sql-reference.md).

**How do I run SPARQL?**

```bash
ontoindex sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

See [sparql-reference.md](sparql-reference.md).

**What happens at 100,000 result rows?**

Both SQL and SPARQL **truncate** at 100,000 rows (no error). LSP responses include `truncated: true` when the cap is hit. See [workspace-limits.md](workspace-limits.md).

**What does `ontoindex validate` check?**

Parse errors plus catalog lint rules: broken imports, undefined prefixes, duplicate/missing labels, orphan classes. See [sql-reference.md](sql-reference.md) (`diagnostics` table).

## Authoring and patches

**Which formats can I edit?**

Turtle (`.ttl`) only for write-back. All supported formats can be indexed and queried.

**Where is the patch JSON format documented?**

[patch-reference.md](patch-reference.md) with copy-paste examples.

## Security and licenses

**Is ontology content uploaded anywhere?**

No. OntoIndex and OntoCode are local-first by default. See [security.md](security.md).

**What about LGPL (horned-owl)?**

`ontoindex-owl` links horned-owl (LGPL-3.0) for Turtle axiom modeling and write-back. See [design/LICENSES.md](design/LICENSES.md).

**Can I expose ontoindex-lsp on the network?**

No. The LSP has no authentication. Use stdio with a trusted editor only. See [security.md](security.md).

## Reasoning

**How do I run the reasoner in VS Code?**

Run **OntoCode: Run Reasoner** from the Command Palette, then use **OntoCode: Set Hierarchy Mode** for asserted / inferred / combined. See [Reasoner guide](guides/reasoner.md).

**How do I classify in CI?**

```bash
ontoindex classify /path/to/ontologies --profile el --format json
```

Exits non-zero when unsatisfiable classes are found. See [CI integration](ci-integration.md) and [workspace-limits.md](workspace-limits.md).

**Why does `dl` or `auto` fail?**

Full OWL 2 DL profiles require OntoLogos 1.0. Use `el`, `rl`, or `rdfs` today.

**Why is explanation empty for a class?**

Explanations require an unsatisfiable class and a prior reasoner run (or successful `classify`). EL explanations are limited compared to full DL clash traces.

## Roadmap

**When will full DL reasoning ship?**

EL/RL/RDFS shipped in **v0.6.0** via OntoLogos 0.9.0. Full OWL 2 DL (`dl` / `auto`) requires OntoLogos 1.0. See [Reasoner guide](guides/reasoner.md) and [design/ROADMAP.md](design/ROADMAP.md).

**How does this compare to Protégé?**

v0.8 ships Git + VS Code workflows: browse, lint, Turtle editing, SQL/SPARQL queries, Manchester (subclass/equivalent/disjoint IRI), **refactoring** (rename, usages, migrate, move, extract), **EL/RL/RDFS reasoning**, **graph visualization**, **OBO index**, and **ROBOT CLI wrappers**. Full Protégé parity (DL reasoning, full OBO write-back, semantic diff) is the v1.0 goal — see [design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) and [SHIPPED.md](SHIPPED.md).

## OBO and graphs

**Can I edit `.obo` files in the inspector?**

No — OBO is indexed and syntax-highlighted, but write-back is **Turtle only**. See [OBO workflow guide](guides/obo-workflow.md).

**How do I open ontology graphs?**

Use **OntoCode: Open Class Graph** (and related commands). See [Graph visualization guide](guides/graph-visualization.md).

**Does ROBOT require Java?**

Yes. `ontoindex robot` and LSP `runRobot` spawn the external `robot` CLI. See [ROBOT interop guide](guides/robot-interop.md).

**Which panels use React vs legacy HTML?**

v0.8: **Entity Inspector**, **graph panels**, **Query Workbench**, **Manchester editor**, and **Refactor Preview** are React. Reasoner and explanation panels remain legacy HTML until v0.9.
