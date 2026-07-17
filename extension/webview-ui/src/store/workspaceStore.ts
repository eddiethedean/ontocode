import { useEffect } from "react";
import { create } from "zustand";
import type { RefactorPlanPayload } from "../messages";
import { emitWorkspaceEvent, subscribeWorkspaceEvents } from "./events";
import type {
  CurrentFocus,
  QueryHistoryEntry,
  QueryResultSnapshot,
  WorkspaceStoreState,
} from "./types";

export const initialWorkspaceState: WorkspaceStoreState = {
  focus: null,
  selection: { items: [] },
  navigation: { stack: [], index: -1 },
  query: {
    language: "sql",
    text: "",
    lastResult: null,
    history: [],
    schemaBrowserExpanded: true,
  },
  reasoning: {
    profile: "auto",
    lastRunAt: null,
    unsatisfiable: [],
    hierarchyMode: "asserted",
  },
  refactoring: { pending: null },
  inspector: { entityIri: null },
  graph: { rootIri: null },
  explorer: { highlightedIri: null },
  plugins: { installed: [], active: [] },
};

export interface WorkspaceStoreActions {
  setFocus: (focus: Omit<CurrentFocus, "timestamp"> & { timestamp?: number }) => void;
  setSelection: (items: string[]) => void;
  navigationBack: () => CurrentFocus | null;
  navigationForward: () => CurrentFocus | null;
  pushNavigation: (focus: CurrentFocus) => void;
  setQueryLanguage: (language: "sql" | "sparql" | "dl") => void;
  setQueryText: (text: string) => void;
  setQueryResult: (result: QueryResultSnapshot | null) => void;
  addQueryHistory: (entry: Omit<QueryHistoryEntry, "id" | "executedAt">) => void;
  setSchemaBrowserExpanded: (expanded: boolean) => void;
  setReasoningProfile: (profile: string) => void;
  setReasoningRunning: (running: boolean, profile?: string) => void;
  applyReasoningState: (payload: import("../messages").ReasoningStatePayload) => void;
  setReasoningResult: (unsatisfiable: string[], profile?: string) => void;
  setHierarchyMode: (mode: "asserted" | "inferred" | "combined") => void;
  setPendingRefactor: (plan: RefactorPlanPayload | null) => void;
  setInspectorEntityIri: (iri: string | null) => void;
  setGraphRootIri: (iri: string | null) => void;
  setExplorerHighlight: (iri: string | null) => void;
  setPlugins: (installed: import("./types").PluginDescriptorState[]) => void;
  hydrateFocus: (focus: CurrentFocus | null) => void;
  reset: () => void;
}

export type WorkspaceStore = WorkspaceStoreState & WorkspaceStoreActions;

function withTimestamp(
  focus: Omit<CurrentFocus, "timestamp"> & { timestamp?: number }
): CurrentFocus {
  return {
    ...focus,
    timestamp: focus.timestamp ?? Date.now(),
  };
}

/** Sync inspector/graph/explorer slices when focus is an entity (#352). */
function entityFocusSliceUpdates(
  focus: CurrentFocus
): Pick<WorkspaceStoreState, "inspector" | "graph" | "explorer"> | null {
  if (focus.kind !== "entity") {
    return null;
  }
  return {
    inspector: { entityIri: focus.id },
    graph: { rootIri: focus.id },
    explorer: { highlightedIri: focus.id },
  };
}

