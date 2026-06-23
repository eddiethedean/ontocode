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
    Array.isArray(e.comments) &&
    typeof e.deprecated === "boolean"
  );
}

function isOntologyDocument(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const d = value as Record<string, unknown>;
  return (
    typeof d.id === "string" &&
    typeof d.path === "string" &&
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

export function assertIndexWorkspaceResult(
  value: unknown
): { stats: { class_count: number; error_count: number }; indexed_at: number } {
  if (!isIndexWorkspaceResult(value)) {
    throw new Error("Invalid index workspace result from language server");
  }
  return value;
}
