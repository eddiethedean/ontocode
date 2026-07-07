# CI integration

Run OntoCore validation in continuous integration to catch ontology lint and parse errors before merge.

## Exit codes

`ontocore validate` follows the rules in [workspace-limits.md](workspace-limits.md):

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

      - name: Install ontocore CLI
        run: cargo install ontocore-cli --locked --version 0.12.0

      - name: Validate ontology files
        run: ontocore validate .
```

Adjust the path (`.` or `ontologies/`) to the directory containing your `.ttl`, `.owl`, etc.

!!! tip "Pin the CLI version pre-1.0"
    Use `--version 0.12.0` with `--locked` so CI does not pick up breaking minor releases unexpectedly. See [FAQ](faq.md).

## Classify in CI

Fail the job when EL classification finds unsatisfiable classes:

```yaml
      - name: Install ontocore CLI
        run: cargo install ontocore-cli --locked --version 0.12.0

      - name: Classify ontologies (EL)
        run: ontocore classify . --profile el --format json
```

`classify` exits **non-zero** when `consistent` is false. See [workspace-limits.md](workspace-limits.md) and [Reasoner guide](guides/reasoner.md).

## GitHub Actions (release binary)

For faster CI without compiling Rust dependencies:

```yaml
      - name: Download and validate ontology files
        run: |
          VERSION=0.12.0
          ASSET="ontocore-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
          BIN="ontocore-v${VERSION}-x86_64-unknown-linux-gnu"
          curl -fsSL -o "${ASSET}" \
            "https://github.com/eddiethedean/ontocode/releases/download/v${VERSION}/${ASSET}"
          tar xzf "${ASSET}"
          chmod +x "${BIN}"
          ./"${BIN}" validate .
```

Verify checksums per [release-integrity.md](release-integrity.md) in production pipelines.

## Query diagnostics in CI

```bash
ontocore query . "SELECT code, severity, message, file FROM diagnostics WHERE severity = 'error'"
```

Use `--format json` for machine-readable output.

## Patch automation

Apply Turtle patches in CI (preview first):

```bash
ontocore patch ./ontology.ttl patches.json --preview
ontocore patch ./ontology.ttl patches.json
ontocore validate .
```

Patch format: [patch-reference.md](patch-reference.md).

## Semantic diff in CI

Compare git refs in pull requests (requires a git checkout with history):

```yaml
      - name: Install ontocore CLI
        run: cargo install ontocore-cli --locked --version 0.12.0

      - name: Semantic diff (breaking changes only)
        run: ontocore diff --format markdown --breaking-only HEAD..WORKTREE
```

See [Semantic diff guide](ontocode/semantic-diff.md) for ref syntax and VS Code panel usage.

## Tips

- Index only the ontology subtree if the repo is large: `ontocore validate ./src/ontologies`
- Resource limits apply — see [workspace-limits.md](workspace-limits.md)
- For VS Code-only workflows, the same rules apply via `ontocore validate` in CI; the extension is not required in the pipeline
