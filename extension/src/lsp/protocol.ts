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
  obo_id?: string;
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
  reasoner?: ReasonerSnapshot;
  stats?: CatalogStats;
  active_ontology_id?: string;
}

export type HierarchyMode = "asserted" | "inferred" | "combined";

export interface ReasonerWarning {
  code: string;
  message: string;
  suggested_profile?: string;
}

export interface InferredHierarchy {
  edges: SubclassEdge[];
  unsatisfiable: string[];
  combined: ClassHierarchy;
}

export interface ReasonerSnapshot {
  profile_used: string;
  consistent: boolean;
  unsatisfiable: string[];
  inferred: InferredHierarchy;
  new_inferences: SubclassEdge[];
  warnings: ReasonerWarning[];
  duration_ms: number;
  classified_at: number;
  consistency?: {
    consistent: boolean;
    complete: boolean;
    ontology_consistent: boolean;
    abox_clashes: string[];
    unsatisfiable: string[];
  };
  realization?: {
    profile_used: string;
    individuals: Array<{
      individual_iri: string;
      types: string[];
      most_specific: string[];
      asserted: string[];
      inferred: string[];
    }>;
    duration_ms: number;
  };
  inferred_assertions?: {
    class_assertions: Array<{ individual_iri: string; class_iri: string }>;
    object_property_assertions: Array<{
      subject_iri: string;
      property_iri: string;
      object_iri: string;
    }>;
    same_as_clusters: Array<{ individuals: string[] }>;
  };
}

export interface CheckInstanceParams {
  individual_iri: string;
  class_iri: string;
  profile?: string;
}

export interface CheckInstanceResult {
  individual_iri: string;
  class_iri: string;
  entailed: boolean;
  profile_used: string;
  duration_ms: number;
}

export interface DlQueryParams {
  expression: string;
  profile?: string;
  /** `inferred` (default) or `asserted` */
  mode?: "inferred" | "asserted" | string;
  document_uri?: string;
}

export interface DlQueryResult {
  expression: string;
  normalized: string;
  query_class_iri: string;
  subclasses: string[];
  superclasses: string[];
  equivalents: string[];
  instances: string[];
  profile: string;
  mode: string;
  duration_ms: number;
  warnings?: string[];
  diagnostics?: string[];
}

export interface SearchParams {
  query: string;
  limit?: number;
}

export interface SearchResult {
  entities: EntityDetail[];
}

export interface RunReasonerParams {
  profile?: string;
  auto_detect?: boolean;
}

export interface RunReasonerResult {
  profile_used: string;
  consistent: boolean;
  unsatisfiable: string[];
  inferred_edge_count: number;
  new_inferences: SubclassEdge[];
  warnings: ReasonerWarning[];
  duration_ms: number;
  snapshot: ReasonerSnapshot;
}

export interface GetExplanationParams {
  class_iri: string;
  profile?: string;
}

export interface ExplanationStep {
  index: number;
  rule: string;
  display: string;
  subject_iri?: string;
  object_iri?: string;
}

