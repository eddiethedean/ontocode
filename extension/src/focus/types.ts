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

export interface ReasoningStatePayload {
  profile: string;
  unsatisfiable: string[];
  hierarchyMode?: "asserted" | "inferred" | "combined";
  lastRunAt: number;
  /** True when ontology edits occurred after last run. */
  dirty?: boolean;
  /** True while a reasoner run is in-flight. */
  running?: boolean;
}
