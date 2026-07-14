# Patch reference (OntoCore v0.21)

> **Status:** Documents behavior in **OntoCore v0.21.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

Patch write-back uses a JSON array of patch operations. The CLI (`ontocore patch`) and LSP (`ontocore/applyAxiomPatch`) accept the same envelope; operation sets differ by file extension.

Supported formats: **Turtle (`.ttl`)**, **OBO (`.obo`)**, **RDF/XML (`.owl`/`.rdf`)**, **OWL/XML (`.owx`)**. XML uses full-document re-serialize ([ADR-0021](design/adr/0021-deterministic-xml-serializers.md)). See [Supported formats](supported-formats.md).

**Apply path (v0.20):** inbound patch JSON is wrapped as an `ontocore_edit::Transaction` and applied through format adapters (`TurtleAdapter` / `OboAdapter`) before the existing `apply_patches_to_text` engines run. Legacy patch arrays remain accepted; an optional forward envelope `{ "transaction": { "changes": [...] } }` is also supported.

**Source of truth:** [`patch.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-owl/src/patch.rs)

## Format

- JSON **array** of operation objects
- Each object has an `"op"` field (snake_case)
- Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`) documents (dispatch by extension)

## Turtle operations

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `create_entity` | `entity_iri`, `kind` | Create class, property, or individual |
| `delete_entity` | `entity_iri` | Remove entity block from file |
| `set_label` | `entity_iri`, `value` | Replace all `rdfs:label` values with one |
| `add_label` | `entity_iri`, `value` | Add a label |
| `remove_label` | `entity_iri`, `value` | Remove a matching label |
| `set_comment` | `entity_iri`, `value` | Replace all `rdfs:comment` values |
| `add_comment` | `entity_iri`, `value` | Add a comment |
| `remove_comment` | `entity_iri`, `value` | Remove a matching comment |
| `add_sub_class_of` | `entity_iri`, `parent_iri` | Add `rdfs:subClassOf` parent (named class IRI) |
| `remove_sub_class_of` | `entity_iri`, `parent_iri` | Remove a `subClassOf` axiom |
| `add_complex_sub_class_of` | `entity_iri`, `manchester` | Add complex `SubClassOf` from Manchester expression |
| `remove_complex_sub_class_of` | `entity_iri`, `manchester` | Remove complex `SubClassOf` matching Manchester text |
| `add_equivalent_class` | `entity_iri`, `manchester` | Add `owl:equivalentClass` from Manchester expression |
| `remove_equivalent_class` | `entity_iri`, `manchester` | Remove equivalent class axiom |
| `set_equivalent_class` | `entity_iri`, `manchester` | Replace equivalent class axioms with one expression |
| `add_disjoint_class` | `entity_iri`, `other_iri` | Add `owl:disjointWith` to another named class |
| `remove_disjoint_class` | `entity_iri`, `other_iri` | Remove a `disjointWith` axiom |
| `set_deprecated` | `entity_iri`, `value` | Set `owl:deprecated` (`true` or `false`) |
| `add_import` | `ontology_iri`, `import_iri` | Add `owl:imports` to ontology header |
| `remove_import` | `ontology_iri`, `import_iri` | Remove matching `owl:imports` triple |
| `add_domain` | `entity_iri`, `class_iri` | Add `rdfs:domain` for object/data property |
| `remove_domain` | `entity_iri`, `class_iri` | Remove matching domain axiom |
| `add_range` | `entity_iri`, `range_iri` | Add `rdfs:range` (class or datatype IRI) |
| `remove_range` | `entity_iri`, `range_iri` | Remove matching range axiom |
| `set_functional` | `entity_iri`, `value` | Toggle `owl:FunctionalProperty` |
| `set_inverse_functional` | `entity_iri`, `value` | Toggle `owl:InverseFunctionalProperty` |
| `set_transitive` | `entity_iri`, `value` | Toggle `owl:TransitiveProperty` |
| `set_symmetric` | `entity_iri`, `value` | Toggle `owl:SymmetricProperty` |
| `set_asymmetric` | `entity_iri`, `value` | Toggle `owl:AsymmetricProperty` |
| `set_reflexive` | `entity_iri`, `value` | Toggle `owl:ReflexiveProperty` |
| `set_irreflexive` | `entity_iri`, `value` | Toggle `owl:IrreflexiveProperty` |
| `add_property_chain` | `entity_iri`, `properties` | Add `owl:propertyChainAxiom` (ordered IRI array) |
| `remove_property_chain` | `entity_iri`, `properties` | Remove matching property chain |
| `add_class_assertion` | `entity_iri`, `class_iri` | Add individual `rdf:type` |
| `remove_class_assertion` | `entity_iri`, `class_iri` | Remove class assertion |
| `add_object_property_assertion` | `entity_iri`, `property_iri`, `target_iri` | Add object property assertion |
| `remove_object_property_assertion` | `entity_iri`, `property_iri`, `target_iri` | Remove object property assertion |
| `add_data_property_assertion` | `entity_iri`, `property_iri`, `value` | Add data property assertion with literal |
| `remove_data_property_assertion` | `entity_iri`, `property_iri`, `value` | Remove data property assertion |
| `add_annotation` | `entity_iri`, `predicate`, `value` | Add generic annotation assertion |
| `remove_annotation` | `entity_iri`, `predicate`, `value` | Remove generic annotation assertion |

