import { useCallback, useEffect, useState } from "react";
import { DialogShell } from "../components/DialogShell";
import { FormField, Input, InlineCode } from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";

const IRI_PATTERN = /^https?:\/\/\S+$/;

export function NewOntologyDialog(): JSX.Element {
  const [path, setPath] = useState("");
  const [ontologyIri, setOntologyIri] = useState("https://example.org/ontology");
  const [versionIri, setVersionIri] = useState("");
  const ontologyError = IRI_PATTERN.test(ontologyIri)
    ? undefined
    : "Ontology IRI must start with http:// or https://.";
  const versionError =
    versionIri && !IRI_PATTERN.test(versionIri)
      ? "Version IRI must start with http:// or https://."
      : undefined;
  const validationMessage = ontologyError ?? versionError;

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "newOntology" });
    const handler = (event: MessageEvent): void => {
      const message = event.data as {
        type?: string;
        path?: string;
        defaultIri?: string;
      };
      if (message.type === "loadNewOntology") {
        setPath(message.path ?? "");
        setOntologyIri(message.defaultIri ?? "https://example.org/ontology");
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  const close = useCallback(() => getVsCodeApi().postMessage({ type: "closeDialog" }), []);
  const create = useCallback(() => {
    getVsCodeApi().postMessage({
      type: "submitNewOntology",
      ontologyIri: ontologyIri.trim(),
      versionIri: versionIri.trim() || undefined,
    });
  }, [ontologyIri, versionIri]);

  return (
    <DialogShell
      title="New Ontology"
      primaryLabel="Create"
      validationMessage={validationMessage}
      primaryDisabled={Boolean(validationMessage)}
      onPrimary={create}
      onCancel={close}
    >
      {path ? <p>File: <InlineCode>{path}</InlineCode></p> : null}
      <FormField label="Ontology IRI">
        <Input
          autoFocus
          value={ontologyIri}
          onChange={(event) => setOntologyIri(event.target.value)}
          placeholder="https://example.org/ontology"
        />
      </FormField>
      <FormField label="Version IRI" hint="Optional">
        <Input
          value={versionIri}
          onChange={(event) => setVersionIri(event.target.value)}
          placeholder="https://example.org/ontology/1.0"
        />
      </FormField>
    </DialogShell>
  );
}
