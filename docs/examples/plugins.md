# Plugins cookbook (SDK 1.0)

CLI recipes for discovering and running workspace plugins. Authoring: [Plugin authoring](../guides/plugins.md) · Policy: [Plugin policy](../guides/plugin-policy.md).

From a git clone, use `cargo run --` or an installed `ontocore`. Samples below use [`examples/plugin-workspace/`](https://github.com/eddiethedean/ontocode/tree/v0.26.1/examples/plugin-workspace).

## List and inspect

```bash
ontocore plugins list examples/plugin-workspace
ontocore plugins list examples/plugin-workspace --format json

ontocore plugins info org.example.demo-graph examples/plugin-workspace
ontocore plugins info org.example.demo-graph examples/plugin-workspace --format json
```

`info` prints lifecycle fields (`state`, `activation`, `enabled`, `depends_on`).

## Enable / disable

```bash
ontocore plugins disable org.example.demo-graph examples/plugin-workspace
ontocore plugins enable org.example.demo-graph examples/plugin-workspace
```

`enable` / `disable` follow activation policy and cascade dependents.

## Run actions

```bash
ontocore plugins run org.example.demo-reasoner --action reasoner.classify examples/plugin-workspace
ontocore plugins run org.example.demo-query --action query.run --query "SELECT short_name FROM classes" examples/plugin-workspace
ontocore plugins run org.example.demo-refactor --action refactor.preview --iri http://example.org/Person examples/plugin-workspace
ontocore plugins run org.example.demo-graph --action graph.build --iri http://example.org/Person examples/plugin-workspace
```

Legacy actions also work: `validate`, `export`, `workflow` (with `--step` when needed). Full flag table: [CLI reference — plugins](../cli-reference.md#plugins).

## Clone shortcut

```bash
cargo run -- plugins list examples/plugin-workspace
cargo run -- validate examples/plugin-workspace
```

## Next

| Goal | Document |
|------|----------|
| Manifests and providers | [Plugin authoring](../guides/plugins.md) |
| Compatibility promise | [Plugin policy](../guides/plugin-policy.md) |
| IDE plugin UI | [Feature tour — Plugins](../ontocode/feature-tour.md) |