## OBO operations (`.obo`)

See [ADR-0019](design/adr/0019-obo-write-back.md). Ops use `term_id` (OBO id, e.g. `GO:0008150`).

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `set_name` | `term_id`, `value` | Set term name |
| `add_synonym` | `term_id`, `value`, `scope` | Add synonym (`exact`, `related`, …) |
| `remove_synonym` | `term_id`, `value` | Remove synonym |
| `add_def` | `term_id`, `value` | Add definition |
| `remove_def` | `term_id` | Remove definition |
| `add_xref` | `term_id`, `xref` | Add xref |
| `remove_xref` | `term_id`, `xref` | Remove xref |
| `set_namespace` | `term_id`, `namespace` | Set namespace |
| `set_deprecated` | `term_id`, `value` | Set `is_obsolete` |
| `add_is_a` | `term_id`, `parent_id` | Add `is_a` parent |
| `remove_is_a` | `term_id`, `parent_id` | Remove `is_a` parent |

## RDF/XML and OWL/XML operations (`.owl` / `.rdf` / `.owx`)

Same Turtle-shaped `PatchOp` JSON applies via Horned load → mutate → full-document re-serialize (v0.21+). First-cut supported ops: create/delete entity, labels/comments/annotations, SubClassOf add/remove, imports, ontology/version IRI, class assertions. Unsupported ops return structured errors and leave the file unchanged. Details: [OWL/XML write-back](guides/owl-xml-workflow.md).

### `kind` values for `create_entity`

`class`, `object_property`, `data_property`, `annotation_property`, `individual`

## Examples

### Create a class with label and parent

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

### Edit labels and comments

```json
[
  {
    "op": "set_label",
    "entity_iri": "http://example.org/people#Person",
    "value": "Human being"
  },
  {
    "op": "add_comment",
    "entity_iri": "http://example.org/people#Person",
    "value": "A human person."
  }
]
```

### Create an object property

```json
[
  {
    "op": "create_entity",
    "entity_iri": "http://example.org/people#knows",
    "kind": "object_property"
  },
  {
    "op": "add_label",
    "entity_iri": "http://example.org/people#knows",
    "value": "knows"
  }
]
```

### Mark deprecated and delete

```json
[
  {
    "op": "set_deprecated",
    "entity_iri": "http://example.org/people#LegacyClass",
    "value": true
  }
]
```

```json
[
  {
    "op": "delete_entity",
    "entity_iri": "http://example.org/people#LegacyClass"
  }
]
```

### Complex subclass (Manchester)

```json
[
  {
    "op": "add_complex_sub_class_of",
    "entity_iri": "http://example.org/clinic#Patient",
    "manchester": "ex:hasRecord some ex:MedicalRecord"
  }
]
```

### Equivalent class (Manchester)

```json
[
  {
    "op": "set_equivalent_class",
    "entity_iri": "http://example.org/clinic#Staff",
    "manchester": "ex:Employee"
  }
]
```

### Disjoint classes

```json
[
  {
    "op": "add_disjoint_class",
    "entity_iri": "http://example.org/org#Cat",
    "other_iri": "http://example.org/org#Dog"
  }
]
```

## CLI usage

```bash
# Preview changes (stdout JSON, no write)
ontocore patch ./ontology.ttl patches.json --preview

# Apply patches
ontocore patch ./ontology.ttl patches.json

# Validate after apply
ontocore validate .
```

### Response shape (`ApplyPatchResult`)

```json
{
  "applied": true,
  "preview_text": "...",
  "diagnostics": [],
  "document_path": "/path/to/ontology.ttl"
}
```

- `applied`: `true` when changes were written (false for `--preview` or on error)
- `preview_text`: resulting Turtle text when content changed
- `diagnostics`: patch errors (non-empty means apply failed)

## LSP usage

Method: `ontocore/applyAxiomPatch`

```json
{
  "document_uri": "file:///path/to/ontology.ttl",
  "patches": [
    { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
  ],
  "preview_only": false
}
```

See [lsp-api.md](lsp-api.md) and [authoring.md](authoring.md).

## Limitations (v0.14)

- Write-back: **Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), OWL/XML (`.owx`)**; JSON-LD and line-oriented RDF are read-only. XML is semantic re-serialize — [OWL/XML write-back](guides/owl-xml-workflow.md)
- Simple `add_sub_class_of` parent must be a **named class IRI**; use Manchester ops (`add_complex_sub_class_of`, `add_equivalent_class`, etc.) for class expressions
- Manchester: `SubClassOf`, `EquivalentClasses`, and `DisjointClasses`; property chains editable via patch ops and inspector (v0.13)
- Patch engine uses targeted text edits; unusual formatting may need manual review
