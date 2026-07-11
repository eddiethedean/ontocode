import { useCallback, useEffect, useState } from "react";
import { DialogShell } from "../components/DialogShell";
import { LoadingState, Section } from "../components/ui";
import type { CatalogStats } from "../messages";
import { getVsCodeApi } from "../vscodeApi";

const METRICS: Array<[keyof CatalogStats, string]> = [
  ["ontology_count", "Ontologies"],
  ["class_count", "Classes"],
  ["object_property_count", "Object properties"],
  ["data_property_count", "Data properties"],
  ["annotation_property_count", "Annotation properties"],
  ["individual_count", "Individuals"],
  ["axiom_count", "Axioms"],
  ["annotation_count", "Annotations"],
  ["triple_count", "Triples"],
  ["diagnostic_error_count", "Diagnostic errors"],
  ["diagnostic_warning_count", "Diagnostic warnings"],
  ["diagnostic_info_count", "Diagnostic information"],
  ["error_count", "Parse errors"],
];

export function MetricsPanel(): JSX.Element {
  const [stats, setStats] = useState<CatalogStats | null>(null);
  const close = useCallback(() => getVsCodeApi().postMessage({ type: "closeDialog" }), []);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "metrics" });
    const handler = (event: MessageEvent): void => {
      const message = event.data as { type?: string; stats?: CatalogStats };
      if (message.type === "loadMetrics" && message.stats) setStats(message.stats);
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  return (
    <DialogShell
      title="Ontology Metrics"
      primaryLabel="Close"
      cancelLabel="Close"
      onPrimary={close}
      onCancel={close}
    >
      {stats ? (
        <Section title="Catalog statistics">
          <dl className="oc-metrics">
            {METRICS.map(([key, label]) => (
              <div key={key} className="oc-metrics-row">
                <dt>{label}</dt>
                <dd>{stats[key].toLocaleString()}</dd>
              </div>
            ))}
          </dl>
        </Section>
      ) : (
        <LoadingState label="Loading catalog statistics…" />
      )}
    </DialogShell>
  );
}
