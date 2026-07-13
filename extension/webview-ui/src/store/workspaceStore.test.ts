import { describe, it, expect, beforeEach } from "vitest";
import {
  initialWorkspaceState,
  resetWorkspaceEventsForTests,
  subscribeWorkspaceEvents,
  useWorkspaceStore,
} from "./index";

describe("workspaceStore", () => {
  beforeEach(() => {
    useWorkspaceStore.getState().reset();
    resetWorkspaceEventsForTests();
  });

  it("initializes with null focus", () => {
    expect(useWorkspaceStore.getState().focus).toBeNull();
    expect(useWorkspaceStore.getState().selection.items).toEqual([]);
  });

  it("setFocus updates focus and related slices", () => {
    useWorkspaceStore.getState().setFocus({
      kind: "entity",
      id: "http://example.org#A",
      source: "test",
    });
    const state = useWorkspaceStore.getState();
    expect(state.focus?.id).toBe("http://example.org#A");
    expect(state.inspector.entityIri).toBe("http://example.org#A");
    expect(state.graph.rootIri).toBe("http://example.org#A");
  });

  it("setSelection updates selection items", () => {
    useWorkspaceStore.getState().setSelection(["a", "b"]);
    expect(useWorkspaceStore.getState().selection.items).toEqual(["a", "b"]);
  });

  it("emits FocusChanged on setFocus", () => {
    const events: string[] = [];
    subscribeWorkspaceEvents((e) => {
      if (e.type === "FocusChanged") {
        events.push(e.focus.id);
      }
    });
    useWorkspaceStore.getState().setFocus({
      kind: "entity",
      id: "http://example.org#B",
      source: "test",
    });
    expect(events).toEqual(["http://example.org#B"]);
  });

  it("navigation back returns previous focus", () => {
    const store = useWorkspaceStore.getState();
    store.setFocus({ kind: "entity", id: "A", source: "t" });
    store.setFocus({ kind: "entity", id: "B", source: "t" });
    const prev = store.navigationBack();
    expect(prev?.id).toBe("A");
    expect(useWorkspaceStore.getState().focus?.id).toBe("A");
  });

  it("reset restores initial state", () => {
    useWorkspaceStore.getState().setFocus({
      kind: "entity",
      id: "X",
      source: "t",
    });
    useWorkspaceStore.getState().reset();
    expect(useWorkspaceStore.getState().focus).toBeNull();
    expect(useWorkspaceStore.getState().query).toEqual(initialWorkspaceState.query);
  });

  it("hydrateFocus updates slices without navigation push", () => {
    const store = useWorkspaceStore.getState();
    store.setFocus({ kind: "entity", id: "A", source: "panel", timestamp: 50 });
    const stackLen = useWorkspaceStore.getState().navigation.stack.length;
    store.hydrateFocus({
      kind: "entity",
      id: "http://example.org#Relayed",
      source: "explorer",
      timestamp: 99,
    });
    expect(useWorkspaceStore.getState().focus?.id).toBe("http://example.org#Relayed");
    expect(useWorkspaceStore.getState().inspector.entityIri).toBe("http://example.org#Relayed");
    expect(useWorkspaceStore.getState().navigation.stack.length).toBe(stackLen);
  });

  it("hydrateFocus ignores older timestamps", () => {
    const store = useWorkspaceStore.getState();
    store.setFocus({
      kind: "entity",
      id: "http://example.org#Newer",
      source: "panel",
      timestamp: 100,
    });
    store.hydrateFocus({
      kind: "entity",
      id: "http://example.org#Stale",
      source: "explorer",
      timestamp: 50,
    });
    expect(useWorkspaceStore.getState().focus?.id).toBe("http://example.org#Newer");
  });

  it("setQueryResult emits QueryExecuted", () => {
    const languages: string[] = [];
    subscribeWorkspaceEvents((e) => {
      if (e.type === "QueryExecuted") {
        languages.push(e.language);
      }
    });
    useWorkspaceStore.getState().setQueryResult({
      columns: ["a"],
      rows: [{ a: "1" }],
      truncated: false,
    });
    expect(languages).toEqual(["sql"]);
  });

  it("reasoning and refactor slices update and emit events", () => {
    const events: string[] = [];
    subscribeWorkspaceEvents((e) => events.push(e.type));
    const store = useWorkspaceStore.getState();
    store.setReasoningResult(["http://ex#Bad"], "el");
    store.setHierarchyMode("inferred");
    store.setPendingRefactor({
      planId: "p1",
      title: "Rename",
      changes: [],
    } as never);
    const after = useWorkspaceStore.getState();
    expect(after.reasoning.unsatisfiable).toEqual(["http://ex#Bad"]);
    expect(after.reasoning.hierarchyMode).toBe("inferred");
    expect(events).toContain("ReasoningCompleted");
    expect(events).toContain("RefactorPreviewReady");
    store.setPendingRefactor(null);
    expect(events).toContain("RefactorCleared");
  });

  it("applyReasoningState running preserves lastRunAt (#220)", () => {
    useWorkspaceStore.getState().setReasoningResult(["http://ex#Bad"], "el");
    const before = useWorkspaceStore.getState().reasoning.lastRunAt;
    useWorkspaceStore.getState().applyReasoningState({
      profile: "rl",
      unsatisfiable: ["http://ex#Bad"],
      lastRunAt: before ?? 0,
      dirty: true,
      running: true,
    });
    const after = useWorkspaceStore.getState().reasoning;
    expect(after.running).toBe(true);
    expect(after.lastRunAt).toBe(before);
    expect(after.profile).toBe("rl");
  });

  it("applyReasoningState dirty cancel does not emit ReasoningCompleted (#221)", () => {
    const events: string[] = [];
    subscribeWorkspaceEvents((e) => events.push(e.type));
    useWorkspaceStore.getState().applyReasoningState({
      profile: "el",
      unsatisfiable: [],
      lastRunAt: 50,
      dirty: true,
      running: false,
    });
    expect(events.filter((t) => t === "ReasoningCompleted")).toHaveLength(0);
    expect(useWorkspaceStore.getState().reasoning.lastRunAt).toBe(50);
    expect(useWorkspaceStore.getState().reasoning.dirty).toBe(true);
  });
});
