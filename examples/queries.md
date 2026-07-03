# Query cookbook

Runnable against the `fixtures/` directory. Use `cargo run --` from the repo root or `ontocore` if installed.

## Classes and entities

```bash
ontocore query fixtures "SELECT * FROM classes"
ontocore query fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontocore query fixtures "SELECT * FROM individuals"
ontocore query fixtures "SELECT * FROM entities"
```

## Properties

```bash
ontocore query fixtures "SELECT * FROM object_properties"
ontocore query fixtures "SELECT * FROM data_properties"
ontocore query fixtures "SELECT * FROM properties"
```

## Annotations and axioms

```bash
ontocore query fixtures "SELECT * FROM annotations"
ontocore query fixtures "SELECT * FROM axioms"
```

## Ontology metadata

```bash
ontocore query fixtures "SELECT * FROM ontologies"
ontocore query fixtures "SELECT * FROM namespaces"
ontocore query fixtures "SELECT * FROM imports"
```

## Diagnostics and validation (v0.3+)

```bash
ontocore query fixtures "SELECT code, severity, message, file FROM diagnostics"
ontocore validate fixtures
```

## SPARQL

```bash
ontocore sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
ontocore sparql fixtures "PREFIX ex: <http://example.org/> SELECT ?label WHERE { ex:Person rdfs:label ?label }"
```

## Export formats

```bash
ontocore query fixtures "SELECT * FROM classes" --format json
ontocore query fixtures "SELECT * FROM classes" --format csv
```

## CI validation

```bash
ontocore validate fixtures   # exit 0 on success
ontocore validate .          # validate current directory
```

Full column reference: [docs/sql-reference.md](../docs/sql-reference.md). SPARQL: [docs/sparql-reference.md](../docs/sparql-reference.md). Errors: [docs/errors.md](../docs/errors.md).
