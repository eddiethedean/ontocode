/** Shared webview message types (keep in sync with webview-ui/src/messages.ts). */

export type PanelKind = "smoke" | "inspector" | "graph";

export interface PatchOp {
  op: string;
  entity_iri?: string;
  value?: string;
  parent_iri?: string;
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

export function isWebviewMessage(data: unknown): data is WebviewMessage {
  return (
    typeof data === "object" &&
    data !== null &&
    "type" in data &&
    typeof (data as WebviewMessage).type === "string"
  );
}
