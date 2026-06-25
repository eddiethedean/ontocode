import { CatalogSnapshot } from "./protocol";

const ENTITY_KINDS = new Set([
  "class",
  "object_property",
  "data_property",
  "annotation_property",
  "individual",
  "ontology",
  "other",
]);

const PARSE_STATUSES = new Set(["ok", "warning", "error"]);

function isEntity(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const e = value as Record<string, unknown>;
  return (
    typeof e.iri === "string" &&
    typeof e.short_name === "string" &&
    typeof e.kind === "string" &&
    ENTITY_KINDS.has(e.kind) &&
    typeof e.ontology_id === "string" &&
    Array.isArray(e.labels) &&
    e.labels.every((l) => typeof l === "string") &&
    Array.isArray(e.comments) &&
    e.comments.every((c) => typeof c === "string") &&
    typeof e.deprecated === "boolean"
  );
}

function isHierarchyEdge(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const edge = value as Record<string, unknown>;
  return typeof edge.child === "string" && typeof edge.parent === "string";
}

function isOntologyDocument(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const d = value as Record<string, unknown>;
  return (
    typeof d.id === "string" &&
    typeof d.path === "string" &&
    typeof d.format === "string" &&
    typeof d.parse_status === "string" &&
    PARSE_STATUSES.has(d.parse_status)
  );
}

function isDiagnosticSummary(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const d = value as Record<string, unknown>;
  return (
    typeof d.code === "string" &&
    typeof d.severity === "string" &&
    typeof d.message === "string" &&
    typeof d.file === "string"
  );
}

export function isCatalogSnapshot(value: unknown): value is CatalogSnapshot {
  if (!value || typeof value !== "object") {
    return false;
  }

  const snapshot = value as Record<string, unknown>;
  if (!Array.isArray(snapshot.documents) || !Array.isArray(snapshot.entities)) {
    return false;
  }

  if (!snapshot.documents.every(isOntologyDocument)) {
    return false;
  }
  if (!snapshot.entities.every(isEntity)) {
    return false;
  }

  const hierarchy = snapshot.hierarchy;
  if (!hierarchy || typeof hierarchy !== "object") {
    return false;
  }

  const h = hierarchy as Record<string, unknown>;
  const diagnosticsOk =
    snapshot.diagnostics === undefined ||
    (Array.isArray(snapshot.diagnostics) &&
      snapshot.diagnostics.every(isDiagnosticSummary));

  return (
    diagnosticsOk &&
    Array.isArray(h.edges) &&
    h.edges.every(isHierarchyEdge) &&
    typeof h.parents === "object" &&
    h.parents !== null &&
    typeof h.children === "object" &&
    h.children !== null
  );
}

export function isIndexWorkspaceResult(
  value: unknown
): value is { stats: { class_count: number; error_count: number }; indexed_at: number } {
  if (!value || typeof value !== "object") {
    return false;
  }

  const result = value as Record<string, unknown>;
  const stats = result.stats;
  if (!stats || typeof stats !== "object") {
    return false;
  }

  const s = stats as Record<string, unknown>;
  return typeof s.class_count === "number" && typeof s.error_count === "number";
}

export function assertCatalogSnapshot(value: unknown): CatalogSnapshot {
  if (!isCatalogSnapshot(value)) {
    throw new Error(
      "Invalid catalog snapshot from language server (expected snake_case entity kinds and parse_status values)"
    );
  }
  const snapshot = value as CatalogSnapshot;
  if (!snapshot.diagnostics) {
    snapshot.diagnostics = [];
  }
  return snapshot;
}

export function assertGetEntityResult(value: unknown): import("./protocol").GetEntityResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid getEntity result from language server");
  }
  const result = value as Record<string, unknown>;
  const detail = result.detail;
  if (!detail || typeof detail !== "object") {
    throw new Error("Invalid getEntity result: missing detail");
  }
  const d = detail as Record<string, unknown>;
  if (!d.entity || typeof d.entity !== "object" || !isEntity(d.entity)) {
    throw new Error("Invalid getEntity result: missing entity");
  }
  if (d.parents !== undefined && (!Array.isArray(d.parents) || !d.parents.every((p) => typeof p === "string"))) {
    throw new Error("Invalid getEntity result: parents must be string array");
  }
  if (d.children !== undefined && (!Array.isArray(d.children) || !d.children.every((c) => typeof c === "string"))) {
    throw new Error("Invalid getEntity result: children must be string array");
  }
  return value as import("./protocol").GetEntityResult;
}

export function assertApplyPatchResult(value: unknown): import("./protocol").ApplyPatchResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid applyAxiomPatch result from language server");
  }
  return value as import("./protocol").ApplyPatchResult;
}

export function assertIndexWorkspaceResult(
  value: unknown
): { stats: { class_count: number; error_count: number }; indexed_at: number } {
  if (!isIndexWorkspaceResult(value)) {
    throw new Error("Invalid index workspace result from language server");
  }
  return value;
}

export function assertTabularQueryResult(
  value: unknown
): import("./protocol").TabularQueryResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid query result from language server");
  }
  const v = value as Record<string, unknown>;
  if (!Array.isArray(v.columns) || !Array.isArray(v.rows)) {
    throw new Error("Invalid query result shape");
  }
  return value as import("./protocol").TabularQueryResult;
}

export function assertParseManchesterResult(
  value: unknown
): import("./protocol").ParseManchesterResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid parseManchester result from language server");
  }
  const v = value as Record<string, unknown>;
  if (typeof v.normalized !== "string" || typeof v.turtle_fragment !== "string") {
    throw new Error("Invalid parseManchester result shape");
  }
  return value as import("./protocol").ParseManchesterResult;
}

export function assertRunReasonerResult(
  value: unknown
): import("./protocol").RunReasonerResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid runReasoner result from language server");
  }
  const v = value as Record<string, unknown>;
  if (typeof v.profile_used !== "string" || typeof v.consistent !== "boolean") {
    throw new Error("Invalid runReasoner result shape");
  }
  return value as import("./protocol").RunReasonerResult;
}

export function assertGetExplanationResult(
  value: unknown
): import("./protocol").GetExplanationResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid getExplanation result from language server");
  }
  const v = value as Record<string, unknown>;
  if (typeof v.class_iri !== "string" || !Array.isArray(v.steps)) {
    throw new Error("Invalid getExplanation result shape");
  }
  return value as import("./protocol").GetExplanationResult;
}

export function assertGetGraphResult(
  value: unknown
): import("./protocol").GetGraphResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid getGraph result from language server");
  }
  const v = value as Record<string, unknown>;
  if (!v.graph || typeof v.graph !== "object") {
    throw new Error("Invalid getGraph result shape");
  }
  return value as import("./protocol").GetGraphResult;
}

export function assertRunRobotResult(
  value: unknown
): import("./protocol").RunRobotResult {
  if (!value || typeof value !== "object") {
    throw new Error("Invalid runRobot result from language server");
  }
  const v = value as Record<string, unknown>;
  if (typeof v.exit_code !== "number") {
    throw new Error("Invalid runRobot result shape");
  }
  return value as import("./protocol").RunRobotResult;
}
