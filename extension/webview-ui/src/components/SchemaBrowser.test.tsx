import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { SchemaBrowser } from "./SchemaBrowser";
import { useWorkspaceStore } from "../store";
import { resetWorkspaceStoreForTests } from "../test/render";

const schema = [
  {
    name: "classes",
    columns: [
      { name: "iri", type: "string" },
      { name: "short_name", type: "string" },
    ],
  },
  {
    name: "domain_axioms",
    columns: [
      { name: "property_iri", type: "string" },
      { name: "domain", type: "string" },
    ],
  },
];

describe("SchemaBrowser", () => {
  it("renders nothing when schema is empty", () => {
    resetWorkspaceStoreForTests();
    const { container } = render(<SchemaBrowser schema={[]} onInsert={() => {}} />);
    expect(container).toBeEmptyDOMElement();
  });

  it("expands table and inserts column snippet", async () => {
    resetWorkspaceStoreForTests();
    const user = userEvent.setup();
    const onInsert = vi.fn();

    render(<SchemaBrowser schema={schema} onInsert={onInsert} />);

    await user.click(screen.getByRole("button", { name: "Table classes" }));
    await user.click(screen.getByRole("button", { name: "Insert column iri" }));

    expect(onInsert).toHaveBeenCalledWith("iri");
  });

  it("inserts full table query from table actions", async () => {
    resetWorkspaceStoreForTests();
    const user = userEvent.setup();
    const onInsert = vi.fn();

    render(<SchemaBrowser schema={schema} onInsert={onInsert} />);
    await user.click(screen.getByRole("button", { name: "Table domain_axioms" }));
    await user.click(screen.getByRole("button", { name: "Insert table query" }));

    expect(onInsert).toHaveBeenCalledWith("SELECT * FROM domain_axioms");
  });

  it("toggles browser visibility via store", async () => {
    resetWorkspaceStoreForTests();
    useWorkspaceStore.getState().setSchemaBrowserExpanded(true);
    const user = userEvent.setup();

    render(<SchemaBrowser schema={schema} onInsert={() => {}} />);
    expect(screen.getByRole("button", { name: "Collapse schema browser" })).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Collapse schema browser" }));
    expect(useWorkspaceStore.getState().query.schemaBrowserExpanded).toBe(false);
  });
});
