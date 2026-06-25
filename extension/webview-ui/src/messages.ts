/** Webview panel kinds (query param ?panel=). */
export type PanelKind = "smoke" | "inspector" | "graph";

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

/** Host → React */
export type HostMessage =
  | { type: "init"; panel: PanelKind }
  | { type: "loadEntity"; detail: EntityDetailPayload; classOptions: string[] }
  | { type: "graphData"; graph: GraphPayload }
  | { type: "preview"; text: string }
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
  | { type: "openGraph"; rootIri?: string };

export function isHostMessage(data: unknown): data is HostMessage {
  return (
    typeof data === "object" &&
    data !== null &&
    "type" in data &&
    typeof (data as HostMessage).type === "string"
  );
}
