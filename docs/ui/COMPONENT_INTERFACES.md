# Component Interfaces

## 1. Purpose

This document defines implementation-facing TypeScript interfaces for core UI components.

## 2. Shared Types

```ts
export interface SemanticRef {
  id: string
  iri?: string
  label: string
  kind: SemanticKind
}

export type SemanticKind =
  | "class"
  | "individual"
  | "objectProperty"
  | "dataProperty"
  | "annotationProperty"
  | "ontology"
  | "query"
  | "diagnostic"
```

## 3. WorkspaceLayout

```ts
interface WorkspaceLayoutProps {
  workspaceId: string
  currentFocus?: SemanticRef
  layout: LayoutState
  onLayoutChange(layout: LayoutState): void
}
```

## 4. ExplorerTree

```ts
interface ExplorerTreeProps {
  roots: ExplorerNode[]
  selectedIds: string[]
  expandedIds: string[]
  onSelect(node: ExplorerNode): void
  onExpand(nodeId: string): void
  onContextMenu(node: ExplorerNode, event: MouseEvent): void
}
```

## 5. EntityEditor

```ts
interface EntityEditorProps {
  entityId: string
  activeTab: EntityEditorTab
  onTabChange(tab: EntityEditorTab): void
  onCommand(command: EntityCommand): void
}
```

## 6. InspectorCard

```ts
interface InspectorCardProps {
  title: string
  icon?: IconName
  status?: StatusKind
  actions?: Action[]
  children: React.ReactNode
}
```

## 7. SemanticGraph

```ts
interface SemanticGraphProps {
  nodes: GraphNode[]
  edges: GraphEdge[]
  selection: string[]
  layout: GraphLayout
  overlays: GraphOverlay[]
  onSelect(ref: SemanticRef): void
  onExpand(nodeId: string): void
}
```

## 8. QueryEditor

```ts
interface QueryEditorProps {
  language: QueryLanguage
  value: string
  schema: QuerySchema
  diagnostics: Diagnostic[]
  onChange(value: string): void
  onRun(): void
}
```

## 9. AIActionBar

```ts
interface AIActionBarProps {
  context: AIContext
  actions: AIAction[]
  onInvoke(action: AIAction): void
}
```

## 10. Extension Slots

```ts
interface ExtensionSlot<TProps> {
  id: string
  region: string
  render(props: TProps): React.ReactNode
}
```
