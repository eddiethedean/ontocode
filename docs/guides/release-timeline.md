# Release timeline (non-commitment)

Planning ranges for OntoCode / OntoCore. **These are product goals, not contractual delivery dates.** Shipped behavior is always defined by [What ships today](../SHIPPED.md) for the version you deploy.

## Current release

| Version | Status | Date (changelog) |
|---------|--------|------------------|
| **0.11.2** | Current | 2026-07-06 |
| **0.11.1** | Previous | 2026-07-06 |
| **0.11.0** | Previous | 2026-07-05 |

Pre-1.0: minor releases may change library APIs, LSP JSON, and SQL virtual table columns — [README](https://github.com/eddiethedean/ontocode/blob/main/README.md).

## Documented milestone goals (not dates)

| Target | Goal | Shipped in |
|--------|------|------------|
| **v0.9** | OntoCore identity — `ontocore` façade, branding, documentation; OntoLogos 1.0 DL/auto classification | **Shipped** (2026-07-03) |
| **v0.10** | Semantic workspace — incremental index, multi-root, stable `Workspace` API, semantic diff, optional disk cache | **Shipped** (2026-07-04) |
| **v0.11** | Editor depth & distribution — LSP completion, code actions, docs export, imports UI, Open VSX, OBO fastobo read | **Shipped** (2026-07-05) |
| **v1.0** | Protégé-competitive OWL + OBO in VS Code; full axiom catalog; plugin host | Planned |

Canonical forward plan: [Platform roadmap](../roadmap.md). Engineering milestone history: [Milestones (shipped)](../design/ROADMAP.md).

**There is no documented calendar date for v1.0.** Enterprise plans should not assume a quarter or year without maintainer confirmation outside these docs.

## What each near-term milestone implies

### v0.10 (shipped)

- Semantic diff for PR review workflows (CLI, LSP, VS Code panel)
- Incremental indexing, multi-root workspaces, optional `.ontocore/cache`
- Stable `ontocore::Workspace` API
- Does **not** by itself complete Protégé parity or full OBO write-back

### v1.0 (planned)

- Full OWL 2 DL axiom catalog, OBO write-back, and extended Protégé round-trip playbooks per parity matrix
- Installable plugin host / owlmake-style workflow integration
- Formal performance benchmarks (currently v1.0 backlog — [performance sizing](performance-sizing.md))
- Extended Protégé migration playbooks (today: [first-week guide](protege-migration.md), [coexistence](protege-coexistence.md), [decision matrix](protege-decision.md))

## How to plan enterprise adoption without a v1.0 date

1. **Now (v0.11):** CI gates + controlled IDE pilot — [production readiness](production-readiness.md)
2. **Run** [production evidence protocol](production-evidence.md) on your corpus
3. **Re-evaluate** at each pinned minor bump using [migration index](../migration/README.md)
4. **Do not** retire Protégé for DL/OBO workflows until parity matrix items you need are green in SHIPPED

## Design docs vs shipped docs

| Document type | Trust for procurement |
|---------------|----------------------|
| [SHIPPED.md](../SHIPPED.md) | **Canonical** for deployed version |
| User guides under VS Code / Rust tabs | **Implemented** behavior for current release |
| Contributing → Design (specs, ADRs, ROADMAP) | **Target / vision** — may not be implemented |

## Related

- [Governance](governance.md)
- [Roadmap (engineering detail)](../design/ROADMAP.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Changelog](../changelog.md)
