import { describe, it, expect, beforeEach } from "vitest";
import { getVsCodeApi, resetVsCodeApiForTests } from "./vscodeApi";
import { vscodePostMessage } from "./test/vscodeMock";

describe("vscodeApi", () => {
  beforeEach(() => {
    resetVsCodeApiForTests();
    vscodePostMessage.mockClear();
  });

  it("returns a cached VS Code API instance", () => {
    const a = getVsCodeApi();
    const b = getVsCodeApi();
    expect(a).toBe(b);
  });

  it("forwards postMessage to acquireVsCodeApi", () => {
    getVsCodeApi().postMessage({ type: "ready", panel: "smoke" });
    expect(vscodePostMessage).toHaveBeenCalledWith({ type: "ready", panel: "smoke" });
  });

  it("resetVsCodeApiForTests clears the cached instance", () => {
    const first = getVsCodeApi();
    resetVsCodeApiForTests();
    const second = getVsCodeApi();
    expect(second).not.toBe(first);
  });
});
