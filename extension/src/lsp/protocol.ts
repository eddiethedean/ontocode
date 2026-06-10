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
}

export interface OntologyDocument {
  id: string;
  path: string;
  format: string;
  base_iri?: string;
  parse_status: string;
  parse_message?: string;
}

export interface Entity {
  iri: string;
  short_name: string;
  kind: string;
  ontology_id: string;
  labels: string[];
  comments: string[];
  deprecated: boolean;
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

export interface CatalogSnapshot {
  documents: OntologyDocument[];
  entities: Entity[];
  hierarchy: ClassHierarchy;
}

export interface SourceHint {
  path: string;
  line: number;
  column: number;
}

export interface EntityDetail {
  entity: Entity;
  parents: string[];
  children: string[];
  axioms: string[];
  source?: SourceHint;
}

export interface IndexWorkspaceResult {
  stats: CatalogStats;
  indexed_at: number;
}

export interface GetEntityResult {
  detail: EntityDetail;
}
