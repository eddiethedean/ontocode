# OntoCore SQL views

OntoCore exposes ontology data as **SQL virtual tables** over the indexed catalog. Queries run locally — no database server required.

## Quick example

```bash
ontoindex query ./ontology "SELECT short_name, labels FROM classes"
```

```rust
use ontocore::workspace::Workspace;

let ws = Workspace::open("./ontology")?;
let result = ws.query("SELECT short_name FROM classes WHERE deprecated = false")?;
```

## Virtual tables

| Table | Description |
|-------|-------------|
| `ontologies` | Indexed ontology documents |
| `classes` | OWL/RDFS classes |
| `object_properties` | OWL object properties |
| `data_properties` | OWL datatype properties |
| `annotation_properties` | OWL annotation properties |
| `individuals` | OWL named individuals |
| `entities` | All extracted entities |
| `annotations` | Label/comment and other annotation triples |
| `axioms` | Extracted axioms (e.g. SubClassOf) |
| `namespaces` | Namespace prefixes |
| `imports` | Ontology imports |
| `diagnostics` | Lint and parse diagnostics |
| `properties` | Union of all property kinds |

Full column schemas, types, and examples: **[SQL reference](../sql-reference.md)**.

SPARQL over the triple store: **[SPARQL reference](../sparql-reference.md)**.
