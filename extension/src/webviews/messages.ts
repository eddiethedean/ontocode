/** Shared webview message types (keep in sync with webview-ui/src/messages.ts). */

export type PanelKind =
  | "smoke"
  | "inspector"
  | "graph"
  | "refactorPreview"
  | "queryWorkbench"
  | "manchesterEditor"
  | "semanticDiff"
  | "imports"
  | "metrics"
  | "about"
  | "newOntology"
  | "prefixManager"
  | "reasoner"
  | "explanation";

export interface CatalogStats {
  ontology_count: number;
  class_count: number;
  object_property_count: number;
  data_property_count: number;
  annotation_property_count: number;
  individual_count: number;
  axiom_count: number;
  annotation_count: number;
  triple_count: number;
  error_count: number;
  diagnostic_error_count: number;
  diagnostic_warning_count: number;
  diagnostic_info_count: number;
}

export interface PatchOp {
  op: string;
  entity_iri?: string;
  term_id?: string;
  ontology_iri?: string;
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
    other_iri?: string;
    properties?: string[];
    editable: boolean;
  }>;
  annotations?: Array<{
    predicate: string;
    value: string;
    editable: boolean;
  }>;
  characteristics?: {
    functional?: boolean;
    inverse_functional?: boolean;
    transitive?: boolean;
    symmetric?: boolean;
    asymmetric?: boolean;
    reflexive?: boolean;
    irreflexive?: boolean;
  };
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

export interface ImportsOntologyOption {
  iri: string;
  path: string;
  label: string;
}

export interface ImportsDocumentPayload {
  path: string;
  ontology_iri?: string;
  imports_editable: boolean;
  error?: string;
  imports: string[];
  options: ImportsOntologyOption[];
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
  summary_counts?: {
    entities: number;
    axioms: number;
    annotations: number;
    imports: number;
    inferences: number;
    breaking: number;
  };
}

export interface ReasonerResultPayload {
  profile_used: string;
  consistent: boolean;
  unsatisfiable: string[];
  inferred_edge_count: number;
  new_inferences: Array<{ child: string; parent: string }>;
  warnings: Array<{ code: string; message: string; suggested_profile?: string }>;
  duration_ms: number;
}

export interface ExplanationStepPayload {
  index: number;
  rule: string;
  display: string;
  subject_iri?: string;
  object_iri?: string;
}

export interface ExplanationJustification {
  title: string;
  steps: ExplanationStepPayload[];
  text: string;
}

export interface ExplanationPayload {
  classIri: string;
  profile: string;
  stale: boolean;
  justifications: ExplanationJustification[];
  indexed_at: number;
  content_hash: string;
}

import type { CurrentFocus, ReasoningStatePayload } from "../focus/types";

/** Host → React */
export type HostMessage =
  | { type: "init"; panel: PanelKind }
  | { type: "loadMetrics"; stats: CatalogStats }
  | { type: "loadNewOntology"; path: string; defaultIri: string }
  | { type: "loadPrefixes"; path: string; prefixes: Record<string, string> }
  | { type: "focusState"; focus: CurrentFocus }
  | { type: "reasoningState"; reasoning: ReasoningStatePayload }
  | { type: "loadEntity"; detail: EntityDetailPayload; classOptions: string[]; objectPropertyOptions?: string[] }
  | { type: "graphData"; graph: GraphPayload; rootIri?: string }
  | { type: "preview"; text: string }
  | { type: "loadRefactorPlan"; plan: RefactorPlanPayload }
  | { type: "queryInit"; saved: SavedQuery[]; history: SavedQuery[]; sqlTables: string[]; sqlSchema?: Array<{ name: string; columns: Array<{ name: string; type: string }> }> }
  | { type: "queryResult"; runId: number; result?: TabularQueryResult; error?: string }
  | { type: "manchesterInit"; entityIri: string; axiomKind: string; expression: string; completions: ManchesterCompletions }
  | { type: "manchesterValidation"; seq: number; result?: ManchesterValidationResult; error?: string }
  | { type: "loading" }
  | { type: "semanticDiffData"; diff: DiffPayload }
  | { type: "loadImports"; payload: ImportsDocumentPayload }
  | {
      type: "pluginsLoaded";
      plugins: Array<{
        id: string;
        name: string;
        version: string;
        kind: string;
        inspector_cards: Array<{
          id: string;
          title: string;
          applies_to: string[];
          command?: string;
        }>;
      }>;
    }
  | { type: "reasonerSyncRunId"; runId: number }
  | { type: "reasonerRunCancelled"; runId: number }
  | {
      type: "reasonerResult";
      runId: number;
      result?: ReasonerResultPayload;
      summary?: string;
      error?: string;
    }
  | { type: "loadExplanation"; payload: ExplanationPayload }
  | {
      type: "workspaceEvent";
      event: {
        type: string;
        payload?: unknown;
        timestamp: number;
      };
    }
  | { type: "error"; message: string };

