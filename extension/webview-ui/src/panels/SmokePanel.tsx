import { useEffect } from "react";
import { getVsCodeApi } from "../vscodeApi";

export function SmokePanel(): JSX.Element {
  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "smoke" });
  }, []);

  return (
    <div style={{ padding: 16 }}>
      <h1>OntoCode React</h1>
      <p className="muted">Webview foundation is active.</p>
    </div>
  );
}
