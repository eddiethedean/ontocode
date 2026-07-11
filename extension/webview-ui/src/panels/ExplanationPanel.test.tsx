import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { ExplanationPanel } from "./ExplanationPanel";

const postMessage = vi.fn();

vi.mock("../vscodeApi", () => ({
  getVsCodeApi: () => ({ postMessage }),
}));

describe("ExplanationPanel", () => {
  beforeEach(() => {
    postMessage.mockClear();
  });

  it("posts ready and shows loading", () => {
    render(<ExplanationPanel />);
    expect(postMessage).toHaveBeenCalledWith({
      type: "ready",
      panel: "explanation",
    });
    expect(screen.getByRole("status")).toHaveTextContent("Loading explanation…");
  });

  it("renders explanation payload", () => {
    render(<ExplanationPanel />);
    fireEvent(
      window,
      new MessageEvent("message", {
        data: {
          type: "loadExplanation",
          payload: {
            classIri: "http://ex.org#Bad",
            profile: "el",
            stale: true,
            indexed_at: 1,
            content_hash: "abc",
            justifications: [
              {
                title: "Justification 1",
                text: "Because…",
                steps: [
                  {
                    index: 1,
                    rule: "r1",
                    display: "Bad SubClassOf Nothing",
                    subject_iri: "http://ex.org#Bad",
                  },
                ],
              },
            ],
          },
        },
      })
    );
    expect(screen.getByText("Stale")).toBeInTheDocument();
    expect(screen.getByText("Bad SubClassOf Nothing")).toBeInTheDocument();
    expect(screen.getByText("Because…")).toBeInTheDocument();
  });
});
