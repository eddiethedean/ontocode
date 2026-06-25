import { useCallback, useEffect, useState } from "react";
import { getVsCodeApi } from "../vscodeApi";
import {
  EntityDetailPayload,
  HostMessage,
  isHostMessage,
  PatchOp,
} from "../messages";

function shortLabel(iri: string): string {
  const hash = iri.lastIndexOf("#");
  const slash = iri.lastIndexOf("/");
  const idx = Math.max(hash, slash);
  return idx >= 0 ? iri.slice(idx + 1) : iri;
}

function kindLabel(kind: string): string {
  return kind.replace(/_/g, " ");
}

function EntityList({
  items,
  onSelect,
}: {
  items: string[];
  onSelect?: (iri: string) => void;
}): JSX.Element {
  if (items.length === 0) {
    return <p className="muted">None</p>;
  }
  return (
    <ul>
      {items.map((i) => (
        <li key={i}>
          {onSelect ? (
            <button
              type="button"
              className="secondary"
              style={{ padding: "2px 6px", marginRight: 6 }}
              onClick={() => onSelect(i)}
            >
              <code>{shortLabel(i)}</code>
            </button>
          ) : (
            <code>{shortLabel(i)}</code>
          )}{" "}
          <span className="muted">{i}</span>
        </li>
      ))}
    </ul>
  );
}

export function EntityInspectorPanel(): JSX.Element {
  const [detail, setDetail] = useState<EntityDetailPayload | null>(null);
  const [classOptions, setClassOptions] = useState<string[]>([]);
  const [preview, setPreview] = useState("");
  const [newLabel, setNewLabel] = useState("");
  const [newComment, setNewComment] = useState("");
  const [parentPick, setParentPick] = useState("");

  const openEntity = useCallback((iri: string) => {
    getVsCodeApi().postMessage({ type: "openEntity", iri });
  }, []);

  const apply = useCallback((patches: PatchOp[], previewOnly: boolean) => {
    getVsCodeApi().postMessage({ type: "applyPatch", patches, previewOnly });
  }, []);

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "inspector" });

    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "loadEntity") {
        setDetail(msg.detail);
        setClassOptions(msg.classOptions);
        setPreview("");
      } else if (msg.type === "preview") {
        setPreview(msg.text);
      } else if (msg.type === "error") {
        setPreview(`Error: ${msg.message}`);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  if (!detail) {
    return (
      <div style={{ padding: 16 }}>
        <p className="muted">Loading entity…</p>
      </div>
    );
  }

  const { entity, parents, children, axioms, editable } = detail;
  const parentOptions = classOptions.filter((c) => c !== entity.iri);

  return (
    <div style={{ padding: 16 }}>
      <h1>{entity.labels[0] ?? entity.short_name}</h1>
      <p className="muted">
        {kindLabel(entity.kind)}
        {entity.obo_id ? ` · ${entity.obo_id}` : ""}
        {entity.deprecated ? (
          <span className="deprecated"> (deprecated)</span>
        ) : null}
      </p>

      <h2>IRI</h2>
      <p style={{ wordBreak: "break-all" }}>
        <code>{entity.iri}</code>
      </p>

      <h2>Labels</h2>
      <EntityList items={entity.labels} />

      <h2>Comments</h2>
      <EntityList items={entity.comments} />

      <h2>Parents</h2>
      <EntityList items={parents} onSelect={openEntity} />

      <h2>Children</h2>
      <EntityList items={children} onSelect={openEntity} />

      <h2>Axioms</h2>
      {axioms.length > 0 ? (
        <ul>
          {axioms.map((a, idx) => (
            <li key={idx}>
              <code>{a.display}</code>{" "}
              {editable && entity.kind === "class" ? (
                <button
                  type="button"
                  className="secondary"
                  onClick={() =>
                    getVsCodeApi().postMessage({
                      type: "openManchester",
                      axiom: { kind: a.kind, manchester: a.manchester },
                    })
                  }
                >
                  Edit in Manchester
                </button>
              ) : null}
            </li>
          ))}
        </ul>
      ) : (
        <p className="muted">None</p>
      )}
      {editable && entity.kind === "class" ? (
        <button
          type="button"
          className="secondary"
          onClick={() =>
            getVsCodeApi().postMessage({ type: "addManchesterAxiom" })
          }
        >
          Add Manchester axiom
        </button>
      ) : null}

      {editable ? (
        <>
          <h2>Edit</h2>
          <div className="form">
            <label>
              Add label
              <input
                value={newLabel}
                onChange={(e) => setNewLabel(e.target.value)}
              />
            </label>
            <button
              type="button"
              onClick={() => {
                if (!newLabel.trim()) return;
                apply(
                  [{ op: "add_label", entity_iri: entity.iri, value: newLabel.trim() }],
                  false
                );
                setNewLabel("");
              }}
            >
              Add Label
            </button>
            <label>
              Add comment
              <input
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
              />
            </label>
            <button
              type="button"
              onClick={() => {
                if (!newComment.trim()) return;
                apply(
                  [
                    {
                      op: "add_comment",
                      entity_iri: entity.iri,
                      value: newComment.trim(),
                    },
                  ],
                  false
                );
                setNewComment("");
              }}
            >
              Add Comment
            </button>
            <label>
              Add parent
              <select
                value={parentPick}
                onChange={(e) => setParentPick(e.target.value)}
              >
                <option value="">—</option>
                {parentOptions.map((c) => (
                  <option key={c} value={c}>
                    {shortLabel(c)}
                  </option>
                ))}
              </select>
            </label>
            <button
              type="button"
              onClick={() => {
                if (!parentPick) return;
                apply(
                  [
                    {
                      op: "add_sub_class_of",
                      entity_iri: entity.iri,
                      parent_iri: parentPick,
                    },
                  ],
                  false
                );
                setParentPick("");
              }}
            >
              Add Parent (SubClassOf)
            </button>
            <button
              type="button"
              className="secondary"
              onClick={() => {
                if (!newLabel.trim()) return;
                apply(
                  [{ op: "add_label", entity_iri: entity.iri, value: newLabel.trim() }],
                  true
                );
              }}
            >
              Preview
            </button>
            <button
              type="button"
              className="danger"
              onClick={() => {
                if (
                  window.confirm(
                    "Delete this entity from the ontology file?"
                  )
                ) {
                  apply(
                    [{ op: "delete_entity", entity_iri: entity.iri }],
                    false
                  );
                }
              }}
            >
              Delete Entity
            </button>
            {preview ? (
              <pre className="preview muted">{preview}</pre>
            ) : null}
          </div>
        </>
      ) : (
        <p className="muted">
          Editing is available for Turtle (.ttl) documents only.
        </p>
      )}

      <button
        type="button"
        onClick={() => getVsCodeApi().postMessage({ type: "jumpToSource" })}
      >
        Jump to Source
      </button>
      <button
        type="button"
        className="secondary"
        onClick={() =>
          getVsCodeApi().postMessage({
            type: "openGraph",
            rootIri: entity.iri,
          })
        }
      >
        Open Graph
      </button>
    </div>
  );
}
