# Manchester editor

Edit **complex class expressions** in Turtle ontologies using Manchester OWL Syntax — restrictions, intersections, unions, and cardinality.

Requires **v0.5.0** and a **`.ttl`** file.

## Open the editor

| Entry point | How |
|-------------|-----|
| Entity Inspector | Select a class → click **Edit in Manchester** on a complex axiom row |
| Add axiom | Inspector → **Add Manchester axiom** |
| Command Palette | **OntoCode: Open Manchester Editor** (with entity context from explorer) |
| Command Palette | **OntoCode: Add Manchester Axiom** |

## Workflow

1. Choose axiom type: **SubClassOf** or **EquivalentClasses**.
2. Enter a Manchester expression, e.g. `ex:hasRecord some ex:MedicalRecord`.
3. Use **Insert** pickers for classes, object properties, data properties, and XSD datatypes (from the indexed catalog).
4. **Validate** — parse diagnostics and expression tree.
5. **Preview Turtle** — see the Turtle fragment before writing.
6. **Apply** — patch the `.ttl` file and re-index.

### Edit vs add

- **Edit** (existing complex axiom): removes the prior expression and adds the new one (no duplicate axioms).
- **Add**: appends a new complex axiom to the entity block.

Simple named-parent `SubClassOf` (e.g. `ex:Person`) is edited in the inspector quick form, not the Manchester editor.

## MVP expression support

| Construct | Example |
|-----------|---------|
| Named class | `ex:Person` |
| Intersection | `ex:A and ex:B` |
| Union | `ex:A or ex:B` |
| Existential restriction | `ex:hasRecord some ex:MedicalRecord` |
| Universal restriction | `ex:hasRecord only ex:MedicalRecord` |
| Cardinality | `ex:hasPart min 2 ex:Component` |

**Not in MVP:** disjoint classes, property chains, nested property assertions. See [What ships today](../SHIPPED.md).

## CLI equivalent

Patch JSON uses the same operations as the editor:

```json
[
  {
    "op": "add_complex_sub_class_of",
    "entity_iri": "http://example.org/clinic#Patient",
    "manchester": "ex:hasRecord some ex:MedicalRecord"
  }
]
```

```bash
ontoindex patch ./ontology.ttl patches.json --preview
ontoindex patch ./ontology.ttl patches.json
```

Full reference: [Patch JSON](../patch-reference.md).

## Limitations (v0.5)

- **Turtle write-back only** — RDF/XML and other formats are read-only in the inspector.
- Complex axioms do not appear as edges in the **Classes** hierarchy tree (named parents only).
- Autocomplete uses catalog **Insert** pickers, not inline Manchester language-server assist.
- Unsaved buffer edits: patches apply to the **open VS Code buffer** first, then disk. See [Authoring](../authoring.md#unsaved-buffers).

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| No **Edit in Manchester** button | Entity needs a complex axiom in a `.ttl` file; simple named parents use the quick form |
| Validate shows errors | Check prefix declarations (`ex:`) match your ontology `@prefix` lines |
| Apply did not change file | Confirm `result.applied`; check [errors](../errors.md) for `PATCH_INVALID` or `APPLIED_NOT_INDEXED` |

Sample ontology for testing (git clone): `fixtures/complex-classes.ttl`.

More help: [Troubleshooting](../troubleshooting.md) · [Authoring](../authoring.md).
