# Protégé round-trip fixtures (v0.22)

This directory holds Protégé-style ontology exports used by `cargo test -p ontocode --test protege_roundtrip` / `--test owl2_authoring` and for manual coexistence checks with OntoCode.

Fixtures were introduced in v0.12 and expanded through the v0.22 OWL 2 authoring gate.

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
| `owl2-keys.ttl` | Turtle | HasKey, inverseOf, DisjointUnion |
| `owl2-abox.ttl` | Turtle | sameAs, AllDifferent, NegativePropertyAssertion |

Provenance: minimal Protégé Desktop exports shaped for OntoCode regression (not byte-identical to any single Protégé save). Semantic round-trip is verified via `ontocore_owl::compare_ontologies`, not string equality (see ADR-0021).

## Workflow

1. Open the fixture directory as a workspace in OntoCode
2. Browse entities in the Ontologies tree; inspect axioms in Entity Inspector
3. For `.ttl`, `.obo`, `.owl`/`.rdf`, and `.owx`, apply patches via inspector or `ontocore patch`
4. Run `cargo test -p ontocode --test protege_roundtrip` and `--test owl2_authoring`

## Round-trip goal

Semantic equivalence after patch + reload (entity IRIs, labels, parents, imports). XML formats use Horned full-document re-serialize — formatting is **not** byte-identical.
