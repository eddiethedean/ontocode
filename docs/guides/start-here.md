# Start here

Pick the path that matches how you work. Each link is a single next step — not the full documentation map.

New to OWL/RDF vocabulary? Read [Ontology concepts](../concepts.md) first.

## Browse and edit in VS Code

[First success in 10 minutes](first-success.md) — install, download sample ontologies, browse, edit `.ttl`, optional CLI validate.

**Then:** [Install options](../vscode-install.md) · [Authoring guide](../authoring.md)

## Query and validate from the CLI

[CLI getting started](../getting-started.md) — `cargo install ontoindex-cli` or clone the repo.

**Query in VS Code:** [Query Workbench guide](../guides/query-workbench.md)

**Then automate in CI:** [CI integration](../ci-integration.md) — `ontoindex validate` in GitHub Actions.

## Complex axioms (Manchester)

[Manchester editor guide](../guides/manchester-editor.md) — edit `SubClassOf` and `EquivalentClasses` in Turtle.

## Reasoning (EL / RL / RDFS)

[Reasoner guide](../guides/reasoner.md) — classify in VS Code or CLI, toggle inferred hierarchy, open explanations.

## Automate edits (patch JSON)

[Patch reference](../patch-reference.md) — `ontoindex patch` and LSP `applyAxiomPatch` with copy-paste JSON examples.

## Integrate with another editor or tool

[LSP API](../lsp-api.md) — `ontoindex-lsp` over stdio, custom `ontoindex/*` methods.

**Rust embedding:** [Rust library guide](../guides/rust-library.md)

## Evaluate for your team

[Enterprise evaluation](enterprise-eval.md) · [Protégé coexistence](protege-coexistence.md) · [What ships today](../SHIPPED.md) · [FAQ](../faq.md) · [Troubleshooting](../troubleshooting.md) · [Workspace limits](../workspace-limits.md) · [Security](../security.md)

## Contribute to OntoCode / OntoIndex

[Contributing](../contributing.md) · [Design specs (planned)](../design/README.md) · [Releasing](../releasing.md) (maintainers)

## Common questions

[FAQ](../faq.md) — naming, `cargo install` vs clone, multi-root workspaces, LGPL (horned-owl).

[Troubleshooting](../troubleshooting.md) — LSP start failures, empty explorer, patch apply issues.

[Best practices](best-practices.md) — repo layout, SQL vs SPARQL vs classify.

## Full documentation map

Return to the [documentation home](../index.md#documentation-map) for the complete table of contents.
