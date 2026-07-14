import { screen, waitFor } from "@testing-library/react";
import { renderWithProviders } from "../test/render";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { QueryWorkbenchPanel } from "./QueryWorkbench";
import {
  lastPostedMessage,
  postHostMessage,
  postedMessages,
  queryHistory,
  queryResult,
  savedQueries,
} from "../test/fixtures";

describe("QueryWorkbenchPanel", () => {
  it("posts ready and renders toolbar", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);

    expect(screen.getByRole("heading", { name: "Query Workbench" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Run" })).toBeInTheDocument();

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "queryWorkbench" });
    });
  });

  it("loads saved queries and sql tables from queryInit", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({
      type: "queryInit",
      saved: savedQueries,
      history: queryHistory,
      sqlTables: ["classes", "properties"],
    });

    expect(await screen.findByRole("option", { name: "classes" })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: "All classes (sql)" })).toBeInTheDocument();
  });

  it("runs query with incremented runId", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    const textarea = screen.getByRole("textbox");
    await user.clear(textarea);
    await user.type(textarea, "SELECT 1");
    await user.click(screen.getByRole("button", { name: "Run" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "runQuery",
      mode: "sql",
      text: "SELECT 1",
      runId: 1,
    });
  });

  it("switches to SPARQL mode and starter query", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    const modeSelect = screen.getAllByRole("combobox")[0];
    await user.selectOptions(modeSelect, "sparql");

    expect(screen.getByRole("textbox")).toHaveValue(
      "PREFIX ex: <http://example.org/people#>\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
    );
  });

  it("switches to DL mode with Manchester starter and asserted toggle", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    await user.selectOptions(screen.getAllByRole("combobox")[0], "dl");

    expect(screen.getByRole("textbox")).toHaveValue("Person");
    expect(screen.getByText("Manchester class expression")).toBeInTheDocument();
    expect(screen.getByRole("option", { name: "Inferred" })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: "Asserted" })).toBeInTheDocument();
  });

  it("runs DL query with asserted mode", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    await user.selectOptions(screen.getAllByRole("combobox")[0], "dl");
    const reasoningSelect = screen.getAllByRole("combobox")[1];
    await user.selectOptions(reasoningSelect, "asserted");
    await user.click(screen.getByRole("button", { name: "Run" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "runQuery",
      mode: "dl",
      text: "Person",
      dlMode: "asserted",
      runId: 1,
    });
  });

  it("renders DL result tabs and posts openEntity on IRI click", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    await user.selectOptions(screen.getAllByRole("combobox")[0], "dl");
    await user.click(screen.getByRole("button", { name: "Run" }));
    const runId = (lastPostedMessage() as { runId: number }).runId;

    postHostMessage({
      type: "queryResult",
      runId,
      dlResult: {
        expression: "Person",
        normalized: "Person",
        query_class_iri: "http://example.org#Person",
        instances: ["http://example.org#Alice"],
        subclasses: ["http://example.org#Student"],
        superclasses: [],
        equivalents: [],
        profile: "dl",
        mode: "inferred",
        duration_ms: 12,
      },
    });

    expect(await screen.findByRole("button", { name: "Instances (1)" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Subclasses (1)" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Alice/ }));
    expect(lastPostedMessage()).toMatchObject({
      type: "openEntity",
      iri: "http://example.org#Alice",
    });
  });

  it("displays tabular results from host", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);

    await userEvent.setup().click(screen.getByRole("button", { name: "Run" }));
    const runId = (lastPostedMessage() as { runId: number }).runId;

    postHostMessage({ type: "queryResult", runId, result: queryResult });

    expect(await screen.findByRole("columnheader", { name: "short_name" })).toBeInTheDocument();
    expect(screen.getAllByRole("cell", { name: "Person" }).length).toBeGreaterThan(0);
  });

  it("shows error callout from host", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);

    await userEvent.setup().click(screen.getByRole("button", { name: "Run" }));
    const runId = (lastPostedMessage() as { runId: number }).runId;

    postHostMessage({ type: "queryResult", runId, error: "Syntax error near FROM" });

    expect(await screen.findByText("Syntax error near FROM")).toHaveClass("oc-callout--error");
  });

  it("ignores stale query results", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({ type: "queryResult", runId: 999, result: queryResult });

    await waitFor(() => {
      expect(screen.queryByRole("columnheader", { name: "short_name" })).not.toBeInTheDocument();
    });
  });

  it("saves query via prompt", async () => {
    const user = userEvent.setup();
    vi.spyOn(window, "prompt").mockReturnValue("My query");

    renderWithProviders(<QueryWorkbenchPanel />);
    await user.click(screen.getByRole("button", { name: "Save" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "saveQuery",
      name: "My query",
      mode: "sql",
    });
  });

  it("exports csv with current run id", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    await user.click(screen.getByRole("button", { name: "Run" }));
    await user.click(screen.getByRole("button", { name: "Export CSV" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "exportQueryResult",
      format: "csv",
      runId: 1,
    });
  });

  it("sets query text when SQL table is selected", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({
      type: "queryInit",
      saved: [],
      history: [],
      sqlTables: ["classes"],
    });
    await screen.findByRole("option", { name: "classes" });

    const tableSelect = screen.getAllByRole("combobox")[1];
    await user.selectOptions(tableSelect, "classes");

    expect(screen.getByRole("textbox")).toHaveValue("SELECT * FROM classes");
  });

  it("loads query text from saved and history dropdowns", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({
      type: "queryInit",
      saved: savedQueries,
      history: queryHistory,
      sqlTables: [],
    });
    await screen.findByRole("option", { name: "All classes (sql)" });

    const savedSelect = screen.getByRole("option", { name: "All classes (sql)" }).closest("select");
    expect(savedSelect).not.toBeNull();
    await user.selectOptions(savedSelect as HTMLSelectElement, "0");
    expect(screen.getByRole("textbox")).toHaveValue("SELECT * FROM classes");

    const historySelect = screen.getByRole("option", { name: "Recent" }).closest("select");
    expect(historySelect).not.toBeNull();
    await user.selectOptions(historySelect as HTMLSelectElement, "0");
    expect(screen.getByRole("textbox")).toHaveValue("SELECT ?s WHERE { ?s ?p ?o }");
    expect(screen.getAllByRole("combobox")[0]).toHaveValue("sparql");
  });

  it("exports JSON results", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);
    await user.click(screen.getByRole("button", { name: "Run" }));
    await user.click(screen.getByRole("button", { name: "Export JSON" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "exportQueryResult",
      format: "json",
      runId: 1,
    });
  });

  it("does not save when prompt is cancelled", async () => {
    const user = userEvent.setup();
    vi.spyOn(window, "prompt").mockReturnValue(null);

    renderWithProviders(<QueryWorkbenchPanel />);
    const before = postedMessages().length;
    await user.click(screen.getByRole("button", { name: "Save" }));
    expect(postedMessages().length).toBe(before);
  });

  it("shows truncated results banner", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);
    await userEvent.setup().click(screen.getByRole("button", { name: "Run" }));
    const runId = (lastPostedMessage() as { runId: number }).runId;

    postHostMessage({
      type: "queryResult",
      runId,
      result: { ...queryResult, truncated: true },
    });

    expect(
      await screen.findByText(/Results truncated at server row limit/)
    ).toBeInTheDocument();
  });

  it("clears prior error when running a new query", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    await user.click(screen.getByRole("button", { name: "Run" }));
    const runId = (lastPostedMessage() as { runId: number }).runId;
    postHostMessage({ type: "queryResult", runId, error: "Syntax error" });
    expect(await screen.findByText("Syntax error")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Run" }));
    await waitFor(() => {
      expect(screen.queryByText("Syntax error")).not.toBeInTheDocument();
    });
  });

  it("hides table picker in SPARQL mode", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({
      type: "queryInit",
      saved: [],
      history: [],
      sqlTables: ["classes"],
    });
    await screen.findByRole("option", { name: "classes" });

    await user.selectOptions(screen.getAllByRole("combobox")[0], "sparql");
    expect(screen.queryByRole("option", { name: "classes" })).not.toBeInTheDocument();
    expect(screen.getByText("SPARQL query")).toBeInTheDocument();
  });

  it("switches back to SQL starter query", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);

    await user.selectOptions(screen.getAllByRole("combobox")[0], "sparql");
    await user.selectOptions(screen.getAllByRole("combobox")[0], "sql");

    expect(screen.getByRole("textbox")).toHaveValue("SELECT short_name, labels FROM classes");
    expect(screen.getByText("SQL query")).toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({ type: "queryResult", runId: 1 } as never);
    postHostMessage(null as never);
    expect(screen.getByRole("heading", { name: "Query Workbench" })).toBeInTheDocument();
  });

  it("schema browser inserts column into editor", async () => {
    const user = userEvent.setup();
    renderWithProviders(<QueryWorkbenchPanel />);
    postHostMessage({
      type: "queryInit",
      saved: [],
      history: [],
      sqlSchema: [
        {
          name: "domain_axioms",
          columns: [
            { name: "property_iri", type: "string" },
            { name: "domain", type: "string" },
          ],
        },
      ],
    });

    await user.click(await screen.findByRole("button", { name: "Table domain_axioms" }));
    await user.click(screen.getByRole("button", { name: "Insert column property_iri" }));

    const textarea = screen.getByRole("textbox");
    expect((textarea as HTMLTextAreaElement).value).toContain("property_iri");
  });

  it("records history from the query text at run time, not result time", async () => {
    const user = userEvent.setup();
    const { useWorkspaceStore } = await import("../store/workspaceStore");
    useWorkspaceStore.getState().reset();

    renderWithProviders(<QueryWorkbenchPanel />);
    const editor = screen.getByRole("textbox");
    await user.clear(editor);
    await user.type(editor, "SELECT * FROM classes");
    await user.click(screen.getByRole("button", { name: "Run" }));
    const runId = (lastPostedMessage() as { runId: number }).runId;

    await user.clear(editor);
    await user.type(editor, "SELECT * FROM properties");

    postHostMessage({
      type: "queryResult",
      runId,
      result: queryResult,
    });

    await waitFor(() => {
      const history = useWorkspaceStore.getState().query.history;
      expect(history.some((h) => h.text === "SELECT * FROM classes")).toBe(true);
      expect(history.some((h) => h.text === "SELECT * FROM properties")).toBe(false);
    });
  });
});
