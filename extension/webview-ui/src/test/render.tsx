import { render, type RenderOptions } from "@testing-library/react";
import type { ReactElement } from "react";
import { HostProvider } from "../context/HostContext";
import { resetWorkspaceEventsForTests } from "../store/events";
import { useWorkspaceStore, initialWorkspaceState } from "../store/workspaceStore";

export function resetWorkspaceStoreForTests(): void {
  resetWorkspaceEventsForTests();
  useWorkspaceStore.setState(initialWorkspaceState);
}

export function renderWithProviders(
  ui: ReactElement,
  options?: Omit<RenderOptions, "wrapper">
) {
  return render(ui, {
    wrapper: ({ children }) => <HostProvider>{children}</HostProvider>,
    ...options,
  });
}
