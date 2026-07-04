import { useCallback, useEffect, useState } from "react";
import {
  Badge,
  ButtonBar,
  Callout,
  FormField,
  IriList,
  InlineCode,
  Input,
  kindLabel,
  LoadingState,
  Panel,
  PanelHeader,
  Section,
  Select,
  shortLabel,
  StickyActions,
} from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";
import {
  EntityDetailPayload,
  HostMessage,
  isHostMessage,
  PatchOp,
} from "../messages";

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
      <Panel>
        <LoadingState label="Loading entity…" />
      </Panel>
    );
  }

  const { entity, parents, children, axioms, editable } = detail;

  const axiomsByKind = axioms.reduce<Record<string, typeof axioms>>((acc, ax) => {
    const key = ax.kind || "other";
    (acc[key] ??= []).push(ax);
    return acc;
  }, {});

  const kindTitle = (kind: string): string => {
    switch (kind) {
      case "sub_class_of":
        return "SubClassOf";
      case "equivalent_class":
        return "EquivalentClasses";
      case "disjoint_class":
        return "DisjointClasses";
      case "domain":
        return "Domain";
      case "range":
        return "Range";
      case "property_chain":
        return "Property chains";
      default:
        return kind;
    }
  };
  const parentOptions = classOptions.filter((c) => c !== entity.iri);

  return (
    <Panel>
      <PanelHeader
        title={entity.labels[0] ?? entity.short_name}
        subtitle={
          <>
            {kindLabel(entity.kind)}
            {entity.obo_id ? ` · ${entity.obo_id}` : ""}
            {entity.deprecated ? (
              <span className="deprecated"> · deprecated</span>
            ) : null}
          </>
        }
        badges={
          <>
            <Badge variant="kind">{kindLabel(entity.kind)}</Badge>
            {entity.deprecated ? <Badge variant="danger">Deprecated</Badge> : null}
          </>
        }
      />

      <Section title="IRI" card>
        <InlineCode>{entity.iri}</InlineCode>
      </Section>

      <Section title="Labels" card>
        <IriList items={entity.labels} />
      </Section>

      <Section title="Comments" card>
        <IriList items={entity.comments} />
      </Section>

      <Section title="Parents" card>
        <IriList items={parents} onSelect={openEntity} />
      </Section>

      <Section title="Children" card>
        <IriList items={children} onSelect={openEntity} />
      </Section>

      <Section
        title="Axioms"
        card
        action={
          editable && entity.kind === "class" ? (
            <button
              type="button"
              className="secondary"
              onClick={() =>
                getVsCodeApi().postMessage({ type: "addManchesterAxiom" })
              }
            >
              Add Manchester axiom
            </button>
          ) : undefined
        }
      >
        {axioms.length > 0 ? (
          <div>
            {Object.entries(axiomsByKind).map(([kind, items]) => (
              <div key={kind} className="oc-section oc-section--nested">
                <h3>{kindTitle(kind)}</h3>
                <ul className="oc-axiom-list">
                  {items.map((a, idx) => (
                    <li key={`${kind}-${idx}`} className="oc-axiom-item">
                      <InlineCode>{a.display}</InlineCode>
                      {editable &&
                      entity.kind === "class" &&
                      a.editable &&
                      kind !== "property_chain" ? (
                        <button
                          type="button"
                          className="secondary"
                          onClick={() =>
                            getVsCodeApi().postMessage({
                              type: "openManchester",
                              axiom: {
                                kind: a.kind,
                                manchester: a.manchester,
                                other_iri: a.other_iri,
                              },
                            })
                          }
                        >
                          {kind === "disjoint_class"
                            ? "Edit disjoint"
                            : "Edit in Manchester"}
                        </button>
                      ) : null}
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        ) : (
          <p className="oc-muted">None</p>
        )}
      </Section>

      {editable ? (
        <Section title="Edit" card>
            <FormField label="Add label">
              <Input
                value={newLabel}
                onChange={(e) => setNewLabel(e.target.value)}
              />
            </FormField>
            <ButtonBar>
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
            </ButtonBar>

            <FormField label="Add comment">
              <Input
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
              />
            </FormField>
            <ButtonBar>
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
            </ButtonBar>

            <FormField label="Add parent">
              <Select
                value={parentPick}
                onChange={(e) => setParentPick(e.target.value)}
              >
                <option value="">—</option>
                {parentOptions.map((c) => (
                  <option key={c} value={c}>
                    {shortLabel(c)}
                  </option>
                ))}
              </Select>
            </FormField>
            <ButtonBar>
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
            </ButtonBar>

            {preview ? (
              preview.startsWith("Error:") ? (
                <Callout variant="error">{preview}</Callout>
              ) : (
                <pre className="preview oc-muted">{preview}</pre>
              )
            ) : null}
        </Section>
      ) : (
        <Callout variant="info">
          Editing is available for Turtle (.ttl) documents only.
        </Callout>
      )}

      <StickyActions>
        <button
          type="button"
          onClick={() => getVsCodeApi().postMessage({ type: "jumpToSource" })}
        >
          Jump to Source
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() => getVsCodeApi().postMessage({ type: "findUsages" })}
        >
          Find Usages
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() => getVsCodeApi().postMessage({ type: "renameIri" })}
        >
          Rename IRI
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
      </StickyActions>
    </Panel>
  );
}
