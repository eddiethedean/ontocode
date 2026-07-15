# Protégé-ported fixtures (v0.26)

Behavioral **equivalents** of Protégé Desktop `protege-editor-owl` test
resources under
[protegeproject/protege](https://github.com/protegeproject/protege)
(`src/test/resources/ontologies/...`). These are **synthetic** OntoCode
fixtures shaped to the same scenarios (loops, multi-parent trees,
versioned ontology IRIs, minimal idranges)—not byte copies of Protégé
files.

License for this directory: MIT OR Apache-2.0 (OntoCode). Behavioral
inspiration from Protégé test designs; Protégé itself is BSD-2-Clause.

| File | Scenario |
|------|----------|
| `simple_loop.ttl` | Cyclic `rdfs:subClassOf` (AssertedClassHierarchy loop case) |
| `two_parents.ttl` | Class with two asserted parents |
| `versioned_ontology.ttl` | Ontology IRI + version IRI |
| `versioned_ontology.owl` | Same identity in RDF/XML |
| `idranges_minimal.ttl` | Minimal GO-style idranges annotations |
| `tabbed_hierarchy.txt` | Indented hierarchy input for parser → SubClassOf |
| `merge_labels.ttl` | Merge with rdfs:label on source |
| `obofoundry_minimal.json` | Truncated OBO Foundry registry JSON (Wave 3) |
| `imports_home/` | Multi-file imports for axiom `ontology_id` location |

Used by `cargo test -p ontocode --test protege_port_*`.
Copy into a tempfile workspace before mutating; never edit committed files in place.
