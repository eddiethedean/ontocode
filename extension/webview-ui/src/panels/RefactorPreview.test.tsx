import { screen, waitFor } from "@testing-library/react";
import { renderWithProviders } from "../test/render";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { RefactorPreviewPanel } from "./RefactorPreview";
import {
  lastPostedMessage,
  multiFileRefactorPlan,
  postHostMessage,
  postedMessages,
  refactorPlan,
} from "../test/fixtures";

describe("RefactorPreviewPanel", () => {
  it("shows loading state and posts ready", async () => {
    renderWithProviders(<RefactorPreviewPanel />);

    expect(screen.getByRole("status")).toHaveTextContent("Loading refactor preview…");

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "refactorPreview" });
    });
  });

  it("renders plan warnings and diff panes", async () => {
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({ type: "loadRefactorPlan", plan: refactorPlan });

    expect(await screen.findByRole("heading", { name: "Refactor preview" })).toBeInTheDocument();
    expect(screen.getByText("Review import statements")).toBeInTheDocument();
    expect(screen.getByText("ex:OldName a owl:Class .")).toBeInTheDocument();
    expect(screen.getByText("ex:NewName a owl:Class .")).toBeInTheDocument();
    expect(screen.getByText(/1 file · .* entities · .* axioms/)).toBeInTheDocument();
  });

  it("applies refactor from sticky actions", async () => {
    const user = userEvent.setup();
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({ type: "loadRefactorPlan", plan: refactorPlan });
    await screen.findByRole("heading", { name: "Refactor preview" });

    await user.click(screen.getByRole("button", { name: "Apply changes" }));
    expect(lastPostedMessage()).toEqual({ type: "applyRefactor" });
  });

  it("cancels refactor from sticky actions", async () => {
    const user = userEvent.setup();
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({ type: "loadRefactorPlan", plan: refactorPlan });
    await screen.findByRole("heading", { name: "Refactor preview" });

    await user.click(screen.getByRole("button", { name: "Cancel" }));
    expect(lastPostedMessage()).toEqual({ type: "cancelRefactor" });
  });

  it("switches between multiple file diffs", async () => {
    const user = userEvent.setup();
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({ type: "loadRefactorPlan", plan: multiFileRefactorPlan });
    await screen.findByRole("heading", { name: "Refactor preview" });

    expect(screen.getByText("ex:A a owl:Class .")).toBeInTheDocument();
    expect(screen.getByText(/2 files · .* entities · .* axioms/)).toBeInTheDocument();

    await user.selectOptions(screen.getByLabelText("File"), "1");
    expect(screen.getByText("ex:Old a owl:Class .")).toBeInTheDocument();
    expect(screen.getByText("ex:New a owl:Class .")).toBeInTheDocument();
  });

  it("renders without warnings section when plan has none", async () => {
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({
      type: "loadRefactorPlan",
      plan: { ...refactorPlan, warnings: undefined },
    });

    await screen.findByRole("heading", { name: "Refactor preview" });
    expect(screen.queryByText("Review import statements")).not.toBeInTheDocument();
  });

  it("resets selected file when a new plan arrives", async () => {
    const user = userEvent.setup();
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({ type: "loadRefactorPlan", plan: multiFileRefactorPlan });
    await screen.findByRole("heading", { name: "Refactor preview" });

    await user.selectOptions(screen.getByLabelText("File"), "1");
    postHostMessage({ type: "loadRefactorPlan", plan: refactorPlan });

    expect(await screen.findByText("ex:OldName a owl:Class .")).toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    renderWithProviders(<RefactorPreviewPanel />);
    postHostMessage({ type: "loadRefactorPlan", plan: refactorPlan });
    await screen.findByRole("heading", { name: "Refactor preview" });

    postHostMessage(null as never);
    postHostMessage({ type: "preview", text: "ignored" } as never);
    expect(screen.getByRole("heading", { name: "Refactor preview" })).toBeInTheDocument();
  });
});
