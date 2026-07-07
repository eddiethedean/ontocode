import { useState } from "react";
import type { SqlTableSchema } from "../messages";
import { useWorkspaceStore } from "../store";

export function SchemaBrowser({
  schema,
  onInsert,
}: {
  schema: SqlTableSchema[];
  onInsert: (snippet: string) => void;
}): JSX.Element {
  const expanded = useWorkspaceStore((s) => s.query.schemaBrowserExpanded);
  const setExpanded = useWorkspaceStore((s) => s.setSchemaBrowserExpanded);
  const [openTable, setOpenTable] = useState<string | null>(null);

  if (schema.length === 0) {
    return null;
  }

  return (
    <aside className="oc-schema-browser" aria-label="SQL schema browser">
      <button
        type="button"
        className="oc-schema-browser-toggle"
        aria-expanded={expanded}
        aria-label={expanded ? "Collapse schema browser" : "Expand schema browser"}
        onClick={() => setExpanded(!expanded)}
      >
        Schema {expanded ? "▾" : "▸"}
      </button>
      {expanded ? (
        <ul className="oc-schema-table-list">
          {schema.map((table) => (
            <li key={table.name}>
              <button
                type="button"
                className="oc-schema-table-btn"
                aria-expanded={openTable === table.name}
                aria-label={`Table ${table.name}`}
                onClick={() =>
                  setOpenTable(openTable === table.name ? null : table.name)
                }
              >
                {table.name}
              </button>
              {openTable === table.name ? (
                <ul className="oc-schema-column-list">
                  {table.columns.map((col) => (
                    <li key={col.name}>
                      <button
                        type="button"
                        className="oc-schema-column-btn"
                        onClick={() => onInsert(col.name)}
                        title={`Insert column ${col.name}`}
                        aria-label={`Insert column ${col.name}`}
                      >
                        {col.name}
                      </button>
                    </li>
                  ))}
                  <li>
                    <button
                      type="button"
                      className="oc-schema-insert-table"
                      onClick={() => onInsert(`SELECT * FROM ${table.name}`)}
                    >
                      Insert table query
                    </button>
                  </li>
                </ul>
              ) : null}
            </li>
          ))}
        </ul>
      ) : null}
    </aside>
  );
}
