import type { CurrentFocus } from "./store/types";

export type { CurrentFocus };

export interface ReasoningStatePayload {
  profile: string;
  unsatisfiable: string[];
  hierarchyMode?: "asserted" | "inferred" | "combined";
  lastRunAt: number;
  dirty?: boolean;
  running?: boolean;
}
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
  | "prefixManager";

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

export interface SqlTableSchema {
  name: string;
  columns: Array<{ name: string; type: string }>;
}
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
  other_iri?: string;
  /** Member property IRIs for property_chain axioms (ordered). */
  properties?: string[];
  editable: boolean;
}

export interface EntityAnnotationSummary {
  predicate: string;
  value: string;
  editable: boolean;
}

export interface PropertyCharacteristics {
  functional?: boolean;
  inverse_functional?: boolean;
  transitive?: boolean;
  symmetric?: boolean;
  asymmetric?: boolean;
  reflexive?: boolean;
  irreflexive?: boolean;
}

export interface EntityDetailPayload {
  entity: EntitySummary;
  parents: string[];
  children: string[];
  axioms: EntityAxiomSummary[];
  annotations?: EntityAnnotationSummary[];
  characteristics?: PropertyCharacteristics;
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

function isEntityChange(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const change = value as Record<string, unknown>;
  return typeof change.kind === "string" && typeof change.iri === "string";
}

function isAxiomChange(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const change = value as Record<string, unknown>;
  return (
    typeof change.change === "string" &&
    typeof change.subject === "string" &&
    typeof change.predicate === "string" &&
    typeof change.object === "string" &&
    typeof change.axiom_kind === "string"
  );
}

function isAnnotationChange(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const change = value as Record<string, unknown>;
  return (
    typeof change.change === "string" &&
    typeof change.subject === "string" &&
    typeof change.predicate === "string" &&
    typeof change.object === "string"
  );
}

function isImportChange(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const change = value as Record<string, unknown>;
  return (
    typeof change.change === "string" &&
    typeof change.ontology_id === "string" &&
    typeof change.import_iri === "string"
  );
}

function isInferenceChange(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const change = value as Record<string, unknown>;
  return (
    typeof change.class_iri === "string" &&
    typeof change.change === "string" &&
    typeof change.detail === "string"
  );
}

function isBreakingChange(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const change = value as Record<string, unknown>;
  return typeof change.reason === "string" && typeof change.message === "string";
}

export function isDiffPayload(data: unknown): data is DiffPayload {
  if (!data || typeof data !== "object") {
    return false;
  }
  const diff = data as Record<string, unknown>;
  return (
    Array.isArray(diff.entity_changes) &&
    diff.entity_changes.every(isEntityChange) &&
    Array.isArray(diff.axiom_changes) &&
    diff.axiom_changes.every(isAxiomChange) &&
    Array.isArray(diff.annotation_changes) &&
    diff.annotation_changes.every(isAnnotationChange) &&
    Array.isArray(diff.import_changes) &&
    diff.import_changes.every(isImportChange) &&
    Array.isArray(diff.inference_changes) &&
    diff.inference_changes.every(isInferenceChange) &&
    Array.isArray(diff.breaking_changes) &&
    diff.breaking_changes.every(isBreakingChange)
  );
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
  | { type: "queryInit"; saved: SavedQuery[]; history: SavedQuery[]; sqlTables: string[]; sqlSchema?: SqlTableSchema[] }
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
  | { type: "exportGraph"; format: "json" | "csv"; payload: string; suggestedName?: string }
  | { type: "validateManchester"; expression: string; axiomKind: string; seq: number }
  | { type: "applyManchester"; expression: string; axiomKind: string; previewOnly: boolean }
  | { type: "copyMarkdown" }
  | { type: "setFocus"; focus: CurrentFocus }
  | { type: "showNotification"; message: string; level?: "info" | "warning" | "error" };

export function isHostMessage(data: unknown): data is HostMessage {
  if (typeof data !== "object" || data === null || !("type" in data)) {
    return false;
  }
  const msg = data as HostMessage;
  if (typeof msg.type !== "string") {
    return false;
  }
  if (msg.type === "semanticDiffData") {
    return isDiffPayload((data as { diff?: unknown }).diff);
  }
  if (msg.type === "error") {
    return typeof (data as { message?: unknown }).message === "string";
  }
  return true;
}
