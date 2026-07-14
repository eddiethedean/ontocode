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
import { useWorkspaceHost } from "../context/HostContext";
import { postFocusToHost } from "../hooks/useFocusSync";
import { useWorkspaceStore } from "../store";
import {
  EntityDetailPayload,
  HostMessage,
  isHostMessage,
  PatchOp,
  PropertyCharacteristics,
} from "../messages";
import type { WorkspaceProps } from "../workspaces/types";

export function EntityInspectorPanel(_props?: WorkspaceProps): JSX.Element {
  const host = useWorkspaceHost();
  const hydrateFocus = useWorkspaceStore((s) => s.hydrateFocus);
  const focusIri = useWorkspaceStore((s) => s.inspector.entityIri);
  const installedPlugins = useWorkspaceStore((s) => s.plugins.installed);
  const setPlugins = useWorkspaceStore((s) => s.setPlugins);
  const [detail, setDetail] = useState<EntityDetailPayload | null>(null);
  const [classOptions, setClassOptions] = useState<string[]>([]);
  const [objectPropertyOptions, setObjectPropertyOptions] = useState<string[]>([]);
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
  const [charDraft, setCharDraft] = useState<PropertyCharacteristics | null>(null);

  const resetFormState = useCallback(() => {
    setPreview("");
    setEditPreview("");
    setNewLabel("");
    setNewComment("");
    setParentPick("");
    setDomainPick("");
    setRangePick("");
    setAnnotationPredicate("");
    setAnnotationValue("");
    setOboName("");
    setOboSynonym("");
    setOboDef("");
    setOboParent("");
    setCharDraft(null);
  }, []);

  const openEntity = useCallback(
    (iri: string) => {
      postFocusToHost(host, { kind: "entity", id: iri, source: "inspector" });
      host.postToCore({ type: "openEntity", iri });
    },
    [host]
  );

  const apply = useCallback(
    (patches: PatchOp[], previewOnly: boolean) => {
      host.postToCore({ type: "applyPatch", patches, previewOnly });
    },
    [host]
  );

  useEffect(() => {
    host.postToCore({ type: "ready", panel: "inspector" });

    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "loadEntity") {
        setDetail(msg.detail);
        setClassOptions(msg.classOptions);
        setObjectPropertyOptions(msg.objectPropertyOptions ?? []);
        resetFormState();
        // Host-driven loads must not push navigation history (#92).
        // Do not hydrate focus when a newer stamped focus already points elsewhere (#277).
        const currentFocus = useWorkspaceStore.getState().focus;
        const incomingIri = msg.detail.entity.iri;
        const stampedElsewhere =
          currentFocus?.kind === "entity" &&
          typeof currentFocus.timestamp === "number" &&
          currentFocus.id !== incomingIri;
        if (!stampedElsewhere) {
          hydrateFocus({
            kind: "entity",
            id: incomingIri,
            source: "inspector",
          });
        }
      } else if (msg.type === "preview") {
        setPreview(msg.text);
        setEditPreview(msg.text);
      } else if (msg.type === "pluginsLoaded") {
        setPlugins(
          msg.plugins.map((p) => ({
            id: p.id,
            name: p.name,
            version: p.version,
            kind: p.kind,
            inspector_cards: p.inspector_cards ?? [],
          }))
        );
      } else if (msg.type === "error") {
        setPreview(`Error: ${msg.message}`);
        setEditPreview(`Error: ${msg.message}`);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [host, hydrateFocus, setPlugins, resetFormState]);

  if (!detail) {
    return (
      <Panel>
        <LoadingState label={focusIri ? `Loading ${shortLabel(focusIri)}…` : "Loading entity…"} />
      </Panel>
    );
  }

  const { entity, parents, children, axioms, editable, document_path, annotations = [], characteristics } = detail;

  const pluginCards = installedPlugins.flatMap((plugin) =>
    plugin.inspector_cards.filter(
      (card) =>
        card.applies_to.length === 0 || card.applies_to.includes(entity.kind)
    )
  );

  const isObo = Boolean(document_path?.match(/\.obo$/i));
  const isTurtle = Boolean(document_path?.match(/\.ttl$/i));
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
  const chainPropertyOptions = objectPropertyOptions.filter((p) => p !== entity.iri);
  const displayedCharacteristics = charDraft ?? characteristics;
  const characteristicKeys = [
    ["functional", "Functional"],
    ["inverse_functional", "Inverse functional"],
    ["transitive", "Transitive"],
    ["symmetric", "Symmetric"],
    ["asymmetric", "Asymmetric"],
    ["reflexive", "Reflexive"],
    ["irreflexive", "Irreflexive"],
  ] as const;

  const characteristicPatches = (): PatchOp[] => {
    if (!characteristics || !displayedCharacteristics) {
      return [];
    }
    const patches: PatchOp[] = [];
    for (const [key] of characteristicKeys) {
      const next = Boolean(displayedCharacteristics[key]);
      const prev = Boolean(characteristics[key]);
      if (next !== prev) {
        patches.push({
          op: `set_${key}`,
          entity_iri: entity.iri,
          value: next,
        } as PatchOp);
      }
    }
    return patches;
  };

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

      {pluginCards.length > 0 ? (
        <Section title="Plugin insights" card>
          {pluginCards.map((card) => (
            <Callout key={card.id} variant="info">
              <strong>{card.title}</strong>
              <p className="muted">
                Naming convention checks run via workspace plugins. Review the Problems panel for
                plugin diagnostics.
              </p>
            </Callout>
          ))}
        </Section>
      ) : null}

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
          editable && isTurtle && entity.kind === "class" ? (
            <button
              type="button"
              className="secondary"
              onClick={() =>
                host.postToCore({ type: "addManchesterAxiom" })
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
                      isTurtle &&
                      entity.kind === "class" &&
                      a.editable &&
                      kind !== "property_chain" ? (
                        <button
                          type="button"
                          className="secondary"
                          onClick={() =>
                            host.postToCore({
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
                      isTurtle &&
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

      {editable &&
      isTurtle &&
      (entity.kind === "object_property" || entity.kind === "data_property") &&
      displayedCharacteristics ? (
        <Section title="Property characteristics" card>
          {characteristicKeys.map(([key, label]) => (
            <label key={key} className="oc-checkbox">
              <input
                type="checkbox"
                checked={Boolean(displayedCharacteristics[key])}
                onChange={(e) =>
                  setCharDraft({
                    ...(displayedCharacteristics as PropertyCharacteristics),
                    [key]: e.target.checked,
                  })
                }
              />
              {label}
            </label>
          ))}
          <PreviewApplyBar
            preview={editPreview}
            disabled={characteristicPatches().length === 0}
            onPreview={() => {
              const patches = characteristicPatches();
              if (patches.length === 0) return;
              apply(patches, true);
            }}
            onApply={() => {
              const patches = characteristicPatches();
              if (patches.length === 0) return;
              apply(patches, false);
            }}
          />
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
              }}
            />

            {entity.kind === "object_property" ? (
              <PropertyChainEditor
                entityIri={entity.iri}
                chains={axioms
                  .filter((a) => a.kind === "property_chain")
                  .map((a) => ({
                    display: a.display,
                    properties: a.properties ?? [],
                  }))}
                propertyOptions={chainPropertyOptions}
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
          onClick={() => host.postToCore({ type: "jumpToSource" })}
        >
          Jump to Source
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() => host.postToCore({ type: "findUsages" })}
        >
          Find Usages
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() => host.postToCore({ type: "renameIri" })}
        >
          Rename IRI
        </button>
        <button
          type="button"
          className="secondary"
          onClick={() =>
            host.postToCore({
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
