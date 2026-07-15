import { describe, expect, it, vi } from "vitest";
import { getFocusableElements, installFocusTrap } from "./focusTrap";

describe("focusTrap", () => {
  it("lists focusable elements", () => {
    const root = document.createElement("div");
    root.innerHTML = `
      <button type="button">A</button>
      <button type="button" disabled>B</button>
      <input type="text" />
    `;
    document.body.appendChild(root);
    expect(getFocusableElements(root).map((el) => el.tagName)).toEqual([
      "BUTTON",
      "INPUT",
    ]);
    root.remove();
  });

  it("cycles Tab within the container", () => {
    const root = document.createElement("div");
    root.innerHTML = `
      <button type="button" id="first">First</button>
      <button type="button" id="last">Last</button>
    `;
    document.body.appendChild(root);
    const first = root.querySelector("#first") as HTMLButtonElement;
    const last = root.querySelector("#last") as HTMLButtonElement;
    const cleanup = installFocusTrap(root, { initialFocus: first });
    last.focus();
    const event = new KeyboardEvent("keydown", { key: "Tab", bubbles: true });
    const prevent = vi.spyOn(event, "preventDefault");
    root.dispatchEvent(event);
    expect(prevent).toHaveBeenCalled();
    expect(document.activeElement).toBe(first);
    cleanup();
    root.remove();
  });
});
