import { useCallback, useEffect, useRef, useState } from "react";
import {
  Callout,
  CodeEditor,
  FormField,
  IriList,
  Panel,
  PanelHeader,
  Select,
  Toolbar,
  ToolbarGroup,
} from "../components/ui";
import { SchemaBrowser } from "../components/SchemaBrowser";
import { useWorkspaceHost } from "../context/HostContext";
import { useWorkspaceStore } from "../store";
import {
  DlQueryResult,
  HostMessage,
  isHostMessage,
  SavedQuery,
  SqlTableSchema,
} from "../messages";
import type { WorkspaceProps } from "../workspaces/types";

const STARTER_SQL = "SELECT short_name, labels FROM classes";
const STARTER_SPARQL =
  "PREFIX ex: <http://example.org/people#>\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10";
const STARTER_DL = "Person";

type QueryMode = "sql" | "sparql" | "dl";
type DlTab = "instances" | "subclasses" | "superclasses" | "equivalents";
type DlAssertMode = "inferred" | "asserted";

function starterFor(mode: QueryMode): string {
  if (mode === "sql") {
    return STARTER_SQL;
  }
  if (mode === "sparql") {
    return STARTER_SPARQL;
  }
  return STARTER_DL;
}

function dlTabItems(result: DlQueryResult, tab: DlTab): string[] {
  switch (tab) {
    case "instances":
      return result.instances;
    case "subclasses":
      return result.subclasses;
    case "superclasses":
      return result.superclasses;
    case "equivalents":
      return result.equivalents;
  }
}

