import { RunReasonerResult } from "../lsp/protocol";

export type ReasonerProfileId = "el" | "rl" | "rdfs" | "dl" | "auto";

export const AVAILABLE_PROFILES: Array<{
  id: ReasonerProfileId;
  label: string;
  enabled: boolean;
  hint?: string;
}> = [
  { id: "el", label: "OWL EL", enabled: true },
  { id: "rl", label: "OWL RL", enabled: true },
  { id: "rdfs", label: "RDFS", enabled: true },
  {
    id: "dl",
    label: "OWL DL",
    enabled: false,
    hint: "Requires OntoLogos 1.0",
  },
  {
    id: "auto",
    label: "Auto",
    enabled: false,
    hint: "Requires OntoLogos 1.0",
  },
];

export function formatInferenceLine(child: string, parent: string): string {
  return `${shortIri(child)} SubClassOf ${shortIri(parent)}`;
}

export function shortIri(iri: string): string {
  const hash = iri.lastIndexOf("#");
  const slash = iri.lastIndexOf("/");
  const idx = Math.max(hash, slash);
  return idx >= 0 ? iri.slice(idx + 1) : iri;
}

export function summarizeResult(result: RunReasonerResult): string {
  const status = result.consistent ? "Completed" : "Inconsistent";
  return `${status} · ${result.profile_used} · ${result.duration_ms}ms`;
}
