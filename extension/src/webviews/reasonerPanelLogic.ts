import { RunReasonerResult } from "../lsp/protocol";
import type { ReasoningStatePayload } from "../focus/types";

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
  { id: "dl", label: "OWL DL", enabled: true },
  { id: "auto", label: "Auto", enabled: true },
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

/** Snapshot relay state before a reasoner run (#221). */
export function captureReasoningPreRun(
  current: ReasoningStatePayload | null
): ReasoningStatePayload {
  return {
    profile: current?.profile ?? "el",
    unsatisfiable: current?.unsatisfiable ?? [],
    lastRunAt: current?.lastRunAt ?? 0,
    dirty: current?.dirty ?? true,
    running: false,
    hierarchyMode: current?.hierarchyMode,
  };
}

export function reasoningStateForRunStart(
  profile: string,
  preRun: ReasoningStatePayload
): ReasoningStatePayload {
  return {
    ...preRun,
    profile,
    running: true,
  };
}

export function reasoningStateForRunSuccess(
  profile: string,
  unsatisfiable: string[],
  preRun: ReasoningStatePayload
): ReasoningStatePayload {
  return {
    profile,
    unsatisfiable,
    lastRunAt: Date.now(),
    dirty: false,
    running: false,
    hierarchyMode: preRun.hierarchyMode,
  };
}

export function reasoningStateForRunCancel(
  preRun: ReasoningStatePayload
): ReasoningStatePayload {
  return { ...preRun, running: false };
}

export function reasoningStateForRunError(
  profile: string,
  preRun: ReasoningStatePayload,
  lastResultUnsat: string[] | undefined
): ReasoningStatePayload {
  return {
    profile,
    unsatisfiable: lastResultUnsat ?? preRun.unsatisfiable,
    lastRunAt: preRun.lastRunAt,
    dirty: true,
    running: false,
    hierarchyMode: preRun.hierarchyMode,
  };
}
