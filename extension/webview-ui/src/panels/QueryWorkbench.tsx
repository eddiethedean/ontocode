import { useCallback, useEffect, useRef, useState } from "react";
import {
  Callout,
  CodeEditor,
  FormField,
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
  HostMessage,
  isHostMessage,
  SavedQuery,
  SqlTableSchema,
} from "../messages";
import type { WorkspaceProps } from "../workspaces/types";

const STARTER_SQL = "SELECT short_name, labels FROM classes";
const STARTER_SPARQL =
  "PREFIX ex: <http://example.org/people#>\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10";

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
    new Map<number, { mode: "sql" | "sparql"; text: string }>()
  );
  const result = useWorkspaceStore((s) => s.query.lastResult);

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
        if (msg.result) {
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
          setQueryResult(null);
          pendingRunsRef.current.delete(msg.runId);
        }
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [host, setQueryResult, addQueryHistory]);

  const run = useCallback(() => {
    const id = runIdRef.current + 1;
    runIdRef.current = id;
    setRunId(id);
    setError("");
    setQueryResult(null);
    pendingRunsRef.current.set(id, { mode, text });
    host.postToCore({
      type: "runQuery",
      mode,
      text,
      runId: id,
    });
  }, [host, mode, text, setQueryResult]);

  return (
    <Panel>
      <PanelHeader
        title="Query Workbench"
        subtitle="Run SQL or SPARQL against the indexed ontology catalog."
      />

      <Toolbar>
        <ToolbarGroup>
          <FormField label="Mode">
            <Select
              value={mode}
              onChange={(e) => {
                const m = e.target.value as "sql" | "sparql";
                setQueryLanguage(m);
                setQueryText(m === "sql" ? STARTER_SQL : STARTER_SPARQL);
              }}
            >
              <option value="sql">SQL</option>
              <option value="sparql">SPARQL</option>
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
        label={mode === "sql" ? "SQL query" : "SPARQL query"}
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

      {result ? (
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
