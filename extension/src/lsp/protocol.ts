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

export interface OntologyDocument {
  id: string;
  path: string;
  format: string;
  base_iri?: string;
  imports?: string[];
  namespaces?: Record<string, string>;
  parse_status: string;
  content_hash?: string;
  modified_time?: number;
  parse_message?: string;
  parse_error_location?: { line?: number; column?: number };
}

export interface Entity {
  iri: string;
  short_name: string;
  kind: string;
  ontology_id: string;
  labels: string[];
  comments: string[];
  deprecated: boolean;
  source_location?: {
    line?: number;
    column?: number;
    start_byte?: number;
    end_byte?: number;
  };
}

export interface SubclassEdge {
  child: string;
  parent: string;
}

export interface ClassHierarchy {
  edges: SubclassEdge[];
  parents: Record<string, string[]>;
  children: Record<string, string[]>;
}

export interface DiagnosticSummary {
  code: string;
  severity: string;
  message: string;
  file: string;
  line?: number;
  column?: number;
  entity_iri?: string;
}

export interface CatalogSnapshot {
  documents: OntologyDocument[];
  entities: Entity[];
  hierarchy: ClassHierarchy;
  diagnostics: DiagnosticSummary[];
}

export interface SourceHint {
  path: string;
  line: number;
  column: number;
}

export interface EntityAxiomSummary {
  kind: string;
  display: string;
  manchester?: string;
  parent_iri?: string;
  editable: boolean;
}

export interface EntityDetail {
  entity: Entity;
  parents: string[];
  children: string[];
  axioms: EntityAxiomSummary[];
  source?: SourceHint;
  editable: boolean;
  document_path?: string;
}

export interface IndexWorkspaceParams {
  workspace_uri?: string;
}

export interface IndexWorkspaceResult {
  stats: CatalogStats;
  indexed_at: number;
}

export interface GetEntityResult {
  detail: EntityDetail;
}

export type PatchOp =
  | { op: "create_entity"; entity_iri: string; kind: PatchEntityKind }
  | { op: "delete_entity"; entity_iri: string }
  | { op: "set_label"; entity_iri: string; value: string }
  | { op: "add_label"; entity_iri: string; value: string }
  | { op: "remove_label"; entity_iri: string; value: string }
  | { op: "set_comment"; entity_iri: string; value: string }
  | { op: "add_comment"; entity_iri: string; value: string }
  | { op: "remove_comment"; entity_iri: string; value: string }
  | { op: "add_sub_class_of"; entity_iri: string; parent_iri: string }
  | { op: "remove_sub_class_of"; entity_iri: string; parent_iri: string }
  | { op: "add_complex_sub_class_of"; entity_iri: string; manchester: string }
  | { op: "remove_complex_sub_class_of"; entity_iri: string; manchester: string }
  | { op: "add_equivalent_class"; entity_iri: string; manchester: string }
  | { op: "remove_equivalent_class"; entity_iri: string; manchester: string }
  | { op: "set_equivalent_class"; entity_iri: string; manchester: string }
  | { op: "set_deprecated"; entity_iri: string; value: boolean };

export type PatchEntityKind =
  | "class"
  | "object_property"
  | "data_property"
  | "annotation_property"
  | "individual";

export interface PatchDiagnostic {
  severity: string;
  message: string;
}

export interface ApplyPatchResult {
  applied: boolean;
  preview_text?: string;
  diagnostics?: PatchDiagnostic[];
  document_path?: string;
  entity_detail?: EntityDetail;
}

export interface ApplyAxiomPatchParams {
  document_uri: string;
  patches: PatchOp[];
  preview_only?: boolean;
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

export interface ParseManchesterParams {
  expression: string;
  axiom_kind: string;
  entity_iri?: string;
  document_uri?: string;
}

export interface ParseManchesterResult {
  normalized: string;
  turtle_fragment: string;
  tree: unknown;
  diagnostics: PatchDiagnostic[];
  completions: ManchesterCompletions;
}

export interface SavedQuery {
  name: string;
  mode: "sql" | "sparql";
  text: string;
}
