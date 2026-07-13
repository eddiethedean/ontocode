# Documentation index

Master index for OntoCode / OntoCore planning, architecture, and user documentation.

**Latest tagged release:** v0.20.0 · **Canonical capabilities:** [What ships today](SHIPPED.md) · **Limits:** [Known limitations](known-limitations.md) · **Terms:** [Glossary](glossary.md)

The public site navigation is defined in [`mkdocs.yml`](https://github.com/eddiethedean/ontocode/blob/main/mkdocs.yml). This page is a reading-order map — not a second nav.

## Recommended reading order

### Evaluators and adopters

1. [What ships today](SHIPPED.md)
2. [Known limitations](known-limitations.md)
3. [Glossary](glossary.md)
4. [First success (~10 min)](guides/first-success.md)
5. [Roadmap hub](roadmap-hub.md)

### New users

1. [Start here](start.md)
2. [First success](guides/first-success.md)
3. [Feature tour](ontocode/feature-tour.md)
4. [Supported formats](supported-formats.md)

### Contributors

1. [Contributing](contributing.md)
2. [Internals](internals.md)
3. [Engineering docs on GitHub](engineering.md) — UI specs, platform targets, Cursor prompts

---

## Document layers

| Layer | Location | Use for |
|-------|----------|---------|
| **Shipped (this site)** | [SHIPPED.md](SHIPPED.md), guides, reference | What works today |
| **Evaluate** | Vision, architecture, enterprise pack | Adoption decisions |
| **Engineering (GitHub)** | [engineering.md](engineering.md) | Specs, ADRs, UI, prompts — not in public MkDocs search |

Deep planning docs (`docs/ui/`, `docs/platform/`, `docs/cursor-prompts/`, `docs/PROTEGE_REVERSE_ENGINEERING/`) remain in the repository and are linked from [Engineering docs (GitHub)](engineering.md). They are **excluded from the public MkDocs build**.

---

## Site map (public)

| Section | Start here |
|---------|------------|
| **Get started** | [start.md](start.md) → [first-success](guides/first-success.md) or [getting-started](getting-started.md) |
| **Use OntoCode** | [Feature tour](ontocode/feature-tour.md) |
| **Use OntoCore** | [OntoCore overview](ontocore/index.md) · [Examples](examples/index.md) |
| **Reference** | [CLI](cli-reference.md) · [Rust API](ontocore/rust-api.md) · [LSP API](lsp-api.md) · [docs.rs ontocore](https://docs.rs/ontocore) |
| **Evaluate** | [Architecture](architecture.md) · [Enterprise eval](guides/enterprise-eval.md) |
| **Help** | [FAQ](faq.md) · [Troubleshooting](troubleshooting.md) · [Migrations](migration/README.md) |
| **Contribute** | [Contributing](contributing.md) · [Engineering](engineering.md) |

---

## Major planning documents

| Document | Purpose | Audience | Status |
|----------|---------|----------|--------|
| [SHIPPED.md](SHIPPED.md) | Canonical capability matrix | All | Shipped v0.20 |
| [known-limitations.md](known-limitations.md) | Honest limits | All | Active |
| [glossary.md](glossary.md) | Canonical terminology | All | Active |
| [roadmap-hub.md](roadmap-hub.md) | Which roadmap doc to read | All | Active |
| [roadmap.md](roadmap.md) | Platform release plan | All | Active |
| [design/adr/README.md](design/adr/README.md) | Engineering ADRs | Contributor | Active |
| [architecture.md](architecture.md) | User-facing ecosystem overview | Evaluator | Shipped v0.20 |
| [vision.md](vision.md) | Mission and direction | Evaluator | Active |
| [engineering.md](engineering.md) | Pointer to GitHub engineering corpus | Implementer | Active |
| [platform/OVERVIEW.md](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/OVERVIEW.md) | OntoUI / platform (GitHub) | Implementer | Shipped v0.20 |

---

## User documentation

Published site: [Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/) · Start at [index.md](index.md).
