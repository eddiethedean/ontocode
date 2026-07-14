# Plugin policy (pre-1.0)

OntoCore ships a **plugin host MVP** (manifests, subprocess plugins, reference plugins). This page states the **product policy** — not how to author a plugin. For authoring, see [Plugin authoring](plugins.md).

## Support stance

| Topic | Policy |
|-------|--------|
| Commercial plugin support | **Not offered** — community / GitHub issues |
| Marketplace / app store | **None** — no curated marketplace SLA |
| Compatibility promise | Host and manifest schema may change between **0.x** minors; pin OntoCore/OntoCode together |
| Security review | Third-party plugins run as **subprocesses** with declared permissions — treat untrusted plugins as untrusted code |
| Stability target | Semver-stable plugin ecosystem API is a **v1.0** goal — [API stability](api-stability.md) |

## What is supported today

- Built-in and reference plugins shipped with the repo
- Local manifests discovered by CLI / LSP when enabled
- Documented MVP APIs in [Plugin authoring](plugins.md)

## What is not promised

- Binary compatibility across OntoCore minors without rebuilds
- Guaranteed availability of community plugins
- Parity with Protégé’s plugin marketplace

## Enterprise guidance

Treat plugins as **internal tooling**: pin versions, review manifests, and keep a rollback path that disables third-party plugins. See [Production readiness](production-readiness.md) and [Security](../security.md).

## Related

- [Plugin authoring](plugins.md)
- [API stability](api-stability.md)
- [What ships today](../SHIPPED.md)
