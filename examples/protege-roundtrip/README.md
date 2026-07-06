# Protégé round-trip fixtures (v0.12)

This directory holds Protégé-style ontology exports used by `cargo test protege_roundtrip`.

## Fixtures

| File | Format | Covers |
|------|--------|--------|
| `people.ttl` | Turtle | Labels, subclass, individuals |
| `organization.owl` | RDF/XML | Horned catalog for `.owl` |
| `example.owx` | OWL/XML | Native `.owx` Horned load |
| `properties.ttl` | Turtle | Domain, range, characteristics |
| `chains.ttl` | Turtle | Property chains |
| `individuals.ttl` | Turtle | Class/object property assertions |
| `annotations.ttl` | Turtle | Custom annotation properties |

## Workflow

1. Open the fixture directory as a workspace in OntoCode
2. Browse entities in the Ontologies tree; inspect axioms in Entity Inspector
3. For `.ttl` files, apply patches via inspector or `ontocore patch`
4. Run `cargo test protege_roundtrip` to verify indexing and patch preview

## Round-trip goal

Semantic equivalence after patch + reindex (entity IRIs and axiom sets match; Turtle formatting may differ).
