# Query cookbook

Runnable examples against an ontology workspace. Replace `/path/to/ontologies` with your project folder.

```bash
ontoindex query /path/to/ontologies "SELECT * FROM classes"
ontoindex query /path/to/ontologies "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
```

From a git clone, use `fixtures` instead of `/path/to/ontologies`, or `cargo run --` from the repo root.

## Classes and entities

```bash
ontoindex query /path/to/ontologies "SELECT * FROM classes"
ontoindex query /path/to/ontologies "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontoindex query /path/to/ontologies "SELECT * FROM individuals"
ontoindex query /path/to/ontologies "SELECT * FROM entities"
```

## Properties

```bash
ontoindex query /path/to/ontologies "SELECT * FROM object_properties"
ontoindex query /path/to/ontologies "SELECT * FROM data_properties"
ontoindex query /path/to/ontologies "SELECT * FROM properties"
```

## Annotations and axioms

```bash
ontoindex query /path/to/ontologies "SELECT * FROM annotations"
ontoindex query /path/to/ontologies "SELECT * FROM axioms"
```

## Ontology metadata

```bash
ontoindex query /path/to/ontologies "SELECT * FROM ontologies"
ontoindex query /path/to/ontologies "SELECT * FROM namespaces"
ontoindex query /path/to/ontologies "SELECT * FROM imports"
```

## Diagnostics and validation

```bash
ontoindex query /path/to/ontologies "SELECT code, severity, message, file FROM diagnostics"
ontoindex validate /path/to/ontologies
```

## SPARQL

```bash
ontoindex sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
ontoindex sparql /path/to/ontologies "PREFIX ex: <http://example.org/> SELECT ?label WHERE { ex:Person rdfs:label ?label }"
```

## Export formats

```bash
ontoindex query /path/to/ontologies "SELECT * FROM classes" --format json
ontoindex query /path/to/ontologies "SELECT * FROM classes" --format csv
```

## CI validation

```bash
ontoindex validate /path/to/ontologies   # exit 0 when no diagnostic errors
```

Full column reference: [sql-reference.md](../sql-reference.md). SPARQL: [sparql-reference.md](../sparql-reference.md). Errors: [errors.md](../errors.md).
