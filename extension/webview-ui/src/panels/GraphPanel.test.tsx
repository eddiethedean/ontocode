import { screen, waitFor, fireEvent } from "@testing-library/react";
import { renderWithProviders } from "../test/render";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { GraphPanel } from "./GraphPanel";
import {
  graphPayload,
  graphWithInferredEdge,
  lastPostedMessage,
  postHostMessage,
  postedMessages,
  postedMessagesOfType,
} from "../test/fixtures";

describe("GraphPanel", () => {
  it("posts ready on mount and shows empty state", async () => {
    renderWithProviders(<GraphPanel />);

    expect(screen.getByText("No graph data")).toBeInTheDocument();

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "graph" });
    });
  });

  it("requests graph on init when no data loaded", async () => {
    renderWithProviders(<GraphPanel />);

    postHostMessage({ type: "init", panel: "graph" });

    await waitFor(() => {
      expect(lastPostedMessage()).toMatchObject({
        type: "requestGraph",
        graphKind: "class",
        depth: 2,
        includeInferred: false,
        filters: { hide_deprecated: false },
      });
    });
  });

  it("renders graph canvas when data arrives", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });
    expect(screen.getByText("class")).toBeInTheDocument();
  });

  it("shows truncated badge when graph is truncated", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: { ...graphPayload, truncated: true },
    });

    expect(await screen.findByText("Truncated")).toBeInTheDocument();
  });

  it("shows error empty state on host error", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "error", message: "Index missing" });

    expect(await screen.findByText("Graph error")).toBeInTheDocument();
    expect(screen.getByText("Index missing")).toBeInTheDocument();
  });

  it("refresh sends updated filter options", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);

    await user.click(screen.getByLabelText("Include inferred (reasoner)"));
    await user.click(screen.getByRole("button", { name: "Refresh graph" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      includeInferred: true,
    });
  });

  it("updates depth via range control", () => {
    renderWithProviders(<GraphPanel />);

    const slider = screen.getByRole("slider");
    fireEvent.change(slider, { target: { value: "4" } });

    expect(screen.getByText("4")).toBeInTheDocument();
  });

  it("reads graphKind and root from URL query params", async () => {
    window.history.replaceState(
      {},
      "",
      "/?graphKind=property&root=http://example.org%23Person"
    );
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "init", panel: "graph" });

    await waitFor(() => {
      expect(lastPostedMessage()).toMatchObject({
        type: "requestGraph",
        graphKind: "property",
        rootIri: "http://example.org#Person",
      });
    });
  });

  it("updates rootIri when graphData includes rootIri", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: graphPayload,
      rootIri: "http://example.org#Person",
    });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Refresh graph" }));
    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      rootIri: "http://example.org#Person",
    });
  });

  it("does not auto-request graph on init after data is loaded", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });
    await waitFor(() => expect(document.querySelector(".react-flow")).toBeInTheDocument());

    const countBefore = postedMessagesOfType("requestGraph").length;
    postHostMessage({ type: "init", panel: "graph" });
    expect(postedMessagesOfType("requestGraph").length).toBe(countBefore);
  });

  it("includes hide_deprecated filter when toggled", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);

    await user.click(screen.getByLabelText("Hide deprecated"));
    await user.click(screen.getByRole("button", { name: "Refresh graph" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      filters: { hide_deprecated: true },
    });
  });

  it("shows empty state when graph has zero nodes", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: { ...graphPayload, nodes: [] },
    });

    expect(await screen.findByText("No graph data")).toBeInTheDocument();
    expect(document.querySelector(".react-flow")).not.toBeInTheDocument();
  });

  it("renders with inferred edges without crashing", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphWithInferredEdge });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });

    const user = userEvent.setup();
    await user.click(screen.getByLabelText("Show inferred edges"));
    expect(document.querySelector(".react-flow")).toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });
    await waitFor(() => expect(document.querySelector(".react-flow")).toBeInTheDocument());

    postHostMessage(null as never);
    postHostMessage({ type: "init", panel: "inspector" } as never);
    expect(document.querySelector(".react-flow")).toBeInTheDocument();
  });
});
