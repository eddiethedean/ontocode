import { ExplanationStep } from "../lsp/protocol";

export function formatStepLine(step: ExplanationStep): string {
  return `${step.index}. ${step.display}`;
}

export function stepsAsText(steps: ExplanationStep[]): string {
  return steps.map(formatStepLine).join("\n");
}
