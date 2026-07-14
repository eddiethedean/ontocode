# FAQ

Common questions about OntoCode and OntoCore. For step-by-step fixes, see [Troubleshooting](troubleshooting.md).

If you’re stuck or want to report a bug, see [Support and contact](support.md).

## Naming and products

**What is the difference between OntoCode and OntoCore?**

- **OntoCode** — VS Code IDE (explorer, inspector, Query Workbench, Manchester editor, diagnostics).
- **OntoCore** — Rust semantic workspace engine (`ontocore` crate, `ontocore-*` implementation, `ontocore` CLI, `ontocore-lsp`).

OntoCore was previously branded **OntoIndex** (`ontoindex` CLI, `ontoindex-*` crates). As of v0.9 there is no compatibility alias — see [v0.9 migration](migration/v0.9.md). This repository contains both OntoCode and OntoCore.

**Is the API stable?**

Pre-1.0. Published crates are at **0.24.x**. Library APIs, LSP JSON, and SQL table columns may change between minor releases until v1.0. Pin versions in CI with `cargo install ontocore-cli --locked --version 0.24.0`. See [API stability](guides/api-stability.md). Upgrading from **0.17.x**? See [v0.18 migration](migration/v0.18.md). Upgrading from **0.16.x**? See [v0.17 migration](migration/v0.17.md). Upgrading from **0.15.x**? See [v0.16 migration](migration/v0.16.md). Upgrading from **0.14.x** (or earlier)? Start at the [migration index](migration/README.md). The `validate` and `classify` exit codes are documented in [workspace-limits.md](workspace-limits.md).

**What ships in the current release?**

See [What ships today](SHIPPED.md) for the canonical capability matrix.

## Production readiness

**Is OntoCode production-ready?**

**Pilot-ready for many OWL/OBO workflows in VS Code and CI** — not a full Protégé replacement for every profile. Use [What ships today](SHIPPED.md) and [Known limitations](known-limitations.md) for the capability matrix, [Production readiness](guides/production-readiness.md) for pilot vs production tiers, and [Protégé decision guide](guides/protege-decision.md) for gap analysis. Pin releases in CI (`--version 0.24.0`) and review [API stability](guides/api-stability.md) before embedding Rust libraries.

## Installation

**Which version should I install?**

Pin to the tagged release in [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/ontocode/blob/main/docs/TAGGED_RELEASE) (currently **0.24.0**). See [Versions and channels](guides/versions-and-channels.md) for Marketplace vs GitHub Releases vs crates.io vs Read the Docs `latest`.

**Why might Marketplace lag GitHub Releases?**

Marketplace and Open VSX publishes are manual after the release workflow. Prefer the GitHub Release VSIX when you need a specific tag immediately.

**Can I edit Protégé `.owl` / RDF/XML in place?**

**Yes (v0.21+), with caveats.** RDF/XML (`.owl`/`.rdf`) and OWL/XML (`.owx`) support Entity Inspector and `ontocore patch` write-back via full-document re-serialize (semantic fidelity, not Protégé byte-identical). Prefer Turtle when you need byte-stable diffs, full Manchester, or refactor apply. Details: [Supported formats](supported-formats.md) and [OWL/XML and RDF/XML write-back](guides/owl-xml-workflow.md).

**SQL or SPARQL — which should I use?**

Use **catalog SQL** for simple tabular listing (classes, properties). Prefer **SPARQL** for graph patterns, filters, and joins. Catalog SQL is a **subset** — not full SQL.

**What is the workspace runtime (v0.20)?**

Host-owned registry for open ontologies (active target, dirty/save, semantic undo, session restore under `.ontocode/session.json`). See [migration v0.20](migration/v0.20.md).

**I ran `cargo install ontocore-cli` and `ontocore query ./fixtures` failed.**

The `fixtures/` directory exists only in a git clone. Use your own ontology path:

```bash
ontocore query /path/to/your/ontologies "SELECT * FROM classes"
```

See [getting-started.md](getting-started.md). Always pin with `--version 0.24.0` in CI so you do not surprise-upgrade.

**Do I need Rust to use VS Code?**

No. Install the extension from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode), [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor), or a release VSIX. The language server is bundled.

**Can I install without cargo?**

Yes. Download release binaries and VSIX from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases). Linux x64 CLI tarballs are published; macOS/Windows CLI uses `cargo install`. See [release-integrity.md](release-integrity.md).

## VS Code

**The explorer is empty.**

1. Run **OntoCode: Index Workspace** from the Command Palette.
2. Check **View → Output → OntoCore Language Server** for errors.
3. Confirm the folder contains supported files (`.ttl`, `.owl`, etc.).

**How do I run SQL or SPARQL in VS Code?**

Command Palette → **OntoCode: Open Query Workbench**. See [Query Workbench](ontocode/query-workbench.md).

**Does OntoCode support Protégé-style DL Query?**

**Yes (v0.24+).** Query Workbench **DL** mode runs Manchester class expressions with Instances / Subclasses / Superclasses / Equivalents tabs. Also: CLI `ontocore dl-query` and LSP `ontocore/dlQuery`. SQL and SPARQL modes remain available. See [DL Query](guides/dl-query.md).

**How do I edit complex axioms?**

Select a class in a `.ttl` file → Entity Inspector → **Edit in Manchester** or **Add Manchester axiom**. See [Manchester editor](ontocode/manchester-editor.md).

**I cannot edit in the Entity Inspector.**

