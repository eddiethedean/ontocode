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

## CLI

```bash
# Preview patches from a JSON file
ontoindex patch ./ontology.ttl patches.json --preview

# Apply patches
ontoindex patch ./ontology.ttl patches.json
```

Patch JSON is an array of operations (see `ontoindex_owl::PatchOp`).

## Horned-OWL dual stack

For Turtle files, catalog **entities and axioms** come from [Horned-OWL](https://github.com/phillord/horned-owl) via `ontoindex-owl`. Oxigraph remains authoritative for triple counts and SPARQL. CI runs `owl_oxigraph_consistency` tests on fixtures.

## Limitations (v0.4)

- No Manchester editor (v0.5)
- `SubClassOf` parent must be a **named class IRI** (no restrictions)
- Patch engine uses targeted text edits; complex hand-formatted Turtle may need manual review
