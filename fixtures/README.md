# Fixtures

Sample ontology files for tests, examples, and local experimentation.

| File | Format | Purpose |
|------|--------|---------|
| `example.ttl` | Turtle | Primary fixture — classes, properties, individuals |
| `organization.owl` | RDF/XML | Second file format; imports people ontology |

Try:

```bash
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT * FROM classes"
```

More queries: [examples/queries.md](../examples/queries.md).
