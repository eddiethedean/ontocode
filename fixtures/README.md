# Fixtures

Sample ontology files for tests, examples, and local experimentation.

| File | Format | Purpose |
|------|--------|---------|
| `example.ttl` | Turtle | Primary fixture — classes, properties, individuals |
| `complex-classes.ttl` | Turtle | Manchester-eligible restrictions (used by [first-success tutorial](../docs/guides/first-success.md)) |
| `disjoint-classes.ttl` | Turtle | Disjoint class axioms and property chains |
| `reasoner-el.ttl` | Turtle | EL reasoner smoke fixture |
| `reasoner-unsat.ttl` | Turtle | Unsatisfiable class for explanation tests |
| `organization.owl` | RDF/XML | Second file format; imports people ontology |

Diagnostic lint fixtures live under [`tests/fixtures/diagnostics/`](../tests/fixtures/diagnostics/) (not indexed with the main fixture set).

Try:

```bash
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- docs --output /tmp/docs-out fixtures
```

More queries: [examples/queries.md](../examples/queries.md).
