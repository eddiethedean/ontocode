# CI integration

Run OntoIndex validation in continuous integration to catch ontology lint and parse errors before merge.

## Exit codes

`ontoindex validate` follows the rules in [workspace-limits.md](workspace-limits.md):

| Outcome | Exit code |
|---------|-----------|
| No diagnostic **errors** (warnings/info allowed) | **0** |
| One or more diagnostic **errors** | **non-zero** |

Warnings and info are printed to stderr but do not fail the job.

## GitHub Actions (cargo install)

```yaml
name: Ontology validation

on:
  push:
    branches: [main]
  pull_request:

jobs:
  validate-ontologies:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install ontoindex CLI
        run: cargo install ontoindex-cli --locked

      - name: Validate ontology files
        run: ontoindex validate .
```

Adjust the path (`.` or `ontologies/`) to the directory containing your `.ttl`, `.owl`, etc.

## GitHub Actions (release binary)

For faster CI without compiling Rust dependencies:

```yaml
      - name: Download ontoindex CLI
        run: |
          VERSION=0.4.0
          curl -fsSL -o ontoindex.tar.gz \
            "https://github.com/eddiethedean/ontocode/releases/download/v${VERSION}/ontoindex-linux-x64.tar.gz"
          tar xzf ontoindex.tar.gz
          chmod +x ontoindex

      - name: Validate ontology files
        run: ./ontoindex validate .
```

Verify checksums per [release-integrity.md](release-integrity.md) in production pipelines.

## Query diagnostics in CI

```bash
ontoindex query . "SELECT code, severity, message, file FROM diagnostics WHERE severity = 'error'"
```

Use `--format json` for machine-readable output.

## Patch automation

Apply Turtle patches in CI (preview first):

```bash
ontoindex patch ./ontology.ttl patches.json --preview
ontoindex patch ./ontology.ttl patches.json
ontoindex validate .
```

Patch format: [patch-reference.md](patch-reference.md).

## Tips

- Index only the ontology subtree if the repo is large: `ontoindex validate ./src/ontologies`
- Resource limits apply — see [workspace-limits.md](workspace-limits.md)
- For VS Code-only workflows, the same rules apply via `ontoindex validate` in CI; the extension is not required in the pipeline
