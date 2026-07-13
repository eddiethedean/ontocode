import type { OntologyDocument } from "../lsp/protocol";

export type OntologyRole = "root" | "import" | "scratch";

export interface OntologyRegistryEntry {
  id: string;
  uri: string;
  path: string;
  format: string;
  role: OntologyRole;
  editable: boolean;
  dirty: boolean;
  version: number;
  active: boolean;
}

export type WorkspaceEventType =
  | "OntologyOpened"
  | "OntologyClosed"
  | "OntologyActivated"
  | "OntologyReloaded"
  | "OntologySaved"
  | "DirtyChanged"
  | "TransactionCommitted"
  | "TransactionUndone"
  | "TransactionRedone"
  | "FocusChanged"
  | "SelectionChanged"
  | "NavigationChanged";

export interface WorkspaceEvent<T = unknown> {
  type: WorkspaceEventType;
  payload?: T;
  timestamp: number;
}

export interface NavigationEntry {
  kind: "entity" | "ontology" | "panel";
  id: string;
  label?: string;
  source: string;
}

export interface WorkspaceSessionSnapshot {
  openOntologyUris: string[];
  activeOntologyId?: string;
  focus?: {
    kind: string;
    id: string;
    source: string;
  };
  navigation: NavigationEntry[];
  navigationIndex: number;
  panelRestore: Record<string, { command: string; args?: unknown[]; title?: string }>;
}

export interface PendingTransaction {
  documentUri: string;
  documentPath: string;
  patches: unknown[];
  label?: string;
}

export interface CommittedTransaction {
  documentUri: string;
  documentPath: string;
  undoPatches: unknown[];
  label?: string;
}

export function isEditableFormat(format: string): boolean {
  const normalized = format.toLowerCase();
  return normalized === "turtle" || normalized === "obo";
}

export function inferRole(
  document: OntologyDocument,
  allDocuments: OntologyDocument[]
): OntologyRole {
  const canonicalPath = document.path.replace(/\\/g, "/");
  const imported = allDocuments.some((other) =>
    other.imports?.some((imp) => {
      const normalized = imp.replace(/\\/g, "/");
      return (
        normalized === canonicalPath ||
        normalized.endsWith(`/${canonicalPath.split("/").pop()}`)
      );
    })
  );
  if (imported) {
    return "import";
  }
  if (document.parse_status === "scratch") {
    return "scratch";
  }
  return "root";
}

export function entryFromDocument(
  document: OntologyDocument,
  allDocuments: OntologyDocument[],
  options: {
    uri: string;
    dirty?: boolean;
    active?: boolean;
    version?: number;
  }
): OntologyRegistryEntry {
  const role = inferRole(document, allDocuments);
  const editable = isEditableFormat(document.format) && role !== "import";
  return {
    id: document.id,
    uri: options.uri,
    path: document.path,
    format: document.format,
    role,
    editable,
    dirty: options.dirty ?? false,
    version: options.version ?? 0,
    active: options.active ?? false,
  };
}
