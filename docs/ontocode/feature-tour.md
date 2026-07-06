# OntoCode feature tour

A visual and structural overview of the OntoCode VS Code IDE (**v0.11**). For hands-on setup, start with [First success (~10 min core path)](../guides/first-success.md).

Canonical capability list: [What ships today](../SHIPPED.md). **New in v0.11:** [Migration guide](../migration/v0.11.md).

## Activity bar and explorer

The **OntoCode** activity bar hosts five tree views:

| View | Purpose |
|------|---------|
| **Ontologies** | Indexed files, formats, parse status |
| **Classes** | Class hierarchy (asserted, or inferred/combined after reasoner) |
| **Properties** | Object, data, and annotation properties |
| **Individuals** | Named individuals |
| **Diagnostics** | Lint summaries grouped by severity |

![OntoCode explorer and Entity Inspector](../media/explorer-preview.png)

*Explorer sidebar (Ontologies, Classes, Properties, Individuals, Diagnostics) and the React Entity Inspector for `ex:Person` from `example.ttl`.*

**Typical flow:** expand **Classes** → click an entity name → **Entity Inspector** opens on the right.

## Entity Inspector (React)

The inspector shows IRI, kind, labels, comments, parents, children, and axioms. For **`.ttl`** files, the **Edit** section supports:

- Add or change labels and comments
- Add named parent classes
- Delete entity
- **Edit in Manchester** / **Add Manchester axiom** for complex expressions

Other formats (RDF/XML, JSON-LD, `.obo`) are **read-only** in the inspector — index and browse only.

Guide: [Inspector](inspector.md) · [Authoring](../authoring.md)

## Query Workbench (React)

Command Palette → **OntoCode: Open Query Workbench**

- **SQL mode** — catalog virtual tables (`classes`, `properties`, `diagnostics`, …)
- **SPARQL mode** — graph patterns over indexed triples
- Export results to CSV or JSON; history and saved queries

Guide: [Query Workbench](query-workbench.md)

## Manchester editor (React)

Opened from the inspector or Command Palette for complex `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` axioms. Validates Manchester syntax, previews Turtle, then applies to the `.ttl` file.

Guide: [Manchester editor](manchester-editor.md)

## Graph panels (React)

| Command | Graph |
|---------|-------|
| **Open Class Graph** | Subclass neighborhood around a class |
| **Open Property Graph** | Property domain/range neighborhood |
| **Open Import Graph** | Ontology import dependencies |
| **Open Neighborhood Graph** | Mixed entity neighborhood |

Click nodes to jump back to the Entity Inspector.

Guide: [Graph view](graph-view.md)

## Reasoner and explanation

| Panel | Purpose |
|-------|---------|
| **Reasoner Results** | Profile used, consistency, unsatisfiable classes, warnings |
| **Explanation** | EL-first justification for unsatisfiable classes (after reasoner run) |

After classification, use **Set Hierarchy Mode** (`asserted` / `inferred` / `combined`) to update the **Classes** tree.

Guide: [Reasoner](../guides/reasoner.md)

## Refactor preview and semantic diff (React)

| Panel | Purpose |
|-------|---------|
| **Refactor Preview** | Diff before rename, migrate, move, or extract module |
| **Semantic Diff** | Compare versions, directories, or workspace snapshots — axiom-level changes and breaking-change flags |

Guides: [Refactoring](../guides/refactoring.md) · [Semantic diff](semantic-diff.md)

## Manage Imports (v0.11)

Right-click a `.ttl` file in **Ontologies** → **Manage Imports** to add or remove `owl:imports` declarations with preview and apply.

Guide: [Manage Imports](manage-imports.md)

## Turtle completion and quick fixes (v0.11)

In `.ttl` editors:

- **Completion** on `:`, `<`, `@` — prefixes, QNames, catalog IRIs
- **Quick fixes** (lightbulb) for undefined prefix, missing label, and broken import diagnostics

Guide: [Authoring](../authoring.md) · [LSP API](../lsp-api.md)

## Editor integration

Open any supported ontology file (`.ttl`, `.owl`, `.obo`, …) for:

- Hover on IRIs
- Go to definition (`F12`)
- Document outline (`Ctrl+Shift+O` / `Cmd+Shift+O`)
- Workspace symbol search (`Ctrl+T` / `Cmd+T`)
- Inline diagnostics in the **Problems** panel

## Settings worth knowing

| Setting | Default | Notes |
|---------|---------|-------|
| `ontocode.lspPath` | empty | Override bundled `ontocore-lsp` (trusted workspaces only) |
| `ontocode.hierarchy.mode` | `asserted` | Switch after reasoner for inferred tree |
| `ontocode.reasoner.default` | `el` | Default profile for **Run Reasoner** |
| `ontocode.indexCache` | `false` | Optional `.ontocore/cache/` disk index |

Full list: [Install VS Code](../vscode-install.md#settings)

## Next steps

| Goal | Document |
|------|----------|
| Complete the tutorial | [First success](../guides/first-success.md) |
| From Protégé | [Migrating from Protégé](../guides/protege-migration.md) |
| CLI / CI without VS Code | [OntoCore overview](../ontocore/index.md) |
