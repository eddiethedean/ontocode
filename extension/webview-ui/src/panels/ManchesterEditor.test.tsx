import { render, screen, fireEvent, act, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { ManchesterEditorPanel } from "./ManchesterEditor";
import {
  lastPostedMessage,
  manchesterCompletions,
  postHostMessage,
  postedMessages,
} from "../test/fixtures";

describe("ManchesterEditorPanel", () => {
  it("posts ready on mount", async () => {
    render(<ManchesterEditorPanel />);

    expect(screen.getByRole("heading", { name: "Manchester Axiom Editor" })).toBeInTheDocument();

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "manchesterEditor" });
    });
  });

  it("initializes from manchesterInit message", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });

    expect(await screen.findByText("http://example.org#Person")).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toHaveValue("Agent");
  });

  it("debounces validateManchester requests", async () => {
    vi.useFakeTimers();
    try {
      render(<ManchesterEditorPanel />);

      await act(async () => {
        postHostMessage({
          type: "manchesterInit",
          entityIri: "http://example.org#Person",
          axiomKind: "sub_class_of",
          expression: "",
          completions: manchesterCompletions,
        });
      });

      await act(async () => {
        fireEvent.change(screen.getByRole("textbox"), { target: { value: "Agent" } });
      });

      expect(
        postedMessages().some((m) => (m as { type?: string }).type === "validateManchester")
      ).toBe(false);

      await act(async () => {
        await vi.advanceTimersByTimeAsync(500);
      });

      expect(lastPostedMessage()).toMatchObject({
        type: "validateManchester",
        expression: "Agent",
        axiomKind: "sub_class_of",
      });
    } finally {
      vi.useRealTimers();
    }
  });

  it("shows validation diagnostics and turtle preview", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    postHostMessage({
      type: "manchesterValidation",
      seq: Date.now(),
      result: {
        normalized: "Agent",
        turtle_fragment: "rdfs:subClassOf ex:Agent ;",
        tree: { kind: "class" },
        diagnostics: [{ severity: "warning", message: "Consider adding a label" }],
      },
    });

    // seq may not match - need to trigger validate first with fake timers or manual validate click
    await userEvent.setup().click(screen.getByRole("button", { name: "Validate" }));

    postHostMessage({
      type: "manchesterValidation",
      seq: (lastPostedMessage() as { seq: number }).seq,
      result: {
        normalized: "Agent",
        turtle_fragment: "rdfs:subClassOf ex:Agent ;",
        tree: { kind: "class" },
        diagnostics: [{ severity: "warning", message: "Consider adding a label" }],
      },
    });

    expect(await screen.findByText("Consider adding a label")).toBeInTheDocument();
    expect(screen.getByText("rdfs:subClassOf ex:Agent ;")).toBeInTheDocument();
  });

  it("applies manchester with previewOnly flag", async () => {
    const user = userEvent.setup();
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "equivalent_class",
      expression: "Agent and Person",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await user.click(screen.getByRole("button", { name: "Preview Turtle" }));

    expect(lastPostedMessage()).toEqual({
      type: "applyManchester",
      expression: "Agent and Person",
      axiomKind: "equivalent_class",
      previewOnly: true,
    });
  });

  it("shows host preview text in turtle section", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    postHostMessage({ type: "preview", text: "ex:Person rdfs:subClassOf ex:Agent ." });

    expect(await screen.findByText("ex:Person rdfs:subClassOf ex:Agent .")).toBeInTheDocument();
  });

  it("switches to disjoint_class mode and hides completion toolbar", async () => {
    const user = userEvent.setup();
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await user.selectOptions(screen.getByLabelText("Axiom type"), "disjoint_class");

    expect(screen.getByText("Other class IRI")).toBeInTheDocument();
    expect(screen.queryByLabelText("Class")).not.toBeInTheDocument();
    expect(screen.getByRole("textbox")).toHaveAttribute(
      "placeholder",
      expect.stringContaining("OtherClass")
    );
  });

  it("inserts completion term into expression", async () => {
    const user = userEvent.setup();
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await user.selectOptions(screen.getByLabelText("Class"), "ex:Person");
    expect(screen.getByRole("textbox")).toHaveValue("ex:Person");
  });

  it("applies manchester without previewOnly", async () => {
    const user = userEvent.setup();
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await user.click(screen.getByRole("button", { name: "Apply" }));
    expect(lastPostedMessage()).toEqual({
      type: "applyManchester",
      expression: "Agent",
      axiomKind: "sub_class_of",
      previewOnly: false,
    });
  });

  it("ignores stale validation responses by seq", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await userEvent.setup().click(screen.getByRole("button", { name: "Validate" }));
    const currentSeq = (lastPostedMessage() as { seq: number }).seq;

    postHostMessage({
      type: "manchesterValidation",
      seq: currentSeq - 1,
      result: {
        normalized: "stale",
        turtle_fragment: "stale ttl",
        tree: null,
        diagnostics: [{ severity: "warning", message: "stale diagnostic" }],
      },
    });

    expect(screen.queryByText("stale diagnostic")).not.toBeInTheDocument();
    expect(screen.queryByText("stale ttl")).not.toBeInTheDocument();
  });

  it("clears validation state when switching entities via manchesterInit", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await userEvent.setup().click(screen.getByRole("button", { name: "Validate" }));
    postHostMessage({
      type: "manchesterValidation",
      seq: (lastPostedMessage() as { seq: number }).seq,
      result: {
        normalized: "Agent",
        turtle_fragment: "rdfs:subClassOf ex:Agent ;",
        tree: { kind: "class" },
        diagnostics: [{ severity: "warning", message: "from entity A" }],
      },
    });
    expect(await screen.findByText("from entity A")).toBeInTheDocument();

    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Other",
      axiomKind: "sub_class_of",
      expression: "",
      completions: manchesterCompletions,
    });

    expect(await screen.findByText("http://example.org#Other")).toBeInTheDocument();
    expect(screen.queryByText("from entity A")).not.toBeInTheDocument();
    expect(screen.queryByText("rdfs:subClassOf ex:Agent ;")).not.toBeInTheDocument();
  });

  it("shows validation error callout from host", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "bad",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await userEvent.setup().click(screen.getByRole("button", { name: "Validate" }));
    postHostMessage({
      type: "manchesterValidation",
      seq: (lastPostedMessage() as { seq: number }).seq,
      error: "Parse error at column 3",
    });

    expect(await screen.findByText("Parse error at column 3")).toHaveClass("oc-callout--error");
  });

  it("renders expression tree JSON when validation succeeds", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage({
      type: "manchesterInit",
      entityIri: "http://example.org#Person",
      axiomKind: "sub_class_of",
      expression: "Agent",
      completions: manchesterCompletions,
    });
    await screen.findByText("http://example.org#Person");

    await userEvent.setup().click(screen.getByRole("button", { name: "Validate" }));
    postHostMessage({
      type: "manchesterValidation",
      seq: (lastPostedMessage() as { seq: number }).seq,
      result: {
        normalized: "Agent",
        turtle_fragment: "",
        tree: { kind: "class", name: "Agent" },
        diagnostics: [],
      },
    });

    expect(await screen.findByText(/"kind": "class"/)).toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    render(<ManchesterEditorPanel />);
    postHostMessage(null as never);
    postHostMessage({ type: "manchesterValidation", seq: 1 } as never);
    expect(screen.getByRole("heading", { name: "Manchester Axiom Editor" })).toBeInTheDocument();
  });
});
