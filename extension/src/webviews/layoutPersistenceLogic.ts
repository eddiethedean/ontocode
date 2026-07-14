export interface Perspective {
  name: string;
  panels: string[];
}

export interface PanelRestoreState {
  command: string;
  args?: unknown[];
  title?: string;
}

export const PERSPECTIVES: readonly Perspective[] = [
  { name: "Modeling", panels: ["inspector", "query"] },
  { name: "Reasoning", panels: ["reasoner", "explanation", "graph"] },
  { name: "Review", panels: ["semanticDiff", "imports"] },
];

export const DEFAULT_REOPEN: Record<string, PanelRestoreState> = {
  ontocodeInspector: { command: "ontocode.showEntityInspector" },
  ontocodeGraph: { command: "ontocode.openClassGraph" },
  ontocodeQueryWorkbench: { command: "ontocode.openQueryWorkbench" },
  ontocodeImports: { command: "ontocode.manageImports" },
  ontocodeReasoner: { command: "ontocode.runReasoner" },
  ontocodeRefactorPreview: { command: "ontocode.findUsages" },
  ontocodeExplanation: { command: "ontocode.showExplanation" },
  ontocodeSemanticDiff: { command: "ontocode.semanticDiff" },
  ontocodeManchesterEditor: { command: "ontocode.openManchesterEditor" },
};

/** Commands permitted for panel reopen from session / layout restore state. */
export const ALLOWED_PANEL_RESTORE_COMMANDS: ReadonlySet<string> = new Set([
  ...Object.values(DEFAULT_REOPEN).map((state) => state.command),
  "ontocode.openEntity",
  "ontocode.openClassGraph",
  "ontocode.openPropertyGraph",
  "ontocode.openImportGraph",
  "ontocode.openNeighborhoodGraph",
]);

/**
 * Session/layout restore must never execute arbitrary VS Code commands from
 * workspaceState or `.ontocode/session.json` (see #309).
 */
export function isAllowedPanelRestoreCommand(command: string): boolean {
  if (typeof command !== "string" || command.length === 0) {
    return false;
  }
  if (!command.startsWith("ontocode.")) {
    return false;
  }
  // Reject unexpected characters that should never appear in our command IDs.
  if (!/^ontocode\.[A-Za-z0-9.]+$/.test(command)) {
    return false;
  }
  return ALLOWED_PANEL_RESTORE_COMMANDS.has(command);
}

/** Return state only when its reopen command is allowlisted. */
export function sanitizePanelRestoreState(
  state: PanelRestoreState | undefined
): PanelRestoreState | undefined {
  if (!state?.command || !isAllowedPanelRestoreCommand(state.command)) {
    return undefined;
  }
  return state;
}

export function resolvePanelRestoreState(
  saved: Record<string, PanelRestoreState> | undefined,
  viewType: string
): PanelRestoreState | undefined {
  const savedState = sanitizePanelRestoreState(saved?.[viewType]);
  if (savedState) {
    return savedState;
  }
  return DEFAULT_REOPEN[viewType];
}

export interface GraphRestoreOptions {
  graphKind: string;
  rootIri?: string;
}

/** Map active graph mode to the layout-restore command + args. */
export function graphRestoreState(
  options: GraphRestoreOptions,
  title?: string
): PanelRestoreState {
  switch (options.graphKind) {
    case "class":
      return { command: "ontocode.openClassGraph", title };
    case "property":
      return { command: "ontocode.openPropertyGraph", title };
    case "import":
      return { command: "ontocode.openImportGraph", title };
    case "neighborhood":
    default:
      return {
        command: "ontocode.openNeighborhoodGraph",
        args: options.rootIri ? [options.rootIri] : undefined,
        title,
      };
  }
}
