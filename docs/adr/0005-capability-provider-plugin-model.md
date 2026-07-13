# ADR-0005 — Capability Provider plugin model

## Status

Accepted — **shipped v0.14+** (plugin host MVP; ecosystem API still pre-1.0)

## Context

[design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md) and [ui/PLUGIN_PLATFORM.md](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLUGIN_PLATFORM.md) describe extensibility but use inconsistent terminology ("plugin", "extension point").

## Decision

Plugins expose **Capability Providers** — typed interfaces for reasoning, query, refactoring, diagnostics, AI, import/export, documentation. OntoCore hosts provider registration and permissions; OntoUI renders provider contributions (commands, inspector cards).

Built-in features (Ontologos reasoner, SQL query) are default providers with stable IDs.

## Consequences

**Positive:** Third-party features indistinguishable from core; clear semver surface.

**Negative:** Provider ABI design and sandbox IPC complexity deferred to v0.14 implementation.

## References

- [platform/CAPABILITY_PROVIDERS.md](https://github.com/eddiethedean/ontocode/blob/main/docs/platform/CAPABILITY_PROVIDERS.md)
- [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md)