export const useWorkspaceStore = create<WorkspaceStore>((set, get) => ({
  ...initialWorkspaceState,

  setFocus(focusInput) {
    const focus = withTimestamp(focusInput);
    const slices = entityFocusSliceUpdates(focus);
    set({ focus, ...(slices ?? {}) });
    get().pushNavigation(focus);
    emitWorkspaceEvent({ type: "FocusChanged", focus });
  },

  setSelection(items) {
    set({ selection: { items } });
  },

  pushNavigation(focus) {
    const { navigation } = get();
    const truncated = navigation.stack.slice(0, navigation.index + 1);
    truncated.push({ focus });
    set({ navigation: { stack: truncated, index: truncated.length - 1 } });
  },

  navigationBack() {
    const { navigation } = get();
    if (navigation.index <= 0) {
      return null;
    }
    const index = navigation.index - 1;
    const focus = navigation.stack[index]?.focus ?? null;
    const slices = focus ? entityFocusSliceUpdates(focus) : null;
    set({ navigation: { ...navigation, index }, focus, ...(slices ?? {}) });
    if (focus) {
      emitWorkspaceEvent({ type: "FocusChanged", focus });
    }
    return focus;
  },

  navigationForward() {
    const { navigation } = get();
    if (navigation.index >= navigation.stack.length - 1) {
      return null;
    }
    const index = navigation.index + 1;
    const focus = navigation.stack[index]?.focus ?? null;
    const slices = focus ? entityFocusSliceUpdates(focus) : null;
    set({ navigation: { ...navigation, index }, focus, ...(slices ?? {}) });
    if (focus) {
      emitWorkspaceEvent({ type: "FocusChanged", focus });
    }
    return focus;
  },

  setQueryLanguage(language) {
    set((state) => ({ query: { ...state.query, language } }));
  },

  setQueryText(text) {
    set((state) => ({ query: { ...state.query, text } }));
  },

  setQueryResult(result) {
    const { query } = get();
    set({ query: { ...query, lastResult: result } });
    if (result) {
      emitWorkspaceEvent({ type: "QueryExecuted", result, language: query.language });
    }
  },

  addQueryHistory(entry) {
    const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    const full: QueryHistoryEntry = { ...entry, id, executedAt: Date.now() };
    set((state) => ({
      query: {
        ...state.query,
        history: [full, ...state.query.history].slice(0, 50),
      },
    }));
  },

  setSchemaBrowserExpanded(expanded) {
    set((state) => ({ query: { ...state.query, schemaBrowserExpanded: expanded } }));
  },

  setReasoningProfile(profile) {
    set((state) => ({ reasoning: { ...state.reasoning, profile } }));
  },

  setReasoningRunning(running, profile) {
    set((state) => ({
      reasoning: {
        ...state.reasoning,
        running,
        ...(profile !== undefined ? { profile } : {}),
      },
    }));
  },

  applyReasoningState(payload) {
    const reasoning = get().reasoning;
    const lastRunAt =
      payload.lastRunAt > 0 ? payload.lastRunAt : reasoning.lastRunAt;
    const hierarchyMode = payload.hierarchyMode ?? reasoning.hierarchyMode;
    if (payload.running) {
      set({
        reasoning: {
          ...reasoning,
          profile: payload.profile,
          unsatisfiable: payload.unsatisfiable,
          lastRunAt,
          dirty: payload.dirty,
          running: true,
          hierarchyMode,
        },
      });
      return;
    }
    const next = {
      ...reasoning,
      profile: payload.profile,
      unsatisfiable: payload.unsatisfiable,
      lastRunAt,
      dirty: payload.dirty,
      running: false,
      hierarchyMode,
    };
    set({ reasoning: next });
    if (!payload.dirty) {
      emitWorkspaceEvent({
        type: "ReasoningCompleted",
        profile: next.profile,
        unsatisfiable: next.unsatisfiable,
      });
    }
  },

  setReasoningResult(unsatisfiable, profile) {
    const reasoning = get().reasoning;
    const nextProfile = profile ?? reasoning.profile;
    set({
      reasoning: {
        ...reasoning,
        profile: nextProfile,
        unsatisfiable,
        lastRunAt: Date.now(),
      },
    });
    emitWorkspaceEvent({
      type: "ReasoningCompleted",
      profile: nextProfile,
      unsatisfiable,
    });
  },

  setHierarchyMode(hierarchyMode) {
    set((state) => ({ reasoning: { ...state.reasoning, hierarchyMode } }));
  },

  setPendingRefactor(plan) {
    set({ refactoring: { pending: plan } });
    if (plan) {
      emitWorkspaceEvent({ type: "RefactorPreviewReady" });
    } else {
      emitWorkspaceEvent({ type: "RefactorCleared" });
    }
  },

  setInspectorEntityIri(entityIri) {
    set({ inspector: { entityIri } });
  },

  setGraphRootIri(rootIri) {
    set({ graph: { rootIri } });
  },

  setExplorerHighlight(highlightedIri) {
    set({ explorer: { highlightedIri } });
  },

  setPlugins(installed) {
    set({
      plugins: {
        installed,
        active: installed.map((p) => p.id),
      },
    });
  },

  hydrateFocus(focus) {
    const current = get().focus;
    // Stamped focus wins over missing timestamps and older stamps (#161 / #277).
    if (focus && current && typeof current.timestamp === "number") {
      if (
        typeof focus.timestamp !== "number" ||
        focus.timestamp < current.timestamp
      ) {
        return;
      }
    }
    const slices = focus ? entityFocusSliceUpdates(focus) : null;
    set({ focus, ...(slices ?? {}) });
  },

  reset() {
    set(initialWorkspaceState);
  },
}));

export function subscribeFocus(listener: (focus: CurrentFocus) => void): () => void {
  return subscribeWorkspaceEvents((event) => {
    if (event.type === "FocusChanged") {
      listener(event.focus);
    }
  });
}

export function useFocusEffect(listener: (focus: CurrentFocus) => void): void {
  useEffect(() => subscribeFocus(listener), [listener]);
}
