# FAQ

Common questions about OntoCode and OntoCore. For step-by-step fixes, see [Troubleshooting](troubleshooting.md).

## Naming and products

**What is the difference between OntoCode and OntoCore?**

- **OntoCode** — VS Code IDE (explorer, inspector, Query Workbench, Manchester editor, diagnostics).
- **OntoCore** — Rust semantic workspace engine (`ontocore` crate, `ontocore-*` implementation, `ontocore` CLI, `ontocore-lsp`).

OntoCore was previously branded **OntoIndex** (`ontoindex` CLI, `ontoindex-*` crates). As of v0.9 there is no compatibility alias — see [v0.9 migration](migration/v0.9.md). This repository contains both OntoCode and OntoCore.

**Is the API stable?**

Pre-1.0. Published crates are at **0.14.x**. Library APIs, LSP JSON, and SQL table columns may change between minor releases until v1.0. Pin versions in CI with `cargo install ontocore-cli --locked --version 0.14.0`. See [API stability](guides/api-stability.md). Upgrading from **0.13.x**? See [v0.14 migration](migration/v0.14.md). The `validate` and `classify` exit codes are documented in [workspace-limits.md](workspace-limits.md).

**What ships in the current release?**

See [What ships today](SHIPPED.md) for the canonical capability matrix.

## Production readiness

**Is OntoCode production-ready?**

**Pilot-ready for many OWL/OBO workflows in VS Code and CI** — not a full Protégé replacement for every profile. Use [What ships today](SHIPPED.md) for the capability matrix, [Production readiness](guides/production-readiness.md) for pilot vs production tiers, and [Protégé decision guide](guides/protege-decision.md) for gap analysis. Pin releases in CI (`--version 0.14.0`) and review [API stability](guides/api-stability.md) before embedding Rust libraries.

## Installation

**I ran `cargo install ontocore-cli` and `ontocore query ./fixtures` failed.**

The `fixtures/` directory exists only in a git clone. Use your own ontology path:

```bash
ontocore query /path/to/your/ontologies "SELECT * FROM classes"
```

See [getting-started.md](getting-started.md).

**Do I need Rust to use VS Code?**

No. Install the extension from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor), or a release VSIX. The language server is bundled.

**Can I install without cargo?**

Yes. Download release binaries and VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases). See [release-integrity.md](release-integrity.md).

## VS Code

**The explorer is empty.**

1. Run **OntoCode: Index Workspace** from the Command Palette.
2. Check **View → Output → OntoCore Language Server** for errors.
3. Confirm the folder contains supported files (`.ttl`, `.owl`, etc.).

**How do I run SQL or SPARQL in VS Code?**

Command Palette → **OntoCode: Open Query Workbench**. See [Query Workbench](ontocode/query-workbench.md).

**How do I edit complex axioms?**

Select a class in a `.ttl` file → Entity Inspector → **Edit in Manchester** or **Add Manchester axiom**. See [Manchester editor](ontocode/manchester-editor.md).

**I cannot edit in the Entity Inspector.**

Write-back applies to **Turtle (`.ttl`) and OBO (`.obo`)** files (engine v0.12; inspector write-back v0.13). RDF/XML, OWL/XML, and JSON-LD are read-only in the inspector. See [OBO authoring](ontocode/obo-authoring.md).

**How do multi-root VS Code workspaces work?**

Since **v0.10**, the language server indexes **all workspace folders** on open. If you run **OntoCode: Index Workspace** with multiple roots, VS Code may prompt you to pick a folder — automatic indexing still uses every registered root.

**`failed to start language server`**

- Trust the workspace.
- Uninstall duplicate OntoCode versions.
- Set `ontocode.lspPath` to a local `ontocore-lsp` binary (`cargo install ontocore-lsp`).
- See [vscode-install.md](vscode-install.md) and [troubleshooting.md](troubleshooting.md).

**Does `ontocode.autoIndexOnOpen` do anything?**

It is a legacy setting. Indexing is driven by the language server on workspace open.

## CLI and queries

**What SQL is supported?**

A subset: single-table `SELECT`, `FROM`, `WHERE` with `=`, `!=`, `AND`, `OR`, and boolean columns. No `JOIN`, `GROUP BY`, `ORDER BY`, or `LIMIT`. See [sql-reference.md](sql-reference.md).

