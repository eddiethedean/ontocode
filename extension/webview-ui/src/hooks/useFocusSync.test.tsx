import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { FocusSyncBootstrap } from "./useFocusSync";
import { HostProvider } from "../context/HostContext";
import type { WorkspaceHost } from "../host";
import { useWorkspaceStore, subscribeWorkspaceEvents } from "../store";
import { resetWorkspaceStoreForTests } from "../test/render";

describe("FocusSyncBootstrap", () => {
  it("hydrates store from focusState host message", () => {
    resetWorkspaceStoreForTests();
    const listeners: Array<(data: unknown) => void> = [];
    const host: WorkspaceHost = {
      postToCore: () => {},
      getTheme: () => "dark",
      showNotification: () => {},
      onMessage: (handler) => {
        listeners.push(handler);
        return () => {};
      },
    };

    render(
      <HostProvider host={host}>
        <FocusSyncBootstrap />
      </HostProvider>
    );

    const iri = "http://example.org/test#Alice";
    listeners.forEach((fn) =>
      fn({ type: "focusState", focus: { kind: "entity", id: iri, source: "explorer", timestamp: 1 } })
    );

    expect(useWorkspaceStore.getState().focus?.id).toBe(iri);
  });

  it("hydrates reasoning state from host message", () => {
    resetWorkspaceStoreForTests();
    const listeners: Array<(data: unknown) => void> = [];
    const host = {
      postToCore: () => {},
      getTheme: () => "dark" as const,
      showNotification: () => {},
      onMessage: (handler: (data: unknown) => void) => {
        listeners.push(handler);
        return () => {};
      },
    };

    render(
      <HostProvider host={host}>
        <FocusSyncBootstrap />
      </HostProvider>
    );

    listeners.forEach((fn) =>
      fn({
        type: "reasoningState",
        reasoning: {
          unsatisfiable: ["http://example.org#Bad"],
          profile: "el",
          hierarchyMode: "inferred",
        },
      })
    );

    const state = useWorkspaceStore.getState();
    expect(state.reasoning.unsatisfiable).toEqual(["http://example.org#Bad"]);
    expect(state.reasoning.profile).toBe("el");
    expect(state.reasoning.hierarchyMode).toBe("inferred");
  });

  it("reasoningState with running true does not emit completion (#220)", () => {
    const events: string[] = [];
    subscribeWorkspaceEvents((e) => events.push(e.type));
    const listeners: Array<(data: unknown) => void> = [];
    const host = {
      postToCore: () => {},
      getTheme: () => "dark" as const,
      showNotification: () => {},
      onMessage: (handler: (data: unknown) => void) => {
        listeners.push(handler);
        return () => {};
      },
    };

    render(
      <HostProvider host={host}>
        <FocusSyncBootstrap />
      </HostProvider>
    );

    useWorkspaceStore.getState().setReasoningResult(["http://ex#Old"], "el");
    const before = useWorkspaceStore.getState().reasoning.lastRunAt;
    listeners.forEach((fn) =>
      fn({
        type: "reasoningState",
        reasoning: {
          profile: "rl",
          unsatisfiable: ["http://ex#Old"],
          lastRunAt: before ?? 0,
          dirty: true,
          running: true,
        },
      })
    );

    const after = useWorkspaceStore.getState().reasoning;
    expect(after.running).toBe(true);
    expect(after.profile).toBe("rl");
    expect(after.lastRunAt).toBe(before);
    expect(events.filter((t) => t === "ReasoningCompleted").length).toBe(1);
  });
});
