# DL Query cookbook

Manchester class-expression queries via CLI (Query Workbench **DL** mode and LSP `ontocore/dlQuery` share the same engine).

Honesty and limits: [DL Query guide](../guides/dl-query.md).

**Usage:** `ontocore dl-query <expression> [--workspace PATH]` — expression is the only positional; workspace defaults to `.` via `--workspace`.

Expressions accept **prefix:local** QNames (resolved from indexed ontology prefixes) or angle-bracket IRIs `<http://…>`. There is **no** `--prefix` CLI flag. Prefer `<…>` when several files bind the same short prefix (for example both tutorial fixtures use `ex:`).

Samples use the repo [`fixtures/`](https://github.com/eddiethedean/ontocode/tree/v0.26.2/fixtures) corpus. Flags: `--workspace`, `--profile`, `--mode`, `--format` only — [CLI reference](../cli-reference.md).

## Named class (inferred)

```bash
ontocore dl-query 'ex:ClinicPerson' --workspace fixtures \
  --profile rl \
  --mode inferred

ontocore dl-query '<http://example.org/clinic#ClinicPerson>' --workspace fixtures \
  --format json
```

(`ex:` comes from [`complex-classes.ttl`](https://github.com/eddiethedean/ontocode/blob/v0.26.2/fixtures/complex-classes.ttl).)

## Asserted instances

```bash
ontocore dl-query '<http://example.org/people#Person>' --workspace fixtures \
  --mode asserted \
  --format json
```

Use the absolute IRI here so `Person` resolves to the people ontology even when other fixtures also bind `ex:`.

## Anonymous expression

```bash
ontocore dl-query \
  'ex:Patient and ex:hasRecord some ex:MedicalRecord' \
  --workspace fixtures \
  --profile dl
```

## CI tip

Pin CLI with `--version 0.26.2` (or the release tarball for Linux x64). Treat results as OntoLogos-backed — dual-check critical audits against Protégé when required.
