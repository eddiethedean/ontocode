import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { SmokePanel } from "../panels/SmokePanel";
import { postedMessages } from "../test/fixtures";

describe("SmokePanel", () => {
  it("renders brand content and posts ready message", async () => {
    render(<SmokePanel />);

    expect(screen.getByRole("heading", { name: "OntoCode React" })).toBeInTheDocument();
    expect(screen.getByText("Webview foundation is active.")).toBeInTheDocument();

    await waitFor(() => {
      expect(postedMessages()).toContainEqual({ type: "ready", panel: "smoke" });
    });
  });
});
