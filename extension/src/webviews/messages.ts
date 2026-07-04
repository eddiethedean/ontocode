/** Shared webview message types (keep in sync with webview-ui/src/messages.ts). */

export type PanelKind =
  | "smoke"
  | "inspector"
  | "graph"
  | "refactorPreview"
  | "queryWorkbench"
  | "manchesterEditor"
  | "semanticDiff";

export interface PatchOp {
  op: string;
  entity_iri?: string;
  value?: string;
  parent_iri?: string;
  manchester?: string;
  other_iri?: string;
  [key: string]: unknown;
}

export interface GraphFilters {
  ontology_iri?: string;
  hide_deprecated?: boolean;
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

export interface EntityDetailPayload {
  entity: {
    iri: string;
    short_name: string;
    kind: string;
    labels: string[];
    comments: string[];
    deprecated: boolean;
    obo_id?: string;
  };
  parents: string[];
  children: string[];
  axioms: Array<{
    kind: string;
    display: string;
    manchester?: string;
    parent_iri?: string;
    editable: boolean;
  }>;
  editable: boolean;
  document_path?: string;
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

export interface SavedQuery {
  name: string;
  mode: "sql" | "sparql";
  text: string;
}

export interface TabularQueryResult {
  columns: string[];
  rows: Record<string, string>[];
  truncated?: boolean;
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

export interface DiffPayload {
  entity_changes: Array<{ kind: string; iri: string; previous_iri?: string; labels?: string[] }>;
  axiom_changes: Array<{
    change: string;
    subject: string;
    predicate: string;
    object: string;
    axiom_kind: string;
  }>;
  annotation_changes: Array<{
    change: string;
    subject: string;
    predicate: string;
    object: string;
  }>;
  import_changes: Array<{ change: string; ontology_id: string; import_iri: string }>;
  inference_changes: Array<{ class_iri: string; change: string; detail: string }>;
  breaking_changes: Array<{ reason: string; message: string; entity_iri?: string }>;
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
  | { type: "semanticDiffData"; diff: DiffPayload }
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
  | { type: "exportQueryResult"; format: "csv" | "json"; runId?: number }
  | { type: "validateManchester"; expression: string; axiomKind: string; seq: number }
  | { type: "applyManchester"; expression: string; axiomKind: string; previewOnly: boolean }
  | { type: "copyMarkdown" };

export function isWebviewMessage(data: unknown): data is WebviewMessage {
  return (
    typeof data === "object" &&
    data !== null &&
    "type" in data &&
    typeof (data as WebviewMessage).type === "string"
  );
}

/** Validate applyPatch payload; reject missing previewOnly (must not default to write). */
export function parseApplyPatchMessage(
  message: WebviewMessage,
  expectedEntityIri: string | undefined
): { patches: PatchOp[]; previewOnly: boolean } | null {
  if (message.type !== "applyPatch") {
    return null;
  }
  if (typeof message.previewOnly !== "boolean") {
    return null;
  }
  if (!Array.isArray(message.patches) || message.patches.length === 0) {
    return null;
  }
  for (const patch of message.patches) {
    if (!patch || typeof patch !== "object" || typeof patch.op !== "string") {
      return null;
    }
    const entityIri = (patch as PatchOp).entity_iri;
    if (
      expectedEntityIri &&
      typeof entityIri === "string" &&
      entityIri !== expectedEntityIri
    ) {
      return null;
    }
  }
  return { patches: message.patches, previewOnly: message.previewOnly };
}
