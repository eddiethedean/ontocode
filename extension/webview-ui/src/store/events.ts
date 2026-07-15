import type { CurrentFocus, QueryResultSnapshot } from "./types";

export type WorkspaceEvent =
  | { type: "FocusChanged"; focus: CurrentFocus }
  | { type: "QueryExecuted"; result: QueryResultSnapshot; language: "sql" | "sparql" | "dl" }
  | { type: "ReasoningCompleted"; profile: string; unsatisfiable: string[] }
  | { type: "RefactorPreviewReady" }
  | { type: "RefactorCleared" };

export type WorkspaceEventListener = (event: WorkspaceEvent) => void;

const listeners = new Set<WorkspaceEventListener>();

export function emitWorkspaceEvent(event: WorkspaceEvent): void {
  for (const listener of listeners) {
    listener(event);
  }
}

export function subscribeWorkspaceEvents(listener: WorkspaceEventListener): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** Clear all listeners (tests only). */
export function resetWorkspaceEventsForTests(): void {
  listeners.clear();
}
