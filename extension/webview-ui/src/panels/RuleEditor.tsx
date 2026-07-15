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
  StickyActions,
} from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";
import { HostMessage, isHostMessage } from "../messages";

export function RuleEditorPanel(): JSX.Element {
  const [ruleJson, setRuleJson] = useState("");
  const [documentUri, setDocumentUri] = useState("");
  const [ontologyIri, setOntologyIri] = useState("");
  const [diagnostics, setDiagnostics] = useState<
    Array<{ code: string; severity: string; message: string }>
  >([]);
  const [error, setError] = useState("");
  const [preview, setPreview] = useState("");
  const seqRef = useRef(0);

  const validate = useCallback((json: string, nextSeq: number) => {
    getVsCodeApi().postMessage({
      type: "validateSwrlRule",
      ruleJson: json,
      seq: nextSeq,
    });
  }, []);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "ruleEditor" });
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "swrlRuleInit") {
        setRuleJson(msg.ruleJson);
        setDocumentUri(msg.documentUri);
        setOntologyIri(msg.ontologyIri);
        setDiagnostics([]);
        setError("");
        setPreview("");
      }
      if (msg.type === "swrlRuleValidation") {
        if (msg.seq !== seqRef.current) {
          return;
        }
        setError(msg.error ?? "");
        setDiagnostics(msg.diagnostics ?? []);
      }
      if (msg.type === "preview") {
        setPreview(msg.text);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  useEffect(() => {
    if (!ruleJson.trim()) {
      return;
    }
    const timer = setTimeout(() => {
      const next = Date.now();
      seqRef.current = next;
      validate(ruleJson, next);
    }, 400);
    return () => clearTimeout(timer);
  }, [ruleJson, validate]);

  const hasErrors = diagnostics.some((d) => d.severity === "error");

  return (
    <Panel>
      <PanelMain label="SWRL Rule Editor">
      <LiveAnnouncer
        message={
          error
            ? error
            : diagnostics.length
              ? `${diagnostics.length} diagnostic${diagnostics.length === 1 ? "" : "s"}`
              : ""
        }
        politeness={error || hasErrors ? "assertive" : "polite"}
      />
      <PanelHeader
        title="SWRL Rule Editor"
        subtitle={<InlineCode>{ontologyIri || documentUri || "—"}</InlineCode>}
      />
      <FormField label="Rule JSON">
        <CodeEditor
          id="swrl-rule-json"
          aria-label="Rule JSON"
          value={ruleJson}
          onChange={(e) => setRuleJson(e.target.value)}
          rows={14}
        />
      </FormField>
      {error ? <Callout variant="error">{error}</Callout> : null}
      {diagnostics.length > 0 ? (
        <Section title="Diagnostics">
          <ul className="oc-list">
            {diagnostics.map((d, i) => (
              <li key={`${d.code}-${i}`}>
                <strong>{d.severity}</strong> [{d.code}] {d.message}
              </li>
            ))}
          </ul>
        </Section>
      ) : null}
      {preview ? (
        <Section title="Preview">
          <CodeBlock>{preview}</CodeBlock>
        </Section>
      ) : null}
      <StickyActions>
        <button
          type="button"
          className="secondary"
          disabled={!ruleJson.trim() || hasErrors}
          onClick={() =>
            getVsCodeApi().postMessage({
              type: "applySwrlRule",
              ruleJson,
              previewOnly: true,
            })
          }
        >
          Preview
        </button>
        <button
          type="button"
          disabled={!ruleJson.trim() || hasErrors}
          onClick={() =>
            getVsCodeApi().postMessage({
              type: "applySwrlRule",
              ruleJson,
              previewOnly: false,
            })
          }
        >
          Apply
        </button>
      </StickyActions>
      </PanelMain>
    </Panel>
  );
}
