# Protégé round-trip fixtures (v0.21)

This directory holds Protégé-style ontology exports used by `cargo test -p ontocode --test protege_roundtrip` and for manual coexistence checks with OntoCode.

Fixtures were introduced in v0.12 and expanded through the v0.21 required-format write-back gate.

## Fixtures

| File | Format | Covers |
|------|--------|--------|
| `people.ttl` | Turtle | Labels, subclass, individuals |
| `organization.owl` | RDF/XML | Labels, imports, subclass; **editable** write-back |
| `example.owx` | OWL/XML | Native `.owx` load + label/parent write-back |
| `properties.ttl` | Turtle | Domain, range, characteristics |
| `chains.ttl` | Turtle | Property chains |
| `individuals.ttl` | Turtle | Class/object property assertions |
| `annotations.ttl` | Turtle | Custom annotation properties |

Provenance: minimal Protégé Desktop exports shaped for OntoCode regression (not byte-identical to any single Protégé save). Semantic round-trip is verified via `ontocore_owl::compare_ontologies`, not string equality (see ADR-0021).

## Workflow

1. Open the fixture directory as a workspace in OntoCode
2. Browse entities in the Ontologies tree; inspect axioms in Entity Inspector
3. For `.ttl`, `.obo`, `.owl`/`.rdf`, and `.owx`, apply patches via inspector or `ontocore patch`
4. Run `cargo test -p ontocode --test protege_roundtrip` to verify index + edit → save → reload

## Round-trip goal

Semantic equivalence after patch + reload (entity IRIs, labels, parents, imports). XML formats use Horned full-document re-serialize — formatting is **not** byte-identical.
