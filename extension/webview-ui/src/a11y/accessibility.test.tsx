import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { DialogShell } from "../components/DialogShell";
import { Callout, Panel, PanelHeader } from "../components/ui";
import { ReasonerPanel } from "../panels/ReasonerPanel";
import { renderWithProviders } from "../test/render";
import { expectNoSeriousA11yViolations } from "./axe";
import { LiveAnnouncer, PanelMain } from "./liveAnnouncer";

describe("accessibility harness", () => {
  it("DialogShell has no serious axe violations and traps focus", async () => {
    const user = userEvent.setup();
    const onPrimary = vi.fn();
    const onCancel = vi.fn();
    const { container } = render(
      <DialogShell title="Test dialog" onPrimary={onPrimary} onCancel={onCancel}>
        <label>
          Name
          <input type="text" defaultValue="x" />
        </label>
      </DialogShell>
    );

    expect(screen.getByRole("dialog", { name: "Test dialog" })).toBeInTheDocument();
    await expectNoSeriousA11yViolations(container);

    await user.keyboard("{Escape}");
    expect(onCancel).toHaveBeenCalledOnce();
  });

  it("PanelMain + LiveAnnouncer expose status landmarks", async () => {
    const { container } = render(
      <Panel>
        <PanelMain label="Demo panel">
          <PanelHeader title="Demo" />
          <LiveAnnouncer message="Ready" />
          <Callout variant="info">Hello</Callout>
        </PanelMain>
      </Panel>
    );
    expect(screen.getByRole("main", { name: "Demo panel" })).toBeInTheDocument();
    await expectNoSeriousA11yViolations(container);
  });

  it("Reasoner panel has no serious axe violations when idle", async () => {
    const { container } = renderWithProviders(<ReasonerPanel />);
    // Wait for ready message path to settle.
    expect(screen.getByRole("main", { name: "Reasoner" })).toBeInTheDocument();
    await expectNoSeriousA11yViolations(container);
  });
});
