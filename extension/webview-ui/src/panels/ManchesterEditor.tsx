import { useCallback, useEffect, useState } from "react";
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
      }
      if (msg.type === "manchesterValidation") {
        if (msg.seq !== seq) {
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
  }, [seq]);

  useEffect(() => {
    if (!entityIri) {
      return;
    }
    const timer = setTimeout(() => {
      const next = Date.now();
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

  return (
    <div className="panel">
      <h2>Manchester Axiom Editor</h2>
      <p>
        <code>{entityIri}</code>
      </p>
      <label>
        Axiom type{" "}
        <select
          value={axiomKind}
          onChange={(e) => setAxiomKind(e.target.value)}
        >
          <option value="sub_class_of">SubClassOf</option>
          <option value="equivalent_class">EquivalentClasses</option>
          <option value="disjoint_class">DisjointClasses</option>
        </select>
      </label>
      <h3>{axiomKind === "disjoint_class" ? "Other class IRI" : "Expression"}</h3>
      <textarea
        value={expression}
        onChange={(e) => setExpression(e.target.value)}
        rows={6}
        style={{ width: "100%", fontFamily: "var(--vscode-editor-font-family)" }}
        placeholder={
          axiomKind === "disjoint_class"
            ? "http://example.org#OtherClass or ex:OtherClass"
            : "ex:hasPart some ex:Component"
        }
      />
      {axiomKind !== "disjoint_class" ? (
        <div className="toolbar">
          <label>
            Class{" "}
            <select
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
            </select>
          </label>
          <label>
            Object prop{" "}
            <select
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
            </select>
          </label>
          <label>
            Data prop{" "}
            <select
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
            </select>
          </label>
          <label>
            Datatype{" "}
            <select
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
            </select>
          </label>
        </div>
      ) : null}
      <div className="actions">
        <button
          type="button"
          onClick={() => {
            const next = seq + 1;
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
      </div>
      {error ? <p className="error">{error}</p> : null}
      {validation?.diagnostics?.length ? (
        <ul className="warnings">
          {validation.diagnostics.map((d, i) => (
            <li key={i}>{d.message}</li>
          ))}
        </ul>
      ) : null}
      <h3>Expression tree</h3>
      <pre className="muted">
        {validation?.tree ? JSON.stringify(validation.tree, null, 2) : "—"}
      </pre>
      <h3>Turtle preview</h3>
      <pre className="preview muted">
        {preview || validation?.turtle_fragment || "—"}
      </pre>
    </div>
  );
}
