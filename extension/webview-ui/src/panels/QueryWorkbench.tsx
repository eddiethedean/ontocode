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
  const runIdRef = useRef(0);

  useEffect(() => {
    runIdRef.current = runId;
  }, [runId]);

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
        if (msg.runId !== runIdRef.current) {
          return;
        }
        setError(msg.error ?? "");
        setResult(msg.result ?? null);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  const run = useCallback(() => {
    const id = runIdRef.current + 1;
    runIdRef.current = id;
    setRunId(id);
    setError("");
    setResult(null);
    getVsCodeApi().postMessage({
      type: "runQuery",
      mode,
      text,
      runId: id,
    });
  }, [mode, text]);

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
                setMode(m);
                setText(m === "sql" ? STARTER_SQL : STARTER_SPARQL);
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
              getVsCodeApi().postMessage({
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

      <CodeEditor
        label={mode === "sql" ? "SQL query" : "SPARQL query"}
        value={text}
        onChange={(e) => setText(e.target.value)}
        rows={12}
      />

      <Toolbar>
        <ToolbarGroup>
          <FormField label="Saved">
            <Select
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
            </Select>
          </FormField>
          <FormField label="History">
            <Select
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
                  {result.columns.map((c) => (
                    <td key={c}>{row[c] ?? ""}</td>
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
