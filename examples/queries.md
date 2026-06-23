# Query cookbook

Runnable against the `fixtures/` directory. Use `cargo run --` from the repo root or `ontoindex` if installed.

## Classes and entities

```bash
ontoindex query fixtures "SELECT * FROM classes"
ontoindex query fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontoindex query fixtures "SELECT * FROM individuals"
ontoindex query fixtures "SELECT * FROM entities"
```

## Properties

```bash
ontoindex query fixtures "SELECT * FROM object_properties"
ontoindex query fixtures "SELECT * FROM data_properties"
ontoindex query fixtures "SELECT * FROM properties"
```

## Annotations and axioms

```bash
ontoindex query fixtures "SELECT * FROM annotations"
ontoindex query fixtures "SELECT * FROM axioms"
```

## Ontology metadata

```bash
ontoindex query fixtures "SELECT * FROM ontologies"
ontoindex query fixtures "SELECT * FROM namespaces"
ontoindex query fixtures "SELECT * FROM imports"
```

## Diagnostics and validation (v0.3)

```bash
ontoindex query fixtures "SELECT code, severity, message, file FROM diagnostics"
ontoindex validate fixtures
```

## SPARQL

```bash
ontoindex sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
ontoindex sparql fixtures "SELECT ?label WHERE { ex:Person rdfs:label ?label }"
```

(Add Turtle prefixes in your query or use full IRIs.)

## Export formats

```bash
ontoindex query fixtures "SELECT * FROM classes" --format json
ontoindex query fixtures "SELECT * FROM classes" --format csv
```

## CI validation

```bash
ontoindex validate fixtures   # exit 0 on success
ontoindex validate .          # validate current directory
```

Full column reference: [docs/sql-reference.md](../docs/sql-reference.md).
