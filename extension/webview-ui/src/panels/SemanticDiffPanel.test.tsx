import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { SemanticDiffPanel } from "./SemanticDiffPanel";
import {
  diffPayload,
  largeEntityDiff,
  lastPostedMessage,
  postHostMessage,
  postedMessages,
} from "../test/fixtures";

describe("SemanticDiffPanel", () => {
  it("shows loading state and posts ready", async () => {
    render(<SemanticDiffPanel />);

    expect(screen.getByRole("status")).toHaveTextContent("Computing semantic diff…");

    await waitFor(() => {
      expect(postedMessages()).toContainEqual({ type: "ready", panel: "semanticDiff" });
    });
  });

  it("renders summary stats and change sections", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "semanticDiffData", diff: diffPayload });

    expect(await screen.findByRole("heading", { name: "Semantic diff" })).toBeInTheDocument();
    expect(screen.getByText("Entities")).toBeInTheDocument();
    expect(screen.getByText("Breaking")).toBeInTheDocument();
    expect(screen.getByText("Removed class breaks downstream references")).toBeInTheDocument();
    expect(screen.getAllByText("http://example.org#Person").length).toBeGreaterThan(0);
    expect(screen.getByText(/rdfs:label on/)).toBeInTheDocument();
  });

  it("highlights breaking changes section", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "semanticDiffData", diff: diffPayload });

    await screen.findByRole("heading", { name: "Semantic diff" });
    expect(document.querySelector(".oc-section--breaking")).toBeInTheDocument();
  });

  it("shows error callout from host", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "error", message: "Diff computation failed" });

    expect(await screen.findByText("Diff computation failed")).toHaveClass("oc-callout--error");
  });

  it("shows empty state when diff is cleared after loading", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "loading" });
    postHostMessage({ type: "error", message: "cancelled" });

    expect(await screen.findByText("cancelled")).toBeInTheDocument();
  });

  it("copies markdown summary on button click", async () => {
    const user = userEvent.setup();
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "semanticDiffData", diff: diffPayload });
    await screen.findByRole("heading", { name: "Semantic diff" });

    await user.click(screen.getByRole("button", { name: "Copy Markdown summary" }));
    expect(lastPostedMessage()).toEqual({ type: "copyMarkdown" });
  });

  it("returns to loading state on loading message", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "semanticDiffData", diff: diffPayload });
    await screen.findByRole("heading", { name: "Semantic diff" });

    postHostMessage({ type: "loading" });

    await waitFor(() => {
      expect(screen.getByRole("status")).toHaveTextContent("Computing semantic diff…");
    });
  });

  it("shows truncation banner when entity changes exceed cap", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "semanticDiffData", diff: largeEntityDiff(55) });
    await screen.findByRole("heading", { name: "Semantic diff" });

    expect(screen.getByText("Showing 50 of 55 entity changes.")).toBeInTheDocument();
  });

  it("computes summary counts when summary_counts is omitted", async () => {
    render(<SemanticDiffPanel />);
    const { summary_counts: _ignored, ...diffWithoutSummary } = diffPayload;
    postHostMessage({ type: "semanticDiffData", diff: diffWithoutSummary });

    await screen.findByRole("heading", { name: "Semantic diff" });
    const entitiesStat = screen.getByText("Entities").previousElementSibling;
    expect(entitiesStat).toHaveTextContent("1");
  });

  it("hides empty change sections", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({
      type: "semanticDiffData",
      diff: {
        ...diffPayload,
        axiom_changes: [],
        annotation_changes: [],
        import_changes: [],
        inference_changes: [],
        breaking_changes: [],
        entity_changes: [{ kind: "added", iri: "http://example.org#Only" }],
      },
    });

    await screen.findByRole("heading", { name: "Semantic diff" });
    expect(screen.getByText("Entity changes")).toBeInTheDocument();
    expect(screen.queryByText("Axiom changes")).not.toBeInTheDocument();
    expect(screen.queryByText("Breaking changes")).not.toBeInTheDocument();
  });

  it("uses default stat variant when there are zero breaking changes", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({
      type: "semanticDiffData",
      diff: {
        ...diffPayload,
        breaking_changes: [],
        summary_counts: {
          entities: 1,
          axioms: 0,
          annotations: 0,
          imports: 0,
          inferences: 0,
          breaking: 0,
        },
      },
    });

    await screen.findByRole("heading", { name: "Semantic diff" });
    expect(document.querySelector(".oc-section--breaking")).not.toBeInTheDocument();
    expect(document.querySelector(".oc-stat--danger")).not.toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    render(<SemanticDiffPanel />);
    postHostMessage({ type: "semanticDiffData", diff: diffPayload });
    await screen.findByRole("heading", { name: "Semantic diff" });

    postHostMessage(null as never);
    postHostMessage({ type: "semanticDiffData", diff: {} } as never);
    expect(screen.getByRole("heading", { name: "Semantic diff" })).toBeInTheDocument();
  });
});
