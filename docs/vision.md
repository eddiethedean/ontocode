# Modernizing the Ontology Ecosystem

> **Long-term vision.** For what ships in **v0.23**, see [What ships today](SHIPPED.md). **Protégé replacement is the 1.0 goal — not supported for org-wide retirement today.** Use [Protégé decision guide](guides/protege-decision.md) and [Known limitations](known-limitations.md) for current gaps. Plugin host MVP and owlmake workflow scaffold **shipped in v0.14**; stable plugin ecosystem API is a **v1.0 target**.

## Mission

Build the modern open-source platform for ontology engineering.

The current ontology ecosystem is powerful but fragmented, heavily JVM-centric, and centered around desktop-era workflows. Our goal is not to replace W3C standards—it is to modernize how developers, researchers, and organizations build, validate, query, reason over, and maintain ontologies.

## Long-Term Vision

Four projects work together:

- **Ontologos** — Rust-native reasoning engine.
- **OntoCore** — Semantic workspace engine and reusable platform.
- **OntoCode** — Flagship VS Code IDE powered by OntoCore.
- **OntoUI** — Shared React UI platform ([platform/ONTOUI.md](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/ONTOUI.md)) — **v0.13 foundation shipped** (WorkspaceStore, focus relay, design tokens); **OntoStudio** reuses it (planned post-1.0).

Together they enable modern workflows including CI/CD, team collaboration on version-controlled ontology files, and high-performance local tooling. **Post-1.0 / planned (not shipped today):** AI-assisted development surfaces, Python and TypeScript SDKs, and OntoStudio desktop — see [What ships today](SHIPPED.md) and [Protégé decision](guides/protege-decision.md) for current non-goals.

## Ecosystem Collaboration

OntoCore is the **platform** — workspace indexing, query, diagnostics, refactoring, and **plugin hosting**. It does not absorb every tool in the ontology stack.

**External workflow tools** integrate through OntoCore's plugin APIs. [owlmake](https://github.com/INCATools/owlmake) is the first reference workflow plugin: it demonstrates ROBOT/ODK-style build, validation, and release automation **outside** OntoCore, while OntoCode surfaces those workflows in the IDE. ROBOT and ODK remain the semantic standards for many operations; OntoCore integrates with them rather than rewriting them.

- **Ontologos** — reasoning (classification, consistency, explanations).
- **OntoCore** — semantic workspace engine and plugin platform.
- **owlmake** (and future plugins) — workflow, build, and release automation.
- **OntoCode** — presents workspace editing, reasoning, and toolchain workflows in one modern IDE.

## Guiding Principles

- Standards-first (OWL, RDF, SHACL, SPARQL, OBO)
- Developer-first APIs
- Local-first architecture
- AI-native tooling
- Cross-language support
- Extensible plugin ecosystem — external tools integrate; OntoCore does not monolith
- Open-source and community driven

## Success

The ecosystem succeeds when developers build new tools on OntoCore, Ontologos becomes a trusted reasoning engine, OntoCode is a production-ready alternative to Protégé, and workflow tools like owlmake integrate as first-class citizens without becoming core dependencies.

See also [Architecture](architecture.md) and [Roadmap](roadmap.md).
