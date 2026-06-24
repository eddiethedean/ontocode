# Ontology authoring (v0.4)

OntoCode v0.4 adds **Turtle write-back** for simple ontology edits without Protégé.

## Supported operations

| Operation | LSP / patch `op` |
|-----------|------------------|
| Create class, property, individual | `create_entity` |
| Delete entity | `delete_entity` |
| Add / set / remove label | `add_label`, `set_label`, `remove_label` |
| Add / set / remove comment | `add_comment`, `set_comment`, `remove_comment` |
| Add / remove parent class | `add_sub_class_of`, `remove_sub_class_of` |
| Set deprecated flag | `set_deprecated` |

Full JSON reference: [patch-reference.md](patch-reference.md).

## Format policy

- **Write-back:** Turtle (`.ttl`) only in v0.4
- **Read/index:** all supported RDF/OWL formats (unchanged)
- Non-Turtle files are read-only in the explorer inspector (`editable: false`)

## VS Code workflow

1. Open a `.ttl` ontology and select an entity in the OntoCode explorer.
2. Use the **Entity Inspector** edit section: add labels, comments, parents, or delete.
3. Use context menu **Create Class/Property/Individual** on explorer views.
4. Changes apply via `ontoindex/applyAxiomPatch` and trigger a workspace reindex.
5. VS Code undo works on saved file changes.

### Example: add a label in the inspector

1. Open `fixtures/example.ttl` (or your own `.ttl` file).
2. In **OntoCode → Classes**, click `Person`.
3. In the inspector edit section, add or change a label and save.
4. Confirm the change in the Turtle file and run **OntoCode: Index Workspace** if the tree does not refresh.

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

## Limitations (v0.4)

- No Manchester editor (v0.5)
- `SubClassOf` parent must be a **named class IRI** (no restrictions)
- Patch engine uses targeted text edits; complex hand-formatted Turtle may need manual review
