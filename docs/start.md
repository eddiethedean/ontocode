# Start here

You can be successful with OntoCode/OntoCore in **5–15 minutes** if you follow one path below.

**Primary path for most users:** [First success (~10 min)](guides/first-success.md).

## Which artifact?

| I want to… | Start here |
|------------|------------|
| Edit ontologies in VS Code | [First success](guides/first-success.md) · [Install VS Code](vscode-install.md) |
| Validate / query in CI | [Getting started (CLI)](getting-started.md) |
| Embed in Rust | [Rust library guide](guides/rust-library.md) |
| Compare products | [Which artifact (detail)](guides/which-artifact.md) |

Read [Known limitations](known-limitations.md) before a large evaluation.

## When not to use OntoCode (today)

- You need **OWL/XML or RDF/XML in-place write-back** — edit as Turtle/OBO or use Protégé.
- You need **full SQL** (JOINs, `ORDER BY`, `LIKE`) — use catalog SQL (subset) or SPARQL instead.
- You need a **stable, semver-guaranteed plugin API** or production owlmake integration without subprocess scaffolding — plugin host **MVP shipped in v0.14** ([Plugin authoring](guides/plugins.md)); ecosystem hardening is **v1.0**.

Canonical matrix: [What ships today](SHIPPED.md) · [Known limitations](known-limitations.md).

## Path A — VS Code IDE (recommended for most users)

1. **Complete the tutorial:** [First success (~10 min)](guides/first-success.md)
2. **Confirm your formats:** [Supported formats](supported-formats.md)
3. **Learn the core workflows:**
   - Editing: [Authoring guide](authoring.md)
   - Query: [Query Workbench](ontocode/query-workbench.md)
   - Reasoning: [Reasoner guide](guides/reasoner.md)
   - Imports: [Manage Imports](ontocode/manage-imports.md)

**Then (optional):** [Feature tour](ontocode/feature-tour.md) · [Install options](vscode-install.md) · [Manchester editor](ontocode/manchester-editor.md) · [Refactoring](guides/refactoring.md)

If something doesn’t work:

- [Troubleshooting](troubleshooting.md)
- [FAQ](faq.md)
- [Support and contact](support.md)

## Path B — CI / CLI (recommended for automation and evaluation)

1. **Install + run your first command:** [Getting started](getting-started.md)
2. **Run a CI gate locally:**
   - `ontocore validate /path/to/ontologies`
   - Optional: `ontocore classify /path/to/ontologies --profile el --format json`
3. **Wire into CI:** [CI integration](ci-integration.md)
4. **Know your limits:** [Workspace limits](workspace-limits.md) and [Performance and sizing](guides/performance-sizing.md)
5. **Decide what’s safe to automate:** [Automation and stability](automation-stability.md)

If you plan to embed OntoCore or integrate via LSP:

- Rust: [Rust library guide](guides/rust-library.md)
- LSP: [LSP API](lsp-api.md) (and [schema](lsp-protocol.schema.json))

## If you’re evaluating adoption

Read these in order:

1. [What ships today](SHIPPED.md)
2. [Known limitations](known-limitations.md)
3. [Enterprise evaluation](guides/enterprise-eval.md)
4. [Production readiness](guides/production-readiness.md)
5. [Security policy](security.md)

## Supported since v0.10+

- **Multi-root workspaces:** All workspace folders are indexed on open. Manual **Index Workspace** may prompt you to pick a folder when multiple roots are open.
