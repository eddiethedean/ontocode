import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import {
  Badge,
  ButtonBar,
  Callout,
  Card,
  ChangeList,
  CheckboxRow,
  CodeBlock,
  CodeEditor,
  DiffColumns,
  Divider,
  EmptyState,
  FormField,
  InlineCode,
  Input,
  IriList,
  LoadingSkeleton,
  LoadingState,
  Panel,
  PanelHeader,
  RangeField,
  Section,
  Select,
  StatGrid,
  StickyActions,
  TextArea,
  Toolbar,
  ToolbarGroup,
  shortLabel,
  kindLabel,
} from "../components/ui";

describe("ui components", () => {
  it("renders panel header with title, subtitle, and badges", () => {
    render(
      <Panel>
        <PanelHeader
          title="Entity Inspector"
          subtitle="class · EX:001"
          badges={<Badge variant="kind">class</Badge>}
        />
      </Panel>
    );

    expect(screen.getByRole("heading", { name: "Entity Inspector" })).toBeInTheDocument();
    expect(screen.getByText("class · EX:001")).toBeInTheDocument();
    expect(screen.getByText("class")).toHaveClass("oc-badge--kind");
  });

  it("renders card variants and section with card wrapper", () => {
    render(
      <Section title="Metadata" card>
        <InlineCode>http://example.org#Person</InlineCode>
      </Section>
    );

    expect(screen.getByText("Metadata")).toBeInTheDocument();
    expect(screen.getByText("http://example.org#Person")).toBeInTheDocument();
    expect(document.querySelector(".oc-card")).toBeInTheDocument();
  });

  it("renders callout variants", () => {
    const { rerender } = render(<Callout variant="error">Something failed</Callout>);
    expect(screen.getByText("Something failed")).toHaveClass("oc-callout--error");

    rerender(<Callout variant="info">Note</Callout>);
    expect(screen.getByText("Note")).toHaveClass("oc-callout--info");
  });

  it("renders loading and empty states with accessible status", () => {
    render(<LoadingState label="Loading entity…" />);
    expect(screen.getByRole("status")).toHaveTextContent("Loading entity…");

    render(<EmptyState title="No data" detail="Try refreshing." />);
    expect(screen.getByText("No data")).toBeInTheDocument();
    expect(screen.getByText("Try refreshing.")).toBeInTheDocument();
  });

  it("renders loading skeleton rows", () => {
    render(<LoadingSkeleton rows={2} label="Preparing…" />);
    expect(screen.getByRole("status", { name: "Preparing…" })).toBeInTheDocument();
    expect(document.querySelectorAll(".oc-skeleton-row")).toHaveLength(2);
  });

  it("renders stat grid with danger variant", () => {
    render(
      <StatGrid
        items={[
          { label: "Entities", value: 3 },
          { label: "Breaking", value: 2, variant: "danger" },
        ]}
      />
    );

    expect(screen.getByText("3")).toBeInTheDocument();
    expect(screen.getByText("Breaking")).toBeInTheDocument();
    expect(document.querySelector(".oc-stat--danger")).toBeInTheDocument();
  });

  it("renders interactive IRI list and handles selection", async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();

    render(
      <IriList
        items={["http://example.org#Agent", "http://example.org#Person"]}
        onSelect={onSelect}
      />
    );

    expect(screen.queryByText("None")).not.toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Agent/i }));
    expect(onSelect).toHaveBeenCalledWith("http://example.org#Agent");
  });

  it("shows None for empty IRI list", () => {
    render(<IriList items={[]} />);
    expect(screen.getByText("None")).toBeInTheDocument();
  });

  it("renders change list entries with badges", () => {
    render(
      <ChangeList
        items={[
          {
            key: "1",
            badge: "added",
            primary: "http://example.org#Person",
            secondary: "ontology.ttl",
          },
        ]}
      />
    );

    expect(screen.getByText("added")).toBeInTheDocument();
    expect(screen.getByText("http://example.org#Person")).toBeInTheDocument();
    expect(screen.getByText("ontology.ttl")).toBeInTheDocument();
  });

  it("renders diff columns with before and after panes", () => {
    render(<DiffColumns before="old text" after="new text" />);

    expect(screen.getByText("Before")).toBeInTheDocument();
    expect(screen.getByText("After")).toBeInTheDocument();
    expect(screen.getByText("old text")).toBeInTheDocument();
    expect(screen.getByText("new text")).toBeInTheDocument();
  });

  it("renders form controls inside form field", async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();

    render(
      <FormField label="Name" hint="Required">
        <Input aria-label="name-input" onChange={onChange} />
      </FormField>
    );

    expect(screen.getByText("Name")).toBeInTheDocument();
    expect(screen.getByText("Required")).toBeInTheDocument();
    await user.type(screen.getByLabelText("name-input"), "x");
    expect(onChange).toHaveBeenCalled();
  });

  it("renders checkbox row and range field", async () => {
    const user = userEvent.setup();
    const onCheck = vi.fn();
    const onRange = vi.fn();

    render(
      <>
        <CheckboxRow label="Include inferred" checked={false} onChange={onCheck} />
        <RangeField label="Depth" value={2} min={1} max={5} onChange={onRange} />
      </>
    );

    await user.click(screen.getByLabelText("Include inferred"));
    expect(onCheck).toHaveBeenCalledWith(true);

    const slider = screen.getByRole("slider");
    expect(slider).toHaveValue("2");
    await user.click(slider);
  });

  it("renders sticky actions and button bar", () => {
    render(
      <>
        <ButtonBar>
          <button type="button">Primary</button>
        </ButtonBar>
        <StickyActions>
          <button type="button">Sticky</button>
        </StickyActions>
      </>
    );

    expect(screen.getByRole("button", { name: "Primary" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Sticky" })).toBeInTheDocument();
    expect(document.querySelector(".oc-sticky-actions")).toBeInTheDocument();
  });

  it("renders code block content", () => {
    render(<CodeBlock>{`{"key": "value"}`}</CodeBlock>);
    expect(screen.getByText('{"key": "value"}')).toBeInTheDocument();
  });

  it("shortLabel and kindLabel helpers format strings", () => {
    expect(shortLabel("http://example.org#Person")).toBe("Person");
    expect(kindLabel("sub_class_of")).toBe("sub class of");
  });

  it("card danger variant applies class", () => {
    render(
      <Card variant="danger">
        <span>Breaking</span>
      </Card>
    );
    const card = screen.getByText("Breaking").closest(".oc-card");
    expect(card).toHaveClass("oc-card--danger");
  });

  it("renders all badge variants", () => {
    render(
      <div>
        <Badge variant="default">d</Badge>
        <Badge variant="warning">w</Badge>
        <Badge variant="success">s</Badge>
        <Badge variant="accent">a</Badge>
        <Badge variant="danger">x</Badge>
      </div>
    );
    expect(document.querySelector(".oc-badge--warning")).toBeInTheDocument();
    expect(document.querySelector(".oc-badge--success")).toBeInTheDocument();
    expect(document.querySelector(".oc-badge--accent")).toBeInTheDocument();
  });

  it("renders warning and success callouts", () => {
    render(
      <>
        <Callout variant="warning">Careful</Callout>
        <Callout variant="success">Done</Callout>
      </>
    );
    expect(screen.getByText("Careful")).toHaveClass("oc-callout--warning");
    expect(screen.getByText("Done")).toHaveClass("oc-callout--success");
  });

  it("renders code editor with optional label bar", () => {
    render(
      <CodeEditor label="SQL query" value="SELECT 1" onChange={() => undefined} readOnly />
    );
    expect(screen.getByText("SQL query")).toBeInTheDocument();
    expect(screen.getByDisplayValue("SELECT 1")).toBeInTheDocument();
  });

  it("renders toolbar groups and form primitives", () => {
    render(
      <Toolbar>
        <ToolbarGroup>
          <FormField label="Pick">
            <Select defaultValue="a">
              <option value="a">A</option>
            </Select>
          </FormField>
        </ToolbarGroup>
      </Toolbar>
    );
    expect(document.querySelector(".oc-toolbar")).toBeInTheDocument();
    expect(document.querySelector(".oc-toolbar-group")).toBeInTheDocument();
    expect(screen.getByRole("combobox")).toBeInTheDocument();
  });

  it("renders textarea and panel wide layout", () => {
    render(
      <Panel wide className="custom-panel">
        <TextArea defaultValue="hello" aria-label="notes" />
      </Panel>
    );
    expect(document.querySelector(".oc-panel--wide.custom-panel")).toBeInTheDocument();
    expect(screen.getByLabelText("notes")).toHaveValue("hello");
  });

  it("renders static IRI entries without buttons", () => {
    render(<IriList items={["http://example.org#Thing"]} />);
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
    expect(screen.getByText("Thing")).toBeInTheDocument();
  });

  it("renders section action slot", () => {
    render(
      <Section title="Actions" action={<button type="button">Go</button>}>
        Body
      </Section>
    );
    expect(screen.getByRole("button", { name: "Go" })).toBeInTheDocument();
  });

  it("returns null for empty change list", () => {
    const { container } = render(<ChangeList items={[]} />);
    expect(container).toBeEmptyDOMElement();
  });

  it("renders divider element", () => {
    render(<Divider />);
    expect(document.querySelector(".oc-divider")).toBeInTheDocument();
  });

  it("shortLabel handles slash-delimited IRIs", () => {
    expect(shortLabel("http://example.org/Person")).toBe("Person");
    expect(shortLabel("bare")).toBe("bare");
  });
});
