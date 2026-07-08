import type { PanelKind } from "./messages";
import { FocusSyncBootstrap } from "./hooks/useFocusSync";
import { HostProvider } from "./context/HostContext";
import { getWorkspaceByPanelKind } from "./workspaces";

import { registerBuiltinProviders } from "./capabilities/builtin";

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

function WorkspaceRoot(): JSX.Element {
  const panel = panelFromQuery();
  const workspace = getWorkspaceByPanelKind(panel);
  if (!workspace) {
    const Smoke = getWorkspaceByPanelKind("smoke")!.component;
    return <Smoke workspaceId="smoke" />;
  }
  const Component = workspace.component;
  return <Component workspaceId={workspace.id} />;
}

export default function App(): JSX.Element {
  registerBuiltinProviders();
  return (
    <HostProvider>
      <FocusSyncBootstrap />
      <WorkspaceRoot />
    </HostProvider>
  );
}
