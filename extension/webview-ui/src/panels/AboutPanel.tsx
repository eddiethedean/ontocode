import { useCallback, useEffect } from "react";
import { DialogShell } from "../components/DialogShell";
import { Section } from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";

export function AboutPanel(): JSX.Element {
  const close = useCallback(() => getVsCodeApi().postMessage({ type: "closeDialog" }), []);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "about" });
  }, []);

  return (
    <DialogShell
      title="About OntoCode"
      primaryLabel="Close"
      cancelLabel="Close"
      onPrimary={close}
      onCancel={close}
    >
      <Section>
        <p><strong>OntoCode 0.17.0</strong></p>
        <p>Ontology engineering for VS Code, powered by OntoCore.</p>
        <p>
          <a href="https://ontocode-vs.readthedocs.io/en/latest/">Documentation</a>
          {" · "}
          <a href="https://github.com/eddiethedean/ontocode">Source code</a>
          {" · "}
          <a href="https://github.com/eddiethedean/ontocode/issues">Support</a>
        </p>
      </Section>
    </DialogShell>
  );
}
