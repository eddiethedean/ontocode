export type { WorkspaceDefinition, WorkspaceProps } from "./types";
export {
  getWorkspace,
  getWorkspaceByPanelKind,
  listWorkspaces,
  registerWorkspace,
  resetWorkspaceRegistryForTests,
} from "./registry";
