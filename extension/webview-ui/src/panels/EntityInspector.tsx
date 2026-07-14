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
  const [hasKeyProps, setHasKeyProps] = useState("");
  const [disjointUnionMembers, setDisjointUnionMembers] = useState("");
  const [inverseIri, setInverseIri] = useState("");
  const [sameIndividuals, setSameIndividuals] = useState("");
  const [differentIndividuals, setDifferentIndividuals] = useState("");
  const [negPropIri, setNegPropIri] = useState("");
  const [negTargetIri, setNegTargetIri] = useState("");
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
    setHasKeyProps("");
    setDisjointUnionMembers("");
    setInverseIri("");
    setSameIndividuals("");
    setDifferentIndividuals("");
    setNegPropIri("");
    setNegTargetIri("");
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
      case "has_key":
        return "HasKey";
      case "disjoint_union":
        return "DisjointUnion";
      case "inverse_object_properties":
        return "Inverse object properties";
      case "equivalent_object_properties":
        return "Equivalent object properties";
      case "disjoint_object_properties":
        return "Disjoint object properties";
      case "equivalent_data_properties":
        return "Equivalent data properties";
      case "disjoint_data_properties":
        return "Disjoint data properties";
      case "sub_object_property_of":
        return "SubObjectPropertyOf";
      case "sub_data_property_of":
        return "SubDataPropertyOf";
      case "negative_object_property_assertion":
        return "Negative object property assertions";
      case "negative_data_property_assertion":
        return "Negative data property assertions";
      case "same_individual":
        return "Same individuals";
      case "different_individuals":
        return "Different individuals";
      case "datatype_definition":
        return "Datatype definitions";
      default:
        return kind;
    }
  };

  const axiomOpForKind = (kind: string): string | null => {
    switch (kind) {
      case "sub_class_of":
        return "sub_class_of";
      case "disjoint_class":
        return "disjoint_with";
      case "equivalent_class":
        return "equivalent_class";
      case "domain":
        return "domain";
      case "range":
        return "range";
      case "sub_object_property_of":
        return "sub_object_property_of";
      case "sub_data_property_of":
        return "sub_data_property_of";
      case "inverse_object_properties":
        return "inverse_object_properties";
      case "equivalent_object_properties":
        return "equivalent_property";
      case "disjoint_object_properties":
        return "property_disjoint_with";
      case "equivalent_data_properties":
        return "equivalent_property";
      case "disjoint_data_properties":
        return "property_disjoint_with";
      case "same_individual":
        return "same_individual";
      case "different_individuals":
        return "different_individuals";
      default:
        return null;
    }
  };

  const removePatchForAxiom = (a: (typeof axioms)[number]): PatchOp[] | null => {
    switch (a.kind) {
      case "has_key":
        return [
          {
            op: "remove_has_key",
            class_iri: entity.iri,
            properties: a.properties ?? [],
          },
        ];
      case "disjoint_union":
        return [
          {
            op: "remove_disjoint_union",
            class_iri: entity.iri,
            members: a.properties ?? [],
          },
        ];
      case "inverse_object_properties":
        if (!a.other_iri) return null;
        return [
          {
            op: "remove_inverse_object_properties",
            property_iri: entity.iri,
            inverse_iri: a.other_iri,
          },
        ];
      case "same_individual":
        if (!a.other_iri) return null;
        return [
          {
            op: "remove_same_individual",
            individuals: [entity.iri, a.other_iri],
          },
        ];
      case "different_individuals":
        if (!a.other_iri) return null;
        return [
          {
            op: "remove_different_individuals",
            individuals: [entity.iri, a.other_iri],
          },
        ];
      case "negative_object_property_assertion":
        if (!a.other_iri || !a.predicate) return null;
        return [
          {
            op: "remove_negative_object_property_assertion",
            entity_iri: entity.iri,
            property_iri: a.predicate,
            target_iri: a.other_iri,
          },
        ];
      case "negative_data_property_assertion":
        if (!a.predicate || !a.manchester) return null;
        return [
          {
            op: "remove_negative_data_property_assertion",
            entity_iri: entity.iri,
            property_iri: a.predicate,
            value: a.manchester,
          },
        ];
      case "datatype_definition":
        return [
          {
            op: "remove_datatype_definition",
            datatype_iri: entity.iri,
            manchester: a.manchester ?? a.display.replace(/^DatatypeDefinition\s+/, ""),
          },
        ];
      default:
        return null;
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
                  {items.map((a, idx) => {
                    const removePatches = editable ? removePatchForAxiom(a) : null;
                    const axiomOp = axiomOpForKind(a.kind);
                    return (
                    <li key={`${kind}-${idx}`} className="oc-axiom-item">
                      <InlineCode>{a.display}</InlineCode>
                      {a.annotations && a.annotations.length > 0 ? (
                        <ul className="oc-axiom-list oc-axiom-annos">
                          {a.annotations.map((ann, annIdx) => (
                            <li key={`ann-${idx}-${annIdx}`} className="oc-axiom-item">
                              <InlineCode>
                                {shortLabel(ann.predicate)} → {ann.value}
                              </InlineCode>
                              {editable && axiomOp && ann.editable ? (
                                <button
                                  type="button"
                                  className="secondary"
                                  onClick={() =>
                                    apply(
                                      [
                                        {
                                          op: "remove_axiom_annotation",
                                          axiom_op: axiomOp,
                                          subject_iri: entity.iri,
                                          related_iri:
                                            a.other_iri ?? a.parent_iri ?? undefined,
                                          predicate: ann.predicate,
                                          value: ann.value,
                                        },
                                      ],
                                      false
                                    )
                                  }
                                >
                                  Remove annotation
                                </button>
                              ) : null}
                            </li>
                          ))}
                        </ul>
                      ) : null}
                      {editable &&
                      isTurtle &&
                      entity.kind === "class" &&
                      a.editable &&
                      kind !== "property_chain" &&
                      (kind === "sub_class_of" ||
                        kind === "equivalent_class" ||
                        kind === "disjoint_class") ? (
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
                      {editable && removePatches && a.editable ? (
                        <button
                          type="button"
                          className="secondary"
                          onClick={() => apply(removePatches, false)}
                        >
                          Remove
                        </button>
                      ) : null}
                    </li>
                    );
                  })}
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

            {entity.kind === "class" ? (
              <>
                <FormField label="HasKey properties (IRI list, space/comma-separated)">
                  <Input
                    value={hasKeyProps}
                    onChange={(e) => setHasKeyProps(e.target.value)}
                    placeholder="http://ex#p1 http://ex#p2"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!hasKeyProps.trim()}
                  onPreview={() => {
                    const properties = splitIris(hasKeyProps);
                    if (properties.length === 0) return;
                    apply(
                      [{ op: "add_has_key", class_iri: entity.iri, properties }],
                      true
                    );
                  }}
                  onApply={() => {
                    const properties = splitIris(hasKeyProps);
                    if (properties.length === 0) return;
                    apply(
                      [{ op: "add_has_key", class_iri: entity.iri, properties }],
                      false
                    );
                  }}
                />
                <ButtonBar>
                  <button
                    type="button"
                    className="secondary"
                    disabled={!hasKeyProps.trim()}
                    onClick={() => {
                      const properties = splitIris(hasKeyProps);
                      if (properties.length === 0) return;
                      apply(
                        [{ op: "remove_has_key", class_iri: entity.iri, properties }],
                        false
                      );
                    }}
                  >
                    Remove HasKey
                  </button>
                </ButtonBar>
                <FormField label="Disjoint union members (IRI list)">
                  <Input
                    value={disjointUnionMembers}
                    onChange={(e) => setDisjointUnionMembers(e.target.value)}
                    placeholder="http://ex#A http://ex#B"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!disjointUnionMembers.trim()}
                  onPreview={() => {
                    const members = splitIris(disjointUnionMembers);
                    if (members.length === 0) return;
                    apply(
                      [{ op: "add_disjoint_union", class_iri: entity.iri, members }],
                      true
                    );
                  }}
                  onApply={() => {
                    const members = splitIris(disjointUnionMembers);
                    if (members.length === 0) return;
                    apply(
                      [{ op: "add_disjoint_union", class_iri: entity.iri, members }],
                      false
                    );
                  }}
                />
                <ButtonBar>
                  <button
                    type="button"
                    className="secondary"
                    disabled={!disjointUnionMembers.trim()}
                    onClick={() => {
                      const members = splitIris(disjointUnionMembers);
                      if (members.length === 0) return;
                      apply(
                        [
                          {
                            op: "remove_disjoint_union",
                            class_iri: entity.iri,
                            members,
                          },
                        ],
                        false
                      );
                    }}
                  >
                    Remove disjoint union
                  </button>
                </ButtonBar>
              </>
            ) : null}

            {entity.kind === "object_property" ? (
              <>
                <FormField label="Inverse property IRI">
                  <Input
                    value={inverseIri}
                    onChange={(e) => setInverseIri(e.target.value)}
                    placeholder="http://ex#inverseOf"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!inverseIri.trim()}
                  onPreview={() => {
                    if (!inverseIri.trim()) return;
                    apply(
                      [
                        {
                          op: "add_inverse_object_properties",
                          property_iri: entity.iri,
                          inverse_iri: inverseIri.trim(),
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    if (!inverseIri.trim()) return;
                    apply(
                      [
                        {
                          op: "add_inverse_object_properties",
                          property_iri: entity.iri,
                          inverse_iri: inverseIri.trim(),
                        },
                      ],
                      false
                    );
                  }}
                />
                <ButtonBar>
                  <button
                    type="button"
                    className="secondary"
                    disabled={!inverseIri.trim()}
                    onClick={() => {
                      if (!inverseIri.trim()) return;
                      apply(
                        [
                          {
                            op: "remove_inverse_object_properties",
                            property_iri: entity.iri,
                            inverse_iri: inverseIri.trim(),
                          },
                        ],
                        false
                      );
                    }}
                  >
                    Remove inverse
                  </button>
                </ButtonBar>
              </>
            ) : null}

            {entity.kind === "individual" ? (
              <>
                <FormField label="Same individuals (IRI list)">
                  <Input
                    value={sameIndividuals}
                    onChange={(e) => setSameIndividuals(e.target.value)}
                    placeholder="http://ex#alice http://ex#ally"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!sameIndividuals.trim()}
                  onPreview={() => {
                    const others = splitIris(sameIndividuals);
                    if (others.length === 0) return;
                    apply(
                      [
                        {
                          op: "add_same_individual",
                          individuals: [entity.iri, ...others],
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    const others = splitIris(sameIndividuals);
                    if (others.length === 0) return;
                    apply(
                      [
                        {
                          op: "add_same_individual",
                          individuals: [entity.iri, ...others],
                        },
                      ],
                      false
                    );
                  }}
                />
                <FormField label="Different individuals (IRI list)">
                  <Input
                    value={differentIndividuals}
                    onChange={(e) => setDifferentIndividuals(e.target.value)}
                    placeholder="http://ex#bob http://ex#carol"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!differentIndividuals.trim()}
                  onPreview={() => {
                    const others = splitIris(differentIndividuals);
                    if (others.length === 0) return;
                    apply(
                      [
                        {
                          op: "add_different_individuals",
                          individuals: [entity.iri, ...others],
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    const others = splitIris(differentIndividuals);
                    if (others.length === 0) return;
                    apply(
                      [
                        {
                          op: "add_different_individuals",
                          individuals: [entity.iri, ...others],
                        },
                      ],
                      false
                    );
                  }}
                />
                <FormField label="Negative object property">
                  <Select
                    value={negPropIri}
                    onChange={(e) => setNegPropIri(e.target.value)}
                  >
                    <option value="">—</option>
                    {objectPropertyOptions.map((p) => (
                      <option key={p} value={p}>
                        {shortLabel(p)}
                      </option>
                    ))}
                  </Select>
                </FormField>
                <FormField label="Negative assertion target IRI">
                  <Input
                    value={negTargetIri}
                    onChange={(e) => setNegTargetIri(e.target.value)}
                    placeholder="http://ex#target"
                  />
                </FormField>
                <PreviewApplyBar
                  preview={editPreview}
                  disabled={!negPropIri || !negTargetIri.trim()}
                  onPreview={() => {
                    if (!negPropIri || !negTargetIri.trim()) return;
                    apply(
                      [
                        {
                          op: "add_negative_object_property_assertion",
                          entity_iri: entity.iri,
                          property_iri: negPropIri,
                          target_iri: negTargetIri.trim(),
                        },
                      ],
                      true
                    );
                  }}
                  onApply={() => {
                    if (!negPropIri || !negTargetIri.trim()) return;
                    apply(
                      [
                        {
                          op: "add_negative_object_property_assertion",
                          entity_iri: entity.iri,
                          property_iri: negPropIri,
                          target_iri: negTargetIri.trim(),
                        },
                      ],
                      false
                    );
                  }}
                />
              </>
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

function splitIris(raw: string): string[] {
  return raw
    .split(/[\s,]+/)
    .map((s) => s.trim())
    .filter(Boolean);
}
