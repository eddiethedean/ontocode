# Start here (strict path)

You can be successful with OntoCode/OntoCore in **5–15 minutes** if you follow one of the two paths below.

If you’re not sure what to install, start with [Which artifact do I need?](guides/which-artifact.md).

## Path A — VS Code IDE (recommended for most users)

Do these steps in order. Do not skip ahead.

1. **Complete the tutorial:** [First success (~10 min)](guides/first-success.md)
2. **Confirm your formats:** [Supported formats](supported-formats.md)
3. **Learn the core workflows:**
   - Editing: [Authoring guide](authoring.md)
   - Query: [Query Workbench](ontocode/query-workbench.md)
   - Reasoning: [Reasoner guide](guides/reasoner.md)
   - Imports: [Manage Imports](ontocode/manage-imports.md)

If something doesn’t work, go straight to:
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
2. [Enterprise evaluation](guides/enterprise-eval.md)
3. [Production readiness](guides/production-readiness.md)
4. [Security policy](security.md)

