/** Webview panel kinds (query param ?panel=). */
export type PanelKind =
  | "smoke"
  | "inspector"
  | "graph"
  | "refactorPreview"
  | "queryWorkbench"
  | "manchesterEditor";

/** Entity summary from LSP getEntity. */
export interface EntitySummary {
  iri: string;
  short_name: string;
  kind: string;
  labels: string[];
  comments: string[];
  deprecated: boolean;
  obo_id?: string;
}

export interface EntityAxiomSummary {
  kind: string;
  display: string;
  manchester?: string;
  parent_iri?: string;
  editable: boolean;
}

export interface EntityDetailPayload {
  entity: EntitySummary;
  parents: string[];
  children: string[];
  axioms: EntityAxiomSummary[];
  editable: boolean;
  document_path?: string;
}

export interface PatchOp {
  op: string;
  entity_iri?: string;
  value?: string;
  parent_iri?: string;
  manchester?: string;
  other_iri?: string;
  [key: string]: unknown;
}

export interface GraphNode {
  id: string;
  label: string;
  kind: string;
}

export interface GraphEdge {
  source: string;
  target: string;
  kind: string;
  inferred: boolean;
}

export interface GraphPayload {
  nodes: GraphNode[];
  edges: GraphEdge[];
  truncated: boolean;
  graph_kind: string;
}

export interface GraphFilters {
  ontology_iri?: string;
  hide_deprecated?: boolean;
}

export interface RefactorFileChange {
  path: string;
  preview_text: string;
  original_text: string;
  hunks: Array<{
    start_byte: number;
    end_byte: number;
    old_text: string;
    new_text: string;
  }>;
}

export interface RefactorPlanPayload {
  changes: RefactorFileChange[];
  warnings?: string[];
}

export interface TabularQueryResult {
  columns: string[];
  rows: Record<string, string>[];
  truncated?: boolean;
}

export interface SavedQuery {
  name: string;
  mode: "sql" | "sparql";
  text: string;
}

export interface ManchesterCompletions {
  classes: string[];
  object_properties: string[];
  data_properties: string[];
  datatypes: string[];
}

export interface ManchesterValidationResult {
  normalized: string;
  turtle_fragment: string;
  tree: unknown;
  diagnostics: Array<{ severity: string; message: string }>;
}

/** Host → React */
export type HostMessage =
  | { type: "init"; panel: PanelKind }
  | { type: "loadEntity"; detail: EntityDetailPayload; classOptions: string[] }
  | { type: "graphData"; graph: GraphPayload }
  | { type: "preview"; text: string }
  | { type: "loadRefactorPlan"; plan: RefactorPlanPayload }
  | { type: "queryInit"; saved: SavedQuery[]; history: SavedQuery[]; sqlTables: string[] }
  | { type: "queryResult"; runId: number; result?: TabularQueryResult; error?: string }
  | { type: "manchesterInit"; entityIri: string; axiomKind: string; expression: string; completions: ManchesterCompletions }
  | { type: "manchesterValidation"; seq: number; result?: ManchesterValidationResult; error?: string }
  | { type: "error"; message: string };

/** React → Host */
export type WebviewMessage =
  | { type: "ready"; panel: PanelKind }
  | { type: "applyPatch"; patches: PatchOp[]; previewOnly: boolean }
  | { type: "jumpToSource" }
  | { type: "openManchester"; axiom: { kind: string; manchester?: string } }
  | { type: "addManchesterAxiom" }
  | { type: "requestGraph"; graphKind: string; rootIri?: string; depth?: number; includeInferred?: boolean; filters?: GraphFilters }
  | { type: "selectNode"; iri: string }
  | { type: "openEntity"; iri: string }
  | { type: "openGraph"; rootIri?: string }
  | { type: "findUsages" }
  | { type: "renameIri" }
  | { type: "applyRefactor" }
  | { type: "cancelRefactor" }
  | { type: "runQuery"; mode: "sql" | "sparql"; text: string; runId: number }
  | { type: "saveQuery"; name: string; mode: "sql" | "sparql"; text: string }
  | { type: "exportQueryResult"; format: "csv" | "json" }
  | { type: "validateManchester"; expression: string; axiomKind: string; seq: number }
  | { type: "applyManchester"; expression: string; axiomKind: string; previewOnly: boolean };

export function isHostMessage(data: unknown): data is HostMessage {
  return (
    typeof data === "object" &&
    data !== null &&
    "type" in data &&
    typeof (data as HostMessage).type === "string"
  );
}
