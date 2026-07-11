import { ExplanationStep } from "../lsp/protocol";

export function formatStepLine(step: ExplanationStep): string {
  return `${step.index}. ${step.display}`;
}

export function stepsAsText(steps: ExplanationStep[]): string {
  return steps.map(formatStepLine).join("\n");
}

/**
 * Prefer an explicit profile, then the last successful classify profile, then settings default.
 */
export function resolveExplanationProfile(options: {
  explicit?: string;
  lastRunProfile?: string | null;
  settingsDefault?: string | null;
}): string {
  const explicit = options.explicit?.trim();
  if (explicit) {
    return explicit;
  }
  const last = options.lastRunProfile?.trim();
  if (last) {
    return last;
  }
  return options.settingsDefault?.trim() || "el";
}

/**
 * Fresh explanation RPC results match the catalog they were generated against.
 * "Stale" only applies when a previously shown fingerprint no longer matches a
 * later catalog fingerprint (e.g. after reindex while the panel stays open).
 */
export function isExplanationStale(options: {
  shownContentHash?: string;
  shownIndexedAt?: number;
  currentContentHash?: string;
  currentIndexedAt?: number;
}): boolean {
  if (options.shownContentHash === undefined || options.currentContentHash === undefined) {
    return false;
  }
  return (
    options.shownContentHash !== options.currentContentHash ||
    options.shownIndexedAt !== options.currentIndexedAt
  );
}
