# Ontology authoring (v0.6)

> **Status:** Documents behavior in **OntoIndex v0.6.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

OntoCode provides **Turtle write-back** for simple and **Manchester** ontology edits without Protégé.

## Supported operations

| Operation | LSP / patch `op` |
|-----------|------------------|
| Create class, property, individual | `create_entity` |
| Delete entity | `delete_entity` |
| Add / set / remove label | `add_label`, `set_label`, `remove_label` |
| Add / set / remove comment | `add_comment`, `set_comment`, `remove_comment` |
| Add / remove parent class (named IRI) | `add_sub_class_of`, `remove_sub_class_of` |
| Add / remove complex `SubClassOf` (Manchester) | `add_complex_sub_class_of`, `remove_complex_sub_class_of` |
| Add / remove / set equivalent class (Manchester) | `add_equivalent_class`, `remove_equivalent_class`, `set_equivalent_class` |
| Set deprecated flag | `set_deprecated` |

Full JSON reference: [patch-reference.md](patch-reference.md).

## Format policy

- **Write-back:** Turtle (`.ttl`) only
- **Read/index:** all supported RDF/OWL formats (unchanged)
- Non-Turtle files are read-only in the explorer inspector (`editable: false`)

## VS Code workflow (simple edits)

1. Open a `.ttl` ontology and select an entity in the OntoCode explorer.
2. Use the **Entity Inspector** edit section: add labels, comments, parents, or delete.
3. Use context menu **Create Class/Property/Individual** on explorer views.
4. Changes apply via `ontoindex/applyAxiomPatch` and trigger a workspace reindex.
5. VS Code undo works on saved file changes.

## Manchester editor

For complex class expressions (restrictions, `and`/`or`, cardinality):

1. Select a class in a `.ttl` file.
2. In the Entity Inspector, click **Edit in Manchester** on a complex axiom row, or **Add Manchester axiom**.
3. Choose axiom type: **SubClassOf** or **EquivalentClasses**.
4. Enter a Manchester expression (e.g. `ex:hasRecord some ex:MedicalRecord`).
5. Use **Insert** pickers for classes, object properties, data properties, and XSD datatypes.
6. **Validate** shows parse diagnostics and an expression tree.
7. **Preview** shows the Turtle fragment; **Apply** writes the patch.

Manchester MVP scope: named classes, `and`/`or`, `some`/`only`, `min`/`max`/`exact` cardinality, and nesting. Disjoint axioms and property chains are deferred to v0.8.

## Query workbench

Run **OntoCode: Open Query Workbench** from the Command Palette.

- Toggle **SQL** or **SPARQL** mode
- Run queries against the indexed workspace
- Export results as CSV or JSON
- Save named queries and reload from history
- Use the virtual table dropdown for SQL table names

## CLI

### Example `patches.json`

```json
[
  {
    "op": "create_entity",
    "entity_iri": "http://example.org/people#Student",
    "kind": "class"
  },
  {
    "op": "add_label",
    "entity_iri": "http://example.org/people#Student",
    "value": "Student"
  },
  {
    "op": "add_sub_class_of",
    "entity_iri": "http://example.org/people#Student",
    "parent_iri": "http://example.org/people#Person"
  }
]
```

```bash
ontoindex patch ./people.ttl patches.json --preview
ontoindex patch ./people.ttl patches.json
ontoindex validate .
```

More examples: [patch-reference.md](patch-reference.md).

## Horned-OWL dual stack

For Turtle files, catalog **entities and axioms** come from [Horned-OWL](https://github.com/phillord/horned-owl) via `ontoindex-owl`. Oxigraph remains authoritative for triple counts and SPARQL. CI runs `owl_oxigraph_consistency` tests on fixtures.

## Unsaved buffers

When you apply a patch or Manchester edit in VS Code, the language server uses the **open editor buffer** as the source of truth (not only the file on disk). Unsaved edits in the buffer are preserved and patched in place. If reindex fails after a successful write, you may see `APPLIED_NOT_INDEXED` — run **Index Workspace**. See [errors.md](errors.md).

## Limitations

- Write-back is **Turtle only**
- Complex axioms appear in the inspector and Manchester editor but **not** as edges in the class hierarchy tree (named-parent edges only; use reasoner for inferred hierarchy)
- Manchester autocomplete uses catalog **Insert** pickers (no inline buffer autocomplete yet)
- No SQL/SPARQL editor autocomplete in the workbench (v0.8+)