/** React → Host */
export type WebviewMessage =
  | { type: "ready"; panel: PanelKind }
  | { type: "closeDialog" }
  | { type: "submitNewOntology"; ontologyIri: string; versionIri?: string }
  | {
      type: "submitPrefix";
      action: "add" | "remove";
      prefix: string;
      namespaceIri?: string;
    }
  | { type: "applyPatch"; patches: PatchOp[]; previewOnly: boolean }
  | { type: "jumpToSource" }
  | {
      type: "openManchester";
      axiom: { kind: string; manchester?: string; other_iri?: string };
    }
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
  | { type: "exportGraph"; format: "json" | "csv"; payload: string; suggestedName?: string }
  | { type: "validateManchester"; expression: string; axiomKind: string; seq: number }
  | { type: "applyManchester"; expression: string; axiomKind: string; previewOnly: boolean }
  | { type: "copyMarkdown" }
  | { type: "setFocus"; focus: CurrentFocus }
  | { type: "showNotification"; message: string; level?: "info" | "warning" | "error" }
  | { type: "runReasoner"; profile: string; autoDetect: boolean; runId: number }
  | { type: "explainUnsat"; classIri: string }
  | { type: "showInferredHierarchy" }
  | { type: "copyText"; text: string }
  | { type: "rerunReasoner" };

export function isWebviewMessage(data: unknown): data is WebviewMessage {
  if (typeof data !== "object" || data === null || !("type" in data)) {
    return false;
  }
  const msg = data as WebviewMessage;
  if (typeof msg.type !== "string") {
    return false;
  }
  switch (msg.type) {
    case "ready":
      return typeof (data as { panel?: unknown }).panel === "string";
    case "closeDialog":
      return true;
    case "submitNewOntology":
      return (
        typeof msg.ontologyIri === "string" &&
        (msg.versionIri === undefined || typeof msg.versionIri === "string")
      );
    case "submitPrefix":
      return (
        (msg.action === "add" || msg.action === "remove") &&
        typeof msg.prefix === "string" &&
        (msg.namespaceIri === undefined || typeof msg.namespaceIri === "string")
      );
    case "applyPatch":
      return parseApplyPatchMessage(msg, undefined) !== null;
    case "applyManchester":
      return parseApplyManchesterMessage(msg) !== null;
    case "runQuery":
      return parseRunQueryMessage(msg) !== null;
    case "saveQuery":
      return parseSaveQueryMessage(msg) !== null;
    case "validateManchester":
      return (
        typeof msg.expression === "string" &&
        typeof msg.axiomKind === "string" &&
        typeof msg.seq === "number"
      );
    case "openManchester": {
      const axiom = (data as { axiom?: unknown }).axiom;
      return (
        typeof axiom === "object" &&
        axiom !== null &&
        typeof (axiom as { kind?: unknown }).kind === "string"
      );
    }
    case "requestGraph":
      return typeof msg.graphKind === "string";
    case "selectNode":
    case "openEntity":
      return typeof msg.iri === "string";
    case "exportQueryResult":
      return (
        (msg.format === "csv" || msg.format === "json") &&
        (msg.runId === undefined || typeof msg.runId === "number")
      );
    case "exportGraph":
      return (
        (msg.format === "csv" || msg.format === "json") &&
        typeof msg.payload === "string" &&
        (msg.suggestedName === undefined || typeof msg.suggestedName === "string")
      );
    case "copyMarkdown":
    case "jumpToSource":
    case "addManchesterAxiom":
    case "openGraph":
    case "findUsages":
    case "renameIri":
    case "applyRefactor":
    case "cancelRefactor":
    case "showInferredHierarchy":
    case "rerunReasoner":
      return true;
    case "runReasoner":
      return (
        typeof msg.profile === "string" &&
        typeof msg.autoDetect === "boolean" &&
        typeof msg.runId === "number"
      );
    case "explainUnsat":
      return typeof msg.classIri === "string";
    case "copyText":
      return typeof msg.text === "string";
    case "setFocus": {
      const focus = (data as { focus?: unknown }).focus;
      return (
        typeof focus === "object" &&
        focus !== null &&
        typeof (focus as { kind?: unknown }).kind === "string" &&
        typeof (focus as { id?: unknown }).id === "string"
      );
    }
    case "showNotification":
      return typeof (data as { message?: unknown }).message === "string";
    default:
      return false;
  }
}

