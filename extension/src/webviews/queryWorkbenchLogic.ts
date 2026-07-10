import { SavedQuery, TabularQueryResult } from "../lsp/protocol";

export const SQL_TABLES = [
  "ontologies",
  "classes",
  "object_properties",
  "data_properties",
  "annotation_properties",
  "individuals",
  "entities",
  "annotations",
  "axioms",
  "namespaces",
  "imports",
  "diagnostics",
  "properties",
] as const;

export const STARTER_SQL = "SELECT short_name, labels FROM classes";

export const STARTER_SPARQL =
  "PREFIX ex: <http://example.org/people#>\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10";

export function exportResultCsv(result: TabularQueryResult): string {
  const header = result.columns.join(",");
  const lines = result.rows.map((row) =>
    result.columns
      .map((col) => {
        const val = row[col] ?? "";
        const escaped = val.replace(/"/g, '""');
        return /[",\n\r]/.test(val) ? `"${escaped}"` : escaped;
      })
      .join(",")
  );
  return [header, ...lines].join("\n");
}

export function exportResultJson(result: TabularQueryResult): string {
  return JSON.stringify(result, null, 2);
}

export function mergeHistory(
  history: SavedQuery[],
  entry: SavedQuery,
  limit: number
): SavedQuery[] {
  const filtered = history.filter(
    (h) => !(h.mode === entry.mode && h.text === entry.text)
  );
  return [{ name: entry.name, mode: entry.mode, text: entry.text }, ...filtered].slice(
    0,
    limit
  );
}

export function upsertSavedQuery(
  saved: SavedQuery[],
  entry: SavedQuery
): SavedQuery[] {
  const without = saved.filter((s) => s.name !== entry.name);
  return [entry, ...without];
}

/** Returns true when an async query result should be delivered to the webview. */
export function shouldDeliverQueryResult(
  requestedRunId: number,
  activeRunId: number
): boolean {
  return requestedRunId === activeRunId;
}
