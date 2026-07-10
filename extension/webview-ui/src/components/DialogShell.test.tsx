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

    expect(screen.getByRole("dialog", { name: "Dialog content" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "OK" }));
    expect(onPrimary).toHaveBeenCalledOnce();
    await user.keyboard("{Escape}");
    expect(onCancel).toHaveBeenCalledOnce();
  });
});
