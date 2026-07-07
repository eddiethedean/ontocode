# Graph Rendering Architecture

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

The Graph Workspace requires high-performance rendering for large semantic graphs.

## 2. Rendering Layers

```text
Graph Data Layer
↓
Layout Engine
↓
Scene Graph
↓
Renderer
↓
Interaction Layer
↓
Selection / Focus Sync
```

## 3. Graph Data Model

```ts
interface GraphNode {
  id: string
  ref: SemanticRef
  position?: Point
  size?: Size
  badges: Badge[]
  collapsed?: boolean
}

interface GraphEdge {
  id: string
  source: string
  target: string
  kind: EdgeKind
  inferred?: boolean
}
```

## 4. Progressive Loading

- Load focus neighborhood first.
- Expand on demand.
- Virtualize offscreen nodes.
- Cache layouts.
- Stream updates.

## 5. Layout Engines

Support:

- force
- hierarchical
- radial
- orthogonal
- manual
- plugin-provided

## 6. Rendering Strategy

Phase 1:

- SVG for clarity and accessibility.
- Canvas for large edge layers.

Phase 2:

- WebGPU for large enterprise graphs.

## 7. Interaction

- 60 FPS pan/zoom target.
- Selection under 50ms.
- Debounced expensive layout recalculations.
- Smooth focus transitions.

## 8. Overlays

Supported overlays:

- diagnostics
- inferred edges
- refactoring impact
- search hits
- AI highlights
- review comments

## 9. Persistence

Persist:

- saved views
- node positions
- collapsed groups
- filters
- zoom
- viewport

## 10. Testing

- layout snapshots
- interaction tests
- performance budgets
- accessibility checks
