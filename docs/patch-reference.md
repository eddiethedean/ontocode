# Patch reference (OntoCore v0.10)

> **Status:** Documents behavior in **OntoCore v0.9.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

Turtle write-back uses a JSON array of patch operations. The CLI (`ontocore patch`) and LSP (`ontocore/applyAxiomPatch`) accept the same format.

**Source of truth:** [`patch.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-owl/src/patch.rs)

## Format

- JSON **array** of operation objects
- Each object has an `"op"` field (snake_case)
- Turtle (`.ttl`) documents only

## Operations

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

## Limitations (v0.9)

- Turtle only; RDF/XML, OWL/XML, JSON-LD are read-only
- Simple `add_sub_class_of` parent must be a **named class IRI**; use Manchester ops (`add_complex_sub_class_of`, `add_equivalent_class`, etc.) for class expressions
- Manchester: `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` — property chains are view-only in the axiom catalog
- Patch engine uses targeted text edits; unusual formatting may need manual review
