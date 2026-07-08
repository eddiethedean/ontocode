import { useEffect } from "react";
import { useWorkspaceHost } from "../context/HostContext";
import { isHostMessage } from "../messages";
import { useWorkspaceStore } from "../store";

/** Hydrates WorkspaceStore from extension-host focus/reasoning relay messages. */
export function FocusSyncBootstrap(): null {
  const host = useWorkspaceHost();
  const hydrateFocus = useWorkspaceStore((s) => s.hydrateFocus);
  const setReasoningResult = useWorkspaceStore((s) => s.setReasoningResult);
  const setHierarchyMode = useWorkspaceStore((s) => s.setHierarchyMode);

  useEffect(() => {
    return host.onMessage((data) => {
      if (!isHostMessage(data)) {
        return;
      }
      if (data.type === "focusState") {
        hydrateFocus(data.focus);
      }
      if (data.type === "reasoningState") {
        setReasoningResult(data.reasoning.unsatisfiable, data.reasoning.profile);
        if (data.reasoning.hierarchyMode) {
          setHierarchyMode(data.reasoning.hierarchyMode);
        }
      }
    });
  }, [host, hydrateFocus, setReasoningResult, setHierarchyMode]);

  return null;
}

/** Post focus updates to the extension host relay. */
export function relayFocus(
  focus: Omit<import("../store/types").CurrentFocus, "timestamp"> & {
    timestamp?: number;
  }
): void {
  const full = {
    ...focus,
    timestamp: focus.timestamp ?? Date.now(),
  };
  useWorkspaceStore.getState().hydrateFocus(full);
  // Avoid double-navigation push when relaying from host
  const store = useWorkspaceStore.getState();
  store.hydrateFocus(full);
}

export function postFocusToHost(
  host: ReturnType<typeof useWorkspaceHost>,
  focus: Omit<import("../store/types").CurrentFocus, "timestamp"> & {
    timestamp?: number;
  }
): void {
  const full = {
    ...focus,
    timestamp: focus.timestamp ?? Date.now(),
  };
  useWorkspaceStore.getState().hydrateFocus(full);
  host.postToCore({ type: "setFocus", focus: full });
}
