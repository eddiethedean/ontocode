import { useEffect, useState } from "react";
import { getVsCodeApi } from "../vscodeApi";
import { DiffPayload, HostMessage, isHostMessage } from "../messages";

const LIST_CAP = 50;

function renderTruncationBanner(shown: number, total: number, label: string): JSX.Element | null {
  if (total <= shown) {
    return null;
  }
  return (
    <p className="muted">
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
    return <p className="muted">Computing semantic diff…</p>;
  }
  if (error) {
    return <p className="error">{error}</p>;
  }
  if (!diff) {
    return <p className="muted">No diff data.</p>;
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
      <h2>Semantic diff</h2>
      <section>
        <h3>Summary</h3>
        <ul>
          <li>Entities: {counts.entities}</li>
          <li>Axioms: {counts.axioms}</li>
          <li>Annotations: {counts.annotations}</li>
          <li>Imports: {counts.imports}</li>
          <li>Inferences: {counts.inferences}</li>
          <li>Breaking: {counts.breaking}</li>
        </ul>
      </section>
      {diff.breaking_changes.length > 0 && (
        <section>
          <h3>Breaking changes</h3>
          {renderTruncationBanner(breakingShown.length, diff.breaking_changes.length, "breaking changes")}
          <ul>
            {breakingShown.map((b, i) => (
              <li key={i}>
                <strong>{b.reason}</strong>: {b.message}
              </li>
            ))}
          </ul>
        </section>
      )}
      {diff.entity_changes.length > 0 && (
        <section>
          <h3>Entity changes</h3>
          {renderTruncationBanner(entityShown.length, diff.entity_changes.length, "entity changes")}
          <ul>
            {entityShown.map((e, i) => (
              <li key={i}>
                {e.kind}: {e.iri}
              </li>
            ))}
          </ul>
        </section>
      )}
      {diff.axiom_changes.length > 0 && (
        <section>
          <h3>Axiom changes</h3>
          {renderTruncationBanner(axiomShown.length, diff.axiom_changes.length, "axiom changes")}
          <ul>
            {axiomShown.map((a, i) => (
              <li key={i}>
                {a.change} {a.axiom_kind} {a.subject}
              </li>
            ))}
          </ul>
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
          <ul>
            {annotationShown.map((a, i) => (
              <li key={i}>
                {a.change} {a.predicate} on {a.subject}
              </li>
            ))}
          </ul>
        </section>
      )}
      {diff.import_changes.length > 0 && (
        <section>
          <h3>Import changes</h3>
          {renderTruncationBanner(importShown.length, diff.import_changes.length, "import changes")}
          <ul>
            {importShown.map((imp, i) => (
              <li key={i}>
                {imp.change} {imp.import_iri} in {imp.ontology_id}
              </li>
            ))}
          </ul>
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
          <ul>
            {inferenceShown.map((inf, i) => (
              <li key={i}>
                {inf.change} {inf.class_iri}: {inf.detail}
              </li>
            ))}
          </ul>
        </section>
      )}
      <button
        type="button"
        onClick={() => getVsCodeApi().postMessage({ type: "copyMarkdown" })}
      >
        Copy Markdown summary
      </button>
    </div>
  );
}
