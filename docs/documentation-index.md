# Documentation index

Master index for OntoCode / OntoCore planning, architecture, and user documentation.

**Current release:** v0.17.0 · **Canonical capabilities:** [What ships today](SHIPPED.md) · **Terms:** [Glossary](glossary.md)

## Recommended reading order

### Evaluators and adopters

1. [What ships today](SHIPPED.md)
2. [Glossary](glossary.md)
3. [Roadmap hub](roadmap-hub.md)
4. [First success (~10 min)](guides/first-success.md)

### Implementers (Shipped v0.17 platform)

1. [Glossary](glossary.md)
2. [Platform overview](platform/OVERVIEW.md)
3. [OntoUI architecture](platform/ONTOUI.md)
4. [Workspace runtime](platform/WORKSPACE_RUNTIME.md)
5. [Plugin authoring](guides/plugins.md)
6. [Cursor implementation prompts](cursor-prompts/README.md)
7. [UI roadmap mapping](ui/ROADMAP_MAPPING.md)

### UX designers

1. [UI spec index](ui/README.md)
2. UX specs under [ui/README.md](ui/README.md) — wireframes, HIG, accessibility
3. Architecture defers to [docs/platform/](platform/OVERVIEW.md)

---

## Major planning documents

| Document | Purpose | Audience | Priority | Status | Owner |
|----------|---------|----------|----------|--------|-------|
| [SHIPPED.md](SHIPPED.md) | Canonical capability matrix | All | P0 | Shipped | Platform |
| [glossary.md](glossary.md) | Canonical terminology | All | P0 | Active | Platform |
| [roadmap-hub.md](roadmap-hub.md) | Which roadmap doc to read | All | P0 | Active | Platform |
| [roadmap.md](roadmap.md) | Platform release plan | All | P1 | Active | Platform |
| [platform/OVERVIEW.md](platform/OVERVIEW.md) | Implementation architecture hub | Architect, implementer | P0 | Active | Platform |
| [platform/ONTOUI.md](platform/ONTOUI.md) | Shared React UI platform | Implementer | P0 | Shipped v0.14 | OntoUI |
| [platform/WORKSPACE_RUNTIME.md](platform/WORKSPACE_RUNTIME.md) | WorkspaceStore, event bus, hosts | Implementer | P0 | Shipped v0.14 | OntoUI |
| [platform/CAPABILITY_PROVIDERS.md](platform/CAPABILITY_PROVIDERS.md) | Plugin capability interfaces | Architect | P1 | Shipped v0.14 MVP | Platform |
| [guides/plugins.md](guides/plugins.md) | Plugin authoring guide | Contributor | P1 | Shipped v0.14 | Platform |
| [ui/ROADMAP_MAPPING.md](ui/ROADMAP_MAPPING.md) | UI items ↔ releases checklist | Implementer | P0 | Active | OntoUI |
| [ui/PRODUCT_ROADMAP_2.0.md](ui/PRODUCT_ROADMAP_2.0.md) | UI phases with milestones | Implementer | P1 | Active | OntoUI |
| [design/ARCHITECTURE.md](design/ARCHITECTURE.md) | OntoCore crate layout | Contributor | P1 | Active | OntoCore |
| [design/PLUGIN_SPEC.md](design/PLUGIN_SPEC.md) | Plugin host engineering spec | Contributor | P2 | Shipped v0.14 MVP | OntoCore |
| [adr/README.md](adr/README.md) | Product/platform ADRs | Architect | P0 | Active | Platform |
| [design/adr/README.md](design/adr/README.md) | Engineering ADRs | Contributor | P1 | Active | OntoCore |
| [cursor-prompts/README.md](cursor-prompts/README.md) | Cursor-safe implementation prompts | Implementer | P0 | Active | OntoUI |
| [architecture.md](architecture.md) | User-facing ecosystem overview | Evaluator | P1 | Shipped | Platform |
| [vision.md](vision.md) | Mission and direction | Evaluator | P2 | Active | Platform |
| [internals.md](internals.md) | Contributor entry (design targets) | Contributor | P1 | Active | Platform |

---

## Document layers

| Layer | Location | Use for |
|-------|----------|---------|
| **Shipped** | [SHIPPED.md](SHIPPED.md), guides, reference | What works today |
| **Platform architecture** | [platform/](platform/OVERVIEW.md) | How to build OntoUI, workspaces, plugins |
| **Product UX** | [ui/](ui/README.md) | Wireframes, HIG, interaction patterns |
| **Engineering** | [design/](design/README.md) | Crate specs, ADRs, backlogs |
| **Execution** | [cursor-prompts/](cursor-prompts/README.md) | Step-by-step implementation tasks |

---

## User documentation

Published site: [Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/) · Start at [index.md](index.md).
