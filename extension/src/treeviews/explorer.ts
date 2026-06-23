import * as path from "path";
import * as vscode from "vscode";
import { CatalogSnapshot, Entity } from "../lsp/protocol";
import { entityKindLabel } from "../utils/iri";
import {
  childEntitiesForClass,
  classRootEntities,
  diagnosticLabel,
  entityDisplayLabel,
  filterEntitiesByKind,
  groupDiagnosticsBySeverity,
  propertyGroupsPresent,
} from "./explorerLogic";

export type ExplorerViewKind =
  | "ontologies"
  | "classes"
  | "properties"
  | "individuals"
  | "diagnostics";

export class OntologyTreeItem extends vscode.TreeItem {
  constructor(
    public readonly nodeKind:
      | "document"
      | "entity"
      | "group"
      | "placeholder"
      | "diagnostic",
    label: string,
    collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly iri?: string,
    public readonly filePath?: string,
    public readonly propertyKind?: string,
    public readonly diagnosticSeverity?: string,
    public readonly diagnostic?: import("../lsp/protocol").DiagnosticSummary
  ) {
    super(label, collapsibleState);
    if (nodeKind === "entity" && iri) {
      this.contextValue = "ontocodeEntity";
      this.tooltip = iri;
      this.command = {
        command: "ontocode.openEntity",
        title: "Open Entity Inspector",
        arguments: [iri],
      };
    }
    if (nodeKind === "document" && filePath) {
      this.command = {
        command: "vscode.open",
        title: "Open File",
        arguments: [vscode.Uri.file(filePath)],
      };
    }
    if (nodeKind === "diagnostic" && diagnostic) {
      this.tooltip = diagnostic.message;
      const args: [vscode.Uri, vscode.Position?] = [
        vscode.Uri.file(diagnostic.file),
      ];
      if (diagnostic.line != null) {
        args.push(
          new vscode.Position(
            Math.max(0, diagnostic.line - 1),
            diagnostic.column ?? 0
          )
        );
      }
      this.command = {
        command: "vscode.open",
        title: "Open Diagnostic",
        arguments: args,
      };
      this.iconPath = new vscode.ThemeIcon(
        diagnostic.severity === "error"
          ? "error"
          : diagnostic.severity === "warning"
            ? "warning"
            : "info"
      );
    }
  }
}

export class ExplorerTreeProvider implements vscode.TreeDataProvider<OntologyTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private snapshot: CatalogSnapshot | undefined;

  constructor(private readonly viewKind: ExplorerViewKind) {}

  setSnapshot(snapshot: CatalogSnapshot | undefined): void {
    this.snapshot = snapshot;
    this.refresh();
  }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: OntologyTreeItem): OntologyTreeItem {
    return element;
  }

  getChildren(element?: OntologyTreeItem): OntologyTreeItem[] {
    if (!this.snapshot) {
      return [
        new OntologyTreeItem(
          "placeholder",
          "Index workspace to browse ontologies",
          vscode.TreeItemCollapsibleState.None
        ),
      ];
    }

    if (element) {
      return this.getChildNodes(element);
    }

    switch (this.viewKind) {
      case "ontologies":
        return this.snapshot.documents.map((doc) => {
          const status =
            doc.parse_status === "ok"
              ? "$(check)"
              : doc.parse_status === "warning"
                ? "$(warning)"
                : "$(error)";
          const item = new OntologyTreeItem(
            "document",
            `${status} ${path.basename(doc.path)}`,
            vscode.TreeItemCollapsibleState.None,
            undefined,
            doc.path
          );
          item.description = doc.base_iri;
          return item;
        });
      case "classes":
        return classRootEntities(this.snapshot).map((e) =>
          this.entityItem(e, this.hasChildren(e.iri))
        );
      case "properties":
        return propertyGroupsPresent(this.snapshot).map(({ kind, label }) => {
          const item = new OntologyTreeItem(
            "group",
            label,
            vscode.TreeItemCollapsibleState.Collapsed,
            undefined,
            undefined,
            kind
          );
          item.iconPath = new vscode.ThemeIcon("folder");
          return item;
        });
      case "individuals":
        return filterEntitiesByKind(this.snapshot.entities, "individual").map(
          (e) => this.entityItem(e)
        );
      case "diagnostics": {
        const groups = groupDiagnosticsBySeverity(
          this.snapshot.diagnostics ?? []
        );
        if (groups.size === 0) {
          return [
            new OntologyTreeItem(
              "placeholder",
              "No diagnostics",
              vscode.TreeItemCollapsibleState.None
            ),
          ];
        }
        return [...groups.entries()].map(([severity, items]) => {
          const item = new OntologyTreeItem(
            "group",
            `${severity} (${items.length})`,
            vscode.TreeItemCollapsibleState.Collapsed,
            undefined,
            undefined,
            undefined,
            severity
          );
          item.contextValue = `diagnostics-${severity}`;
          item.iconPath = new vscode.ThemeIcon(
            severity === "error"
              ? "error"
              : severity === "warning"
                ? "warning"
                : "info"
          );
          return item;
        });
      }
      default:
        return [];
    }
  }

  private getChildNodes(parent: OntologyTreeItem): OntologyTreeItem[] {
    if (!this.snapshot) {
      return [];
    }

    if (parent.nodeKind === "group" && parent.propertyKind) {
      return filterEntitiesByKind(this.snapshot.entities, parent.propertyKind).map(
        (e) => this.entityItem(e)
      );
    }
    if (
      parent.nodeKind === "group" &&
      this.viewKind === "diagnostics" &&
      parent.diagnosticSeverity
    ) {
      return (this.snapshot.diagnostics ?? [])
        .filter((d) => d.severity === parent.diagnosticSeverity)
        .map((d) => this.diagnosticItem(d));
    }
    if (parent.nodeKind !== "entity" || !parent.iri || this.viewKind !== "classes") {
      return [];
    }
    return childEntitiesForClass(this.snapshot, parent.iri).map((e) =>
      this.entityItem(e, this.hasChildren(e.iri))
    );
  }

  private hasChildren(iri: string): boolean {
    return (this.snapshot?.hierarchy.children[iri]?.length ?? 0) > 0;
  }

  private diagnosticItem(
    diagnostic: import("../lsp/protocol").DiagnosticSummary
  ): OntologyTreeItem {
    return new OntologyTreeItem(
      "diagnostic",
      diagnosticLabel(diagnostic),
      vscode.TreeItemCollapsibleState.None,
      undefined,
      diagnostic.file,
      undefined,
      diagnostic
    );
  }

  private entityItem(
    entity: Entity,
    expandable = false
  ): OntologyTreeItem {
    const label = entityDisplayLabel(entity);
    const item = new OntologyTreeItem(
      "entity",
      label,
      expandable
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None,
      entity.iri
    );
    item.description = entityKindLabel(entity.kind);
    if (entity.deprecated) {
      item.iconPath = new vscode.ThemeIcon("warning");
    }
    return item;
  }
}
