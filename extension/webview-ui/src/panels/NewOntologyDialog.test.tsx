import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "../test/render";
import {
  lastPostedMessage,
  postHostMessage,
  postedMessages,
} from "../test/fixtures";
import { NewOntologyDialog } from "./NewOntologyDialog";

describe("NewOntologyDialog", () => {
  it("posts ready on mount", async () => {
    renderWithProviders(<NewOntologyDialog />);
    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "newOntology" });
    });
  });

  it("posts submitNewOntology with ontology and version IRIs", async () => {
    const user = userEvent.setup();
    renderWithProviders(<NewOntologyDialog />);
    postHostMessage({
      type: "loadNewOntology",
      path: "/workspace/new.ttl",
      defaultIri: "https://example.org/ontology",
    });

    expect(await screen.findByText("/workspace/new.ttl")).toBeInTheDocument();

    const ontologyInput = screen.getByRole("textbox", { name: "Ontology IRI" });
    await user.clear(ontologyInput);
    await user.type(ontologyInput, "https://example.org/my-onto");

    const versionInput = screen.getByRole("textbox", { name: /^Version IRI/ });
    await user.type(versionInput, "https://example.org/my-onto/1.0");

    await user.click(screen.getByRole("button", { name: "Create" }));

    expect(lastPostedMessage()).toEqual({
      type: "submitNewOntology",
      ontologyIri: "https://example.org/my-onto",
      versionIri: "https://example.org/my-onto/1.0",
    });
  });

  it("posts submitNewOntology without versionIri when blank", async () => {
    const user = userEvent.setup();
    renderWithProviders(<NewOntologyDialog />);
    postHostMessage({
      type: "loadNewOntology",
      path: "/workspace/new.ttl",
      defaultIri: "https://example.org/ontology",
    });
    await screen.findByText("/workspace/new.ttl");

    await user.click(screen.getByRole("button", { name: "Create" }));

    expect(lastPostedMessage()).toEqual({
      type: "submitNewOntology",
      ontologyIri: "https://example.org/ontology",
      versionIri: undefined,
    });
  });

  it("posts closeDialog on Cancel", async () => {
    const user = userEvent.setup();
    renderWithProviders(<NewOntologyDialog />);
    await user.click(screen.getByRole("button", { name: "Cancel" }));
    expect(lastPostedMessage()).toEqual({ type: "closeDialog" });
  });
});
