import { useCallback, useEffect, useState } from "react";
import { PanelMain } from "../a11y";
import {
  ButtonBar,
  Callout,
  FormField,
  InlineCode,
  LoadingState,
  Panel,
  PanelHeader,
  Section,
  Select,
  StickyActions,
} from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";
import {
  HostMessage,
  ImportsDocumentPayload,
  isHostMessage,
  PatchOp,
} from "../messages";

export function ImportsPanel(): JSX.Element {
  const [payload, setPayload] = useState<ImportsDocumentPayload | null>(null);
  const [preview, setPreview] = useState("");
  const [addTarget, setAddTarget] = useState("");

  const apply = useCallback((patches: PatchOp[], previewOnly: boolean) => {
    getVsCodeApi().postMessage({ type: "applyPatch", patches, previewOnly });
  }, []);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "imports" });

    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "loadImports") {
        setPayload(msg.payload);
        setPreview("");
        setAddTarget("");
      } else if (msg.type === "preview") {
        setPreview(msg.text);
      } else if (msg.type === "error") {
        setPreview(`Error: ${msg.message}`);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  if (!payload) {
    return (
      <Panel>
        <PanelHeader title="Manage Imports" />
        <LoadingState label="Loading ontology imports…" />
      </Panel>
    );
  }

  if (!payload.imports_editable) {
    return (
      <Panel>
        <PanelHeader title="Manage Imports" subtitle={payload.path} />
        {payload.error ? (
          <Callout variant="warning">{payload.error}</Callout>
        ) : (
          <Callout>Imports cannot be edited for this file.</Callout>
        )}
      </Panel>
    );
  }

  const removeImport = (importIri: string): void => {
    if (!payload.ontology_iri) {
      return;
    }
    apply(
      [
        {
          op: "remove_import",
          ontology_iri: payload.ontology_iri,
          import_iri: importIri,
        },
      ],
      false
    );
  };

  const previewRemove = (importIri: string): void => {
    apply(
      [
        {
          op: "remove_import",
          ontology_iri: payload.ontology_iri,
          import_iri: importIri,
        },
      ],
      true
    );
  };

  const addImport = (previewOnly: boolean): void => {
    if (!addTarget || !payload.ontology_iri) {
      return;
    }
    apply(
      [
        {
          op: "add_import",
          ontology_iri: payload.ontology_iri,
          import_iri: addTarget,
        },
      ],
      previewOnly
    );
  };

  return (
    <Panel>
      <PanelMain label="Manage Imports">
      <PanelHeader
        title="Manage Imports"
        subtitle={payload.path}
      />
      <Section title="Ontology">
        <InlineCode>{payload.ontology_iri ?? "—"}</InlineCode>
      </Section>

      {payload.error ? <Callout variant="warning">{payload.error}</Callout> : null}

      <Section title="Current imports">
        {payload.imports.length === 0 ? (
          <Callout>No owl:imports declarations in this file.</Callout>
        ) : (
          <ul className="iri-list">
            {payload.imports.map((imp) => (
              <li key={imp} className="iri-list-item">
                <InlineCode>{imp}</InlineCode>
                <ButtonBar>
                  <button type="button" onClick={() => previewRemove(imp)}>
                    Preview remove
                  </button>
                  <button type="button" onClick={() => removeImport(imp)}>
                    Remove
                  </button>
                </ButtonBar>
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section title="Add import">
        <FormField label="Import from workspace">
          <Select
            value={addTarget}
            onChange={(e) => setAddTarget(e.target.value)}
          >
            <option value="">Select ontology…</option>
            {payload.options.map((opt) => (
              <option key={opt.iri} value={opt.iri}>
                {opt.label} — {opt.iri}
              </option>
            ))}
          </Select>
        </FormField>
        <StickyActions>
          <button type="button" disabled={!addTarget} onClick={() => addImport(true)}>
            Preview add
          </button>
          <button type="button" disabled={!addTarget} onClick={() => addImport(false)}>
            Add import
          </button>
        </StickyActions>
      </Section>

      {preview ? (
        <Section title="Patch preview">
          <pre className="preview-block" role="region" aria-label="Patch preview">
            {preview}
          </pre>
        </Section>
      ) : null}
      </PanelMain>
    </Panel>
  );
}
