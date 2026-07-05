import type { PanelKind } from "./messages";
import { SmokePanel } from "./panels/SmokePanel";
import { EntityInspectorPanel } from "./panels/EntityInspector";
import { GraphPanel } from "./panels/GraphPanel";
import { RefactorPreviewPanel } from "./panels/RefactorPreview";
import { QueryWorkbenchPanel } from "./panels/QueryWorkbench";
import { ManchesterEditorPanel } from "./panels/ManchesterEditor";
import { SemanticDiffPanel } from "./panels/SemanticDiffPanel";
import { ImportsPanel } from "./panels/ImportsPanel";

function panelFromQuery(): PanelKind {
  const params = new URLSearchParams(window.location.search);
  const panel = params.get("panel");
  if (
    panel === "inspector" ||
    panel === "graph" ||
    panel === "smoke" ||
    panel === "refactorPreview" ||
    panel === "queryWorkbench" ||
    panel === "manchesterEditor" ||
    panel === "semanticDiff" ||
    panel === "imports"
  ) {
    return panel;
  }
  return "smoke";
}

export default function App(): JSX.Element {
  const panel = panelFromQuery();
  switch (panel) {
    case "inspector":
      return <EntityInspectorPanel />;
    case "graph":
      return <GraphPanel />;
    case "refactorPreview":
      return <RefactorPreviewPanel />;
    case "queryWorkbench":
      return <QueryWorkbenchPanel />;
    case "manchesterEditor":
      return <ManchesterEditorPanel />;
    case "semanticDiff":
      return <SemanticDiffPanel />;
    case "imports":
      return <ImportsPanel />;
    default:
      return <SmokePanel />;
  }
}
