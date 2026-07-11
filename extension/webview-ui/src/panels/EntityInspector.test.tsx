import { screen, waitFor, within } from "@testing-library/react";
import { renderWithProviders } from "../test/render";
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
    renderWithProviders(<EntityInspectorPanel />);

    expect(screen.getByRole("status")).toHaveTextContent("Loading entity…");

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "inspector" });
    });
  });

  it("renders entity detail from host message", async () => {
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });

    expect(await screen.findByRole("heading", { name: "Person" })).toBeInTheDocument();
    expect(screen.getByText(/EX:001/)).toBeInTheDocument();
    expect(screen.getByText("http://example.org#Person")).toBeInTheDocument();
    expect(screen.getByText("SubClassOf Agent")).toBeInTheDocument();
    expect(screen.queryByText("Deprecated")).not.toBeInTheDocument();
  });

  it("shows deprecated badge when entity is deprecated", async () => {
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    // Keep typed text until host confirms success via loadEntity (#41).
    expect(labelInput).toHaveValue("New label");
  });

  it("resets form pickers when navigating to another entity", async () => {
    const user = userEvent.setup();
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.selectOptions(screen.getByLabelText("Add parent"), "http://example.org#Agent");
    expect(screen.getByLabelText("Add parent")).toHaveValue("http://example.org#Agent");

    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        entity: {
          ...entityDetail.entity,
          iri: "http://example.org#Student",
          short_name: "Student",
          labels: ["Student"],
        },
      },
      classOptions,
    });
    await screen.findByRole("heading", { name: "Student" });
    expect(screen.getByLabelText("Add parent")).toHaveValue("");
  });

  it("detects Turtle and OBO extensions case-insensitively", async () => {
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: { ...entityDetail, document_path: "/workspace/ontology.TTL" },
      classOptions,
    });
    await screen.findByRole("heading", { name: "Person" });
    expect(screen.getByText("Edit")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Edit in Manchester" })).toBeInTheDocument();
  });

  it("hides property characteristics when not editable", async () => {
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        editable: false,
        entity: { ...entityDetail.entity, kind: "object_property", labels: ["hasPart"] },
        characteristics: { functional: true },
      },
      classOptions,
    });
    await screen.findByRole("heading", { name: "hasPart" });
    expect(screen.queryByText("Property characteristics")).not.toBeInTheDocument();
  });

  it("previews property characteristic changes instead of live-applying", async () => {
    const user = userEvent.setup();
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        entity: { ...entityDetail.entity, kind: "object_property", labels: ["hasPart"] },
        characteristics: { functional: false },
      },
      classOptions,
    });
    await screen.findByRole("heading", { name: "hasPart" });

    await user.click(screen.getByLabelText("Functional"));
    await user.click(screen.getAllByRole("button", { name: "Preview" })[0]);

    expect(lastPostedMessage()).toEqual({
      type: "applyPatch",
      patches: [
        {
          op: "set_functional",
          entity_iri: "http://example.org#Person",
          value: true,
        },
      ],
      previewOnly: true,
    });
  });

  it("posts jumpToSource from sticky actions", async () => {
    const user = userEvent.setup();
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Jump to Source" }));
    expect(lastPostedMessage()).toEqual({ type: "jumpToSource" });
  });

  it("shows preview text from host", async () => {
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    postHostMessage({ type: "preview", text: "@prefix ex: <http://example.org#> ." });
    expect(await screen.findByText(/@prefix ex:/)).toBeInTheDocument();
  });

  it("shows error callout from host error message", async () => {
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    postHostMessage({ type: "error", message: "Patch failed" });
    expect(await screen.findByText("Error: Patch failed")).toBeInTheDocument();
  });

  it("shows read-only callout when not editable", async () => {
    renderWithProviders(<EntityInspectorPanel />);
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

    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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

    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    const before = postedMessages().length;
    await user.click(screen.getByRole("button", { name: "Delete Entity" }));
    expect(postedMessages().length).toBe(before);
  });

  it("posts findUsages, renameIri, and openGraph actions", async () => {
    const user = userEvent.setup();
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: "Add Manchester axiom" }));
    expect(lastPostedMessage()).toEqual({ type: "addManchesterAxiom" });
  });

  it("navigates to child entity on click", async () => {
    const user = userEvent.setup();
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    await screen.findByRole("heading", { name: "Person" });

    await user.click(screen.getByRole("button", { name: /Student/i }));
    expect(lastPostedMessage()).toEqual({
      type: "openEntity",
      iri: "http://example.org#Student",
    });
  });

  it("clears preview when entity is reloaded", async () => {
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: { ...entityDetail, axioms: [] },
      classOptions,
    });
    await screen.findByRole("heading", { name: "Person" });
    expect(screen.getByText("None")).toBeInTheDocument();
  });

  it("hides Manchester controls for non-class entities", async () => {
    renderWithProviders(<EntityInspectorPanel />);
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

  it("property chain editor lists object properties not classes", async () => {
    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({
      type: "loadEntity",
      detail: {
        ...entityDetail,
        entity: {
          ...entityDetail.entity,
          kind: "object_property",
          iri: "http://example.org#worksFor",
          short_name: "worksFor",
          labels: ["works for"],
        },
        axioms: [],
      },
      classOptions: ["http://example.org#Person", "http://example.org#Agent"],
      objectPropertyOptions: [
        "http://example.org#worksFor",
        "http://example.org#knows",
      ],
    });
    await screen.findByText("Property chains");
    const chainSection = screen.getByText("Property chains").closest(".oc-section");
    expect(chainSection).not.toBeNull();
    const chainOptions = within(chainSection as HTMLElement);
    expect(chainOptions.getByRole("option", { name: "knows" })).toBeInTheDocument();
    expect(chainOptions.queryByRole("option", { name: "Person" })).not.toBeInTheDocument();
    expect(chainOptions.queryByRole("option", { name: "Agent" })).not.toBeInTheDocument();
  });

  it("shows Edit disjoint label for disjoint_class axioms", async () => {
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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
    renderWithProviders(<EntityInspectorPanel />);
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

  it("loadEntity hydrates focus without pushing navigation history", async () => {
    const { useWorkspaceStore } = await import("../store/workspaceStore");
    useWorkspaceStore.getState().reset();
    useWorkspaceStore.getState().setFocus({
      kind: "entity",
      id: "http://example.org#Seed",
      source: "explorer",
      timestamp: 1,
    });
    const stackBefore = useWorkspaceStore.getState().navigation.stack.length;

    renderWithProviders(<EntityInspectorPanel />);
    postHostMessage({ type: "loadEntity", detail: entityDetail, classOptions });
    expect(await screen.findByRole("heading", { name: "Person" })).toBeInTheDocument();

    expect(useWorkspaceStore.getState().focus?.id).toBe("http://example.org#Person");
    expect(useWorkspaceStore.getState().navigation.stack.length).toBe(stackBefore);
  });
});