Write-back applies to **Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`)**. For the full matrix (index/query vs write-back), see [Supported formats](supported-formats.md). JSON-LD and line-oriented RDF are read-only in the inspector. See [OBO authoring](ontocode/obo-authoring.md) and [OWL/XML write-back](guides/owl-xml-workflow.md).

**How do multi-root VS Code workspaces work?**

Since **v0.10**, the language server indexes **all workspace folders** on open. If you run **OntoCode: Index Workspace** with multiple roots, VS Code may prompt you to pick a folder — automatic indexing still uses every registered root.

**`failed to start language server`**

- Check **Output → OntoCore Language Server** and uninstall duplicate OntoCode versions.
- OntoCode’s **bundled** language server works in trusted and Restricted Mode. **Trust** only if you set custom `ontocode.lspPath` or `ontocode.robotPath`.
- Set `ontocode.lspPath` to a local `ontocore-lsp` binary (`cargo install ontocore-lsp`) when debugging a custom build — trusted workspaces only.
- See [vscode-install.md](vscode-install.md) and [troubleshooting.md](troubleshooting.md).

**Does `ontocode.autoIndexOnOpen` do anything?**

It is a legacy setting. Indexing is driven by the language server on workspace open.

## CLI and queries

**What SQL is supported?**

A **catalog SQL (subset)**: single-table `SELECT`, `FROM`, `WHERE` with `=`, `!=`, `AND`, `OR`, and boolean columns. No `JOIN`, `GROUP BY`, `ORDER BY`, or `LIMIT`. See [sql-reference.md](sql-reference.md) and [Known limitations](known-limitations.md).

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

Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`) for write-back. JSON-LD and line-oriented RDF can be indexed and queried but are read-only in the inspector. See [Supported formats](supported-formats.md).

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

If classification fails, check that your ontology is within [workspace limits](workspace-limits.md) and that constructs are supported by **Ontologos** 1.x for the selected profile. Use `el`, `rl`, or `rdfs` for lighter-weight profiles when DL is not required.

**Why is explanation empty for a class?**

Explanations require an unsatisfiable class and a prior reasoner run (or successful `classify`). On the **DL** profile, explanations are DL-first; EL/RL/RDFS alternatives depend on OntoLogos coverage for that profile.

## Roadmap

**When did full DL reasoning ship?**

EL/RL/RDFS shipped in **v0.6.0** (Ontologos 0.9.0). Full OWL 2 DL classification (`dl` / `auto`) is **available in v0.9.0+** via **Ontologos** 1.x. Results are **not** certified identical to Protégé + HermiT — use dual-tool checks for critical audits. ABox realization, instance checking, and SWRL authoring/validation ship in **v0.23**. See [Reasoner guide](guides/reasoner.md).

**How does this compare to Protégé?**

OntoCode targets OWL/OBO workflows in VS Code: browse and edit Turtle, OBO, RDF/XML, and OWL/XML; SQL/SPARQL queries; EL–DL reasoning; refactoring; graph views; Turtle completion; diagnostic quick fixes; Manage Imports; property chain editing; **semantic diff** (CLI, LSP, and VS Code panel); and **plugin host MVP** (manifests, reference plugins, CLI/LSP hooks — v0.14). Gaps vs Protégé today include **byte-identical XML layout**, **full DL axiom catalog for all formats**, and a **stable third-party plugin ecosystem API** (v1.0). For a decision framework see [Protégé vs OntoCode](guides/protege-decision.md); for the live capability matrix see [What ships today](SHIPPED.md) and [Known limitations](known-limitations.md). The historical v0.18 checklist under [design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) is **not current**. For a first-week adoption path, see [Migrating from Protégé](guides/protege-migration.md).

## OBO and graphs

**Can I edit `.obo` files in the inspector?**

**Yes (v0.13+).** OBO terms can be edited in the Entity Inspector and via `ontocore patch` on `.obo` files. RDF/XML and OWL/XML are also writable (v0.21+). JSON-LD and line-oriented RDF remain read-only. See [OBO workflow guide](guides/obo-workflow.md) and [Supported formats](supported-formats.md).

**How do I open ontology graphs?**

Use **OntoCode: Open Class Graph** (and related commands). See [Graph view](ontocode/graph-view.md).

**Does ROBOT require Java?**

Yes. `ontocore robot` and LSP `runRobot` spawn the external `robot` CLI. See [ROBOT interop guide](guides/robot-interop.md).

**Which panels use React vs legacy HTML?**

As of **v0.10**, production webview panels are React: **Entity Inspector**, **graph panels**, **Query Workbench**, **Manchester editor**, **Refactor Preview**, **Semantic Diff**, **Reasoner Results**, and **Explanation**. Legacy HTML panels were removed in v0.9.

## Contributing

**Do I need `cargo test --workspace` for a docs-only PR?**

No. Use the [testing matrix](guides/testing-matrix.md) and scoped PR template checkboxes. Docs-only changes need MkDocs preview (`./scripts/serve-docs.sh` or `./scripts/build-docs.sh`) — not a full Rust build. See [scripts/README.md](https://github.com/eddiethedean/ontocode/blob/main/scripts/README.md).

**When should I run `./scripts/run-ci-local.sh`?**

Before PRs that change CI scripts, release packaging, or broad Rust/extension surfaces — or when you want full Actions parity. Expect **30–60+ minutes** cold. Skip it for docs-only PRs.

**I edited React webviews but F5 still shows the old UI — why?**

`npm run watch` in `extension/` rebuilds the host only. Run `npm run build:webview` or `npm run compile` after `webview-ui/` changes — [Extension development](guides/extension-development.md).
