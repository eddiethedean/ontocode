import { useEffect, useState } from "react";
import {
  Callout,
  DiffColumns,
  FormField,
  LoadingState,
  Panel,
  PanelHeader,
  Select,
  StickyActions,
} from "../components/ui";
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
    return (
      <Panel>
        <LoadingState label="Loading refactor preview…" />
      </Panel>
    );
  }

  const change = plan.changes[selected];

  return (
    <Panel>
      <PanelHeader
        title="Refactor preview"
        subtitle={`${plan.changes.length} file${plan.changes.length === 1 ? "" : "s"} affected`}
      />

      {plan.warnings?.length ? (
        <ul className="warnings">
          {plan.warnings.map((w, i) => (
            <li key={`${i}-${w}`}>{w}</li>
          ))}
        </ul>
      ) : null}

      <FormField label="File">
        <Select
          value={selected}
          onChange={(e) => setSelected(Number(e.target.value))}
        >
          {plan.changes.map((c, i) => (
            <option key={c.path} value={i}>
              {fileName(c.path)}
            </option>
          ))}
        </Select>
      </FormField>

      {change ? (
        <DiffColumns before={change.original_text} after={change.preview_text} />
      ) : null}

      <StickyActions>
        <button
          type="button"
          onClick={() => getVsCodeApi().postMessage({ type: "applyRefactor" })}
        >
          Apply changes
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() => getVsCodeApi().postMessage({ type: "cancelRefactor" })}
        >
          Cancel
        </button>
      </StickyActions>
    </Panel>
  );
}
