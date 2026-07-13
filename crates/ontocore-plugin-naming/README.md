# ontocore-plugin-naming

Reference **naming / label** validator for the OntoCore plugin host.

| | |
|--|--|
| Plugin id | `ontocode.naming-validator` |
| Kind | validator |
| Permission | `workspace.read` |

## What it does

Emits diagnostics when classes/properties are missing `rdfs:label` or IRIs fail a configured prefix check. Invoked during `ontocore validate` / workspace index when the plugin is listed under `.ontocore/plugins/*.toml`.

## Try it

1. Copy a TOML manifest from [examples/plugin-workspace](https://github.com/eddiethedean/ontocode/tree/main/examples/plugin-workspace).
2. Run `ontocore validate .` or open the folder in OntoCode.

## Authoring docs

Canonical contract: [Plugin authoring](https://ontocode-vs.readthedocs.io/en/latest/guides/plugins/). Do **not** implement against historical trait specs or the future OntoUI TypeScript Plugin API.

## License

MIT OR Apache-2.0
