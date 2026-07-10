import { useState } from "react";
import { FormField, Select, shortLabel } from "../components/ui";
import { PreviewApplyBar } from "../components/PreviewApplyBar";
import { PatchOp } from "../messages";

export function PropertyChainEditor({
  entityIri,
  chains,
  propertyOptions,
  preview,
  onPreview,
  onApply,
}: {
  entityIri: string;
  chains: Array<{ display: string; properties: string[] }>;
  propertyOptions: string[];
  preview: string;
  onPreview: (patches: PatchOp[]) => void;
  onApply: (patches: PatchOp[]) => void;
}): JSX.Element {
  const [chainProps, setChainProps] = useState<string[]>([""]);

  return (
    <div className="oc-section oc-section--nested">
      <h3>Property chains</h3>
      {chains.length > 0 ? (
        <ul className="oc-axiom-list">
          {chains.map((c, idx) => (
            <li key={`chain-${idx}`} className="oc-axiom-item">
              <code>{c.display}</code>
              {c.properties.length >= 2 ? (
                <button
                  type="button"
                  className="secondary"
                  onClick={() =>
                    onApply([
                      {
                        op: "remove_property_chain",
                        entity_iri: entityIri,
                        properties: c.properties,
                      },
                    ])
                  }
                >
                  Remove
                </button>
              ) : null}
            </li>
          ))}
        </ul>
      ) : (
        <p className="oc-muted">None</p>
      )}
      <FormField label="New chain (ordered properties)">
        {chainProps.map((value, idx) => (
          <Select
            key={`prop-${idx}`}
            value={value}
            onChange={(e) => {
              const next = [...chainProps];
              next[idx] = e.target.value;
              setChainProps(next);
            }}
          >
            <option value="">—</option>
            {propertyOptions.map((p) => (
              <option key={p} value={p}>
                {shortLabel(p)}
              </option>
            ))}
          </Select>
        ))}
      </FormField>
      <button
        type="button"
        className="secondary"
        onClick={() => setChainProps([...chainProps, ""])}
      >
        Add property to chain
      </button>
      <PreviewApplyBar
        preview={preview}
        disabled={chainProps.filter(Boolean).length < 2}
        onPreview={() => {
          const properties = chainProps.filter(Boolean);
          if (properties.length < 2) return;
          onPreview([
            {
              op: "add_property_chain",
              entity_iri: entityIri,
              properties,
            },
          ]);
        }}
        onApply={() => {
          const properties = chainProps.filter(Boolean);
          if (properties.length < 2) return;
          onApply([
            {
              op: "add_property_chain",
              entity_iri: entityIri,
              properties,
            },
          ]);
        }}
      />
    </div>
  );
}
