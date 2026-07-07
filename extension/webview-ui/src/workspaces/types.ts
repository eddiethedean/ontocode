import type { ComponentType } from "react";
import type { PanelKind } from "../messages";

export interface WorkspaceProps {
  /** Workspace id from registry. */
  workspaceId: string;
}

export interface WorkspaceDefinition {
  id: string;
  title: string;
  /** Legacy ?panel= query param (backward compatible). */
  panelKind: PanelKind;
  component: ComponentType<WorkspaceProps>;
}
