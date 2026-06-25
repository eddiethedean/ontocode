# CLI reference (OntoIndex v0.7)

The `ontoindex` binary indexes ontology workspaces and exposes query, validation, patch, and reasoning commands.

Install:

```bash
cargo install ontoindex-cli --locked
```

From a git clone, use `cargo run --` instead of `ontoindex`.

## Global output formats

Several commands accept `--format text|json|csv` (where noted). Default is `text`.

## Commands

### `index`

Scan and index ontology files in a workspace.

```bash
ontoindex index [workspace]
ontoindex index ./ontologies --format json
```

Default workspace: `.` (current directory).

### `inspect`

Print catalog statistics (same data as `index`, human-oriented output by default).

```bash
ontoindex inspect fixtures
ontoindex inspect /path/to/ontologies --format json
```

### `query`

Run a SQL-like query against virtual tables. See [SQL reference](sql-reference.md).

```bash
ontoindex query fixtures "SELECT short_name, labels FROM classes"
ontoindex query . "SELECT code, message FROM diagnostics WHERE severity = 'error'" --format json
```

**Exit:** 0 on success; non-zero on parse/unsupported SQL/I/O errors.

### `sparql`

Run SPARQL against indexed triples. See [SPARQL reference](sparql-reference.md).

```bash
ontoindex sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

**Exit:** 0 on success; non-zero on failure. Results truncate at 100,000 rows.

### `validate`

Index workspace and fail if diagnostic **errors** exist (warnings allowed).

```bash
ontoindex validate .
ontoindex validate /path/to/ontologies
```

**Exit:** 0 when no errors; non-zero otherwise. Stable for CI — [ci-integration.md](ci-integration.md).

### `patch`

Apply Turtle patch operations from a JSON file. See [patch reference](patch-reference.md).

```bash
ontoindex patch ./ontology.ttl patches.json --preview
ontoindex patch ./ontology.ttl patches.json
```

**Exit:** 0 on success; non-zero on invalid patch or unsupported format.

### `robot`

Run [ROBOT](http://robot.obolibrary.org/) CLI subcommands. Requires Java and `robot` on `PATH` (or `--robot-path`). See [ROBOT interop guide](guides/robot-interop.md).

```bash
ontoindex robot validate path/to/ontology.owl
ontoindex robot merge --inputs a.owl --inputs b.owl --output merged.owl
ontoindex robot report path/to/ontology.owl --report report.tsv
```

| Subcommand | Description |
|------------|-------------|
| `validate` | `robot validate` on a file |
| `merge` | Merge multiple ontology files |
| `report` | Generate a ROBOT report |

Optional `--robot-path` overrides the `robot` executable (same as VS Code `ontocode.robotPath`).

**Exit:** matches ROBOT exit code (0 on success).

### `classify`

Run OWL classification via OntoLogos 0.9.0.

```bash
ontoindex classify ./ontologies --profile el
ontoindex classify . --profile rl --format json
ontoindex classify . --profile el --no-auto-profile
```

| Flag | Default | Description |
|------|---------|-------------|
| `--profile` | `el` | `el`, `rl`, `rdfs` (`dl`/`auto` error until OntoLogos 1.0) |
| `--auto-profile` | `true` | Emit profile-detection warnings |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 when consistent (no unsatisfiable classes); non-zero on unsatisfiable classes or reasoner error.

JSON includes: `profile_used`, `consistent`, `unsatisfiable`, `warnings`, `duration_ms`, inferred hierarchy fields.

See [Reasoner guide](guides/reasoner.md) and [workspace-limits.md](workspace-limits.md).

### `explain`

Explain unsatisfiability for a class IRI (requires OntoLogos explain support).

```bash
ontoindex explain ./ontologies --class 'http://example.org#Invalid' --profile el
ontoindex explain . --class 'http://example.org#Invalid' --format json
```

| Flag | Default | Description |
|------|---------|-------------|
| `--class` | *(required)* | Class IRI |
| `--profile` | `el` | Reasoner profile |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 when explanation produced; non-zero if class not found or explanation unavailable.

## Related

- [Getting started](getting-started.md)
- [CI integration](ci-integration.md)
- [Errors reference](errors.md)
- [What ships today](SHIPPED.md)
