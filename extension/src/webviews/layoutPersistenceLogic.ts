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

export function resolvePanelRestoreState(
  saved: Record<string, PanelRestoreState> | undefined,
  viewType: string
): PanelRestoreState | undefined {
  return saved?.[viewType] ?? DEFAULT_REOPEN[viewType];
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
