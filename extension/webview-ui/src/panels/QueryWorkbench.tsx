import { useCallback, useEffect, useState } from "react";
import { getVsCodeApi } from "../vscodeApi";
import {
  HostMessage,
  isHostMessage,
  SavedQuery,
  TabularQueryResult,
} from "../messages";

const STARTER_SQL = "SELECT short_name, labels FROM classes";
const STARTER_SPARQL =
  "PREFIX ex: <http://example.org/people#>\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10";

export function QueryWorkbenchPanel(): JSX.Element {
  const [mode, setMode] = useState<"sql" | "sparql">("sql");
  const [text, setText] = useState(STARTER_SQL);
  const [saved, setSaved] = useState<SavedQuery[]>([]);
  const [history, setHistory] = useState<SavedQuery[]>([]);
  const [sqlTables, setSqlTables] = useState<string[]>([]);
  const [result, setResult] = useState<TabularQueryResult | null>(null);
  const [error, setError] = useState("");
  const [runId, setRunId] = useState(0);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "queryWorkbench" });
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "queryInit") {
        setSaved(msg.saved);
        setHistory(msg.history);
        setSqlTables(msg.sqlTables);
      }
      if (msg.type === "queryResult") {
        if (msg.runId !== runId) {
          return;
        }
        setError(msg.error ?? "");
        setResult(msg.result ?? null);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [runId]);

  const run = useCallback(() => {
    const id = runId + 1;
    setRunId(id);
    getVsCodeApi().postMessage({
      type: "runQuery",
      mode,
      text,
      runId: id,
    });
  }, [mode, text, runId]);

  return (
    <div className="panel">
      <h2>Query Workbench</h2>
      <div className="toolbar">
        <label>
          Mode{" "}
          <select
            value={mode}
            onChange={(e) => {
              const m = e.target.value as "sql" | "sparql";
              setMode(m);
              setText(m === "sql" ? STARTER_SQL : STARTER_SPARQL);
            }}
          >
            <option value="sql">SQL</option>
            <option value="sparql">SPARQL</option>
          </select>
        </label>
        {mode === "sql" ? (
          <label>
            Table{" "}
            <select
              onChange={(e) => {
                if (e.target.value) {
                  setText(`SELECT * FROM ${e.target.value}`);
                }
              }}
            >
              <option value="">—</option>
              {sqlTables.map((t) => (
                <option key={t} value={t}>
                  {t}
                </option>
              ))}
            </select>
          </label>
        ) : null}
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
            getVsCodeApi().postMessage({
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
            getVsCodeApi().postMessage({
              type: "exportQueryResult",
              format: "csv",
            })
          }
        >
          Export CSV
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() =>
            getVsCodeApi().postMessage({
              type: "exportQueryResult",
              format: "json",
            })
          }
        >
          Export JSON
        </button>
      </div>
      <textarea
        value={text}
        onChange={(e) => setText(e.target.value)}
        rows={10}
        style={{ width: "100%", fontFamily: "var(--vscode-editor-font-family)" }}
      />
      <div className="toolbar">
        <label>
          Saved{" "}
          <select
            onChange={(e) => {
              const item = saved[Number(e.target.value)];
              if (item) {
                setMode(item.mode);
                setText(item.text);
              }
            }}
          >
            <option value="">—</option>
            {saved.map((s, i) => (
              <option key={s.name} value={i}>
                {s.name} ({s.mode})
              </option>
            ))}
          </select>
        </label>
        <label>
          History{" "}
          <select
            onChange={(e) => {
              const item = history[Number(e.target.value)];
              if (item) {
                setMode(item.mode);
                setText(item.text);
              }
            }}
          >
            <option value="">—</option>
            {history.map((h, i) => (
              <option key={`${h.name}-${i}`} value={i}>
                {h.name}
              </option>
            ))}
          </select>
        </label>
      </div>
      {error ? <p className="error">{error}</p> : null}
      {result ? (
        <div className="results">
          {result.truncated ? (
            <p className="muted">Results truncated at server row limit.</p>
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
                  {result.columns.map((c) => (
                    <td key={c}>{row[c] ?? ""}</td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ) : null}
    </div>
  );
}
