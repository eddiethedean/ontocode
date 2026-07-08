# ADR-0003 — Current Focus as central UX concept

## Status

Accepted — **implemented v0.13**

## Context

Multiple UI specs describe a single active semantic object driving the IDE ([ui/DESIGN_PHILOSOPHY.md](../ui/DESIGN_PHILOSOPHY.md), [ui/INTERACTION_PRINCIPLES.md](../ui/INTERACTION_PRINCIPLES.md)) — **implemented in v0.13** via `WorkspaceStore` + extension-host focus relay.

## Decision

**Current Focus** is the canonical active semantic object (`entity`, `axiom`, `query`, `diagnostic`, `graphNode`, …). Changing focus emits `FocusChanged` on the workspace event bus; all workspaces subscribe and update context.

Selection (multi-select in explorer/graph) is separate from focus but may update focus on primary item.

## Consequences

**Positive:** Explorer, inspector, graph, and AI share one coordination primitive.

**Negative:** Every workspace must handle focus events; legacy panels need migration.

## References

- [platform/WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)
- [glossary.md](../glossary.md)
