import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { ReasonerPanel } from "./ReasonerPanel";

const postMessage = vi.fn();

vi.mock("../vscodeApi", () => ({
  getVsCodeApi: () => ({ postMessage }),
}));

describe("ReasonerPanel", () => {
  beforeEach(() => {
    postMessage.mockClear();
  });

  it("posts ready and shows empty state", () => {
    render(<ReasonerPanel />);
    expect(postMessage).toHaveBeenCalledWith({ type: "ready", panel: "reasoner" });
    expect(screen.getByText("No reasoner results yet")).toBeInTheDocument();
  });

  it("renders reasoner results from host", () => {
    render(<ReasonerPanel />);
    fireEvent(
      window,
      new MessageEvent("message", {
        data: {
          type: "reasonerResult",
          runId: 1,
          summary: "Completed · el · 12ms",
          result: {
            profile_used: "el",
            consistent: true,
            unsatisfiable: ["http://ex.org#Bad"],
            inferred_edge_count: 1,
            new_inferences: [
              { child: "http://ex.org#A", parent: "http://ex.org#B" },
            ],
            warnings: [],
            duration_ms: 12,
          },
        },
      })
    );
    expect(screen.getByText("Completed · el · 12ms")).toBeInTheDocument();
    expect(screen.getByText("Bad")).toBeInTheDocument();
    expect(screen.getByText(/SubClassOf/)).toBeInTheDocument();
  });

  it("reasonerSyncRunId does not clear running state (#212)", () => {
    render(<ReasonerPanel />);
    fireEvent.click(screen.getAllByRole("button", { name: "Run Reasoner" })[0]);
    expect(screen.getByText("Running reasoner…")).toBeInTheDocument();
    fireEvent(
      window,
      new MessageEvent("message", {
        data: { type: "reasonerSyncRunId", runId: 99 },
      })
    );
    expect(screen.getByText("Running reasoner…")).toBeInTheDocument();
  });

  it("shows auto-detect profile checkbox label (#223)", () => {
    render(<ReasonerPanel />);
    expect(screen.getByLabelText("Auto-detect profile")).toBeInTheDocument();
  });
});