**How do I run SPARQL?**

```bash
ontocore sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

See [sparql-reference.md](sparql-reference.md).

**What happens at 100,000 result rows?**

Both SQL and SPARQL **truncate** at 100,000 rows (no error). LSP responses include `truncated: true` when the cap is hit. See [workspace-limits.md](workspace-limits.md).

**What does `ontocore validate` check?**

Parse errors plus catalog lint rules: broken imports, undefined prefixes, duplicate/missing labels, orphan classes. See [sql-reference.md](sql-reference.md) (`diagnostics` table).

## Authoring and patches

**Which formats can I edit?**

Turtle (`.ttl`) and OBO (`.obo`) for write-back. RDF/XML, OWL/XML, and JSON-LD can be indexed and queried but are read-only in the inspector.

**Where is the patch JSON format documented?**

[patch-reference.md](patch-reference.md) with copy-paste examples.

## Security and licenses

**Is ontology content uploaded anywhere?**

No. OntoCore and OntoCode are local-first by default. See [security.md](security.md).

**What about LGPL (horned-owl)?**

`ontocore-owl` links horned-owl (LGPL-3.0) for Turtle axiom modeling and write-back. See [design/LICENSES.md](design/LICENSES.md).

**Can I expose ontocore-lsp on the network?**

No. The LSP has no authentication. Use stdio with a trusted editor only. See [security.md](security.md).

## Reasoning

**How do I run the reasoner in VS Code?**

Run **OntoCode: Run Reasoner** from the Command Palette, then use **OntoCode: Set Hierarchy Mode** for asserted / inferred / combined. See [Reasoner guide](guides/reasoner.md).

**How do I classify in CI?**

```bash
ontocore classify /path/to/ontologies --profile el --format json
```

Exits non-zero when unsatisfiable classes are found. See [CI integration](ci-integration.md) and [workspace-limits.md](workspace-limits.md).

**Why does `dl` or `auto` fail?**

If classification fails, check that your ontology is within [workspace limits](workspace-limits.md) and that constructs are supported by OntoLogos 1.x for the selected profile. Use `el`, `rl`, or `rdfs` for lighter-weight profiles when DL is not required.

**Why is explanation empty for a class?**

Explanations require an unsatisfiable class and a prior reasoner run (or successful `classify`). EL explanations are limited compared to full DL clash traces.

## Roadmap

**When did full DL reasoning ship?**

EL/RL/RDFS shipped in **v0.6.0** (OntoLogos 0.9.0). Full OWL 2 DL classification (`dl` / `auto`) is **available in v0.9.0+** via **OntoLogos 1.x** (HermiT parity). Explanations remain EL-first. See [Reasoner guide](guides/reasoner.md).

**How does this compare to Protégé?**

OntoCode targets OWL/OBO workflows in VS Code: browse and edit Turtle and OBO, SQL/SPARQL queries, EL–DL reasoning, refactoring, graph views, Turtle completion, diagnostic quick fixes, Manage Imports, property chain editing, and **semantic diff** (CLI, LSP, and VS Code panel). Gaps vs Protégé today include **OWL/XML write-back**, **full DL axiom catalog for all formats**, and a **plugin host** — see the [Protégé parity matrix](design/PROTEGE_PARITY.md) and [What ships today](SHIPPED.md). For a first-week adoption path, see [Migrating from Protégé](guides/protege-migration.md).

## OBO and graphs

**Can I edit `.obo` files in the inspector?**

**Yes (v0.13+).** OBO terms can be edited in the Entity Inspector and via `ontocore patch` on `.obo` files. RDF/XML and JSON-LD remain read-only. See [OBO workflow guide](guides/obo-workflow.md).

**How do I open ontology graphs?**

Use **OntoCode: Open Class Graph** (and related commands). See [Graph view](ontocode/graph-view.md).

**Does ROBOT require Java?**

Yes. `ontocore robot` and LSP `runRobot` spawn the external `robot` CLI. See [ROBOT interop guide](guides/robot-interop.md).

**Which panels use React vs legacy HTML?**

As of **v0.10**, production webview panels are React: **Entity Inspector**, **graph panels**, **Query Workbench**, **Manchester editor**, **Refactor Preview**, **Semantic Diff**, **Reasoner Results**, and **Explanation**. Legacy HTML panels were removed in v0.9.
