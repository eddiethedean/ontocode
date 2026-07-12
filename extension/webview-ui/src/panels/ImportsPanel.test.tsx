import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "../test/render";
import {
  lastPostedMessage,
  postHostMessage,
  postedMessages,
} from "../test/fixtures";
import { ImportsPanel } from "./ImportsPanel";

const importsPayload = {
  path: "/workspace/people.ttl",
  ontology_iri: "http://example.org/people",
  imports: ["http://example.org/org"],
  options: [
    {
      iri: "http://example.org/clinic",
      label: "Clinic",
      path: "/workspace/clinic.ttl",
    },
  ],
  imports_editable: true,
};

describe("ImportsPanel", () => {
  it("posts ready and renders imports from host payload", async () => {
    renderWithProviders(<ImportsPanel />);
    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "imports" });
    });
    expect(screen.getByText(/Loading ontology imports/i)).toBeInTheDocument();

    postHostMessage({ type: "loadImports", payload: importsPayload });

    expect(await screen.findByText("http://example.org/org")).toBeInTheDocument();
    expect(screen.getByText("http://example.org/people")).toBeInTheDocument();
  });

  it("posts remove_import patch with previewOnly false", async () => {
    const user = userEvent.setup();
    renderWithProviders(<ImportsPanel />);
    postHostMessage({ type: "loadImports", payload: importsPayload });
    await screen.findByText("http://example.org/org");

    await user.click(screen.getByRole("button", { name: /^Remove$/i }));

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      previewOnly: false,
      patches: [
        {
          op: "remove_import",
          ontology_iri: "http://example.org/people",
          import_iri: "http://example.org/org",
        },
      ],
    });
  });

  it("posts add_import preview when selecting a workspace ontology", async () => {
    const user = userEvent.setup();
    renderWithProviders(<ImportsPanel />);
    postHostMessage({ type: "loadImports", payload: importsPayload });
    await screen.findByText("Manage Imports");

    await user.selectOptions(
      screen.getByLabelText(/Import from workspace/i),
      "http://example.org/clinic"
    );
    await user.click(screen.getByRole("button", { name: /Preview add/i }));

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      previewOnly: true,
      patches: [
        {
          op: "add_import",
          ontology_iri: "http://example.org/people",
          import_iri: "http://example.org/clinic",
        },
      ],
    });
  });

  it("shows non-editable callout when imports_editable is false", async () => {
    renderWithProviders(<ImportsPanel />);
    postHostMessage({
      type: "loadImports",
      payload: {
        ...importsPayload,
        imports_editable: false,
        error: "OWL/XML is read-only",
      },
    });
    expect(await screen.findByText(/OWL\/XML is read-only/i)).toBeInTheDocument();
  });
});
