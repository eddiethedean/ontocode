# Migrating from Protégé — first week

This guide helps ontology teams adopt OntoCode **v0.20** alongside or instead of [Protégé](https://protege.stanford.edu/). For a capability comparison, see [Protégé vs OntoCode](protege-decision.md) and [What ships today](../SHIPPED.md).

## Before you start

**OntoCode fits well when you:**

- Version-control ontology files (Turtle, OWL, OBO) in shared repositories
- Want VS Code editing, CI validation, and semantic diff
- Can edit **Turtle (`.ttl`)** or **OBO (`.obo`)** for write-back (RDF/XML and OWL/XML are read-only in the inspector)
- Need Protégé-like menus, reasoner workflows, explanations, graphs, and imports in the IDE

**Keep Protégé (for now) when you need:**

- **OWL/XML or RDF/XML in-place editing**
- A **full DL axiom catalog UI** for every axiom kind and format
- Protégé-specific plugins or a stable marketplace API (OntoCode plugin host is MVP; semver-stable API is **v1.0**)
- WebProtégé-style live collaboration

Many teams use **both**: Protégé for heavy axiom authoring in OWL/XML, OntoCode for browse, lint, diff, reasoning, and CI. See [Protégé coexistence](protege-coexistence.md).

## Honest desktop known gaps (v0.20 tagged)

See [Versions & channels](versions-and-channels.md) if Marketplace lags behind the GitHub Release VSIX.

| Gap | Status |
|-----|--------|
| OWL/XML · RDF/XML write-back | Planned **v0.21** — [roadmap](../roadmap.md) |
| Multi-step semantic undo | Deferred to **v1.0** |
| Full OntoGraf filter/layout suite | Partial graphs shipped; polish → **v1.0** |
| Explain all inference kinds (not only unsat) | Unsat explanations shipped |
| Mid-classify thread kill on the Rust reasoner | Client cancel + ignore late results (v0.18); server may finish CPU work |
| Stable plugin marketplace API | **v1.0** |

Full matrix: [known-limitations](../known-limitations.md) · [SHIPPED](../SHIPPED.md).

## Day 1 — Install and open your project

1. Install [OntoCode from the Marketplace](../vscode-install.md) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode).
2. **File → Open Folder…** and select your ontology repository (not a single file).
3. OntoCode’s **bundled** language server works in Restricted Mode — **Trust** only if you set custom `ontocode.lspPath` or `ontocode.robotPath`.
4. Open the **OntoCode** activity bar and confirm **Ontologies** lists your `.ttl` / `.owl` files.
5. Expand **Classes** and click an entity to open the **Entity Inspector**.

Follow the [first success core path](../guides/first-success.md) if anything is empty after trust + index.

## Day 2 — Map Protégé habits to OntoCode

| In Protégé | In OntoCode v0.20 |
|------------|-------------------|
| Class hierarchy tab | **Classes** explorer; toggle **asserted / inferred / combined** after reasoner |
| Entity editor (labels, parents) | **Entity Inspector** (`.ttl` and `.obo`) |
| Manchester syntax | **Manchester editor** panel |
| DL query tab | **Query Workbench** — SQL catalog tables or SPARQL |
| Reasoner (HermiT, etc.) | **Start / Synchronize / Classify / Consistency** — EL/RL/RDFS/DL/auto via OntoLogos; **Stop** cancels the client request |
| Explanations | **Explanation** panel (unsat; stale banner after reindex) |
| OWLViz / OntoGraf | **Class / property / import / neighborhood** graphs (asserted/inferred/combined) |
| Imports | **Manage Imports** panel |
| Preferences | OntoCode settings + plugin preference pages |
| Active ontology | **Active Ontology** selector (multi-root supported) |
| Refactor / move axioms | **Rename**, **Migrate Namespace**, **Move**, **Extract Module** (CLI); **Merge**, **Replace** (IDE only) |
| Diff between versions | **Semantic Diff** panel or `ontocore diff` in CI |
| Layout / perspectives | Named **Modeling / Reasoning / Review** perspectives; restored tabs offer **Reopen panel** |

## Day 3 — Validate in CI

```yaml
- run: cargo install ontocore-cli --locked --version 0.21.0
- run: ontocore validate ./src/ontologies
```

Optional: fail on unsatisfiable classes:

```yaml
- run: ontocore classify . --profile el --format json
```

Full examples: [CI integration](../ci-integration.md).

## Day 4 — Queries, imports, diagnostics

1. Open **Query Workbench** → run `SELECT short_name, labels FROM classes`.
2. Use **Manage Imports** to add/remove Turtle imports; **Reload Imports** after external changes.
3. Inspect lint issues in **Diagnostics** and the **Problems** panel.

SQL subset limits: [SQL reference](../sql-reference.md). SPARQL: [SPARQL reference](../sparql-reference.md).

## Day 5 — Reasoning, explanations, visualization

1. **Synchronize Reasoner** (reindex + classify) or **Classify Ontology**.
2. Review unsatisfiable classes; open **Explanation** and regenerate if the stale banner appears after edits.
3. Open **Class Graph** in asserted vs inferred mode; truncated graphs show a warning on large ontologies.
4. Set **Hierarchy Mode** to **inferred** or **combined**.

Guide: [Reasoner](reasoner.md).

## Week 1 checkpoint

- [ ] Browse and edit Turtle/OBO entities in VS Code
- [ ] Run reasoner lifecycle (classify / consistency / stop) and open an explanation
- [ ] Run `ontocore validate` (or classify) in CI
- [ ] Compare a branch with **Semantic Diff** or `ontocore diff`
- [ ] Document which tasks stay in Protégé vs OntoCode for your team

## Common friction points

| Issue | Resolution |
|-------|------------|
| Cannot edit OWL/XML in inspector | Convert module to Turtle for write-back, or edit in Protégé |
| SQL query fails | OntoCore SQL is single-table subset — use SPARQL for graph patterns |
| Reasoner slow or fails on DL | Check [workspace limits](../workspace-limits.md); try `el` profile first |
| Restored panel looks empty | Click **Reopen panel** on the recovery tab (context is reloaded from the last command) |
| Team expects stable plugin ecosystem API | Plugin host MVP shipped; stable semver API is **v1.0** — see [Plugin authoring](plugins.md) |

## Next steps

| Goal | Document |
|------|----------|
| What changed in v0.18 | [migration/v0.18.md](../migration/v0.18.md) |
| Enterprise evaluation pack | [Enterprise evaluation](enterprise-eval.md) |
| Split workflow with Protégé | [Protégé coexistence](protege-coexistence.md) |
| Semantic diff for releases | [Semantic diff](../ontocode/semantic-diff.md) |
| OBO / ROBOT pipelines | [OBO workflow](obo-workflow.md) · [ROBOT interop](robot-interop.md) |
