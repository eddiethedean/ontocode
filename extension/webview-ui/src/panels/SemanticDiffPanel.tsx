import { useEffect, useState } from "react";
import { getVsCodeApi } from "../vscodeApi";
import { DiffPayload, HostMessage, isHostMessage } from "../messages";

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

  const counts = {
    entities: diff.entity_changes.length,
    axioms: diff.axiom_changes.length,
    annotations: diff.annotation_changes.length,
    imports: diff.import_changes.length,
    inferences: diff.inference_changes.length,
    breaking: diff.breaking_changes.length,
  };

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
          <ul>
            {diff.breaking_changes.map((b, i) => (
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
          <ul>
            {diff.entity_changes.map((e, i) => (
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
          <ul>
            {diff.axiom_changes.slice(0, 50).map((a, i) => (
              <li key={i}>
                {a.change} {a.axiom_kind} {a.subject}
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
