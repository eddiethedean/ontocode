# Migrating from Protégé — first week

This guide helps ontology teams adopt OntoCode **v0.11** alongside or instead of [Protégé](https://protege.stanford.edu/). For a capability comparison, see [Protégé vs OntoCode](protege-decision.md) and [What ships today](../SHIPPED.md).

## Before you start

**OntoCode fits well when you:**

- Store ontologies in Git (Turtle, OWL, OBO)
- Want VS Code editing, CI validation, and semantic diff
- Can edit **Turtle (`.ttl`)** for write-back (other formats are read-only in the inspector)

**Keep Protégé (for now) when you need:**

- Full **OBO write-back** in the IDE
- **Property chain editing** or a full DL axiom catalog UI
- Desktop-only workflows with no Git/CI requirement

Many teams use **both**: Protégé for heavy axiom authoring, OntoCode for Git-native browse, lint, diff, and CI. See [Protégé coexistence](protege-coexistence.md).

## Day 1 — Install and open your project

1. Install [OntoCode from the Marketplace](../vscode-install.md) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode).
2. **File → Open Folder…** and select your ontology repository (not a single file).
3. **Trust** the workspace when prompted.
4. Open the **OntoCode** activity bar and confirm **Ontologies** lists your `.ttl` / `.owl` files.
5. Expand **Classes** and click an entity to open the **Entity Inspector**.

Follow the [first success core path](../guides/first-success.md) if anything is empty after trust + index.

## Day 2 — Map Protégé habits to OntoCode

| In Protégé | In OntoCode v0.11 |
|------------|-------------------|
| Class hierarchy tab | **Classes** explorer view; toggle **asserted / inferred / combined** after reasoner |
| Entity editor (labels, parents) | **Entity Inspector** edit section (`.ttl` only) |
| DL query tab | **Query Workbench** — SQL catalog tables or SPARQL |
| Reasoner (HermiT, etc.) | **OntoCode: Run Reasoner** — EL/RL/RDFS/DL/auto via OntoLogos 1.0 |
| Active ontology | All workspace folders indexed (multi-root supported) |
| Refactor / move axioms | **Rename Entity IRI**, **Migrate Namespace**, **Move Entity**, **Extract Module** |
| Diff between versions | **Semantic Diff** panel or `ontocore diff` in CI |

## Day 3 — Validate in CI

Add a pipeline gate so Protégé-only mistakes are caught before merge:

```yaml
- run: cargo install ontocore-cli --locked --version 0.11.0
- run: ontocore validate ./src/ontologies
```

Optional: fail on unsatisfiable classes:

```yaml
- run: ontocore classify . --profile el --format json
```

Full examples: [CI integration](../ci-integration.md).

## Day 4 — Queries and diagnostics

1. Open **Query Workbench** → run `SELECT short_name, labels FROM classes`.
2. Inspect lint issues in **Diagnostics** and the **Problems** panel.
3. Export query results (CSV/JSON) for reporting.

SQL subset limits: [SQL reference](../sql-reference.md). SPARQL: [SPARQL reference](../sparql-reference.md).

## Day 5 — Reasoning and hierarchy

1. Run **OntoCode: Run Reasoner** with profile `el` (or `auto` for DL ontologies).
2. Review unsatisfiable classes in **Reasoner Results**.
3. Set **Hierarchy Mode** to **inferred** or **combined** to see inferred parents in the explorer.

Guide: [Reasoner](reasoner.md). Profile selection: [FAQ](../faq.md).

## Week 1 checkpoint

By end of week one you should be able to:

- [ ] Browse and edit Turtle entities in VS Code
- [ ] Run `ontocore validate` (or classify) in CI
- [ ] Compare a branch with **Semantic Diff** or `ontocore diff`
- [ ] Document which tasks stay in Protégé vs OntoCode for your team

## Common friction points

| Issue | Resolution |
|-------|------------|
| Cannot edit OWL/XML in inspector | Convert module to Turtle for write-back, or edit in Protégé |
| SQL query fails | OntoCore SQL is single-table subset — use SPARQL for graph patterns |
| Reasoner slow or fails on DL | Check [workspace limits](../workspace-limits.md); try `el` profile first |
| Team expects plugin ecosystem | Plugin host is **v1.0 target** — not installable in v0.11 |

## Next steps

| Goal | Document |
|------|----------|
| Enterprise evaluation pack | [Enterprise evaluation](enterprise-eval.md) |
| Split workflow with Protégé | [Protégé coexistence](protege-coexistence.md) |
| Semantic diff for releases | [Semantic diff](../ontocode/semantic-diff.md) |
| OBO / ROBOT pipelines | [OBO workflow](obo-workflow.md) · [ROBOT interop](robot-interop.md) |
