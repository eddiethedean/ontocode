import { useCallback, useEffect, useState } from "react";
import {
  Badge,
  Button,
  ButtonBar,
  Callout,
  CheckboxRow,
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
import { LiveAnnouncer, PanelMain } from "../a11y";
import {
  HostMessage,
  isHostMessage,
  ReasonerResultPayload,
} from "../messages";
import { getVsCodeApi } from "../vscodeApi";
import type { WorkspaceProps } from "../workspaces/types";

const PROFILES = [
  { id: "el", label: "OWL EL" },
  { id: "rl", label: "OWL RL" },
  { id: "rdfs", label: "RDFS" },
  { id: "dl", label: "OWL DL" },
  { id: "auto", label: "Auto" },
] as const;

export function ReasonerPanel(_props?: WorkspaceProps): JSX.Element {
  const [profile, setProfile] = useState("el");
  const [autoDetect, setAutoDetect] = useState(true);
  const [runId, setRunId] = useState(0);
  const [running, setRunning] = useState(false);
  const [summary, setSummary] = useState<string | undefined>();
  const [error, setError] = useState<string | undefined>();
  const [result, setResult] = useState<ReasonerResultPayload | undefined>();

  useEffect(() => {
    const vscode = getVsCodeApi();
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg = event.data as HostMessage;
      if (msg.type === "reasonerSyncRunId") {
        // Host sync for start / presentResult — do not clear running (#212).
        setRunId(msg.runId);
        return;
      }
      if (msg.type === "reasonerRunCancelled") {
        // Stop Reasoner: advance runId and clear spinner (#269).
        setRunId(msg.runId);
        setRunning(false);
        return;
      }
      if (msg.type === "reasonerResult") {
        if (msg.runId !== undefined) {
          setRunId(msg.runId);
        }
        setRunning(false);
        if (msg.error) {
          setError(msg.error);
          setSummary(undefined);
          setResult(undefined);
          return;
        }
        setError(undefined);
        setSummary(msg.summary);
        setResult(msg.result);
      }
    };
    window.addEventListener("message", handler);
    vscode.postMessage({ type: "ready", panel: "reasoner" });
    return () => window.removeEventListener("message", handler);
  }, []);

  const run = useCallback(() => {
    const next = runId + 1;
    setRunId(next);
    setRunning(true);
    setError(undefined);
    getVsCodeApi().postMessage({
      type: "runReasoner",
      profile,
      autoDetect,
      runId: next,
    });
  }, [runId, profile, autoDetect]);

  const statusAnnounce = running
    ? "Running reasoner"
    : error
      ? `Reasoner error: ${error}`
      : summary
        ? summary
        : result
          ? result.consistent
            ? "Ontology is consistent"
            : "Ontology is inconsistent"
          : "";

  return (
    <Panel>
      <PanelMain label="Reasoner">
      <LiveAnnouncer
        message={statusAnnounce}
        politeness={error || result?.consistent === false ? "assertive" : "polite"}
      />
      <PanelHeader
        title="Reasoner"
        subtitle="Classify the ontology and inspect unsatisfiable classes"
        badges={
          result ? (
            <Badge variant={result.consistent ? "success" : "danger"}>
              {result.consistent ? "Consistent" : "Inconsistent"}
            </Badge>
          ) : undefined
        }
      />

      <Section title="Profile" card>
        <FormField label="Reasoner profile">
          <Select
            aria-label="Reasoner profile"
            value={profile}
            onChange={(e) => setProfile(e.target.value)}
          >
            {PROFILES.map((p) => (
              <option key={p.id} value={p.id}>
                {p.label}
              </option>
            ))}
          </Select>
        </FormField>
        <CheckboxRow
          label="Auto-detect profile"
          checked={autoDetect}
          onChange={setAutoDetect}
        />
      </Section>

      <StickyActions>
        <ButtonBar>
          <Button variant="primary" onClick={run} disabled={running}>
            {running ? "Running…" : "Run Reasoner"}
          </Button>
          <Button
            variant="secondary"
            onClick={() =>
              getVsCodeApi().postMessage({ type: "showInferredHierarchy" })
            }
          >
            Show Inferred Hierarchy
          </Button>
        </ButtonBar>
      </StickyActions>

      {running ? <LoadingState label="Running reasoner…" /> : null}

      {!running && error ? <Callout variant="error">{error}</Callout> : null}

      {!running && !error && summary ? (
        <Callout variant={result?.consistent === false ? "warning" : "success"}>
          {summary}
        </Callout>
      ) : null}

      {!running && !error && !result && !summary ? (
        <EmptyState
          title="No reasoner results yet"
          detail="Choose a profile and run the reasoner to classify the ontology."
          action={
            <Button variant="primary" onClick={run}>
              Run Reasoner
            </Button>
          }
        />
      ) : null}

      {result ? (
        <>
          <Section
            title={`Unsatisfiable (${result.unsatisfiable.length})`}
            card
          >
            {result.unsatisfiable.length === 0 ? (
              <p className="oc-muted">No unsatisfiable classes.</p>
            ) : (
              <ul className="oc-list">
                {result.unsatisfiable.map((iri) => (
                  <li key={iri}>
                    <button
                      type="button"
                      className="oc-link-btn"
                      onClick={() =>
                        getVsCodeApi().postMessage({
                          type: "explainUnsat",
                          classIri: iri,
                        })
                      }
                    >
                      <InlineCode>{shortLabel(iri)}</InlineCode>
                    </button>
                    <span className="oc-muted"> Explain</span>
                  </li>
                ))}
              </ul>
            )}
          </Section>

          <Section
            title={`Inferred changes (${result.new_inferences.length})`}
            card
          >
            {result.new_inferences.length === 0 ? (
              <p className="oc-muted">No new subclass inferences.</p>
            ) : (
              <ul className="oc-list">
                {result.new_inferences.map((edge, i) => (
                  <li key={`${edge.child}-${edge.parent}-${i}`}>
                    <InlineCode>{shortLabel(edge.child)}</InlineCode>
                    {" SubClassOf "}
                    <InlineCode>{shortLabel(edge.parent)}</InlineCode>
                  </li>
                ))}
              </ul>
            )}
          </Section>

          <Section title={`Warnings (${result.warnings.length})`} card>
            {result.warnings.length === 0 ? (
              <p className="oc-muted">No warnings.</p>
            ) : (
              <ul className="oc-list">
                {result.warnings.map((w, i) => (
                  <li key={`${w.message}-${i}`}>{w.message}</li>
                ))}
              </ul>
            )}
          </Section>

          {result.snapshot?.consistency &&
          (result.snapshot.consistency.abox_clashes?.length ?? 0) > 0 ? (
            <Section
              title={`ABox clashes (${result.snapshot.consistency.abox_clashes.length})`}
              card
            >
              <ul className="oc-list">
                {result.snapshot.consistency.abox_clashes.map((c, i) => (
                  <li key={`${c}-${i}`}>{c}</li>
                ))}
              </ul>
            </Section>
          ) : null}

          {result.snapshot?.realization &&
          result.snapshot.realization.individuals.length > 0 ? (
            <Section
              title={`Realization (${result.snapshot.realization.individuals.length})`}
              card
            >
              <ul className="oc-list">
                {result.snapshot.realization.individuals
                  .slice(0, 50)
                  .map((ind) => (
                    <li key={ind.individual_iri}>
                      <InlineCode>{shortLabel(ind.individual_iri)}</InlineCode>
                      {": "}
                      {(ind.most_specific.length
                        ? ind.most_specific
                        : ind.types
                      )
                        .map((t) => shortLabel(t))
                        .join(", ")}
                    </li>
                  ))}
              </ul>
            </Section>
          ) : null}
        </>
      ) : null}
      </PanelMain>
    </Panel>
  );
}