const MAX_QUERY_TEXT_BYTES = 1_048_576;

/** Validate applyManchester payload; reject missing previewOnly (must not default to write). */
export function parseApplyManchesterMessage(
  message: WebviewMessage
): { expression: string; axiomKind: string; previewOnly: boolean } | null {
  if (message.type !== "applyManchester") {
    return null;
  }
  if (typeof message.previewOnly !== "boolean") {
    return null;
  }
  if (typeof message.expression !== "string" || typeof message.axiomKind !== "string") {
    return null;
  }
  return {
    expression: message.expression,
    axiomKind: message.axiomKind,
    previewOnly: message.previewOnly,
  };
}

export function parseRunQueryMessage(
  message: WebviewMessage
): { mode: "sql" | "sparql"; text: string; runId: number } | null {
  if (message.type !== "runQuery") {
    return null;
  }
  if (message.mode !== "sql" && message.mode !== "sparql") {
    return null;
  }
  if (typeof message.text !== "string" || typeof message.runId !== "number") {
    return null;
  }
  if (message.text.length > MAX_QUERY_TEXT_BYTES) {
    return null;
  }
  return { mode: message.mode, text: message.text, runId: message.runId };
}

export function parseSaveQueryMessage(
  message: WebviewMessage
): { name: string; mode: "sql" | "sparql"; text: string } | null {
  if (message.type !== "saveQuery") {
    return null;
  }
  if (message.mode !== "sql" && message.mode !== "sparql") {
    return null;
  }
  if (typeof message.name !== "string" || typeof message.text !== "string") {
    return null;
  }
  if (!message.name.trim() || message.text.length > MAX_QUERY_TEXT_BYTES) {
    return null;
  }
  return { name: message.name.trim(), mode: message.mode, text: message.text };
}

/** Validate applyPatch payload; reject missing previewOnly (must not default to write). */
export function parseApplyPatchMessage(
  message: WebviewMessage,
  expectedEntityIri: string | undefined,
  expectedOboId?: string
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
    const entityIri = patch.entity_iri;
    const termId = patch.term_id;
    const ontologyIri = patch.ontology_iri;

    if (typeof termId === "string") {
      if (expectedOboId !== undefined && termId !== expectedOboId) {
        return null;
      }
      if (expectedEntityIri !== undefined && expectedOboId === undefined) {
        return null;
      }
      continue;
    }

    if (typeof entityIri === "string") {
      if (expectedEntityIri !== undefined && entityIri !== expectedEntityIri) {
        return null;
      }
      continue;
    }

    if (typeof ontologyIri === "string") {
      // Imports panel passes no expected entity; Inspector must reject ontology-only ops (#310).
      if (expectedEntityIri !== undefined || expectedOboId !== undefined) {
        return null;
      }
      continue;
    }

    if (expectedEntityIri !== undefined || expectedOboId !== undefined) {
      return null;
    }
  }
  return { patches: message.patches, previewOnly: message.previewOnly };
}
