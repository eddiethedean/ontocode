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
});