export function QueryWorkbenchPanel(_props?: WorkspaceProps): JSX.Element {
  const host = useWorkspaceHost();
  const mode = useWorkspaceStore((s) => s.query.language);
  const text = useWorkspaceStore((s) => s.query.text);
  const setQueryLanguage = useWorkspaceStore((s) => s.setQueryLanguage);
  const setQueryText = useWorkspaceStore((s) => s.setQueryText);
  const setQueryResult = useWorkspaceStore((s) => s.setQueryResult);
  const addQueryHistory = useWorkspaceStore((s) => s.addQueryHistory);
  const [saved, setSaved] = useState<SavedQuery[]>([]);
  const [history, setHistory] = useState<SavedQuery[]>([]);
  const [sqlSchema, setSqlSchema] = useState<SqlTableSchema[]>([]);
  const sqlTables = sqlSchema.map((t) => t.name);
  const [error, setError] = useState("");
  const [runId, setRunId] = useState(0);
  const runIdRef = useRef(0);
  const pendingRunsRef = useRef(
    new Map<number, { mode: QueryMode; text: string }>()
  );
  const result = useWorkspaceStore((s) => s.query.lastResult);
  const [dlResult, setDlResult] = useState<DlQueryResult | null>(null);
  const [dlTab, setDlTab] = useState<DlTab>("instances");
  const [dlMode, setDlMode] = useState<DlAssertMode>("inferred");

  useEffect(() => {
    runIdRef.current = runId;
  }, [runId]);

  useEffect(() => {
    if (!useWorkspaceStore.getState().query.text) {
      setQueryText(STARTER_SQL);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps -- init once
  }, []);

  useEffect(() => {
    host.postToCore({ type: "ready", panel: "queryWorkbench" });
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "queryInit") {
        setSaved(msg.saved);
        setHistory(msg.history);
        setSqlSchema(msg.sqlSchema ?? msg.sqlTables.map((name) => ({ name, columns: [] })));
      }
      if (msg.type === "queryResult") {
        if (msg.runId !== runIdRef.current) {
          return;
        }
        setError(msg.error ?? "");
        if (msg.dlResult) {
          setDlResult(msg.dlResult);
          setQueryResult(null);
          const ran = pendingRunsRef.current.get(msg.runId);
          pendingRunsRef.current.delete(msg.runId);
          if (ran) {
            addQueryHistory({ language: ran.mode, text: ran.text });
          }
        } else if (msg.result) {
          setDlResult(null);
          const snapshot = {
            columns: msg.result.columns,
            rows: msg.result.rows.map((row) =>
              msg.result!.columns.map((col) => row[col] ?? "")
            ),
            truncated: msg.result.truncated,
          };
          setQueryResult(snapshot);
          const ran = pendingRunsRef.current.get(msg.runId);
          pendingRunsRef.current.delete(msg.runId);
          if (ran) {
            addQueryHistory({ language: ran.mode, text: ran.text });
          }
        } else {
          setDlResult(null);
          setQueryResult(null);
          pendingRunsRef.current.delete(msg.runId);
        }
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [host, setQueryResult, addQueryHistory]);

  const openEntity = useCallback(
    (iri: string) => {
      host.postToCore({ type: "openEntity", iri });
    },
    [host]
  );

  const run = useCallback(() => {
    const id = runIdRef.current + 1;
    runIdRef.current = id;
    setRunId(id);
    setError("");
    setQueryResult(null);
    setDlResult(null);
    pendingRunsRef.current.set(id, { mode, text });
    host.postToCore({
      type: "runQuery",
      mode,
      text,
      runId: id,
      ...(mode === "dl" ? { dlMode } : {}),
    });
  }, [host, mode, text, dlMode, setQueryResult]);

  const editorLabel =
    mode === "sql"
      ? "SQL query"
      : mode === "sparql"
        ? "SPARQL query"
        : "Manchester class expression";

  return (
    <Panel>
      <PanelHeader
        title="Query Workbench"
        subtitle="Run SQL, SPARQL, or DL Query against the indexed ontology."
      />

      <Toolbar>
        <ToolbarGroup>
          <FormField label="Mode">
            <Select
              value={mode}
              onChange={(e) => {
                const m = e.target.value as QueryMode;
                setQueryLanguage(m);
                setQueryText(starterFor(m));
                setDlResult(null);
                setQueryResult(null);
              }}
            >
              <option value="sql">SQL</option>
              <option value="sparql">SPARQL</option>
              <option value="dl">DL Query</option>
            </Select>
          </FormField>
          {mode === "sql" ? (
            <FormField label="Table">
              <Select
                onChange={(e) => {
                  if (e.target.value) {
                    setQueryText(`SELECT * FROM ${e.target.value}`);
                  }
                }}
              >
                <option value="">—</option>
                {sqlTables.map((t) => (
                  <option key={t} value={t}>
                    {t}
                  </option>
                ))}
              </Select>
            </FormField>
          ) : null}
          {mode === "dl" ? (
            <FormField label="Reasoning">
              <Select
                value={dlMode}
                onChange={(e) => setDlMode(e.target.value as DlAssertMode)}
              >
                <option value="inferred">Inferred</option>
                <option value="asserted">Asserted</option>
              </Select>
            </FormField>
          ) : null}
        </ToolbarGroup>
        <ToolbarGroup>
          <button type="button" onClick={run}>
            Run
          </button>
          <button
            type="button"
            className="secondary"
            onClick={() => {
              const name = window.prompt("Query name");
              if (!name) {
                return;
              }
              host.postToCore({
                type: "saveQuery",
                name,
                mode,
                text,
                ...(mode === "dl" ? { dlMode } : {}),
              });
            }}
          >
            Save
          </button>
          <button
            type="button"
            className="secondary"
            onClick={() =>
              host.postToCore({
                type: "exportQueryResult",
                format: "csv",
                runId: runIdRef.current,
              })
            }
          >
            Export CSV
          </button>
          <button
            type="button"
            className="secondary"
            onClick={() =>
              host.postToCore({
                type: "exportQueryResult",
                format: "json",
                runId: runIdRef.current,
              })
            }
          >
            Export JSON
          </button>
        </ToolbarGroup>
      </Toolbar>

      {mode === "sql" && sqlSchema.length > 0 ? (
        <SchemaBrowser
          schema={sqlSchema}
          onInsert={(snippet) => {
            setQueryText(text ? `${text} ${snippet}` : snippet);
          }}
        />
      ) : null}

      <CodeEditor
        label={editorLabel}
        value={text}
        onChange={(e) => setQueryText(e.target.value)}
        rows={12}
      />

      <Toolbar>
        <ToolbarGroup>
          <FormField label="Saved">
            <Select
              onChange={(e) => {
                const item = saved[Number(e.target.value)];
                if (item) {
                  setQueryLanguage(item.mode);
                  setQueryText(item.text);
                  if (item.mode === "dl") {
                    setDlMode(item.dlMode === "asserted" ? "asserted" : "inferred");
                  }
                }
              }}
            >
              <option value="">—</option>
              {saved.map((s, i) => (
                <option key={s.name} value={i}>
                  {s.name} ({s.mode})
                </option>
              ))}
            </Select>
          </FormField>
          <FormField label="History">
            <Select
              onChange={(e) => {
                const item = history[Number(e.target.value)];
                if (item) {
                  setQueryLanguage(item.mode);
                  setQueryText(item.text);
                  if (item.mode === "dl") {
                    setDlMode(item.dlMode === "asserted" ? "asserted" : "inferred");
                  }
                }
              }}
            >
              <option value="">—</option>
              {history.map((h, i) => (
                <option key={`${h.name}-${i}`} value={i}>
                  {h.name}
                </option>
              ))}
            </Select>
          </FormField>
        </ToolbarGroup>
      </Toolbar>

      {error ? <Callout variant="error">{error}</Callout> : null}

      {dlResult ? (
        <div className="results">
          {(dlResult.warnings?.length ?? 0) > 0 || (dlResult.diagnostics?.length ?? 0) > 0 ? (
            <Callout variant="warning">
              {[...(dlResult.diagnostics ?? []), ...(dlResult.warnings ?? [])].join(" · ")}
            </Callout>
          ) : null}
          <p className="oc-muted oc-results-banner">
            {dlResult.normalized} · {dlResult.mode} · {dlResult.duration_ms} ms
          </p>
          <Toolbar>
            <ToolbarGroup>
              {(
                [
                  ["instances", "Instances"],
                  ["subclasses", "Subclasses"],
                  ["superclasses", "Superclasses"],
                  ["equivalents", "Equivalents"],
                ] as const
              ).map(([id, label]) => (
                <button
                  key={id}
                  type="button"
                  className={dlTab === id ? undefined : "secondary"}
                  onClick={() => setDlTab(id)}
                >
                  {label} ({dlTabItems(dlResult, id).length})
                </button>
              ))}
            </ToolbarGroup>
          </Toolbar>
          <IriList items={dlTabItems(dlResult, dlTab)} onSelect={openEntity} />
        </div>
      ) : null}

      {!dlResult && result ? (
        <div className="results">
          {result.truncated ? (
            <p className="oc-muted oc-results-banner">Results truncated at server row limit.</p>
          ) : null}
          <table>
            <thead>
              <tr>
                {result.columns.map((c) => (
                  <th key={c}>{c}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {result.rows.map((row, ri) => (
                <tr key={ri}>
                  {row.map((cell, ci) => (
                    <td key={`${ri}-${ci}`}>{cell}</td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ) : null}
    </Panel>
  );
}
