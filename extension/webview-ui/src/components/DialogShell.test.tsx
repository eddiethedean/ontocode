import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { DialogShell } from "./DialogShell";

describe("DialogShell", () => {
  it("renders an accessible dialog and handles its actions", async () => {
    const user = userEvent.setup();
    const onPrimary = vi.fn();
    const onCancel = vi.fn();

    render(
      <DialogShell title="Test dialog" onPrimary={onPrimary} onCancel={onCancel}>
        Dialog content
      </DialogShell>
    );

    expect(screen.getByRole("dialog", { name: "Test dialog" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "OK" }));
    expect(onPrimary).toHaveBeenCalledOnce();
    await user.keyboard("{Escape}");
    expect(onCancel).toHaveBeenCalledOnce();
  });

  it("restores focus to the previously focused element on unmount/cancel", async () => {
    const user = userEvent.setup();
    const opener = document.createElement("button");
    opener.textContent = "Open";
    document.body.appendChild(opener);
    opener.focus();

    const onCancel = vi.fn(() => {
      // Simulate parent unmounting dialog on cancel.
    });
    const { unmount } = render(
      <DialogShell title="Focus restore" onPrimary={() => undefined} onCancel={onCancel}>
        Body
      </DialogShell>
    );

    await user.keyboard("{Escape}");
    expect(onCancel).toHaveBeenCalled();
    unmount();
    expect(document.activeElement).toBe(opener);
    opener.remove();
  });
});
