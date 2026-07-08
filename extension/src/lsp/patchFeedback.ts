import type { ApplyAxiomPatchClientResult, ApplyPatchResult } from "./protocol";

/** User-facing message when applyAxiomPatch returns applied: false or preview diagnostics. */
export function patchFailureMessage(result: ApplyPatchResult): string {
  const first = result.diagnostics?.find((d) => d.message.trim().length > 0);
  if (first) {
    return first.message;
  }
  return "OntoCode: patch was not applied";
}

export function hasPatchFailureDiagnostics(result: ApplyPatchResult): boolean {
  return (result.diagnostics?.length ?? 0) > 0;
}

/** True when patch was written and the open editor buffer was synced (or no sync was needed). */
export function isPatchFullySynced(result: ApplyAxiomPatchClientResult): boolean {
  return result.applied && result.editor_synced;
}
