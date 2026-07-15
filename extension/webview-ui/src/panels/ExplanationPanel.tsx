import { useEffect, useMemo, useState } from "react";
import { LiveAnnouncer, PanelMain } from "../a11y";
import {
  Badge,
  Button,
  ButtonBar,
  Callout,
  CodeBlock,
  EmptyState,
  FormField,
  InlineCode,
  LoadingState,
  Panel,
  PanelHeader,
  Section,
  Select,
  StickyActions,
  shortLabel,
} from "../components/ui";
import {
  ExplanationPayload,
  HostMessage,
  isHostMessage,
} from "../messages";
import { getVsCodeApi } from "../vscodeApi";
import type { WorkspaceProps } from "../workspaces/types";

export function ExplanationPanel(_props?: WorkspaceProps): JSX.Element {
  const [payload, setPayload] = useState<ExplanationPayload | null>(null);
  const [index, setIndex] = useState(0);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const vscode = getVsCodeApi();
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg = event.data as HostMessage;
      if (msg.type === "loading") {
        setLoading(true);
        return;
      }
      if (msg.type === "loadExplanation") {
        setPayload(msg.payload);
        setIndex(0);
        setLoading(false);
      }
      if (msg.type === "error") {
        setPayload(null);
        setLoading(false);
      }
    };
    window.addEventListener("message", handler);
    vscode.postMessage({ type: "ready", panel: "explanation" });
    return () => window.removeEventListener("message", handler);
  }, []);

  const justification = useMemo(() => {
    if (!payload || payload.justifications.length === 0) {
      return undefined;
    }
    return payload.justifications[Math.min(index, payload.justifications.length - 1)];
  }, [payload, index]);

  if (loading) {
    return (
      <Panel>
        <PanelHeader title="Explanation" />
        <LoadingState label="Loading explanation…" />
      </Panel>
    );
  }

  if (!payload || !justification) {
    return (
      <Panel>
        <PanelHeader title="Explanation" />
        <EmptyState
          title="No explanation"
          detail="Select an unsatisfiable class from the Reasoner panel to explain it."
        />
      </Panel>
    );
  }

  return (
    <Panel>
      <PanelMain label="Explanation">
      <LiveAnnouncer
        message={`Explanation for ${shortLabel(payload.classIri)}, justification ${index + 1} of ${payload.justifications.length}`}
      />
      <PanelHeader
        title="Explanation"
        subtitle={<InlineCode>{payload.classIri}</InlineCode>}
        badges={
          <>
            <Badge variant="kind">{payload.profile}</Badge>
            {payload.stale ? <Badge variant="warning">Stale</Badge> : null}
          </>
        }
      />

      {payload.stale ? (
        <Callout variant="warning">
          Ontology or reasoner state changed since this explanation was generated.
          Re-run the reasoner to ensure correctness.
        </Callout>
      ) : null}

      <Section title="Justification" card>
        <FormField label="Alternative">
          <Select
            aria-label="Justification"
            value={String(index)}
            onChange={(e) => setIndex(Number(e.target.value))}
          >
            {payload.justifications.map((j, i) => (
              <option key={j.title} value={String(i)}>
                {j.title}
              </option>
            ))}
          </Select>
        </FormField>
        <p className="oc-muted">
          indexed_at={payload.indexed_at} · content_hash={shortLabel(payload.content_hash)}
        </p>
        <ol className="oc-list oc-list--ordered">
          {justification.steps.map((step) => (
            <li key={step.index}>
              <span>{step.display}</span>
              {step.subject_iri ? (
                <>
                  {" "}
                  <button
                    type="button"
                    className="oc-link-btn"
                    onClick={() =>
                      getVsCodeApi().postMessage({
                        type: "openEntity",
                        iri: step.subject_iri!,
                      })
                    }
                  >
                    subject
                  </button>
                </>
              ) : null}
              {step.object_iri ? (
                <>
                  {" "}
                  <button
                    type="button"
                    className="oc-link-btn"
                    onClick={() =>
                      getVsCodeApi().postMessage({
                        type: "openEntity",
                        iri: step.object_iri!,
                      })
                    }
                  >
                    object
                  </button>
                </>
              ) : null}
            </li>
          ))}
        </ol>
        <CodeBlock>{justification.text}</CodeBlock>
      </Section>

      <StickyActions>
        <ButtonBar>
          <Button
            variant="secondary"
            onClick={() =>
              getVsCodeApi().postMessage({
                type: "copyText",
                text: justification.text,
              })
            }
          >
            Copy
          </Button>
          <Button
            variant="primary"
            onClick={() => getVsCodeApi().postMessage({ type: "rerunReasoner" })}
          >
            Re-run Reasoner
          </Button>
        </ButtonBar>
      </StickyActions>
      </PanelMain>
    </Panel>
  );
}
