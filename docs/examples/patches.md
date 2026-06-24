# Sample patch JSON

Copy-paste example for `ontoindex patch`. Full reference: [patch-reference.md](../patch-reference.md).

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
```

Download: [patches.json](patches.json)

## Complex axioms (v0.5)

```json
[
  {
    "op": "add_complex_sub_class_of",
    "entity_iri": "http://example.org/clinic#Patient",
    "manchester": "ex:hasRecord some ex:MedicalRecord"
  },
  {
    "op": "set_equivalent_class",
    "entity_iri": "http://example.org/clinic#Staff",
    "manchester": "ex:Employee"
  }
]
```
