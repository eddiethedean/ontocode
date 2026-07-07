# Internals (design targets)

> **These pages describe target architecture and contributor specs — not necessarily what ships in the current release.**
>
> For product capabilities, use **[What ships today](SHIPPED.md)**. For a 10-minute tutorial, use **[First success](guides/first-success.md)**.

Design documents, ADRs, and backlogs live under [design/](design/README.md). **Platform implementation architecture:** [platform/OVERVIEW.md](platform/OVERVIEW.md). Product UX specs: [ui/README.md](ui/README.md) — mapped to releases in [ROADMAP_MAPPING.md](ui/ROADMAP_MAPPING.md). **Product ADRs:** [adr/README.md](adr/README.md). **Cursor prompts:** [cursor-prompts/README.md](cursor-prompts/README.md). Always cross-check [SHIPPED.md](SHIPPED.md). See also [Roadmap hub](roadmap-hub.md) and [Glossary](glossary.md).

## Start here for contributors

- [Documentation index](documentation-index.md)
- [Glossary](glossary.md)
- [Platform architecture](platform/OVERVIEW.md)
- [Contributing guide](contributing.md)
- [Debugging guide](debugging.md)
- [Cursor implementation prompts](cursor-prompts/README.md)
- [Design overview](design/README.md)
- [Product design / UI specs](ui/README.md)
- [UI roadmap mapping](ui/ROADMAP_MAPPING.md)
- [OntoCore implementation architecture](design/ARCHITECTURE.md)
- [Product ADRs](adr/README.md)
- [Engineering ADRs](design/adr/README.md)
- [Releasing](releasing.md)

## Plugin model

The plugin host is a **v1.0 design** and is **not installable in the current release (v0.13)**. See [plugin model](ontocore/plugin-model.md) and [PLUGIN_SPEC.md](design/PLUGIN_SPEC.md).
