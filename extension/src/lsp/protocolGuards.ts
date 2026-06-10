import { CatalogSnapshot } from "./protocol";

export function isCatalogSnapshot(value: unknown): value is CatalogSnapshot {
  if (!value || typeof value !== "object") {
    return false;
  }

  const snapshot = value as Record<string, unknown>;
  if (!Array.isArray(snapshot.documents) || !Array.isArray(snapshot.entities)) {
    return false;
  }

  const hierarchy = snapshot.hierarchy;
  if (!hierarchy || typeof hierarchy !== "object") {
    return false;
  }

  const h = hierarchy as Record<string, unknown>;
  return Array.isArray(h.edges) && typeof h.parents === "object" && h.parents !== null;
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
