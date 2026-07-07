# ADR-0001 — OntoUI as shared React platform

## Status

Accepted — **partially implemented** (v0.12 per-panel React webviews)

## Context

OntoCode v0.7+ ships React webviews ([design/adr/0017](../design/adr/0017-react-webview-ui.md)) as isolated panels routed by `?panel=` in `extension/webview-ui/src/App.tsx`. OntoStudio and future hosts need the same UI without duplicating components.

## Decision

Introduce **OntoUI** as the named shared React platform:

- Package: `extension/webview-ui/` (may extract to workspace package later)
- Host-agnostic components and state via **WorkspaceHost**
- OntoCode is one host; OntoStudio uses the same OntoUI bundle with a different host adapter

## Consequences

**Positive:** Single component library; OntoStudio reuse; clearer ownership vs extension host code.

**Negative:** Migration from per-panel state to WorkspaceStore (v0.13); host adapter abstraction required.

## References

- [platform/ONTOUI.md](../platform/ONTOUI.md)
- [design/adr/0017-react-webview-ui.md](../design/adr/0017-react-webview-ui.md)
- [glossary.md](../glossary.md)
