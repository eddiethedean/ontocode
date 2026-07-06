# Internals (design targets)

> **These pages describe target architecture and contributor specs — not necessarily what ships in the current release.**
>
> For product capabilities, use **[What ships today](SHIPPED.md)**. For a 10-minute tutorial, use **[First success](guides/first-success.md)**.

Design documents, ADRs, and backlogs live under [design/](design/README.md). They may mention features that are **planned** (plugins, SHACL, semantic diff, SDKs). Always cross-check [SHIPPED.md](SHIPPED.md).

## Start here for contributors

- [Contributing guide](contributing.md)
- [Design overview](design/README.md)
- [Implementation architecture](design/ARCHITECTURE.md)
- [ADR index](design/adr/README.md)
- [Releasing](releasing.md)

## Plugin model

The plugin host is a **v1.0 design** and is **not installable in v0.11**. See [plugin model](ontocore/plugin-model.md) and [PLUGIN_SPEC.md](design/PLUGIN_SPEC.md).
