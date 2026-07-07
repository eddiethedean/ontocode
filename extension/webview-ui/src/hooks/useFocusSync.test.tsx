import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { FocusSyncBootstrap } from "./useFocusSync";
import { HostProvider } from "../context/HostContext";
import type { WorkspaceHost } from "../host";
import { useWorkspaceStore } from "../store";
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
});
