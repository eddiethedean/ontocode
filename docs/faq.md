# FAQ

Common questions about OntoCode and OntoIndex.

## Naming and products

**What is the difference between OntoCode and OntoIndex?**

- **OntoCode** — VS Code extension (explorer UI, inspector, diagnostics, editing).
- **OntoIndex** — Rust engine (`ontoindex` CLI, `ontoindex-*` crates, `ontoindex-lsp`).

This repository contains both.

**Is the API stable?**

Pre-1.0. Published crates are at 0.4.x. Library APIs, LSP JSON, and SQL table columns may change between minor releases until v1.0. The `validate` exit code (errors fail, warnings pass) is stable — see [workspace-limits.md](workspace-limits.md).

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

**I cannot edit in the Entity Inspector.**

Write-back is **Turtle (`.ttl`) only** in v0.4. RDF/XML, OWL, and JSON-LD files are read-only in the inspector.

**Only one folder in my multi-root workspace is indexed.**

Known limitation: only the **first** workspace folder is indexed. Open the primary ontology project as a single-root folder, or as the first folder in a multi-root workspace.

**`failed to start language server`**

- Trust the workspace.
- Uninstall duplicate OntoCode versions.
- Set `ontocode.lspPath` to a local `ontoindex-lsp` binary (`cargo install ontoindex-lsp`).
- See [vscode-install.md](vscode-install.md).

**Does `ontocode.autoIndexOnOpen` do anything?**

It is a legacy setting. Indexing is driven by the language server on startup in v0.4.

## CLI and queries

**What SQL is supported?**

A subset: single-table `SELECT`, `FROM`, `WHERE` with `=`, `!=`, `AND`, `OR`, and boolean columns. No `JOIN`, `GROUP BY`, or `ORDER BY`. See [sql-reference.md](sql-reference.md).

**How do I run SPARQL?**

```bash
ontoindex sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

See [sparql-reference.md](sparql-reference.md).

**What does `ontoindex validate` check?**

Parse errors plus catalog lint rules: broken imports, undefined prefixes, duplicate/missing labels, orphan classes. See [sql-reference.md](sql-reference.md) (`diagnostics` table).

## Authoring and patches

**Which formats can I edit?**

Turtle (`.ttl`) only in v0.4. All supported formats can be indexed and queried.

**Where is the patch JSON format documented?**

[patch-reference.md](patch-reference.md) with copy-paste examples.

## Security and licenses

**Is ontology content uploaded anywhere?**

No. OntoIndex and OntoCode are local-first by default. See [security.md](security.md).

**What about LGPL (horned-owl)?**

`ontoindex-owl` links horned-owl (LGPL-3.0) for Turtle axiom modeling and write-back. See [design/LICENSES.md](design/LICENSES.md).

**Can I expose ontoindex-lsp on the network?**

No. The LSP has no authentication. Use stdio with a trusted editor only. See [security.md](security.md).

## Roadmap

**When will Manchester editing / reasoning ship?**

Manchester MVP is planned for v0.5; reasoning via OntoLogos for v0.6. See [design/ROADMAP.md](design/ROADMAP.md).

**How does this compare to Protégé?**

v0.4 targets Git + VS Code workflows with browse, lint, and simple Turtle editing. Full Protégé parity is the v1.0 goal — see [design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md).
