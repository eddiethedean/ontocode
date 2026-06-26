import { useEffect, useState } from "react";
import { getVsCodeApi } from "../vscodeApi";
import { HostMessage, isHostMessage, RefactorPlanPayload } from "../messages";

function fileName(path: string): string {
  const parts = path.split(/[/\\]/);
  return parts[parts.length - 1] ?? path;
}

export function RefactorPreviewPanel(): JSX.Element {
  const [plan, setPlan] = useState<RefactorPlanPayload | null>(null);
  const [selected, setSelected] = useState(0);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "refactorPreview" });
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "loadRefactorPlan") {
        setPlan(msg.plan);
        setSelected(0);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  if (!plan) {
    return <p className="muted">Loading refactor preview…</p>;
  }

  const change = plan.changes[selected];

  return (
    <div className="panel">
      <h2>Refactor preview</h2>
      {plan.warnings?.length ? (
        <ul className="warnings">
          {plan.warnings.map((w) => (
            <li key={w}>{w}</li>
          ))}
        </ul>
      ) : null}
      <div className="row">
        <label>
          File
          <select
            value={selected}
            onChange={(e) => setSelected(Number(e.target.value))}
          >
            {plan.changes.map((c, i) => (
              <option key={c.path} value={i}>
                {fileName(c.path)}
              </option>
            ))}
          </select>
        </label>
      </div>
      {change ? (
        <div className="diff">
          <section>
            <h3>Before</h3>
            <pre>{change.original_text}</pre>
          </section>
          <section>
            <h3>After</h3>
            <pre>{change.preview_text}</pre>
          </section>
        </div>
      ) : null}
      <div className="actions">
        <button
          type="button"
          onClick={() => getVsCodeApi().postMessage({ type: "applyRefactor" })}
        >
          Apply
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() => getVsCodeApi().postMessage({ type: "cancelRefactor" })}
        >
          Cancel
        </button>
      </div>
    </div>
  );
}
