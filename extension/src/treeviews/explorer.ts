import * as path from "path";
import * as vscode from "vscode";
import { CatalogSnapshot, Entity } from "../lsp/protocol";
import { entityKindLabel, shortLabel } from "../utils/iri";

export type ExplorerViewKind =
  | "ontologies"
  | "classes"
  | "properties"
  | "individuals"
  | "diagnostics";

export class OntologyTreeItem extends vscode.TreeItem {
  constructor(
    public readonly nodeKind: "document" | "entity" | "group" | "placeholder",
    label: string,
    collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly iri?: string,
    public readonly filePath?: string,
    public readonly propertyKind?: string
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
        return this.buildClassRoots();
      case "properties":
        return this.buildPropertyGroups();
      case "individuals":
        return this.filterEntities("individual").map((e) => this.entityItem(e));
      case "diagnostics":
        return [
          new OntologyTreeItem(
            "placeholder",
            "Diagnostics available in v0.3",
            vscode.TreeItemCollapsibleState.None
          ),
        ];
      default:
        return [];
    }
  }

  private getChildNodes(parent: OntologyTreeItem): OntologyTreeItem[] {
    if (parent.nodeKind === "group" && parent.propertyKind) {
      return this.filterEntities(parent.propertyKind).map((e) =>
        this.entityItem(e)
      );
    }
    if (parent.nodeKind !== "entity" || !parent.iri || this.viewKind !== "classes") {
      return [];
    }
    const children = this.snapshot?.hierarchy.children[parent.iri] ?? [];
    return children
      .map((iri) => this.snapshot?.entities.find((e) => e.iri === iri))
      .filter((e): e is Entity => !!e)
      .map((e) => this.entityItem(e, this.hasChildren(e.iri)));
  }

  private buildClassRoots(): OntologyTreeItem[] {
    const classes = this.filterEntities("class");
    const childSet = new Set(
      this.snapshot?.hierarchy.edges.map((e) => e.child) ?? []
    );
    const roots = classes.filter((c) => !childSet.has(c.iri));
    return roots.map((e) => this.entityItem(e, this.hasChildren(e.iri)));
  }

  private buildPropertyGroups(): OntologyTreeItem[] {
    const groups: Array<{ kind: string; label: string }> = [
      { kind: "object_property", label: "Object Properties" },
      { kind: "data_property", label: "Data Properties" },
      { kind: "annotation_property", label: "Annotation Properties" },
    ];

    return groups
      .filter(({ kind }) => this.filterEntities(kind).length > 0)
      .map(({ kind, label }) => {
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
  }

  private filterEntities(kind: string): Entity[] {
    return (this.snapshot?.entities ?? []).filter((e) => e.kind === kind);
  }

  private hasChildren(iri: string): boolean {
    return (this.snapshot?.hierarchy.children[iri]?.length ?? 0) > 0;
  }

  private entityItem(
    entity: Entity,
    expandable = false
  ): OntologyTreeItem {
    const label =
      entity.labels[0] ?? entity.short_name ?? shortLabel(entity.iri);
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
