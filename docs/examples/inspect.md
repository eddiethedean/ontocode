# Index vs inspect cookbook

Both commands index the workspace and print catalog statistics. Choose based on your goal.

| Command | Output | Use when |
|---------|--------|----------|
| `ontocore index` | Stats only | CI scripts, machine-readable JSON |
| `ontocore inspect` | Stats + diagnostic summary (counts + up to 10 samples) | Quick human health check |

## Index (stats only)

```bash
ontocore index /path/to/ontologies
ontocore index /path/to/ontologies --format json
```

## Inspect (stats + diagnostics)

```bash
ontocore inspect /path/to/ontologies
ontocore inspect /path/to/ontologies --format json
```

For full diagnostic listing and CI gating, use `ontocore validate`:

```bash
ontocore validate /path/to/ontologies
```

## From a git clone

```bash
cargo run -- inspect fixtures
cargo run -- index fixtures --format json
```

## Related

- [CLI reference](../cli-reference.md)
- [Production evidence protocol](../guides/production-evidence.md) — uses `inspect --format json` for sizing
