import * as path from "path";
import * as vscode from "vscode";
import { CatalogSnapshot, Entity } from "../lsp/protocol";
import { entityKindLabel } from "../utils/iri";
import {
  activeHierarchy,
  childEntitiesForClass,
  classRootEntities,
  diagnosticLabel,
  entityDisplayLabel,
  filterEntitiesByKind,
  groupDiagnosticsBySeverity,
  hierarchyModeFromConfig,
  isUnsatisfiable,
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
      const uri = vscode.Uri.file(filePath);
      if (vscode.workspace.getWorkspaceFolder(uri)) {
        this.command = {
          command: "vscode.open",
          title: "Open File",
          arguments: [uri],
        };
      }
    }
    if (nodeKind === "diagnostic" && diagnostic) {
      this.tooltip = diagnostic.message;
      this.command = {
        command: "ontocode.openDiagnostic",
        title: "Open Diagnostic",
        arguments: [diagnostic],
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

    const hierarchyMode = hierarchyModeFromConfig(
      vscode.workspace.getConfiguration("ontocode").get<string>("hierarchy.mode")
    );

    if (element) {
      return this.getChildNodes(element, hierarchyMode);
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
          if (doc.format === "turtle") {
            item.contextValue = "ontocodeTurtleDocument";
          }
          return item;
        });
      case "classes":
        return classRootEntities(this.snapshot, hierarchyMode).map((e) =>
          this.entityItem(e, this.hasChildren(e.iri, hierarchyMode), hierarchyMode)
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

  private getChildNodes(
    parent: OntologyTreeItem,
    hierarchyMode: import("../lsp/protocol").HierarchyMode
  ): OntologyTreeItem[] {
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
    return childEntitiesForClass(this.snapshot, parent.iri, hierarchyMode).map((e) =>
      this.entityItem(e, this.hasChildren(e.iri, hierarchyMode), hierarchyMode)
    );
  }

  private hasChildren(
    iri: string,
    hierarchyMode: import("../lsp/protocol").HierarchyMode
  ): boolean {
    if (!this.snapshot) {
      return false;
    }
    const hierarchy = activeHierarchy(this.snapshot, hierarchyMode);
    return (hierarchy.children[iri]?.length ?? 0) > 0;
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
      undefined,
      diagnostic
    );
  }

  private entityItem(
    entity: Entity,
    expandable = false,
    hierarchyMode: import("../lsp/protocol").HierarchyMode = "asserted"
  ): OntologyTreeItem {
    const label = entityDisplayLabel(entity);
    const unsat = this.snapshot && isUnsatisfiable(this.snapshot, entity.iri);
    const displayLabel = unsat ? `${label} (unsat)` : label;
    const item = new OntologyTreeItem(
      "entity",
      displayLabel,
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
