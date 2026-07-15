import { useCallback, useEffect, useRef, useState } from "react";
import { LiveAnnouncer, PanelMain } from "../a11y";
import {
  Callout,
  CodeBlock,
  CodeEditor,
  FormField,
  InlineCode,
  Panel,
  PanelHeader,
  Section,
  Select,
  StickyActions,
  Toolbar,
  ToolbarGroup,
} from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";
import {
  HostMessage,
  isHostMessage,
  ManchesterCompletions,
  ManchesterValidationResult,
} from "../messages";

export function ManchesterEditorPanel(): JSX.Element {
  const [entityIri, setEntityIri] = useState("");
  const [axiomKind, setAxiomKind] = useState("sub_class_of");
  const [expression, setExpression] = useState("");
  const [completions, setCompletions] = useState<ManchesterCompletions>({
    classes: [],
    object_properties: [],
    data_properties: [],
    datatypes: [],
  });
  const [validation, setValidation] = useState<ManchesterValidationResult | null>(
    null
  );
  const [error, setError] = useState("");
  const [preview, setPreview] = useState("");
  const [seq, setSeq] = useState(0);
  const seqRef = useRef(0);

  useEffect(() => {
    seqRef.current = seq;
  }, [seq]);

  const validate = useCallback(
    (expr: string, kind: string, nextSeq: number) => {
      getVsCodeApi().postMessage({
        type: "validateManchester",
        expression: expr,
        axiomKind: kind,
        seq: nextSeq,
      });
    },
    []
  );

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "manchesterEditor" });
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "manchesterInit") {
        setEntityIri(msg.entityIri);
        setAxiomKind(msg.axiomKind);
        setExpression(msg.expression);
        setCompletions(msg.completions);
        setValidation(null);
        setError("");
        setPreview("");
      }
      if (msg.type === "manchesterValidation") {
        if (msg.seq !== seqRef.current) {
          return;
        }
        setError(msg.error ?? "");
        setValidation(msg.result ?? null);
      }
      if (msg.type === "preview") {
        setPreview(msg.text);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  useEffect(() => {
    if (!entityIri) {
      return;
    }
    const timer = setTimeout(() => {
      const next = Date.now();
      seqRef.current = next;
      setSeq(next);
      validate(expression, axiomKind, next);
    }, 500);
    return () => clearTimeout(timer);
  }, [expression, axiomKind, entityIri, validate]);

  const formatInsertTerm = (term: string): string => {
    if (term.includes(":") && !term.startsWith("http")) {
      return term;
    }
    const local = term.split(/[#/]/).pop() ?? term;
    const prefixed = `ex:${local}`;
    const lists = [
      ...completions.classes,
      ...completions.object_properties,
      ...completions.data_properties,
    ];
    if (lists.includes(prefixed)) {
      return prefixed;
    }
    return term;
  };

  const insertTerm = (term: string): void => {
    const formatted = formatInsertTerm(term);
    setExpression((prev) => (prev ? `${prev} ${formatted}` : formatted));
  };

  const turtlePreview = preview || validation?.turtle_fragment || "—";

  const validationAnnounce = error
    ? error
    : validation?.diagnostics?.length
      ? `${validation.diagnostics.length} validation issue${validation.diagnostics.length === 1 ? "" : "s"}`
      : validation
        ? "Expression validated"
        : "";

  return (
    <Panel>
      <PanelMain label="Manchester Axiom Editor">
      <LiveAnnouncer
        message={validationAnnounce}
        politeness={error ? "assertive" : "polite"}
      />
      <PanelHeader
        title="Manchester Axiom Editor"
        subtitle={<InlineCode>{entityIri || "—"}</InlineCode>}
      />

      <FormField label="Axiom type">
        <Select
          value={axiomKind}
          onChange={(e) => setAxiomKind(e.target.value)}
          aria-label="Axiom type"
        >
          <option value="sub_class_of">SubClassOf</option>
          <option value="equivalent_class">EquivalentClasses</option>
          <option value="disjoint_class">DisjointClasses</option>
        </Select>
      </FormField>

      <FormField
        label={axiomKind === "disjoint_class" ? "Other class IRI" : "Expression"}
      >
        <CodeEditor
          id="manchester-expression"
          aria-label={
            axiomKind === "disjoint_class" ? "Other class IRI" : "Manchester expression"
          }
          value={expression}
          onChange={(e) => setExpression(e.target.value)}
          rows={6}
          placeholder={
            axiomKind === "disjoint_class"
              ? "http://example.org#OtherClass or ex:OtherClass"
              : "ex:hasPart some ex:Component"
          }
        />
      </FormField>

      {axiomKind !== "disjoint_class" ? (
        <Toolbar>
          <ToolbarGroup>
            <FormField label="Class">
              <Select
                onChange={(e) => {
                  if (e.target.value) {
                    insertTerm(e.target.value);
                  }
                }}
              >
                <option value="">—</option>
                {completions.classes.slice(0, 40).map((c) => (
                  <option key={c} value={c}>
                    {c}
                  </option>
                ))}
              </Select>
            </FormField>
            <FormField label="Object prop">
              <Select
                onChange={(e) => {
                  if (e.target.value) {
                    insertTerm(e.target.value);
                  }
                }}
              >
                <option value="">—</option>
                {completions.object_properties.slice(0, 40).map((c) => (
                  <option key={c} value={c}>
                    {c}
                  </option>
                ))}
              </Select>
            </FormField>
          </ToolbarGroup>
          <ToolbarGroup>
            <FormField label="Data prop">
              <Select
                onChange={(e) => {
                  if (e.target.value) {
                    insertTerm(e.target.value);
                  }
                }}
              >
                <option value="">—</option>
                {completions.data_properties.slice(0, 40).map((c) => (
                  <option key={c} value={c}>
                    {c}
                  </option>
                ))}
              </Select>
            </FormField>
            <FormField label="Datatype">
              <Select
                onChange={(e) => {
                  if (e.target.value) {
                    insertTerm(e.target.value);
                  }
                }}
              >
                <option value="">—</option>
                {completions.datatypes.slice(0, 40).map((c) => (
                  <option key={c} value={c}>
                    {c}
                  </option>
                ))}
              </Select>
            </FormField>
          </ToolbarGroup>
        </Toolbar>
      ) : null}

      <StickyActions>
        <button
          type="button"
          onClick={() => {
            const next = seq + 1;
            seqRef.current = next;
            setSeq(next);
            validate(expression, axiomKind, next);
          }}
        >
          Validate
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() =>
            getVsCodeApi().postMessage({
              type: "applyManchester",
              expression,
              axiomKind,
              previewOnly: true,
            })
          }
        >
          Preview Turtle
        </button>
        <button
          type="button"
          onClick={() =>
            getVsCodeApi().postMessage({
              type: "applyManchester",
              expression,
              axiomKind,
              previewOnly: false,
            })
          }
        >
          Apply
        </button>
      </StickyActions>

      {error ? <Callout variant="error">{error}</Callout> : null}
      {validation?.diagnostics?.length ? (
        <ul className="warnings">
          {validation.diagnostics.map((d, i) => (
            <li key={i}>{d.message}</li>
          ))}
        </ul>
      ) : null}

      <Section title="Expression tree">
        <CodeBlock>
          {validation?.tree ? JSON.stringify(validation.tree, null, 2) : "—"}
        </CodeBlock>
      </Section>

      <Section title="Turtle preview">
        <CodeBlock>{turtlePreview}</CodeBlock>
      </Section>
      </PanelMain>
    </Panel>
  );
}
