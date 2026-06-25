import { useCallback, useEffect, useMemo, useState } from "react";
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { getVsCodeApi } from "../vscodeApi";
import {
  GraphPayload,
  HostMessage,
  isHostMessage,
} from "../messages";

function layoutNodes(graph: GraphPayload): Node[] {
  const byKind = new Map<string, number>();
  return graph.nodes.map((n, i) => {
    const row = byKind.get(n.kind) ?? 0;
    byKind.set(n.kind, row + 1);
    return {
      id: n.id,
      position: { x: (i % 8) * 180, y: row * 100 + (n.kind === "ontology" ? 0 : 40) },
      data: { label: n.label || n.id },
      style: {
        background: "var(--vscode-editor-background)",
        color: "var(--vscode-foreground)",
        border: "1px solid var(--vscode-panel-border)",
        fontSize: 12,
        padding: 8,
        maxWidth: 160,
      },
    };
  });
}

function toFlowEdges(graph: GraphPayload, showInferred: boolean): Edge[] {
  return graph.edges
    .filter((e) => showInferred || !e.inferred)
    .map((e, i) => ({
      id: `${e.source}-${e.target}-${e.kind}-${i}`,
      source: e.source,
      target: e.target,
      label: e.kind,
      animated: e.inferred,
      style: e.inferred
        ? { stroke: "var(--vscode-charts-orange)" }
        : undefined,
    }));
}

export function GraphPanel(): JSX.Element {
  const [graph, setGraph] = useState<GraphPayload | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [depth, setDepth] = useState(2);
  const [includeInferred, setIncludeInferred] = useState(false);
  const [showInferred, setShowInferred] = useState(true);
  const [hideDeprecated, setHideDeprecated] = useState(false);
  const [graphKind, setGraphKind] = useState("class");
  const [rootIri, setRootIri] = useState<string | undefined>();

  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);

  const requestGraph = useCallback(() => {
    getVsCodeApi().postMessage({
      type: "requestGraph",
      graphKind,
      rootIri,
      depth,
      includeInferred,
      filters: { hide_deprecated: hideDeprecated },
    });
  }, [graphKind, rootIri, depth, includeInferred, hideDeprecated]);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "graph" });

    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "graphData") {
        setGraph(msg.graph);
        setGraphKind(msg.graph.graph_kind);
      } else if (msg.type === "init" && msg.panel === "graph") {
        requestGraph();
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [requestGraph]);

  useEffect(() => {
    if (!graph) {
      return;
    }
    setNodes(layoutNodes(graph));
    setEdges(toFlowEdges(graph, showInferred));
  }, [graph, showInferred, setNodes, setEdges]);

  const selectedNode = useMemo(
    () => graph?.nodes.find((n) => n.id === selectedId),
    [graph, selectedId]
  );

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
            onNodeClick={(_, node) => {
              setSelectedId(node.id);
              getVsCodeApi().postMessage({ type: "selectNode", iri: node.id });
            }}
          >
            <Background />
            <Controls />
            <MiniMap />
          </ReactFlow>
        ) : (
          <div style={{ padding: 16 }}>
            <p className="muted">No graph data. Adjust filters or index the workspace.</p>
          </div>
        )}
      </div>
      <aside className="graph-sidebar">
        <h2>Graph</h2>
        <p className="muted">Kind: {graphKind}</p>
        {graph?.truncated ? (
          <p className="muted">Graph truncated (size limit).</p>
        ) : null}
        <label>
          Depth
          <input
            type="range"
            min={1}
            max={5}
            value={depth}
            onChange={(e) => setDepth(Number(e.target.value))}
          />
          {depth}
        </label>
        <label>
          <input
            type="checkbox"
            checked={includeInferred}
            onChange={(e) => setIncludeInferred(e.target.checked)}
          />{" "}
          Include inferred (reasoner)
        </label>
        <label>
          <input
            type="checkbox"
            checked={showInferred}
            onChange={(e) => setShowInferred(e.target.checked)}
          />{" "}
          Show inferred edges
        </label>
        <label>
          <input
            type="checkbox"
            checked={hideDeprecated}
            onChange={(e) => setHideDeprecated(e.target.checked)}
          />{" "}
          Hide deprecated
        </label>
        <button type="button" onClick={requestGraph}>
          Refresh
        </button>
        {selectedNode ? (
          <>
            <h2>Selected</h2>
            <p>
              <code>{selectedNode.label}</code>
            </p>
            <p className="muted">{selectedNode.id}</p>
            <button
              type="button"
              onClick={() =>
                getVsCodeApi().postMessage({
                  type: "selectNode",
                  iri: selectedNode.id,
                })
              }
            >
              Inspect
            </button>
          </>
        ) : null}
      </aside>
    </div>
  );
}
