import { useEffect, useState } from "react";
import {
  Callout,
  ChangeList,
  EmptyState,
  LoadingState,
  PanelHeader,
  StatGrid,
  StickyActions,
} from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";
import { DiffPayload, HostMessage, isHostMessage } from "../messages";

const LIST_CAP = 50;

function renderTruncationBanner(shown: number, total: number, label: string): JSX.Element | null {
  if (total <= shown) {
    return null;
  }
  return (
    <p className="oc-muted">
      Showing {shown} of {total} {label}.
    </p>
  );
}

export function SemanticDiffPanel(): JSX.Element {
  const [diff, setDiff] = useState<DiffPayload | null>(null);
  const [error, setError] = useState<string | undefined>();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const vscode = getVsCodeApi();
    const handler = (event: MessageEvent): void => {
      const data: unknown = event.data;
      if (!isHostMessage(data)) {
        return;
      }
      const msg = data as HostMessage;
      if (msg.type === "loading") {
        setLoading(true);
        setError(undefined);
      }
      if (msg.type === "semanticDiffData") {
        setDiff(msg.diff);
        setError(undefined);
        setLoading(false);
      }
      if (msg.type === "error") {
        setError(msg.message);
        setLoading(false);
      }
    };
    window.addEventListener("message", handler);
    vscode.postMessage({ type: "ready", panel: "semanticDiff" });
    return () => window.removeEventListener("message", handler);
  }, []);

  if (loading) {
    return (
      <div className="semantic-diff">
        <LoadingState label="Computing semantic diff…" />
      </div>
    );
  }
  if (error) {
    return (
      <div className="semantic-diff">
        <Callout variant="error">{error}</Callout>
      </div>
    );
  }
  if (!diff) {
    return (
      <div className="semantic-diff">
        <EmptyState title="No diff data" />
      </div>
    );
  }

  const counts = diff.summary_counts ?? {
    entities: diff.entity_changes.length,
    axioms: diff.axiom_changes.length,
    annotations: diff.annotation_changes.length,
    imports: diff.import_changes.length,
    inferences: diff.inference_changes.length,
    breaking: diff.breaking_changes.length,
  };

  const breakingShown = diff.breaking_changes.slice(0, LIST_CAP);
  const entityShown = diff.entity_changes.slice(0, LIST_CAP);
  const axiomShown = diff.axiom_changes.slice(0, LIST_CAP);
  const annotationShown = diff.annotation_changes.slice(0, LIST_CAP);
  const importShown = diff.import_changes.slice(0, LIST_CAP);
  const inferenceShown = diff.inference_changes.slice(0, LIST_CAP);

  return (
    <div className="semantic-diff">
      <PanelHeader title="Semantic diff" />

      <section>
        <h3>Summary</h3>
        <StatGrid
          items={[
            { label: "Entities", value: counts.entities },
            { label: "Axioms", value: counts.axioms },
            { label: "Annotations", value: counts.annotations },
            { label: "Imports", value: counts.imports },
            { label: "Inferences", value: counts.inferences },
            {
              label: "Breaking",
              value: counts.breaking,
              variant: counts.breaking > 0 ? "danger" : "default",
            },
          ]}
        />
      </section>

      {diff.breaking_changes.length > 0 && (
        <section className="oc-section--breaking">
          <h3>Breaking changes</h3>
          {renderTruncationBanner(breakingShown.length, diff.breaking_changes.length, "breaking changes")}
          <ChangeList
            items={breakingShown.map((b, i) => ({
              key: `breaking-${i}`,
              badge: b.reason,
              primary: b.message,
            }))}
          />
        </section>
      )}

      {diff.entity_changes.length > 0 && (
        <section>
          <h3>Entity changes</h3>
          {renderTruncationBanner(entityShown.length, diff.entity_changes.length, "entity changes")}
          <ChangeList
            items={entityShown.map((e, i) => ({
              key: `entity-${i}`,
              badge: e.kind,
              primary: e.iri,
            }))}
          />
        </section>
      )}

      {diff.axiom_changes.length > 0 && (
        <section>
          <h3>Axiom changes</h3>
          {renderTruncationBanner(axiomShown.length, diff.axiom_changes.length, "axiom changes")}
          <ChangeList
            items={axiomShown.map((a, i) => ({
              key: `axiom-${i}`,
              badge: a.change,
              primary: `${a.axiom_kind} · ${a.subject}`,
            }))}
          />
        </section>
      )}

      {diff.annotation_changes.length > 0 && (
        <section>
          <h3>Annotation changes</h3>
          {renderTruncationBanner(
            annotationShown.length,
            diff.annotation_changes.length,
            "annotation changes"
          )}
          <ChangeList
            items={annotationShown.map((a, i) => ({
              key: `annotation-${i}`,
              badge: a.change,
              primary: `${a.predicate} on ${a.subject}`,
            }))}
          />
        </section>
      )}

      {diff.import_changes.length > 0 && (
        <section>
          <h3>Import changes</h3>
          {renderTruncationBanner(importShown.length, diff.import_changes.length, "import changes")}
          <ChangeList
            items={importShown.map((imp, i) => ({
              key: `import-${i}`,
              badge: imp.change,
              primary: imp.import_iri,
              secondary: imp.ontology_id,
            }))}
          />
        </section>
      )}

      {diff.inference_changes.length > 0 && (
        <section>
          <h3>Inference changes</h3>
          {renderTruncationBanner(
            inferenceShown.length,
            diff.inference_changes.length,
            "inference changes"
          )}
          <ChangeList
            items={inferenceShown.map((inf, i) => ({
              key: `inference-${i}`,
              badge: inf.change,
              primary: inf.class_iri,
              secondary: inf.detail,
            }))}
          />
        </section>
      )}

      <StickyActions>
        <button
          type="button"
          onClick={() => getVsCodeApi().postMessage({ type: "copyMarkdown" })}
        >
          Copy Markdown summary
        </button>
      </StickyActions>
    </div>
  );
}
