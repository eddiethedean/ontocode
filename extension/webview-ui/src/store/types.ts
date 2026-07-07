import type { RefactorPlanPayload } from "../messages";

/** Semantic object currently driving UI synchronization. */
export type FocusKind =
  | "entity"
  | "axiom"
  | "query"
  | "diagnostic"
  | "graphNode"
  | "documentation"
  | "review";

export interface CurrentFocus {
  kind: FocusKind;
  id: string;
  source: string;
  timestamp: number;
}

export interface SelectionState {
  /** Multi-select IRIs when applicable. */
  items: string[];
}

export interface NavigationEntry {
  focus: CurrentFocus;
}

export interface NavigationState {
  stack: NavigationEntry[];
  index: number;
}

export interface QueryResultSnapshot {
  columns: string[];
  rows: string[][];
  truncated?: boolean;
}

export interface QueryHistoryEntry {
  id: string;
  language: "sql" | "sparql";
  text: string;
  executedAt: number;
}

export interface QueryState {
  language: "sql" | "sparql";
  text: string;
  lastResult: QueryResultSnapshot | null;
  history: QueryHistoryEntry[];
  schemaBrowserExpanded: boolean;
}

export interface ReasoningState {
  profile: string;
  lastRunAt: number | null;
  unsatisfiable: string[];
  hierarchyMode: "asserted" | "inferred" | "combined";
}

export interface RefactoringState {
  pending: RefactorPlanPayload | null;
}

export interface InspectorState {
  /** Last loaded entity IRI for the inspector workspace. */
  entityIri: string | null;
}

export interface GraphState {
  rootIri: string | null;
}

export interface ExplorerState {
  /** Explorer highlight IRI (planned v1.0 tree sync). */
  highlightedIri: string | null;
}

export interface WorkspaceStoreState {
  focus: CurrentFocus | null;
  selection: SelectionState;
  navigation: NavigationState;
  query: QueryState;
  reasoning: ReasoningState;
  refactoring: RefactoringState;
  inspector: InspectorState;
  graph: GraphState;
  explorer: ExplorerState;
}