export interface GetExplanationResult {
  class_iri: string;
  steps: ExplanationStep[];
  text: string;
  alternatives?: Array<{
    class_iri: string;
    steps: ExplanationStep[];
    text: string;
  }>;
  indexed_at: number;
  content_hash: string;
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
  other_iri?: string;
  /** Property / relation IRI for assertion-like axioms. */
  predicate?: string;
  /** Member property IRIs for property_chain / has_key / disjoint_union (ordered). */
  properties?: string[];
  /** Annotations attached to this axiom. */
  annotations?: EntityAnnotationSummary[];
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

export interface EntityDetail {
  entity: Entity;
  parents: string[];
  children: string[];
  axioms: EntityAxiomSummary[];
  annotations?: EntityAnnotationSummary[];
  characteristics?: PropertyCharacteristics;
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

export interface GraphFilters {
  ontology_iri?: string;
  hide_deprecated?: boolean;
}

export interface GetGraphParams {
  graph_kind: string;
  root_iri?: string;
  depth?: number;
  include_inferred?: boolean;
  filters?: GraphFilters;
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

export interface GetGraphResult {
  graph: GraphPayload;
}

export interface RunRobotParams {
  subcommand: string;
  args?: string[];
  robot_path?: string;
}

export interface RunRobotResult {
  exit_code: number;
  stdout: string;
  stderr: string;
}

export type PatchOp =
  | { op: "add_prefix"; prefix: string; namespace_iri: string }
  | { op: "remove_prefix"; prefix: string }
  | { op: "set_prefix"; prefix: string; namespace_iri: string }
  | { op: "set_ontology_iri"; ontology_iri: string }
  | { op: "set_version_iri"; ontology_iri: string; version_iri: string }
  | { op: "add_ontology_annotation"; ontology_iri: string; predicate: string; value: string }
  | { op: "remove_ontology_annotation"; ontology_iri: string; predicate: string; value: string }
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
  | { op: "set_deprecated"; entity_iri: string; value: boolean }
  | { op: "add_disjoint_class"; entity_iri: string; other_iri: string }
  | { op: "remove_disjoint_class"; entity_iri: string; other_iri: string }
  | { op: "add_import"; ontology_iri: string; import_iri: string }
  | { op: "remove_import"; ontology_iri: string; import_iri: string }
  | { op: "add_domain"; entity_iri: string; class_iri: string }
  | { op: "remove_domain"; entity_iri: string; class_iri: string }
  | { op: "add_range"; entity_iri: string; range_iri: string }
  | { op: "remove_range"; entity_iri: string; range_iri: string }
  | { op: "set_functional"; entity_iri: string; value: boolean }
  | { op: "set_inverse_functional"; entity_iri: string; value: boolean }
  | { op: "set_transitive"; entity_iri: string; value: boolean }
  | { op: "set_symmetric"; entity_iri: string; value: boolean }
  | { op: "set_asymmetric"; entity_iri: string; value: boolean }
  | { op: "set_reflexive"; entity_iri: string; value: boolean }
  | { op: "set_irreflexive"; entity_iri: string; value: boolean }
  | { op: "add_property_chain"; entity_iri: string; properties: string[] }
  | { op: "remove_property_chain"; entity_iri: string; properties: string[] }
  | { op: "add_class_assertion"; entity_iri: string; class_iri: string }
  | { op: "remove_class_assertion"; entity_iri: string; class_iri: string }
  | { op: "add_object_property_assertion"; entity_iri: string; property_iri: string; target_iri: string }
  | { op: "remove_object_property_assertion"; entity_iri: string; property_iri: string; target_iri: string }
  | { op: "add_data_property_assertion"; entity_iri: string; property_iri: string; value: string }
  | { op: "remove_data_property_assertion"; entity_iri: string; property_iri: string; value: string }
  | { op: "add_annotation"; entity_iri: string; predicate: string; value: string }
  | { op: "remove_annotation"; entity_iri: string; predicate: string; value: string }
  | { op: "add_has_key"; class_iri: string; properties: string[] }
  | { op: "remove_has_key"; class_iri: string; properties: string[] }
  | { op: "add_disjoint_union"; class_iri: string; members: string[] }
  | { op: "remove_disjoint_union"; class_iri: string; members: string[] }
  | { op: "add_inverse_object_properties"; property_iri: string; inverse_iri: string }
  | { op: "remove_inverse_object_properties"; property_iri: string; inverse_iri: string }
  | { op: "add_equivalent_object_properties"; properties: string[] }
  | { op: "remove_equivalent_object_properties"; properties: string[] }
  | { op: "add_disjoint_object_properties"; properties: string[] }
  | { op: "remove_disjoint_object_properties"; properties: string[] }
  | { op: "add_equivalent_data_properties"; properties: string[] }
  | { op: "remove_equivalent_data_properties"; properties: string[] }
  | { op: "add_disjoint_data_properties"; properties: string[] }
  | { op: "remove_disjoint_data_properties"; properties: string[] }
  | { op: "add_sub_object_property_of"; property_iri: string; parent_iri: string }
  | { op: "remove_sub_object_property_of"; property_iri: string; parent_iri: string }
  | { op: "add_sub_data_property_of"; property_iri: string; parent_iri: string }
  | { op: "remove_sub_data_property_of"; property_iri: string; parent_iri: string }
  | {
      op: "add_negative_object_property_assertion";
      entity_iri: string;
      property_iri: string;
      target_iri: string;
    }
  | {
      op: "remove_negative_object_property_assertion";
      entity_iri: string;
      property_iri: string;
      target_iri: string;
    }
  | {
      op: "add_negative_data_property_assertion";
      entity_iri: string;
      property_iri: string;
      value: string;
    }
  | {
      op: "remove_negative_data_property_assertion";
      entity_iri: string;
      property_iri: string;
      value: string;
    }
  | { op: "add_same_individual"; individuals: string[] }
  | { op: "remove_same_individual"; individuals: string[] }
  | { op: "add_different_individuals"; individuals: string[] }
  | { op: "remove_different_individuals"; individuals: string[] }
  | { op: "add_datatype_definition"; datatype_iri: string; manchester: string }
  | { op: "remove_datatype_definition"; datatype_iri: string; manchester: string }
  | {
      op: "add_axiom_annotation";
      axiom_op: string;
      subject_iri: string;
      related_iri?: string;
      predicate: string;
      value: string;
    }
  | {
      op: "remove_axiom_annotation";
      axiom_op: string;
      subject_iri: string;
      related_iri?: string;
      predicate: string;
      value: string;
    }
  | { op: "set_name"; term_id: string; value: string }
  | { op: "add_synonym"; term_id: string; value: string; scope: string }
  | { op: "remove_synonym"; term_id: string; value: string }
  | { op: "add_def"; term_id: string; value: string }
  | { op: "remove_def"; term_id: string }
  | { op: "add_xref"; term_id: string; xref: string }
  | { op: "remove_xref"; term_id: string; xref: string }
  | { op: "set_namespace"; term_id: string; namespace: string }
  | { op: "add_is_a"; term_id: string; parent_id: string }
  | { op: "remove_is_a"; term_id: string; parent_id: string };

export type PatchEntityKind =
  | "class"
  | "object_property"
  | "data_property"
  | "annotation_property"
  | "individual"
  | "datatype";

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
  reindex_warning?: string;
  workspace_edit?: LspWorkspaceEdit;
  /** Inverted patch ops for workspace-level semantic undo (v0.20+). */
  undo_patches?: Array<PatchOp | ({ op: string } & Record<string, unknown>)>;
}

/** Client-side result after optional VS Code workspace edit sync. */
export interface ApplyAxiomPatchClientResult extends ApplyPatchResult {
  /** False when disk write succeeded but the user cancelled editor sync. */
  editor_synced: boolean;
}

export interface ApplyAxiomPatchParams {
  document_uri: string;
  patches: Array<PatchOp | ({ op: string } & Record<string, unknown>)>;
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

export interface SwrlDiagnostic {
  code: string;
  severity: "error" | "warning" | "info";
  message: string;
}

export interface SwrlRuleListItem {
  id: string;
  label: string;
  body_count: number;
  head_count: number;
  enabled: boolean;
  rule_json?: string;
  document_uri?: string;
  ontology_iri?: string;
}

export interface ListSwrlRulesResult {
  rules: SwrlRuleListItem[];
}

export interface ValidateSwrlRuleParams {
  rule_json: string;
}

export interface ValidateSwrlRuleResult {
  diagnostics: SwrlDiagnostic[];
}

export interface ParseSwrlRuleParams {
  rule_json: string;
}

export interface ParseSwrlRuleResult {
  rule: unknown;
  diagnostics: SwrlDiagnostic[];
}

export interface SavedQuery {
  name: string;
  mode: "sql" | "sparql" | "dl";
  text: string;
  /** Asserted vs inferred for DL queries; ignored for SQL/SPARQL. */
  dlMode?: "asserted" | "inferred";
}

export interface UsageSummary {
  iri: string;
  referenced_iri: string;
  file: string;
  line?: number;
  column?: number;
  kind: string;
  context: string;
}

export interface FindUsagesResult {
  usages: UsageSummary[];
}

export interface RefactorHunk {
  start_byte: number;
  end_byte: number;
  old_text: string;
  new_text: string;
}

export interface RefactorFileChange {
  path: string;
  preview_text: string;
  original_text: string;
  hunks: RefactorHunk[];
}

export interface RefactorPlan {
  changes: RefactorFileChange[];
  warnings?: string[];
  affected_entity_count?: number;
  affected_axiom_count?: number;
}

export type RefactorRequest =
  | { kind: "rename_iri"; from_iri: string; to_iri: string }
  | { kind: "merge_entities"; keep_iri: string; merge_iri: string }
  | { kind: "replace_entity"; from_iri: string; to_iri: string }
  | { kind: "migrate_namespace"; from_base: string; to_base: string }
  | { kind: "move_entity"; entity_iri: string; target_file: string }
  | {
      kind: "extract_module";
      entity_iris: string[];
      output_file: string;
      leave_stub?: boolean;
      locality?: boolean;
    }
  | { kind: "merge_ontologies"; source_paths: string[]; target_file: string }
  | { kind: "flatten_imports"; ontology_file: string }
  | { kind: "cleanup_imports"; ontology_file: string }
  | {
      kind: "move_axioms";
      entity_iri: string;
      target_file: string;
      statement_indexes?: number[];
      exclude_primary?: boolean;
    };

export interface PreviewRefactorResult {
  changes: RefactorFileChange[];
  warnings?: string[];
  affected_entity_count?: number;
  affected_axiom_count?: number;
}

export interface ApplyRefactorResult {
  files_written: number;
  reindex_warning?: string;
  workspace_edit?: LspWorkspaceEdit;
}

export interface DiffSummaryCounts {
  entities: number;
  axioms: number;
  annotations: number;
  imports: number;
  inferences: number;
  breaking: number;
}

export interface BreakingChangeSummary {
  reason: string;
  message: string;
  entity_iri?: string;
}

export interface EntityChangeSummary {
  kind: string;
  iri: string;
  previous_iri?: string;
  labels?: string[];
}

export interface AxiomChangeSummary {
  change: string;
  subject: string;
  predicate: string;
  object: string;
  axiom_kind: string;
}

export interface DiffPayload {
  entity_changes: EntityChangeSummary[];
  axiom_changes: AxiomChangeSummary[];
  annotation_changes: Array<{
    change: string;
    subject: string;
    predicate: string;
    object: string;
  }>;
  import_changes: Array<{
    change: string;
    ontology_id: string;
    import_iri: string;
  }>;
  inference_changes: Array<{
    class_iri: string;
    change: string;
    detail: string;
  }>;
  breaking_changes: BreakingChangeSummary[];
  summary_counts?: DiffSummaryCounts;
}

export interface SemanticDiffParams {
  left_ref?: string;
  right_ref?: string;
  left_path?: string;
  right_path?: string;
  /** When true, merge unsatisfiable-class changes from reasoner classification. */
  reasoner?: boolean;
}

export interface SemanticDiffResult {
  diff: DiffPayload;
}

export interface LspPosition {
  line: number;
  character: number;
}

export interface LspRange {
  start: LspPosition;
  end: LspPosition;
}

export interface LspTextEdit {
  range: LspRange;
  new_text?: string;
  newText?: string;
}

export interface LspTextDocumentEdit {
  text_document: { uri: string; version?: number | null };
  edits: LspTextEdit[];
}

export interface LspWorkspaceEdit {
  document_changes?: LspTextDocumentEdit[];
  change_annotations?: Record<
    string,
    { label: string; needs_confirmation?: boolean; description?: string }
  >;
}

export type CommandEnablement =
  | "always"
  | "has_ontology"
  | "is_dirty"
  | "has_selection"
  | "reasoner_running"
  | "reasoner_idle"
  | "can_edit_selection";

export interface CommandDescriptor {
  id: string;
  title: string;
  category: string;
  enablement?: CommandEnablement[];
  undo_label?: string;
  dialog_id?: string;
}

export interface ListCommandsResult {
  commands: CommandDescriptor[];
}

export interface OntologyRegistryEntrySnapshot {
  id: string;
  uri: string;
  path: string;
  format: string;
  role: "root" | "import" | "scratch";
  editable: boolean;
  dirty: boolean;
  version: number;
  active: boolean;
}

export interface WorkspaceUiStateParams {
  selection_iri?: string;
  dirty_document_count?: number;
  active_ontology_id?: string;
  ontology_registry?: OntologyRegistryEntrySnapshot[];
}

export interface WorkspaceUiState {
  has_ontology: boolean;
  ontology_count: number;
  is_dirty: boolean;
  has_selection: boolean;
  selection_iri?: string;
  selection_editable: boolean;
  reasoner_running: boolean;
  reasoner_dirty: boolean;
  reasoner_consistent?: boolean;
  active_ontology_id?: string;
  stats?: CatalogStats;
  ontology_registry?: OntologyRegistryEntrySnapshot[];
}

export interface DialogFieldSchema {
  id: string;
  label: string;
  field_type: string;
  required: boolean;
  placeholder?: string;
  validation?: string[];
}

export interface DialogSchema {
  id: string;
  title: string;
  fields: DialogFieldSchema[];
  primary_action: string;
}

export interface GetDialogSchemaResult {
  schema: DialogSchema;
}

export interface CreateOntologyParams {
  path: string;
  ontology_iri: string;
  version_iri?: string;
  format?: string;
  prefixes?: Record<string, string>;
}

export interface CreateOntologyResult {
  path: string;
  ontology_iri: string;
}

export interface ExportOntologyParams {
  source_path: string;
  output_path: string;
  format?: string;
}

export interface ExportOntologyResult {
  output_path: string;
  success: boolean;
  logs?: string;
}

export interface SetActiveOntologyParams {
  ontology_id: string;
}

export interface SetActiveOntologyResult {
  active_ontology_id: string;
}

export interface DeleteImpactParams {
  entity_iri: string;
}

export interface DeleteImpactResult {
  entity_iri: string;
  usage_count: number;
  axiom_count: number;
  referencing_entities: string[];
  warnings: string[];
}

export interface PluginCommandContribution {
  id: string;
  title: string;
  scope?: string;
}

export interface PluginViewContribution {
  id: string;
  title: string;
  kind?: string;
  command?: string;
}

export interface PluginPreferencePageContribution {
  id: string;
  title: string;
  category?: string;
}

export interface PluginContextActionContribution {
  id: string;
  title: string;
  scope?: string;
  applies_to?: string[];
  command: string;
}

export interface PluginInspectorCard {
  id: string;
  title: string;
  applies_to?: string[];
  command?: string;
}

export interface PluginUiContributions {
  commands: PluginCommandContribution[];
  views: PluginViewContribution[];
  preferences_pages: PluginPreferencePageContribution[];
  context_actions: PluginContextActionContribution[];
  inspector_cards: PluginInspectorCard[];
}

export interface PluginCapabilities {
  build: boolean;
  validate: boolean;
  release: boolean;
  diagnostics: boolean;
  export: boolean;
}

export interface PluginDescriptor {
  id: string;
  name: string;
  version: string;
  kind: string;
  api_version?: string;
  permissions?: string[];
  capabilities: PluginCapabilities;
  manifest_path: string;
  ui: PluginUiContributions;
  in_process: boolean;
  disabled?: boolean;
}

export interface ListPluginsResult {
  plugins: PluginDescriptor[];
}

export interface RunPluginParams {
  plugin_id: string;
  action?: string;
  step?: string;
  view_id?: string;
}

export interface RunPluginResult {
  diagnostics: DiagnosticSummary[];
  output_paths?: string[];
  logs?: string;
  view_html?: string;
  success: boolean;
}
