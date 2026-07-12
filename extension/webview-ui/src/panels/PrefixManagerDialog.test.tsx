import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "../test/render";
import {
  lastPostedMessage,
  postHostMessage,
  postedMessages,
} from "../test/fixtures";
import { PrefixManagerDialog } from "./PrefixManagerDialog";

describe("PrefixManagerDialog", () => {
  it("posts ready on mount", async () => {
    renderWithProviders(<PrefixManagerDialog />);
    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({
        type: "ready",
        panel: "prefixManager",
      });
    });
  });

  it("posts submitPrefix add with namespace IRI", async () => {
    const user = userEvent.setup();
    renderWithProviders(<PrefixManagerDialog />);
    postHostMessage({
      type: "loadPrefixes",
      path: "/workspace/people.ttl",
      prefixes: { ex: "http://example.org/people#" },
    });

    expect(await screen.findByText("/workspace/people.ttl")).toBeInTheDocument();
    expect(screen.getByText("ex:")).toBeInTheDocument();

    await user.type(screen.getByLabelText("Prefix"), "demo");
    await user.type(
      screen.getByLabelText("Namespace IRI"),
      "https://example.org/demo#"
    );
    await user.click(screen.getByRole("button", { name: "Add or update" }));

    expect(lastPostedMessage()).toEqual({
      type: "submitPrefix",
      action: "add",
      prefix: "demo",
      namespaceIri: "https://example.org/demo#",
    });
  });

  it("posts submitPrefix remove without namespaceIri", async () => {
    const user = userEvent.setup();
    renderWithProviders(<PrefixManagerDialog />);
    postHostMessage({
      type: "loadPrefixes",
      path: "/workspace/people.ttl",
      prefixes: { ex: "http://example.org/people#" },
    });
    await screen.findByText("ex:");

    await user.selectOptions(screen.getByLabelText("Action"), "remove");
    await user.type(screen.getByLabelText("Prefix"), "ex");
    await user.click(screen.getByRole("button", { name: "Remove" }));

    expect(lastPostedMessage()).toEqual({
      type: "submitPrefix",
      action: "remove",
      prefix: "ex",
      namespaceIri: undefined,
    });
  });

  it("posts closeDialog on Cancel", async () => {
    const user = userEvent.setup();
    renderWithProviders(<PrefixManagerDialog />);
    await user.click(screen.getByRole("button", { name: "Cancel" }));
    expect(lastPostedMessage()).toEqual({ type: "closeDialog" });
  });
});
