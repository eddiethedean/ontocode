import { useEffect } from "react";
import { Panel } from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";

export function SmokePanel(): JSX.Element {
  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "smoke" });
  }, []);

  return (
    <Panel>
      <div className="oc-brand">
        <div className="oc-brand-mark" aria-hidden="true" />
        <h1>OntoCode React</h1>
        <p className="oc-muted">Webview foundation is active.</p>
      </div>
    </Panel>
  );
}
