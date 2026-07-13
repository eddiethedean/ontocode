/** 0-based document line index for a 1-based usage line from the LSP. */
export function usageJumpLineIndex(usageLine: number | undefined): number {
  return Math.max(0, (usageLine ?? 1) - 1);
}

/** Whether a usage jump line is within the open document. */
export function isUsageJumpLineInDocument(line: number, lineCount: number): boolean {
  return line >= 0 && line < lineCount;
}
