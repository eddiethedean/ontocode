# Modernizing the Ontology Ecosystem

> **Long-term vision.** For what ships in **v0.11**, see [What ships today](https://ontocode-vs.readthedocs.io/en/latest/SHIPPED/). Plugin hosting and owlmake integration are **v1.0 targets** — not installable yet.

## Mission

Build the modern open-source platform for ontology engineering.

The current ontology ecosystem is powerful but fragmented, heavily JVM-centric, and centered around desktop-era workflows. Our goal is not to replace W3C standards—it is to modernize how developers, researchers, and organizations build, validate, query, reason over, and maintain ontologies.

## Long-Term Vision

Three foundational projects work together:

- **Ontologos** — Rust-native reasoning engine.
- **OntoCore** — Semantic workspace engine and reusable platform.
- **OntoCode** — Flagship VS Code IDE powered by OntoCore.

Together they enable modern workflows including AI-assisted development, CI/CD, team collaboration on version-controlled ontology files, Python and TypeScript integration, and high-performance local tooling.

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

See also [ARCHITECTURE.md](ARCHITECTURE.md) and [ROADMAP.md](ROADMAP.md).
