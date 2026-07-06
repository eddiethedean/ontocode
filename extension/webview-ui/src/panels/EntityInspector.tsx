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
import { PreviewApplyBar } from "../components/PreviewApplyBar";
import { PropertyChainEditor } from "./PropertyChainEditor";
import { getVsCodeApi } from "../vscodeApi";
import {
  EntityDetailPayload,
  HostMessage,
  isHostMessage,
  PatchOp,
  PropertyCharacteristics,
} from "../messages";

export function EntityInspectorPanel(): JSX.Element {
  const [detail, setDetail] = useState<EntityDetailPayload | null>(null);
  const [classOptions, setClassOptions] = useState<string[]>([]);
  const [preview, setPreview] = useState("");
  const [newLabel, setNewLabel] = useState("");
  const [newComment, setNewComment] = useState("");
  const [parentPick, setParentPick] = useState("");
  const [domainPick, setDomainPick] = useState("");
  const [rangePick, setRangePick] = useState("");
  const [annotationPredicate, setAnnotationPredicate] = useState("");
  const [annotationValue, setAnnotationValue] = useState("");
  const [oboName, setOboName] = useState("");
  const [oboSynonym, setOboSynonym] = useState("");
  const [oboDef, setOboDef] = useState("");
  const [oboParent, setOboParent] = useState("");
  const [editPreview, setEditPreview] = useState("");

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
        setEditPreview("");
      } else if (msg.type === "preview") {
        setPreview(msg.text);
        setEditPreview(msg.text);
      } else if (msg.type === "error") {
        setPreview(`Error: ${msg.message}`);
        setEditPreview(`Error: ${msg.message}`);
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

  const { entity, parents, children, axioms, editable, document_path, annotations = [], characteristics } = detail;

  const isObo = document_path?.endsWith(".obo") ?? false;
  const isTurtle = document_path?.endsWith(".ttl") ?? false;
  const isReadOnlyOwl =
    !editable &&
    Boolean(document_path?.match(/\.(owl|owx|rdf)$/i));

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
      case "class_assertion":
        return "Types";
      case "object_property_assertion":
        return "Object property assertions";
      case "data_property_assertion":
        return "Data property assertions";
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
                      {editable &&
                      entity.kind === "individual" &&
                      a.kind === "class_assertion" &&
                      a.editable &&
                      a.parent_iri ? (
                        <button
                          type="button"
                          className="secondary"
                          onClick={() =>
                            apply(
                              [
                                {
                                  op: "remove_class_assertion",
                                  entity_iri: entity.iri,
                                  class_iri: a.parent_iri!,
                                },
                              ],
                              false
                            )
                          }
                        >
                          Remove
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

      {annotations.length > 0 ? (
        <Section title="Annotations" card>
          <ul className="oc-axiom-list">
            {annotations.map((a, idx) => (
              <li key={`ann-${idx}`} className="oc-axiom-item">
                <InlineCode>
                  {shortLabel(a.predicate)} → {a.value}
                </InlineCode>
              </li>
            ))}
          </ul>
        </Section>
      ) : null}

      {isTurtle &&
      (entity.kind === "object_property" || entity.kind === "data_property") &&
      characteristics ? (
        <Section title="Property characteristics" card>
          {(
            [
              ["functional", "Functional"],
              ["inverse_functional", "Inverse functional"],
              ["transitive", "Transitive"],
              ["symmetric", "Symmetric"],
              ["asymmetric", "Asymmetric"],
              ["reflexive", "Reflexive"],
              ["irreflexive", "Irreflexive"],
            ] as const
          ).map(([key, label]) => (
            <label key={key} className="oc-checkbox">
              <input
                type="checkbox"
                checked={Boolean(characteristics[key as keyof PropertyCharacteristics])}
                onChange={(e) =>
                  apply(
                    [
                      {
                        op: `set_${key}`,
                        entity_iri: entity.iri,
                        value: e.target.checked,
                      } as PatchOp,
                    ],
                    false
                  )
                }
              />
              {label}
            </label>
          ))}
        </Section>
      ) : null}

      {editable ? (
        <Section title="Edit" card>
          {isObo ? (
            <>
              <FormField label="Name">
                <Input value={oboName} onChange={(e) => setOboName(e.target.value)} />
              </FormField>
              <PreviewApplyBar
                preview={editPreview}
                disabled={!oboName.trim() || !entity.obo_id}
                onPreview={() => {
                  if (!entity.obo_id || !oboName.trim()) return;
                  apply(
                    [{ op: "set_name", term_id: entity.obo_id, value: oboName.trim() }],
                    true
                  );
                }}
                onApply={() => {
                  if (!entity.obo_id || !oboName.trim()) return;
                  apply(
                    [{ op: "set_name", term_id: entity.obo_id, value: oboName.trim() }],
                    false
                  );
                  setOboName("");
                }}
              />
              <FormField label="Synonym">
                <Input value={oboSynonym} onChange={(e) => setOboSynonym(e.target.value)} />
              </FormField>
              <PreviewApplyBar
                preview={editPreview}
                disabled={!oboSynonym.trim() || !entity.obo_id}
                onPreview={() => {
                  if (!entity.obo_id || !oboSynonym.trim()) return;
                  apply(
                    [
                      {
                        op: "add_synonym",
                        term_id: entity.obo_id,
                        value: oboSynonym.trim(),
                        scope: "exact",
                      },
                    ],
                    true
                  );
                }}
                onApply={() => {
                  if (!entity.obo_id || !oboSynonym.trim()) return;
                  apply(
                    [
                      {
                        op: "add_synonym",
                        term_id: entity.obo_id,
                        value: oboSynonym.trim(),
                        scope: "exact",
                      },
                    ],
                    false
                  );
                  setOboSynonym("");
                }}
              />
              <FormField label="Definition">
                <Input value={oboDef} onChange={(e) => setOboDef(e.target.value)} />
              </FormField>
              <PreviewApplyBar
                preview={editPreview}
                disabled={!oboDef.trim() || !entity.obo_id}
                onPreview={() => {
                  if (!entity.obo_id || !oboDef.trim()) return;
                  apply(
                    [{ op: "add_def", term_id: entity.obo_id, value: oboDef.trim() }],
                    true
                  );
                }}
                onApply={() => {
                  if (!entity.obo_id || !oboDef.trim()) return;
                  apply(
                    [{ op: "add_def", term_id: entity.obo_id, value: oboDef.trim() }],
                    false
                  );
                  setOboDef("");
                }}
              />
              <FormField label="is_a parent">
                <Input value={oboParent} onChange={(e) => setOboParent(e.target.value)} />
              </FormField>
              <PreviewApplyBar
                preview={editPreview}
                disabled={!oboParent.trim() || !entity.obo_id}
                onPreview={() => {
                  if (!entity.obo_id || !oboParent.trim()) return;
                  apply(
                    [
                      {
                        op: "add_is_a",
                        term_id: entity.obo_id,
                        parent_id: oboParent.trim(),
                      },
                    ],
                    true
                  );
                }}
                onApply={() => {
                  if (!entity.obo_id || !oboParent.trim()) return;
                  apply(
                    [
                      {
                        op: "add_is_a",
                        term_id: entity.obo_id,
                        parent_id: oboParent.trim(),
                      },
                    ],
                    false
                  );
                  setOboParent("");
                }}
              />
            </>
          ) : (
            <>
            <FormField label="Add label">
              <Input
                value={newLabel}
                onChange={(e) => setNewLabel(e.target.value)}
              />
            </FormField>
            <PreviewApplyBar
              preview={editPreview}
              disabled={!newLabel.trim()}
              onPreview={() => {
                if (!newLabel.trim()) return;
                apply(
                  [{ op: "add_label", entity_iri: entity.iri, value: newLabel.trim() }],
                  true
                );
              }}
              onApply={() => {
                if (!newLabel.trim()) return;
                apply(
                  [{ op: "add_label", entity_iri: entity.iri, value: newLabel.trim() }],
                  false
                );
                setNewLabel("");
              }}
            />

            <FormField label="Add comment">
              <Input
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
              />
            </FormField>
            <PreviewApplyBar
              preview={editPreview}
              disabled={!newComment.trim()}
              onPreview={() => {
                if (!newComment.trim()) return;
                apply(
                  [
                    {
                      op: "add_comment",
                      entity_iri: entity.iri,
                      value: newComment.trim(),
                    },
                  ],
                  true
                );
              }}
              onApply={() => {
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
            />

            {entity.kind === "class" ? (
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
            ) : null}
            {entity.kind === "class" ? (
              <PreviewApplyBar
                preview={editPreview}
                disabled={!parentPick}
                onPreview={() => {
                  if (!parentPick) return;
                  apply(
                    [
                      {
                        op: "add_sub_class_of",
                        entity_iri: entity.iri,
                        parent_iri: parentPick,
                      },
                    ],
                    true
                  );
                }}
                onApply={() => {
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
              />
            ) : null}

            {(entity.kind === "object_property" || entity.kind === "data_property") && (
              <>
                <FormField label="Domain (class)">
                  <Select value={domainPick} onChange={(e) => setDomainPick(e.target.value)}>
                    <option value="">—</option>
                    {parentOptions.map((c) => (
                      <option key={c} value={c}>
                        {shortLabel(c)}
                      </option>
                    ))}
                  </Select>
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!domainPick}
                  onPreview={() => {
                    if (!domainPick) return;
                    apply(
                      [
                        {
                          op: "add_domain",
                          entity_iri: entity.iri,
                          class_iri: domainPick,
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    if (!domainPick) return;
                    apply(
                      [
                        {
                          op: "add_domain",
                          entity_iri: entity.iri,
                          class_iri: domainPick,
                        },
                      ],
                      false
                    );
                    setDomainPick("");
                  }}
                />
                <FormField label="Range">
                  <Input
                    value={rangePick}
                    onChange={(e) => setRangePick(e.target.value)}
                    placeholder="Class or datatype IRI"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!rangePick.trim()}
                  onPreview={() => {
                    if (!rangePick.trim()) return;
                    apply(
                      [
                        {
                          op: "add_range",
                          entity_iri: entity.iri,
                          range_iri: rangePick.trim(),
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    if (!rangePick.trim()) return;
                    apply(
                      [
                        {
                          op: "add_range",
                          entity_iri: entity.iri,
                          range_iri: rangePick.trim(),
                        },
                      ],
                      false
                    );
                    setRangePick("");
                  }}
                />
              </>
            )}

            {entity.kind === "individual" && (
              <>
                <FormField label="Add type (class assertion)">
                  <Select value={parentPick} onChange={(e) => setParentPick(e.target.value)}>
                    <option value="">—</option>
                    {parentOptions.map((c) => (
                      <option key={c} value={c}>
                        {shortLabel(c)}
                      </option>
                    ))}
                  </Select>
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!parentPick}
                  onPreview={() => {
                    if (!parentPick) return;
                    apply(
                      [
                        {
                          op: "add_class_assertion",
                          entity_iri: entity.iri,
                          class_iri: parentPick,
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    if (!parentPick) return;
                    apply(
                      [
                        {
                          op: "add_class_assertion",
                          entity_iri: entity.iri,
                          class_iri: parentPick,
                        },
                      ],
                      false
                    );
                    setParentPick("");
                  }}
                />
              </>
            )}

            <FormField label="Annotation predicate IRI">
              <Input
                value={annotationPredicate}
                onChange={(e) => setAnnotationPredicate(e.target.value)}
              />
            </FormField>
            <FormField label="Annotation value">
              <Input
                value={annotationValue}
                onChange={(e) => setAnnotationValue(e.target.value)}
              />
            </FormField>
            <PreviewApplyBar
              preview={editPreview}
              disabled={!annotationPredicate.trim() || !annotationValue.trim()}
              onPreview={() => {
                if (!annotationPredicate.trim() || !annotationValue.trim()) return;
                apply(
                  [
                    {
                      op: "add_annotation",
                      entity_iri: entity.iri,
                      predicate: annotationPredicate.trim(),
                      value: annotationValue.trim(),
                    },
                  ],
                  true
                );
              }}
              onApply={() => {
                if (!annotationPredicate.trim() || !annotationValue.trim()) return;
                apply(
                  [
                    {
                      op: "add_annotation",
                      entity_iri: entity.iri,
                      predicate: annotationPredicate.trim(),
                      value: annotationValue.trim(),
                    },
                  ],
                  false
                );
                setAnnotationPredicate("");
                setAnnotationValue("");
              }}
            />

            {entity.kind === "object_property" ? (
              <PropertyChainEditor
                entityIri={entity.iri}
                chains={axioms
                  .filter((a) => a.kind === "property_chain")
                  .map((a) => ({ display: a.display, properties: [] }))}
                propertyOptions={parentOptions}
                preview={editPreview}
                onPreview={(patches) => apply(patches, true)}
                onApply={(patches) => apply(patches, false)}
              />
            ) : null}

            <ButtonBar>
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
            </>
          )}
          {editPreview ? (
            editPreview.startsWith("Error:") ? (
              <Callout variant="error">{editPreview}</Callout>
            ) : (
              <pre className="preview oc-muted">{editPreview}</pre>
            )
          ) : null}
        </Section>
      ) : isReadOnlyOwl ? (
        <Callout variant="info">
          Read-only — edit as Turtle (.ttl) or OBO (.obo). OWL/XML and RDF/XML are view-only in
          v0.12.
        </Callout>
      ) : (
        <Callout variant="info">
          Editing is available for Turtle (.ttl) and OBO (.obo) documents only.
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
