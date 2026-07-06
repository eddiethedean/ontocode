import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { EntityInspectorPanel } from "./EntityInspector";
import {
  classOptions,
  entityDetail,
  lastPostedMessage,
  postHostMessage,
  postedMessages,
} from "../test/fixtures";

describe("EntityInspectorPanel", () => {
  it("shows loading state and posts ready on mount", async () => {
    render(<EntityInspectorPanel />);

    expect(screen.getByRole("status")).toHaveTextContent("Loading entity…");

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "inspector" });
    });
  });

  it("renders entity detail from host message", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });

    expect(await screen.findByRole("heading", { name: "Person" })).toBeInTheDocument();
    expect(screen.getByText(/EX:001/)).toBeInTheDocument();
    expect(screen.getByText("http://example.org#Person")).toBeInTheDocument();
    expect(screen.getByText("SubClassOf Agent")).toBeInTheDocument();
    expect(screen.queryByText("Deprecated")).not.toBeInTheDocument();
  });

  it("shows deprecated badge when entity is deprecated", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        entity: { ...entityDetail.entity, deprecated: true },
      },
      classOptions,
    });

    expect(await screen.findByText("Deprecated")).toBeInTheDocument();
  });

  it("navigates to parent entity on click", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });

    await screen.findByRole("heading", { name: "Person" });
    await user.click(screen.getByRole("button", { name: /Agent/i }));

    expect(lastPostedMessage()).toEqual({
      type: "openEntity",
      iri: "http://example.org#Agent",
    });
  });

  it("adds label via applyPatch message", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    const labelInput = screen.getByLabelText("Add label");
    await user.type(labelInput, "New label");
    await user.click(screen.getAllByRole("button", { name: "Apply" })[0]);

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "add_label",
          entity_iri: "http://example.org#Person",
          value: "New label",
        },
      ],
      previewOnly: false,
    });
  });

  it("posts jumpToSource from sticky actions", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Jump to Source" }));
    expect(lastPostedMessage()).toEqual({ type: "jumpToSource" });
  });

  it("shows preview text from host", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    postHostMessage({ type: "preview", text: "@prefix ex: <http://example.org#> ." });
    expect(await screen.findByText(/@prefix ex:/)).toBeInTheDocument();
  });

  it("shows error callout from host error message", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    postHostMessage({ type: "error", message: "Patch failed" });
    expect(await screen.findByText("Error: Patch failed")).toBeInTheDocument();
  });

  it("shows read-only callout when not editable", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: { ...entityDetail, editable: false },
      classOptions,
    });

    expect(
      await screen.findByText(/Turtle.*OBO/)
    ).toBeInTheDocument();
    expect(screen.queryByText("Edit")).not.toBeInTheDocument();
  });

  it("deletes entity after confirmation", async () => {
    const user = userEvent.setup();
    vi.spyOn(window, "confirm").mockReturnValue(true);

    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Delete Entity" }));

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [{ op: "delete_entity", entity_iri: "http://example.org#Person" }],
      previewOnly: false,
    });
  });

  it("uses short_name when entity has no labels", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        entity: { ...entityDetail.entity, labels: [], short_name: "PersonShort" },
      },
      classOptions,
    });

    expect(await screen.findByRole("heading", { name: "PersonShort" })).toBeInTheDocument();
  });

  it("adds comment and parent via applyPatch", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.type(screen.getByLabelText("Add comment"), "A note");
    await user.click(screen.getAllByRole("button", { name: "Apply" })[1]);
    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "add_comment",
          entity_iri: "http://example.org#Person",
          value: "A note",
        },
      ],
      previewOnly: false,
    });

    await user.selectOptions(screen.getByLabelText("Add parent"), "http://example.org#Agent");
    await user.click(screen.getAllByRole("button", { name: "Apply" })[2]);
    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "add_sub_class_of",
          entity_iri: "http://example.org#Person",
          parent_iri: "http://example.org#Agent",
        },
      ],
      previewOnly: false,
    });
  });

  it("requests preview-only patch without clearing label input", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.type(screen.getByLabelText("Add label"), "Preview me");
    await user.click(screen.getAllByRole("button", { name: "Preview" })[0]);

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "add_label",
          entity_iri: "http://example.org#Person",
          value: "Preview me",
        },
      ],
      previewOnly: true,
    });
    expect(screen.getByLabelText("Add label")).toHaveValue("Preview me");
  });

  it("ignores empty label, comment, and parent submissions", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    const before = postedMessages().length;
    await user.click(screen.getAllByRole("button", { name: "Apply" })[0]);
    await user.click(screen.getAllByRole("button", { name: "Apply" })[1]);
    await user.click(screen.getAllByRole("button", { name: "Apply" })[2]);

    expect(postedMessages().length).toBe(before);
  });

  it("does not delete when confirmation is declined", async () => {
    const user = userEvent.setup();
    vi.spyOn(window, "confirm").mockReturnValue(false);

    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    const before = postedMessages().length;
    await user.click(screen.getByRole("button", { name: "Delete Entity" }));
    expect(postedMessages().length).toBe(before);
  });

  it("posts findUsages, renameIri, and openGraph actions", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Find Usages" }));
    expect(lastPostedMessage()).toEqual({ type: "findUsages" });

    await user.click(screen.getByRole("button", { name: "Rename IRI" }));
    expect(lastPostedMessage()).toEqual({ type: "renameIri" });

    await user.click(screen.getByRole("button", { name: "Open Graph" }));
    expect(lastPostedMessage()).toEqual({
      type: "openGraph",
      rootIri: "http://example.org#Person",
    });
  });

  it("opens Manchester editor for editable axioms", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Edit in Manchester" }));
    expect(lastPostedMessage()).toEqual({
      type: "openManchester",
      axiom: {
        kind: "sub_class_of",
        manchester: "Agent",
        other_iri: undefined,
      },
    });
  });

  it("posts addManchesterAxiom from section action", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Add Manchester axiom" }));
    expect(lastPostedMessage()).toEqual({ type: "addManchesterAxiom" });
  });

  it("navigates to child entity on click", async () => {
    const user = userEvent.setup();
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: /Student/i }));
    expect(lastPostedMessage()).toEqual({
      type: "openEntity",
      iri: "http://example.org#Student",
    });
  });

  it("clears preview when entity is reloaded", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    postHostMessage({ type: "preview", text: "preview text" });
    expect(await screen.findByText("preview text")).toBeInTheDocument();

    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await waitFor(() => {
      expect(screen.queryByText("preview text")).not.toBeInTheDocument();
    });
  });

  it("shows None when axioms list is empty", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: { ...entityDetail, axioms: [] },
      classOptions,
    });
    await screen.findByRole("heading", { name: "Person" });
    expect(screen.getByText("None")).toBeInTheDocument();
  });

  it("hides Manchester controls for non-class entities", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        entity: { ...entityDetail.entity, kind: "object_property" },
      },
      classOptions,
    });
    await screen.findByRole("heading", { name: "Person" });

    expect(screen.queryByRole("button", { name: "Add Manchester axiom" })).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Edit in Manchester" })).not.toBeInTheDocument();
  });

  it("shows Edit disjoint label for disjoint_class axioms", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        axioms: [
          {
            kind: "disjoint_class",
            display: "DisjointWith Agent",
            other_iri: "http://example.org#Agent",
            editable: true,
          },
        ],
      },
      classOptions,
    });
    await screen.findByRole("heading", { name: "Person" });
    expect(screen.getByRole("button", { name: "Edit disjoint" })).toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    postHostMessage({ type: "error" } as never);
    postHostMessage(null as never);
    expect(screen.getByRole("heading", { name: "Person" })).toBeInTheDocument();
  });

  it("adds class assertion for individuals via applyPatch", async () => {
    const user = userEvent.setup();
    const individualDetail = {
      ...entityDetail,
      entity: {
        ...entityDetail.entity,
        iri: "http://example.org#Alice",
        short_name: "Alice",
        kind: "individual",
        labels: ["Alice"],
      },
      parents: [],
      children: [],
      axioms: [],
    };
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: individualDetail, classOptions });
    await screen.findByRole("heading", { name: "Alice" });

    await user.selectOptions(
      screen.getByLabelText("Add type (class assertion)"),
      "http://example.org#Person"
    );
    await user.click(screen.getAllByRole("button", { name: "Apply" })[2]);

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "add_class_assertion",
          entity_iri: "http://example.org#Alice",
          class_iri: "http://example.org#Person",
        },
      ],
      previewOnly: false,
    });
  });

  it("removes class assertion for individuals via applyPatch", async () => {
    const user = userEvent.setup();
    const individualDetail = {
      ...entityDetail,
      entity: {
        ...entityDetail.entity,
        iri: "http://example.org#Alice",
        short_name: "Alice",
        kind: "individual",
        labels: ["Alice"],
      },
      parents: [],
      children: [],
      axioms: [
        {
          kind: "class_assertion",
          display: "ClassAssertion http://example.org#Person",
          parent_iri: "http://example.org#Person",
          editable: true,
        },
      ],
    };
    render(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: individualDetail, classOptions });
    await screen.findByRole("heading", { name: "Alice" });

    await user.click(screen.getByRole("button", { name: "Remove" }));

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "remove_class_assertion",
          entity_iri: "http://example.org#Alice",
          class_iri: "http://example.org#Person",
        },
      ],
      previewOnly: false,
    });
  });
});
