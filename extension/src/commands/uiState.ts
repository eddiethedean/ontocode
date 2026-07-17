import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";

const ONTOLOGY_EXTENSIONS = /\.(ttl|owl|rdf|owx|jsonld|json-ld|nt|nq|trig|obo)$/i;

export function isOntologyDocument(document: vscode.TextDocument): boolean {
  return document.uri.scheme === "file" && ONTOLOGY_EXTENSIONS.test(document.uri.fsPath);
}

export function getDirtyOntologyDocumentCount(): number {
  return vscode.workspace.textDocuments.filter(
    (document) => document.isDirty && isOntologyDocument(document)
  ).length;
}

export function getFocusedEntityIri(): string | undefined {
  const focus = focusRelay.getFocus();
  return focus?.kind === "entity" || focus?.kind === "graphNode"
    ? focus.id
    : undefined;
}

export function hasFocusSelection(): boolean {
  return getFocusedEntityIri() !== undefined;
}
