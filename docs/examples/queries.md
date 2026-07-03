# Query cookbook

Runnable examples against an ontology workspace. Replace `/path/to/ontologies` with your project folder.

```bash
ontocore query /path/to/ontologies "SELECT * FROM classes"
ontocore query /path/to/ontologies "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
```

From a git clone, use `fixtures` instead of `/path/to/ontologies`, or `cargo run --` from the repo root.

## Classes and entities

```bash
ontocore query /path/to/ontologies "SELECT * FROM classes"
ontocore query /path/to/ontologies "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
ontocore query /path/to/ontologies "SELECT * FROM individuals"
ontocore query /path/to/ontologies "SELECT * FROM entities"
```

## Properties

```bash
ontocore query /path/to/ontologies "SELECT * FROM object_properties"
ontocore query /path/to/ontologies "SELECT * FROM data_properties"
ontocore query /path/to/ontologies "SELECT * FROM properties"
```

## Annotations and axioms

```bash
ontocore query /path/to/ontologies "SELECT * FROM annotations"
ontocore query /path/to/ontologies "SELECT * FROM axioms"
```

## Ontology metadata

```bash
ontocore query /path/to/ontologies "SELECT * FROM ontologies"
ontocore query /path/to/ontologies "SELECT * FROM namespaces"
ontocore query /path/to/ontologies "SELECT * FROM imports"
```

## Diagnostics and validation

```bash
ontocore query /path/to/ontologies "SELECT code, severity, message, file FROM diagnostics"
ontocore validate /path/to/ontologies
```

## SPARQL

```bash
ontocore sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
ontocore sparql /path/to/ontologies "PREFIX ex: <http://example.org/> SELECT ?label WHERE { ex:Person rdfs:label ?label }"
```

## Export formats

```bash
ontocore query /path/to/ontologies "SELECT * FROM classes" --format json
ontocore query /path/to/ontologies "SELECT * FROM classes" --format csv
```

## CI validation

```bash
ontocore validate /path/to/ontologies   # exit 0 when no diagnostic errors
```

Full column reference: [sql-reference.md](../sql-reference.md). SPARQL: [sparql-reference.md](../sparql-reference.md). Errors: [errors.md](../errors.md).
