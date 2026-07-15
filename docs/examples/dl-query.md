# DL Query cookbook

Manchester class-expression queries via CLI (Query Workbench **DL** mode and LSP `ontocore/dlQuery` share the same engine). Replace `fixtures` with your ontology root.

Honesty and limits: [DL Query guide](../guides/dl-query.md).

## Named class (inferred)

```bash
ontocore dl-query fixtures 'ex:ClinicPerson' \
  --prefix 'ex=http://example.org/clinic#' \
  --profile rl \
  --mode inferred

ontocore dl-query fixtures 'ex:ClinicPerson' \
  --prefix 'ex=http://example.org/clinic#' \
  --format json
```

## Asserted instances

```bash
ontocore dl-query fixtures 'ex:Person' \
  --prefix 'ex=http://example.org#' \
  --mode asserted \
  --format json
```

## Anonymous expression

```bash
ontocore dl-query fixtures 'ex:Person and ex:hasRole some ex:Clinician' \
  --prefix 'ex=http://example.org/clinic#' \
  --profile dl
```

## CI tip

Pin CLI with `--version 0.24.0` (or the release tarball for Linux x64). Treat results as OntoLogos-backed — dual-check critical audits against Protégé when required.
