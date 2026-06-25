import type { PanelKind } from "./messages";
import { SmokePanel } from "./panels/SmokePanel";
import { EntityInspectorPanel } from "./panels/EntityInspector";
import { GraphPanel } from "./panels/GraphPanel";

function panelFromQuery(): PanelKind {
  const params = new URLSearchParams(window.location.search);
  const panel = params.get("panel");
  if (panel === "inspector" || panel === "graph" || panel === "smoke") {
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
    default:
      return <SmokePanel />;
  }
}
