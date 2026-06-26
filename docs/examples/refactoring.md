# Refactoring cookbook

CLI examples for workspace refactoring. Replace `/path/to/ontologies` with your project folder (or use `fixtures` from a git clone).

## Find usages

```bash
ontoindex refactor usages /path/to/ontologies 'http://example.org/people#Person'
ontoindex refactor usages /path/to/ontologies 'http://example.org/people#Person' --format json
```

## Rename IRI

```bash
# Preview
ontoindex refactor rename /path/to/ontologies \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human' \
  --preview --format json

# Apply
ontoindex refactor rename /path/to/ontologies \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human'
```

## Migrate namespace

```bash
ontoindex refactor migrate-namespace /path/to/ontologies \
  --from 'http://example.org/people#' \
  --to 'http://example.org/v2/people#' \
  --preview
```

## Move entity

```bash
ontoindex refactor move /path/to/ontologies 'http://example.org/people#Student' \
  --to /path/to/ontologies/students.ttl \
  --preview
```

## Extract module

```bash
ontoindex refactor extract /path/to/ontologies \
  --entities 'http://example.org/people#Person,http://example.org/people#Student' \
  --out /path/to/ontologies/core.ttl \
  --leave-stub \
  --preview
```

## Validate after refactor

```bash
ontoindex validate /path/to/ontologies
```

User guide: [Refactoring guide](../guides/refactoring.md)
