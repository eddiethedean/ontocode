import { useCallback, useEffect, useState } from "react";
import { DialogShell } from "../components/DialogShell";
import { FormField, Input, InlineCode, Select } from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";

const IRI_PATTERN = /^https?:\/\/\S+$/;
const PREFIX_PATTERN = /^[A-Za-z_][A-Za-z0-9_-]*$/;

export function PrefixManagerDialog(): JSX.Element {
  const [path, setPath] = useState("");
  const [prefixes, setPrefixes] = useState<Record<string, string>>({});
  const [action, setAction] = useState<"add" | "remove">("add");
  const [prefix, setPrefix] = useState("");
  const [namespaceIri, setNamespaceIri] = useState("");
  const prefixError = PREFIX_PATTERN.test(prefix)
    ? undefined
    : "Prefix must start with a letter or underscore and contain only letters, digits, _ or -.";
  const iriError =
    action === "add" && !IRI_PATTERN.test(namespaceIri)
      ? "Namespace IRI must start with http:// or https://."
      : undefined;
  const validationMessage = prefixError ?? iriError;

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "prefixManager" });
    const handler = (event: MessageEvent): void => {
      const message = event.data as {
        type?: string;
        path?: string;
        prefixes?: Record<string, string>;
      };
      if (message.type === "loadPrefixes") {
        setPath(message.path ?? "");
        setPrefixes(message.prefixes ?? {});
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  const close = useCallback(() => getVsCodeApi().postMessage({ type: "closeDialog" }), []);
  const apply = useCallback(() => {
    getVsCodeApi().postMessage({
      type: "submitPrefix",
      action,
      prefix: prefix.trim(),
      namespaceIri: action === "add" ? namespaceIri.trim() : undefined,
    });
  }, [action, namespaceIri, prefix]);

  return (
    <DialogShell
      title="Prefix Manager"
      primaryLabel={action === "add" ? "Add or update" : "Remove"}
      validationMessage={validationMessage}
      primaryDisabled={Boolean(validationMessage)}
      onPrimary={apply}
      onCancel={close}
    >
      {path ? <p>Ontology: <InlineCode>{path}</InlineCode></p> : null}
      {Object.keys(prefixes).length ? (
        <dl className="oc-prefix-list">
          {Object.entries(prefixes).map(([name, iri]) => (
            <div key={name}>
              <dt>{name}:</dt>
              <dd><InlineCode>{iri}</InlineCode></dd>
            </div>
          ))}
        </dl>
      ) : null}
      <FormField label="Action">
        <Select value={action} onChange={(event) => setAction(event.target.value as "add" | "remove")}>
          <option value="add">Add or update</option>
          <option value="remove">Remove</option>
        </Select>
      </FormField>
      <FormField label="Prefix">
        <Input
          autoFocus
          value={prefix}
          onChange={(event) => {
            const value = event.target.value;
            setPrefix(value);
            if (prefixes[value]) setNamespaceIri(prefixes[value]);
          }}
          placeholder="ex"
        />
      </FormField>
      {action === "add" ? (
        <FormField label="Namespace IRI">
          <Input
            value={namespaceIri}
            onChange={(event) => setNamespaceIri(event.target.value)}
            placeholder="https://example.org/"
          />
        </FormField>
      ) : null}
    </DialogShell>
  );
}
