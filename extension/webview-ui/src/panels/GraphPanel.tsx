import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
  type ReactFlowInstance,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import {
  Badge,
  ButtonBar,
  Callout,
  Card,
  CheckboxRow,
  EmptyState,
  InlineCode,
  RangeField,
  Section,
} from "../components/ui";
import { useWorkspaceHost } from "../context/HostContext";
import { postFocusToHost } from "../hooks/useFocusSync";
import { subscribeFocus, useWorkspaceStore } from "../store";
import {
  GraphPayload,
  HostMessage,
  isHostMessage,
} from "../messages";
import type { WorkspaceProps } from "../workspaces/types";

function readInitialParams(): { graphKind: string; rootIri?: string } {
  const params = new URLSearchParams(window.location.search);
  return {
    graphKind: params.get("graphKind") ?? "class",
    rootIri: params.get("root") ?? undefined,
  };
}

export function GraphPanel(_props?: WorkspaceProps): JSX.Element {
  const host = useWorkspaceHost();
  const storeRootIri = useWorkspaceStore((s) => s.graph.rootIri);
  const initial = useMemo(() => readInitialParams(), []);
  const [graph, setGraph] = useState<GraphPayload | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [depth, setDepth] = useState(2);
  const [graphMode, setGraphMode] = useState<"asserted" | "inferred" | "combined">(
    "asserted"
  );
  const [hideDeprecated, setHideDeprecated] = useState(false);
  const [graphKind, setGraphKind] = useState(initial.graphKind);
  const [rootIri, setRootIri] = useState<string | undefined>(
    initial.rootIri ?? storeRootIri ?? undefined
  );
  const [layout, setLayout] = useState<"grid" | "circle" | "stack">("grid");
  const [search, setSearch] = useState("");
  const [error, setError] = useState("");
  const hasGraphData = useRef(false);
  const graphKindRef = useRef(graphKind);
  graphKindRef.current = graphKind;

  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
  const rf = useRef<ReactFlowInstance | null>(null);

  const requestGraph = useCallback(() => {
    host.postToCore({
      type: "requestGraph",
      graphKind,
      rootIri,
      depth,
      includeInferred: graphMode !== "asserted",
      filters: { hide_deprecated: hideDeprecated },
    });
  }, [host, graphKind, rootIri, depth, graphMode, hideDeprecated]);

  // Entity focus updates selection; only neighborhood graphs follow focus as root (#154).
  useEffect(() => {
    return subscribeFocus((focus) => {
      if (focus.kind !== "entity") {
        return;
      }
      setSelectedId(focus.id);
      if (graphKindRef.current === "neighborhood" && focus.id !== rootIri) {
        setRootIri(focus.id);
      }
    });
  }, [rootIri]);

  useEffect(() => {
    if (
      storeRootIri &&
      graphKind === "neighborhood" &&
      storeRootIri !== rootIri
    ) {
      setRootIri(storeRootIri);
    }
  }, [storeRootIri, rootIri, graphKind]);

  useEffect(() => {
    host.postToCore({ type: "ready", panel: "graph" });

    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "focusState" && msg.focus.kind === "entity") {
        setSelectedId(msg.focus.id);
        if (graphKindRef.current === "neighborhood") {
          setRootIri(msg.focus.id);
        }
      }
      if (msg.type === "graphData") {
        hasGraphData.current = true;
        setError("");
        setGraph(msg.graph);
        setGraphKind(msg.graph.graph_kind);
        if (msg.rootIri !== undefined) {
          setRootIri(msg.rootIri);
        }
      } else if (msg.type === "error") {
        setGraph(null);
        setError(msg.message);
      } else if (msg.type === "init" && msg.panel === "graph") {
        if (!hasGraphData.current) {
          requestGraph();
        }
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [requestGraph, host]);

  useEffect(() => {
    if (rootIri) {
      requestGraph();
    }
  }, [rootIri, requestGraph]);

  // Depth / mode / filter changes should refresh (incremental expansion).
  useEffect(() => {
    if (!hasGraphData.current) {
      return;
    }
    requestGraph();
  }, [depth, graphMode, hideDeprecated, requestGraph]);

  useEffect(() => {
    if (!graph) {
      return;
    }
    setNodes(layoutNodes(graph, layout));
    setEdges(toFlowEdges(graph, graphMode));
  }, [graph, graphMode, layout, setNodes, setEdges]);

  const selectedNode = useMemo(
    () => graph?.nodes.find((n) => n.id === selectedId),
    [graph, selectedId]
  );

  const filteredNodeIds = useMemo(() => {
    const q = search.trim().toLowerCase();
    if (!q || !graph) {
      return null;
    }
    const ids = new Set<string>();
    for (const n of graph.nodes) {
      if ((n.label || n.id).toLowerCase().includes(q)) {
        ids.add(n.id);
      }
    }
    return ids;
  }, [search, graph]);

  return (
    <div className="graph-layout">
      <div className="graph-canvas">
        {graph && graph.nodes.length > 0 ? (
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            fitView
            onInit={(instance) => {
              rf.current = instance;
            }}
            onNodeClick={(_, node) => {
              setSelectedId(node.id);
            }}
          >
            <Background />
            <Controls />
            <MiniMap />
          </ReactFlow>
        ) : (
          <EmptyState
            title={error ? "Graph error" : "No graph data"}
            detail={
              error ||
              "Adjust filters or index the workspace, then refresh."
            }
          />
        )}
      </div>
      <aside className="graph-sidebar">
        <Section title="Overview" card>
          <div className="oc-badge-row">
            <Badge variant="kind">{graphKind}</Badge>
            {graph?.truncated ? (
              <Badge variant="warning" title="Graph capped for large ontologies; narrow search or reduce depth">
                Truncated (large ontology)
              </Badge>
            ) : null}
          </div>
        </Section>

        <Section title="Controls" card>
          <label className="oc-field">
            <div className="oc-field-label">Mode</div>
            <select
              aria-label="Mode"
              value={graphMode}
              onChange={(e) => setGraphMode(e.target.value as typeof graphMode)}
            >
              <option value="asserted">Asserted</option>
              <option value="inferred">Inferred only</option>
              <option value="combined">Combined</option>
            </select>
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Layout</div>
            <select
              aria-label="Layout"
              value={layout}
              onChange={(e) => setLayout(e.target.value as typeof layout)}
            >
              <option value="grid">Grid</option>
              <option value="circle">Circle</option>
              <option value="stack">Stack by kind</option>
            </select>
          </label>
          <RangeField
            label="Depth"
            value={depth}
            min={1}
            max={5}
            onChange={setDepth}
          />
          <CheckboxRow
            label="Hide deprecated"
            checked={hideDeprecated}
            onChange={setHideDeprecated}
          />
          <label className="oc-field">
            <div className="oc-field-label">Search</div>
            <input
              aria-label="Search"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Label or IRI…"
            />
          </label>
          <ButtonBar>
            <button type="button" onClick={requestGraph}>
              Refresh graph
            </button>
            <button
              type="button"
              onClick={() => {
                setDepth((d) => Math.min(5, d + 1));
              }}
            >
              Expand
            </button>
            <button
              type="button"
              onClick={async () => {
                if (!graph) {
                  return;
                }
                const text = JSON.stringify(graph, null, 2);
                await navigator.clipboard.writeText(text);
              }}
            >
              Copy JSON
            </button>
            <button
              type="button"
              onClick={() => {
                if (!graph) {
                  return;
                }
                host.postToCore({
                  type: "exportGraph",
                  format: "json",
                  payload: JSON.stringify(graph, null, 2),
                  suggestedName: `ontocode-${graphKind}-graph.json`,
                });
              }}
            >
              Export JSON…
            </button>
            <button
              type="button"
              onClick={() => {
                if (!graph) {
                  return;
                }
                const header = "row_kind,id,label,node_kind,source,target,inferred\n";
                const nodeRows = graph.nodes
                  .map(
                    (n) =>
                      `node,${csvCell(n.id)},${csvCell(n.label ?? "")},${csvCell(n.kind)},,,`
                  )
                  .join("\n");
                const edgeRows = graph.edges
                  .map(
                    (e) =>
                      `edge,,,,${csvCell(e.source)},${csvCell(e.target)},${e.inferred ? "true" : "false"}`
                  )
                  .join("\n");
                host.postToCore({
                  type: "exportGraph",
                  format: "csv",
                  payload: `${header}${nodeRows}\n${edgeRows}\n`,
                  suggestedName: `ontocode-${graphKind}-graph.csv`,
                });
              }}
            >
              Export CSV…
            </button>
            <button
              type="button"
              onClick={() => {
                if (!filteredNodeIds || filteredNodeIds.size === 0) {
                  return;
                }
                const id = [...filteredNodeIds][0];
                setSelectedId(id);
                const node = nodes.find((n) => n.id === id);
                if (node) {
                  rf.current?.fitView({
                    nodes: [node],
                    padding: 0.5,
                    duration: 300,
                  });
                }
              }}
            >
              Center match
            </button>
          </ButtonBar>
        </Section>

        {selectedNode ? (
          <Section title="Selected node" card>
            <Card variant="inset">
              <p>
                <InlineCode>{selectedNode.label}</InlineCode>
              </p>
              <p className="oc-muted">{selectedNode.id}</p>
            </Card>
            <ButtonBar>
              <button
                type="button"
                onClick={() => {
                  postFocusToHost(host, {
                    kind: "entity",
                    id: selectedNode.id,
                    source: "graph",
                  });
                  host.postToCore({ type: "selectNode", iri: selectedNode.id });
                }}
              >
                Inspect entity
              </button>
            </ButtonBar>
          </Section>
        ) : (
          <Callout variant="info">Click a node on the canvas to inspect it.</Callout>
        )}
      </aside>
    </div>
  );
}

function layoutNodes(graph: GraphPayload, layout: "grid" | "circle" | "stack"): Node[] {
  if (layout === "circle") {
    const r = 320;
    const n = Math.max(1, graph.nodes.length);
    return graph.nodes.map((node, i) => {
      const angle = (2 * Math.PI * i) / n;
      return {
        id: node.id,
        position: { x: Math.cos(angle) * r, y: Math.sin(angle) * r },
        data: { label: node.label || node.id },
        className: "oc-graph-node",
      };
    });
  }
  if (layout === "stack") {
    const byKind = new Map<string, number>();
    const colByKind = new Map<string, number>();
    const kinds = Array.from(new Set(graph.nodes.map((n) => n.kind)));
    kinds.forEach((k, i) => colByKind.set(k, i));
    return graph.nodes.map((n) => {
      const row = byKind.get(n.kind) ?? 0;
      byKind.set(n.kind, row + 1);
      const col = colByKind.get(n.kind) ?? 0;
      return {
        id: n.id,
        position: { x: col * 240, y: row * 110 },
        data: { label: n.label || n.id },
        className: "oc-graph-node",
      };
    });
  }
  // Default grid layout.
  const byKind = new Map<string, number>();
  return graph.nodes.map((n, i) => {
    const row = byKind.get(n.kind) ?? 0;
    byKind.set(n.kind, row + 1);
    return {
      id: n.id,
      position: { x: (i % 8) * 180, y: row * 100 + (n.kind === "ontology" ? 0 : 40) },
      data: { label: n.label || n.id },
      className: "oc-graph-node",
    };
  });
}

function toFlowEdges(
  graph: GraphPayload,
  mode: "asserted" | "inferred" | "combined"
): Edge[] {
  return graph.edges
    .filter((e) => {
      if (mode === "combined") {
        return true;
      }
      if (mode === "asserted") {
        return !e.inferred;
      }
      return e.inferred;
    })
    .map((e, i) => ({
      id: `${e.source}-${e.target}-${e.kind}-${i}`,
      source: e.source,
      target: e.target,
      label: e.kind,
      animated: e.inferred,
      className: e.inferred ? "oc-graph-edge oc-graph-edge--inferred" : "oc-graph-edge",
    }));
}

function csvCell(value: string): string {
  if (/[",\n]/.test(value)) {
    return `"${value.replace(/"/g, '""')}"`;
  }
  return value;
}
