import "./vscodeMock";
import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach, vi } from "vitest";
import { resetVsCodeApiForTests } from "../vscodeApi";
import { vscodePostMessage } from "./vscodeMock";
import "../styles/global.css";

class ResizeObserverMock {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
}

beforeEach(() => {
  resetVsCodeApiForTests();
  vscodePostMessage.mockClear();
  window.history.replaceState({}, "", "/");

  vi.stubGlobal("ResizeObserver", ResizeObserverMock);

  HTMLElement.prototype.getBoundingClientRect = () =>
    ({
      width: 800,
      height: 600,
      top: 0,
      left: 0,
      bottom: 600,
      right: 800,
      x: 0,
      y: 0,
      toJSON: () => ({}),
    }) as DOMRect;
});

afterEach(() => {
  vi.restoreAllMocks();
  vi.useRealTimers();
});
