import { vi } from "vitest";

export const vscodePostMessage = vi.fn();

vi.stubGlobal("acquireVsCodeApi", () => ({
  postMessage: vscodePostMessage,
  getState: vi.fn(() => undefined),
  setState: vi.fn(),
}));
