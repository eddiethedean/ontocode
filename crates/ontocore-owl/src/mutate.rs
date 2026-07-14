//! Apply PatchOp mutations onto a Horned ontology (v0.21+ XML write-back).

use crate::error::{OwlError, Result};
use crate::manchester::parse_class_expression;
use crate::patch::{PatchDiagnostic, PatchEntityKind, PatchOp};
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue, Build,
    Class, ClassAssertion, ClassExpression, Component, ComponentKind, DataRange, DeclareClass,
    DeclareDataProperty, DeclareNamedIndividual, DeclareObjectProperty, Import, Individual,
    Literal, MutableOntology, ObjectPropertyExpression, OntologyID, RcAnnotatedComponent, RcStr,
    SubClassOf, SubObjectPropertyExpression,
};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use std::collections::BTreeMap;

const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
const OWL_DEPRECATED: &str = "http://www.w3.org/2002/07/owl#deprecated";

/// Apply inspector-oriented patches to a Horned ontology.
///
/// Unsupported ops append error diagnostics and leave the ontology unchanged for that op.
/// Prefix manager ops always error (Turtle-only; they do not map to the Horned model).
pub fn apply_patches_to_ontology(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    patches: &[PatchOp],
) -> Result<Vec<PatchDiagnostic>> {
    apply_patches_to_ontology_with_ns(ont, patches, &BTreeMap::new())
}

/// Like [`apply_patches_to_ontology`], but uses `namespaces` for Manchester / CURIE resolution.
pub fn apply_patches_to_ontology_with_ns(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    patches: &[PatchOp],
    namespaces: &BTreeMap<String, String>,
) -> Result<Vec<PatchDiagnostic>> {
    let build = Build::new_rc();
    let ns = namespaces_with_defaults(namespaces);
    let mut diagnostics = Vec::new();
    for patch in patches {
        if let Err(msg) = apply_one(ont, &build, &ns, patch) {
            diagnostics.push(PatchDiagnostic { severity: "error".into(), message: msg });
        }
    }
    if diagnostics.iter().any(|d| d.severity == "error") {
        return Err(OwlError::PatchInvalid(
            diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>().join("; "),
        ));
    }
    Ok(diagnostics)
}

fn namespaces_with_defaults(ns: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut out = ns.clone();
    for (prefix, iri) in [
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
        ("owl", "http://www.w3.org/2002/07/owl#"),
        ("xsd", "http://www.w3.org/2001/XMLSchema#"),
    ] {
        out.entry(prefix.into()).or_insert_with(|| iri.into());
    }
    out
}

fn apply_one(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
    namespaces: &BTreeMap<String, String>,
    patch: &PatchOp,
) -> std::result::Result<(), String> {
    match patch {
        PatchOp::CreateEntity { entity_iri, kind } => {
            let cmp = match kind {
                PatchEntityKind::Class => {
                    Component::DeclareClass(DeclareClass(build.class(entity_iri.as_str())))
                }
                PatchEntityKind::ObjectProperty => Component::DeclareObjectProperty(
                    DeclareObjectProperty(build.object_property(entity_iri.as_str())),
                ),
                PatchEntityKind::DataProperty => Component::DeclareDataProperty(
                    DeclareDataProperty(build.data_property(entity_iri.as_str())),
                ),
                PatchEntityKind::AnnotationProperty => Component::DeclareAnnotationProperty(
                    horned_owl::model::DeclareAnnotationProperty(
                        build.annotation_property(entity_iri.as_str()),
                    ),
                ),
                PatchEntityKind::Individual => Component::DeclareNamedIndividual(
                    DeclareNamedIndividual(build.named_individual(entity_iri.as_str())),
                ),
                PatchEntityKind::Datatype => Component::DeclareDatatype(
                    horned_owl::model::DeclareDatatype(build.datatype(entity_iri.as_str())),
                ),
            };
            ont.insert(cmp);
            Ok(())
        }
        PatchOp::DeleteEntity { entity_iri } => {
            remove_entity_components(ont, entity_iri);
            Ok(())
        }
        PatchOp::SetLabel { entity_iri, value } => {
            remove_annotation_assertions(ont, entity_iri, RDFS_LABEL, None);
            insert_literal_annotation(ont, build, entity_iri, RDFS_LABEL, value);
            Ok(())
        }
        PatchOp::AddLabel { entity_iri, value } => {
            insert_literal_annotation(ont, build, entity_iri, RDFS_LABEL, value);
            Ok(())
        }
        PatchOp::RemoveLabel { entity_iri, value } => {
            remove_annotation_assertions(ont, entity_iri, RDFS_LABEL, Some(value));
            Ok(())
        }
        PatchOp::SetComment { entity_iri, value } => {
            remove_annotation_assertions(ont, entity_iri, RDFS_COMMENT, None);
            insert_literal_annotation(ont, build, entity_iri, RDFS_COMMENT, value);
            Ok(())
        }
        PatchOp::AddComment { entity_iri, value } => {
            insert_literal_annotation(ont, build, entity_iri, RDFS_COMMENT, value);
            Ok(())
        }
        PatchOp::RemoveComment { entity_iri, value } => {
            remove_annotation_assertions(ont, entity_iri, RDFS_COMMENT, Some(value));
            Ok(())
        }
        PatchOp::AddAnnotation { entity_iri, predicate, value } => {
            insert_literal_annotation(ont, build, entity_iri, predicate, value);
            Ok(())
        }
        PatchOp::RemoveAnnotation { entity_iri, predicate, value } => {
            remove_annotation_assertions(ont, entity_iri, predicate, Some(value));
            Ok(())
        }
        PatchOp::AddSubClassOf { entity_iri, parent_iri } => {
            ont.insert(Component::SubClassOf(SubClassOf {
                sub: ClassExpression::Class(build.class(entity_iri.as_str())),
                sup: ClassExpression::Class(build.class(parent_iri.as_str())),
            }));
            Ok(())
        }
        PatchOp::RemoveSubClassOf { entity_iri, parent_iri } => {
            remove_subclass_of(ont, entity_iri, parent_iri);
            Ok(())
        }
        PatchOp::AddImport { import_iri, .. } => {
            ont.insert(Component::Import(Import(build.iri(import_iri.as_str()))));
            Ok(())
        }
        PatchOp::RemoveImport { import_iri, .. } => {
            let target =
                AnnotatedComponent::from(Component::Import(Import(build.iri(import_iri.as_str()))));
            let _ = ont.take(&target);
            Ok(())
        }
        PatchOp::SetOntologyIri { ontology_iri } => {
            set_ontology_iri(ont, build, ontology_iri, None);
            Ok(())
        }
        PatchOp::SetVersionIri { ontology_iri, version_iri } => {
            set_ontology_iri(ont, build, ontology_iri, Some(version_iri));
            Ok(())
        }
        PatchOp::AddClassAssertion { entity_iri, class_iri } => {
            ont.insert(Component::ClassAssertion(ClassAssertion {
                ce: ClassExpression::Class(build.class(class_iri.as_str())),
                i: Individual::from(build.named_individual(entity_iri.as_str())),
            }));
            Ok(())
        }
        PatchOp::RemoveClassAssertion { entity_iri, class_iri } => {
            remove_class_assertion(ont, entity_iri, class_iri);
            Ok(())
        }
        PatchOp::AddHasKey { class_iri, properties } => {
            let vpe = properties
                .iter()
                .map(|p| {
                    horned_owl::model::PropertyExpression::ObjectPropertyExpression(
                        ObjectPropertyExpression::ObjectProperty(build.object_property(p.as_str())),
                    )
                })
                .collect();
            ont.insert(Component::HasKey(horned_owl::model::HasKey {
                ce: ClassExpression::Class(build.class(class_iri.as_str())),
                vpe,
            }));
            Ok(())
        }
        PatchOp::RemoveHasKey { class_iri, properties } => {
            remove_has_key(ont, class_iri, properties);
            Ok(())
        }
        PatchOp::AddDisjointUnion { class_iri, members } => {
            let ces =
                members.iter().map(|m| ClassExpression::Class(build.class(m.as_str()))).collect();
            ont.insert(Component::DisjointUnion(horned_owl::model::DisjointUnion(
                build.class(class_iri.as_str()),
                ces,
            )));
            Ok(())
        }
        PatchOp::RemoveDisjointUnion { class_iri, members } => {
            remove_disjoint_union(ont, class_iri, members);
            Ok(())
        }
        PatchOp::AddInverseObjectProperties { property_iri, inverse_iri } => {
            ont.insert(Component::InverseObjectProperties(
                horned_owl::model::InverseObjectProperties(
                    build.object_property(property_iri.as_str()),
                    build.object_property(inverse_iri.as_str()),
                ),
            ));
            Ok(())
        }
        PatchOp::RemoveInverseObjectProperties { property_iri, inverse_iri } => {
            let target = AnnotatedComponent::from(Component::InverseObjectProperties(
                horned_owl::model::InverseObjectProperties(
                    build.object_property(property_iri.as_str()),
                    build.object_property(inverse_iri.as_str()),
                ),
            ));
            let _ = ont.take(&target);
            Ok(())
        }
        PatchOp::AddEquivalentObjectProperties { properties } => {
            let ops = properties
                .iter()
                .map(|p| {
                    ObjectPropertyExpression::ObjectProperty(build.object_property(p.as_str()))
                })
                .collect();
            ont.insert(Component::EquivalentObjectProperties(
                horned_owl::model::EquivalentObjectProperties(ops),
            ));
            Ok(())
        }
        PatchOp::RemoveEquivalentObjectProperties { properties } => {
            remove_equivalent_object_properties(ont, properties);
            Ok(())
        }
        PatchOp::AddDisjointObjectProperties { properties } => {
            let ops = properties
                .iter()
                .map(|p| {
                    ObjectPropertyExpression::ObjectProperty(build.object_property(p.as_str()))
                })
                .collect();
            ont.insert(Component::DisjointObjectProperties(
                horned_owl::model::DisjointObjectProperties(ops),
            ));
            Ok(())
        }
        PatchOp::RemoveDisjointObjectProperties { properties } => {
            remove_disjoint_object_properties(ont, properties);
            Ok(())
        }
        PatchOp::AddEquivalentDataProperties { properties } => {
            let dps = properties.iter().map(|p| build.data_property(p.as_str())).collect();
            ont.insert(Component::EquivalentDataProperties(
                horned_owl::model::EquivalentDataProperties(dps),
            ));
            Ok(())
        }
        PatchOp::RemoveEquivalentDataProperties { properties } => {
            remove_equivalent_data_properties(ont, properties);
            Ok(())
        }
        PatchOp::AddDisjointDataProperties { properties } => {
            let dps = properties.iter().map(|p| build.data_property(p.as_str())).collect();
            ont.insert(Component::DisjointDataProperties(
                horned_owl::model::DisjointDataProperties(dps),
            ));
            Ok(())
        }
        PatchOp::RemoveDisjointDataProperties { properties } => {
            remove_disjoint_data_properties(ont, properties);
            Ok(())
        }
        PatchOp::AddSubObjectPropertyOf { property_iri, parent_iri } => {
            ont.insert(Component::SubObjectPropertyOf(horned_owl::model::SubObjectPropertyOf {
                sub: horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(
                    ObjectPropertyExpression::ObjectProperty(
                        build.object_property(property_iri.as_str()),
                    ),
                ),
                sup: ObjectPropertyExpression::ObjectProperty(
                    build.object_property(parent_iri.as_str()),
                ),
            }));
            Ok(())
        }
        PatchOp::RemoveSubObjectPropertyOf { property_iri, parent_iri } => {
            remove_sub_object_property_of(ont, property_iri, parent_iri);
            Ok(())
        }
        PatchOp::AddSubDataPropertyOf { property_iri, parent_iri } => {
            ont.insert(Component::SubDataPropertyOf(horned_owl::model::SubDataPropertyOf {
                sub: build.data_property(property_iri.as_str()),
                sup: build.data_property(parent_iri.as_str()),
            }));
            Ok(())
        }
        PatchOp::RemoveSubDataPropertyOf { property_iri, parent_iri } => {
            remove_sub_data_property_of(ont, property_iri, parent_iri);
            Ok(())
        }
        PatchOp::AddNegativeObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            ont.insert(Component::NegativeObjectPropertyAssertion(
                horned_owl::model::NegativeObjectPropertyAssertion {
                    ope: ObjectPropertyExpression::ObjectProperty(
                        build.object_property(property_iri.as_str()),
                    ),
                    from: Individual::from(build.named_individual(entity_iri.as_str())),
                    to: Individual::from(build.named_individual(target_iri.as_str())),
                },
            ));
            Ok(())
        }
        PatchOp::RemoveNegativeObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            remove_negative_object_property_assertion(ont, entity_iri, property_iri, target_iri);
            Ok(())
        }
        PatchOp::AddNegativeDataPropertyAssertion { entity_iri, property_iri, value } => {
            ont.insert(Component::NegativeDataPropertyAssertion(
                horned_owl::model::NegativeDataPropertyAssertion {
                    dp: build.data_property(property_iri.as_str()),
                    from: Individual::from(build.named_individual(entity_iri.as_str())),
                    to: Literal::Simple { literal: value.clone() },
                },
            ));
            Ok(())
        }
        PatchOp::RemoveNegativeDataPropertyAssertion { entity_iri, property_iri, value } => {
            remove_negative_data_property_assertion(ont, entity_iri, property_iri, value);
            Ok(())
        }
        PatchOp::AddSameIndividual { individuals } => {
            let inds = individuals
                .iter()
                .map(|i| Individual::from(build.named_individual(i.as_str())))
                .collect();
            ont.insert(Component::SameIndividual(horned_owl::model::SameIndividual(inds)));
            Ok(())
        }
        PatchOp::RemoveSameIndividual { individuals } => {
            remove_same_individual(ont, individuals);
            Ok(())
        }
        PatchOp::AddDifferentIndividuals { individuals } => {
            let inds = individuals
                .iter()
                .map(|i| Individual::from(build.named_individual(i.as_str())))
                .collect();
            ont.insert(Component::DifferentIndividuals(horned_owl::model::DifferentIndividuals(
                inds,
            )));
            Ok(())
        }
        PatchOp::RemoveDifferentIndividuals { individuals } => {
            remove_different_individuals(ont, individuals);
            Ok(())
        }
        PatchOp::AddPrefix { .. } | PatchOp::RemovePrefix { .. } | PatchOp::SetPrefix { .. } => {
            Err(format!(
                "{} is Turtle-only; prefix declarations are not part of the Horned OWL model",
                patch_op_name(patch)
            ))
        }
        PatchOp::AddOntologyAnnotation { predicate, value, .. } => {
            ont.insert(Component::OntologyAnnotation(horned_owl::model::OntologyAnnotation(
                Annotation {
                    ap: build.annotation_property(predicate.as_str()),
                    av: AnnotationValue::Literal(Literal::Simple { literal: value.clone() }),
                },
            )));
            Ok(())
        }
        PatchOp::RemoveOntologyAnnotation { predicate, value, .. } => {
            remove_ontology_annotation(ont, predicate, value);
            Ok(())
        }
        PatchOp::AddComplexSubClassOf { entity_iri, manchester } => {
            let ce = parse_manchester_ce(manchester, namespaces)?;
            ont.insert(Component::SubClassOf(SubClassOf {
                sub: ClassExpression::Class(build.class(entity_iri.as_str())),
                sup: ce,
            }));
            Ok(())
        }
        PatchOp::RemoveComplexSubClassOf { entity_iri, manchester } => {
            let ce = parse_manchester_ce(manchester, namespaces)?;
            remove_complex_subclass_of(ont, entity_iri, &ce);
            Ok(())
        }
        PatchOp::AddEquivalentClass { entity_iri, manchester } => {
            let ce = parse_manchester_ce(manchester, namespaces)?;
            ont.insert(Component::EquivalentClasses(horned_owl::model::EquivalentClasses(vec![
                ClassExpression::Class(build.class(entity_iri.as_str())),
                ce,
            ])));
            Ok(())
        }
        PatchOp::RemoveEquivalentClass { entity_iri, manchester } => {
            let ce = parse_manchester_ce(manchester, namespaces)?;
            remove_equivalent_class(ont, entity_iri, &ce);
            Ok(())
        }
        PatchOp::SetEquivalentClass { entity_iri, manchester } => {
            remove_all_equivalent_classes_for(ont, entity_iri);
            let ce = parse_manchester_ce(manchester, namespaces)?;
            ont.insert(Component::EquivalentClasses(horned_owl::model::EquivalentClasses(vec![
                ClassExpression::Class(build.class(entity_iri.as_str())),
                ce,
            ])));
            Ok(())
        }
        PatchOp::SetDeprecated { entity_iri, value } => {
            remove_annotation_assertions(ont, entity_iri, OWL_DEPRECATED, None);
            if *value {
                ont.insert(Component::AnnotationAssertion(AnnotationAssertion {
                    subject: AnnotationSubject::IRI(build.iri(entity_iri.as_str())),
                    ann: Annotation {
                        ap: build.annotation_property(OWL_DEPRECATED),
                        av: AnnotationValue::Literal(Literal::Datatype {
                            literal: "true".into(),
                            datatype_iri: build.iri("http://www.w3.org/2001/XMLSchema#boolean"),
                        }),
                    },
                }));
            }
            Ok(())
        }
        PatchOp::AddDisjointClass { entity_iri, other_iri } => {
            ont.insert(Component::DisjointClasses(horned_owl::model::DisjointClasses(vec![
                ClassExpression::Class(build.class(entity_iri.as_str())),
                ClassExpression::Class(build.class(other_iri.as_str())),
            ])));
            Ok(())
        }
        PatchOp::RemoveDisjointClass { entity_iri, other_iri } => {
            remove_disjoint_class(ont, entity_iri, other_iri);
            Ok(())
        }
        PatchOp::AddDomain { entity_iri, class_iri } => {
            let ce = ClassExpression::Class(build.class(class_iri.as_str()));
            if is_declared_data_property(ont, entity_iri) {
                ont.insert(Component::DataPropertyDomain(horned_owl::model::DataPropertyDomain {
                    dp: build.data_property(entity_iri.as_str()),
                    ce,
                }));
            } else {
                ont.insert(Component::ObjectPropertyDomain(
                    horned_owl::model::ObjectPropertyDomain {
                        ope: ObjectPropertyExpression::ObjectProperty(
                            build.object_property(entity_iri.as_str()),
                        ),
                        ce,
                    },
                ));
            }
            Ok(())
        }
        PatchOp::RemoveDomain { entity_iri, class_iri } => {
            remove_domain(ont, entity_iri, class_iri);
            Ok(())
        }
        PatchOp::AddRange { entity_iri, range_iri } => {
            if is_declared_data_property(ont, entity_iri)
                || (!is_declared_object_property(ont, entity_iri)
                    && (looks_like_datatype_iri(ont, range_iri)
                        || crate::manchester::parse_data_range(range_iri, namespaces).is_ok()))
            {
                let dr = crate::manchester::parse_data_range(range_iri, namespaces)
                    .unwrap_or_else(|_| DataRange::Datatype(build.datatype(range_iri.as_str())));
                ont.insert(Component::DataPropertyRange(horned_owl::model::DataPropertyRange {
                    dp: build.data_property(entity_iri.as_str()),
                    dr,
                }));
            } else {
                ont.insert(Component::ObjectPropertyRange(
                    horned_owl::model::ObjectPropertyRange {
                        ope: ObjectPropertyExpression::ObjectProperty(
                            build.object_property(entity_iri.as_str()),
                        ),
                        ce: ClassExpression::Class(build.class(range_iri.as_str())),
                    },
                ));
            }
            Ok(())
        }
        PatchOp::RemoveRange { entity_iri, range_iri } => {
            remove_range(ont, entity_iri, range_iri);
            Ok(())
        }
        PatchOp::SetFunctional { entity_iri, value } => {
            set_characteristic(ont, build, entity_iri, *value, CharacteristicKind::Functional);
            Ok(())
        }
        PatchOp::SetInverseFunctional { entity_iri, value } => {
            set_characteristic(
                ont,
                build,
                entity_iri,
                *value,
                CharacteristicKind::InverseFunctional,
            );
            Ok(())
        }
        PatchOp::SetTransitive { entity_iri, value } => {
            set_characteristic(ont, build, entity_iri, *value, CharacteristicKind::Transitive);
            Ok(())
        }
        PatchOp::SetSymmetric { entity_iri, value } => {
            set_characteristic(ont, build, entity_iri, *value, CharacteristicKind::Symmetric);
            Ok(())
        }
        PatchOp::SetAsymmetric { entity_iri, value } => {
            set_characteristic(ont, build, entity_iri, *value, CharacteristicKind::Asymmetric);
            Ok(())
        }
        PatchOp::SetReflexive { entity_iri, value } => {
            set_characteristic(ont, build, entity_iri, *value, CharacteristicKind::Reflexive);
            Ok(())
        }
        PatchOp::SetIrreflexive { entity_iri, value } => {
            set_characteristic(ont, build, entity_iri, *value, CharacteristicKind::Irreflexive);
            Ok(())
        }
        PatchOp::AddPropertyChain { entity_iri, properties } => {
            let chain: Vec<_> = properties
                .iter()
                .map(|p| {
                    ObjectPropertyExpression::ObjectProperty(build.object_property(p.as_str()))
                })
                .collect();
            ont.insert(Component::SubObjectPropertyOf(horned_owl::model::SubObjectPropertyOf {
                sub: horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(chain),
                sup: ObjectPropertyExpression::ObjectProperty(
                    build.object_property(entity_iri.as_str()),
                ),
            }));
            Ok(())
        }
        PatchOp::RemovePropertyChain { entity_iri, properties } => {
            remove_property_chain(ont, entity_iri, properties);
            Ok(())
        }
        PatchOp::AddObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            ont.insert(Component::ObjectPropertyAssertion(
                horned_owl::model::ObjectPropertyAssertion {
                    ope: ObjectPropertyExpression::ObjectProperty(
                        build.object_property(property_iri.as_str()),
                    ),
                    from: Individual::from(build.named_individual(entity_iri.as_str())),
                    to: Individual::from(build.named_individual(target_iri.as_str())),
                },
            ));
            Ok(())
        }
        PatchOp::RemoveObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            remove_object_property_assertion(ont, entity_iri, property_iri, target_iri);
            Ok(())
        }
        PatchOp::AddDataPropertyAssertion { entity_iri, property_iri, value } => {
            ont.insert(Component::DataPropertyAssertion(
                horned_owl::model::DataPropertyAssertion {
                    dp: build.data_property(property_iri.as_str()),
                    from: Individual::from(build.named_individual(entity_iri.as_str())),
                    to: Literal::Simple { literal: value.clone() },
                },
            ));
            Ok(())
        }
        PatchOp::RemoveDataPropertyAssertion { entity_iri, property_iri, value } => {
            remove_data_property_assertion(ont, entity_iri, property_iri, value);
            Ok(())
        }
        PatchOp::AddDatatypeDefinition { datatype_iri, manchester } => {
            if !is_declared_datatype(ont, datatype_iri) {
                ont.insert(Component::DeclareDatatype(horned_owl::model::DeclareDatatype(
                    build.datatype(datatype_iri.as_str()),
                )));
            }
            let range = crate::manchester::parse_data_range(manchester, namespaces)
                .map_err(|e| e.to_string())?;
            ont.insert(Component::DatatypeDefinition(horned_owl::model::DatatypeDefinition {
                kind: build.datatype(datatype_iri.as_str()),
                range,
            }));
            Ok(())
        }
        PatchOp::RemoveDatatypeDefinition { datatype_iri, manchester } => {
            let range = crate::manchester::parse_data_range(manchester, namespaces)
                .map_err(|e| e.to_string())?;
            remove_datatype_definition(ont, datatype_iri, &range);
            Ok(())
        }
        PatchOp::AddAxiomAnnotation { axiom_op, subject_iri, related_iri, predicate, value } => {
            mutate_axiom_annotation(
                ont,
                build,
                axiom_op,
                subject_iri,
                related_iri.as_deref(),
                predicate,
                value,
                true,
            )
        }
        PatchOp::RemoveAxiomAnnotation { axiom_op, subject_iri, related_iri, predicate, value } => {
            mutate_axiom_annotation(
                ont,
                build,
                axiom_op,
                subject_iri,
                related_iri.as_deref(),
                predicate,
                value,
                false,
            )
        }
    }
}

fn patch_op_name(op: &PatchOp) -> &'static str {
    match op {
        PatchOp::AddPrefix { .. } => "AddPrefix",
        PatchOp::RemovePrefix { .. } => "RemovePrefix",
        PatchOp::SetPrefix { .. } => "SetPrefix",
        PatchOp::SetOntologyIri { .. } => "SetOntologyIri",
        PatchOp::SetVersionIri { .. } => "SetVersionIri",
        PatchOp::AddOntologyAnnotation { .. } => "AddOntologyAnnotation",
        PatchOp::RemoveOntologyAnnotation { .. } => "RemoveOntologyAnnotation",
        PatchOp::CreateEntity { .. } => "CreateEntity",
        PatchOp::DeleteEntity { .. } => "DeleteEntity",
        PatchOp::SetLabel { .. } => "SetLabel",
        PatchOp::AddLabel { .. } => "AddLabel",
        PatchOp::RemoveLabel { .. } => "RemoveLabel",
        PatchOp::SetComment { .. } => "SetComment",
        PatchOp::AddComment { .. } => "AddComment",
        PatchOp::RemoveComment { .. } => "RemoveComment",
        PatchOp::AddSubClassOf { .. } => "AddSubClassOf",
        PatchOp::RemoveSubClassOf { .. } => "RemoveSubClassOf",
        PatchOp::AddComplexSubClassOf { .. } => "AddComplexSubClassOf",
        PatchOp::RemoveComplexSubClassOf { .. } => "RemoveComplexSubClassOf",
        PatchOp::AddEquivalentClass { .. } => "AddEquivalentClass",
        PatchOp::RemoveEquivalentClass { .. } => "RemoveEquivalentClass",
        PatchOp::SetEquivalentClass { .. } => "SetEquivalentClass",
        PatchOp::SetDeprecated { .. } => "SetDeprecated",
        PatchOp::AddDisjointClass { .. } => "AddDisjointClass",
        PatchOp::RemoveDisjointClass { .. } => "RemoveDisjointClass",
        PatchOp::AddImport { .. } => "AddImport",
        PatchOp::RemoveImport { .. } => "RemoveImport",
        PatchOp::AddDomain { .. } => "AddDomain",
        PatchOp::RemoveDomain { .. } => "RemoveDomain",
        PatchOp::AddRange { .. } => "AddRange",
        PatchOp::RemoveRange { .. } => "RemoveRange",
        PatchOp::SetFunctional { .. } => "SetFunctional",
        PatchOp::SetInverseFunctional { .. } => "SetInverseFunctional",
        PatchOp::SetTransitive { .. } => "SetTransitive",
        PatchOp::SetSymmetric { .. } => "SetSymmetric",
        PatchOp::SetAsymmetric { .. } => "SetAsymmetric",
        PatchOp::SetReflexive { .. } => "SetReflexive",
        PatchOp::SetIrreflexive { .. } => "SetIrreflexive",
        PatchOp::AddPropertyChain { .. } => "AddPropertyChain",
        PatchOp::RemovePropertyChain { .. } => "RemovePropertyChain",
        PatchOp::AddClassAssertion { .. } => "AddClassAssertion",
        PatchOp::RemoveClassAssertion { .. } => "RemoveClassAssertion",
        PatchOp::AddObjectPropertyAssertion { .. } => "AddObjectPropertyAssertion",
        PatchOp::RemoveObjectPropertyAssertion { .. } => "RemoveObjectPropertyAssertion",
        PatchOp::AddDataPropertyAssertion { .. } => "AddDataPropertyAssertion",
        PatchOp::RemoveDataPropertyAssertion { .. } => "RemoveDataPropertyAssertion",
        PatchOp::AddAnnotation { .. } => "AddAnnotation",
        PatchOp::RemoveAnnotation { .. } => "RemoveAnnotation",
        PatchOp::AddHasKey { .. } => "AddHasKey",
        PatchOp::RemoveHasKey { .. } => "RemoveHasKey",
        PatchOp::AddDisjointUnion { .. } => "AddDisjointUnion",
        PatchOp::RemoveDisjointUnion { .. } => "RemoveDisjointUnion",
        PatchOp::AddInverseObjectProperties { .. } => "AddInverseObjectProperties",
        PatchOp::RemoveInverseObjectProperties { .. } => "RemoveInverseObjectProperties",
        PatchOp::AddEquivalentObjectProperties { .. } => "AddEquivalentObjectProperties",
        PatchOp::RemoveEquivalentObjectProperties { .. } => "RemoveEquivalentObjectProperties",
        PatchOp::AddDisjointObjectProperties { .. } => "AddDisjointObjectProperties",
        PatchOp::RemoveDisjointObjectProperties { .. } => "RemoveDisjointObjectProperties",
        PatchOp::AddEquivalentDataProperties { .. } => "AddEquivalentDataProperties",
        PatchOp::RemoveEquivalentDataProperties { .. } => "RemoveEquivalentDataProperties",
        PatchOp::AddDisjointDataProperties { .. } => "AddDisjointDataProperties",
        PatchOp::RemoveDisjointDataProperties { .. } => "RemoveDisjointDataProperties",
        PatchOp::AddSubObjectPropertyOf { .. } => "AddSubObjectPropertyOf",
        PatchOp::RemoveSubObjectPropertyOf { .. } => "RemoveSubObjectPropertyOf",
        PatchOp::AddSubDataPropertyOf { .. } => "AddSubDataPropertyOf",
        PatchOp::RemoveSubDataPropertyOf { .. } => "RemoveSubDataPropertyOf",
        PatchOp::AddNegativeObjectPropertyAssertion { .. } => "AddNegativeObjectPropertyAssertion",
        PatchOp::RemoveNegativeObjectPropertyAssertion { .. } => {
            "RemoveNegativeObjectPropertyAssertion"
        }
        PatchOp::AddNegativeDataPropertyAssertion { .. } => "AddNegativeDataPropertyAssertion",
        PatchOp::RemoveNegativeDataPropertyAssertion { .. } => {
            "RemoveNegativeDataPropertyAssertion"
        }
        PatchOp::AddSameIndividual { .. } => "AddSameIndividual",
        PatchOp::RemoveSameIndividual { .. } => "RemoveSameIndividual",
        PatchOp::AddDifferentIndividuals { .. } => "AddDifferentIndividuals",
        PatchOp::RemoveDifferentIndividuals { .. } => "RemoveDifferentIndividuals",
        PatchOp::AddDatatypeDefinition { .. } => "AddDatatypeDefinition",
        PatchOp::RemoveDatatypeDefinition { .. } => "RemoveDatatypeDefinition",
        PatchOp::AddAxiomAnnotation { .. } => "AddAxiomAnnotation",
        PatchOp::RemoveAxiomAnnotation { .. } => "RemoveAxiomAnnotation",
    }
}

fn insert_literal_annotation(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
    entity_iri: &str,
    predicate: &str,
    value: &str,
) {
    let ann = Annotation {
        ap: build.annotation_property(predicate),
        av: AnnotationValue::Literal(Literal::Simple { literal: value.to_string() }),
    };
    ont.insert(Component::AnnotationAssertion(AnnotationAssertion {
        subject: AnnotationSubject::IRI(build.iri(entity_iri)),
        ann,
    }));
}

fn remove_annotation_assertions(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    predicate: &str,
    value: Option<&str>,
) {
    let to_remove: Vec<_> = ont
        .i()
        .annotation_assertion()
        .filter(|ax| {
            let subj = match &ax.subject {
                AnnotationSubject::IRI(iri) => iri.to_string(),
                _ => return false,
            };
            if subj != entity_iri {
                return false;
            }
            if ax.ann.ap.to_string() != predicate {
                return false;
            }
            if let Some(v) = value {
                match &ax.ann.av {
                    AnnotationValue::Literal(Literal::Simple { literal }) => literal == v,
                    AnnotationValue::Literal(Literal::Language { literal, .. }) => literal == v,
                    AnnotationValue::Literal(Literal::Datatype { literal, .. }) => literal == v,
                    _ => false,
                }
            } else {
                true
            }
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let cmp = AnnotatedComponent::from(Component::AnnotationAssertion(ax));
        let _ = ont.take(&cmp);
    }
}

fn remove_subclass_of(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    parent_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .sub_class_of()
        .filter(|ax| {
            matches!(&ax.sub, ClassExpression::Class(Class(iri)) if iri.to_string() == entity_iri)
                && matches!(
                    &ax.sup,
                    ClassExpression::Class(Class(iri)) if iri.to_string() == parent_iri
                )
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let cmp = AnnotatedComponent::from(Component::SubClassOf(ax));
        let _ = ont.take(&cmp);
    }
}

fn remove_class_assertion(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    class_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .class_assertion()
        .filter(|ax| {
            ax.i.to_string() == entity_iri
                && matches!(
                    &ax.ce,
                    ClassExpression::Class(Class(iri)) if iri.to_string() == class_iri
                )
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::ClassAssertion(ax)));
    }
}

fn remove_has_key(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    class_iri: &str,
    properties: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .has_key()
        .filter(|ax| {
            matches!(
                &ax.ce,
                ClassExpression::Class(Class(iri)) if iri.to_string() == class_iri
            ) && ax.vpe.len() == properties.len()
                && ax.vpe.iter().zip(properties.iter()).all(|(pe, want)| match pe {
                    horned_owl::model::PropertyExpression::ObjectPropertyExpression(
                        ObjectPropertyExpression::ObjectProperty(op),
                    ) => op.to_string() == *want,
                    horned_owl::model::PropertyExpression::DataProperty(dp) => {
                        dp.to_string() == *want
                    }
                    _ => false,
                })
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::HasKey(ax)));
    }
}

fn remove_disjoint_union(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    class_iri: &str,
    members: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .disjoint_union()
        .filter(|ax| {
            ax.0.to_string() == class_iri
                && ax.1.len() == members.len()
                && ax.1.iter().zip(members.iter()).all(|(ce, want)| {
                    matches!(ce, ClassExpression::Class(Class(iri)) if iri.to_string() == *want)
                })
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::DisjointUnion(ax)));
    }
}

fn remove_equivalent_object_properties(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    properties: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .equivalent_object_properties()
        .filter(|ax| property_exprs_match(&ax.0, properties))
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::EquivalentObjectProperties(ax)));
    }
}

fn remove_disjoint_object_properties(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    properties: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .disjoint_object_properties()
        .filter(|ax| property_exprs_match(&ax.0, properties))
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::DisjointObjectProperties(ax)));
    }
}

fn property_exprs_match(ops: &[ObjectPropertyExpression<RcStr>], properties: &[String]) -> bool {
    ops.len() == properties.len()
        && ops.iter().zip(properties.iter()).all(|(ope, want)| match ope {
            ObjectPropertyExpression::ObjectProperty(op) => op.to_string() == *want,
            _ => false,
        })
}

fn remove_equivalent_data_properties(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    properties: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .equivalent_data_properties()
        .filter(|ax| {
            ax.0.len() == properties.len()
                && ax.0.iter().zip(properties.iter()).all(|(dp, want)| dp.to_string() == *want)
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::EquivalentDataProperties(ax)));
    }
}

fn remove_disjoint_data_properties(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    properties: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .disjoint_data_properties()
        .filter(|ax| {
            ax.0.len() == properties.len()
                && ax.0.iter().zip(properties.iter()).all(|(dp, want)| dp.to_string() == *want)
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::DisjointDataProperties(ax)));
    }
}

fn remove_sub_object_property_of(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    property_iri: &str,
    parent_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .sub_object_property_of()
        .filter(|ax| {
            matches!(
                &ax.sub,
                horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(
                    ObjectPropertyExpression::ObjectProperty(op)
                ) if op.to_string() == property_iri
            ) && matches!(
                &ax.sup,
                ObjectPropertyExpression::ObjectProperty(op) if op.to_string() == parent_iri
            )
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::SubObjectPropertyOf(ax)));
    }
}

fn remove_sub_data_property_of(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    property_iri: &str,
    parent_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .sub_data_property_of()
        .filter(|ax| ax.sub.to_string() == property_iri && ax.sup.to_string() == parent_iri)
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::SubDataPropertyOf(ax)));
    }
}

fn remove_negative_object_property_assertion(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    property_iri: &str,
    target_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .negative_object_property_assertion()
        .filter(|ax| {
            ax.from.to_string() == entity_iri
                && ax.to.to_string() == target_iri
                && matches!(
                    &ax.ope,
                    ObjectPropertyExpression::ObjectProperty(op) if op.to_string() == property_iri
                )
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::NegativeObjectPropertyAssertion(ax)));
    }
}

fn remove_negative_data_property_assertion(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    property_iri: &str,
    value: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .negative_data_property_assertion()
        .filter(|ax| {
            ax.from.to_string() == entity_iri
                && ax.dp.to_string() == property_iri
                && match &ax.to {
                    Literal::Simple { literal } => literal == value,
                    Literal::Language { literal, .. } => literal == value,
                    Literal::Datatype { literal, .. } => literal == value,
                }
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::NegativeDataPropertyAssertion(ax)));
    }
}

fn remove_same_individual(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    individuals: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .same_individual()
        .filter(|ax| {
            ax.0.len() == individuals.len()
                && ax.0.iter().zip(individuals.iter()).all(|(i, want)| i.to_string() == *want)
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::SameIndividual(ax)));
    }
}

fn remove_different_individuals(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    individuals: &[String],
) {
    let want: std::collections::BTreeSet<&str> = individuals.iter().map(String::as_str).collect();
    let candidates: Vec<_> = ont
        .i()
        .different_individuals()
        .filter(|ax| {
            let members: std::collections::BTreeSet<String> =
                ax.0.iter().map(|i| i.to_string()).collect();
            want.iter().all(|w| members.iter().any(|m| m == w))
        })
        .cloned()
        .collect();
    for ax in candidates {
        let remaining: Vec<_> =
            ax.0.iter().filter(|i| !want.contains(i.to_string().as_str())).cloned().collect();
        let _ = ont.take(&AnnotatedComponent::from(Component::DifferentIndividuals(ax)));
        if remaining.len() >= 2 {
            ont.insert(Component::DifferentIndividuals(horned_owl::model::DifferentIndividuals(
                remaining,
            )));
        }
    }
}

fn remove_entity_components(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) {
    // Declarations
    let declares: Vec<_> =
        ont.i().declare_class().filter(|d| d.0.to_string() == entity_iri).cloned().collect();
    for d in declares {
        let _ = ont.take(&AnnotatedComponent::from(Component::DeclareClass(d)));
    }
    let op_declares: Vec<_> = ont
        .i()
        .declare_object_property()
        .filter(|d| d.0.to_string() == entity_iri)
        .cloned()
        .collect();
    for d in op_declares {
        let _ = ont.take(&AnnotatedComponent::from(Component::DeclareObjectProperty(d)));
    }
    let dp_declares: Vec<_> = ont
        .i()
        .declare_data_property()
        .filter(|d| d.0.to_string() == entity_iri)
        .cloned()
        .collect();
    for d in dp_declares {
        let _ = ont.take(&AnnotatedComponent::from(Component::DeclareDataProperty(d)));
    }
    let ni_declares: Vec<_> = ont
        .i()
        .declare_named_individual()
        .filter(|d| d.0.to_string() == entity_iri)
        .cloned()
        .collect();
    for d in ni_declares {
        let _ = ont.take(&AnnotatedComponent::from(Component::DeclareNamedIndividual(d)));
    }

    // All annotation assertions on the entity (#304) — not just label/comment.
    let anns: Vec<_> = ont
        .i()
        .annotation_assertion()
        .filter(|ax| {
            matches!(
                &ax.subject,
                AnnotationSubject::IRI(iri) if iri.to_string() == entity_iri
            )
        })
        .cloned()
        .collect();
    for ax in anns {
        let _ = ont.take(&AnnotatedComponent::from(Component::AnnotationAssertion(ax)));
    }

    let sco: Vec<_> = ont
        .i()
        .sub_class_of()
        .filter(|ax| {
            class_expr_mentions(&ax.sub, entity_iri) || class_expr_mentions(&ax.sup, entity_iri)
        })
        .cloned()
        .collect();
    for ax in sco {
        let _ = ont.take(&AnnotatedComponent::from(Component::SubClassOf(ax)));
    }

    let eqs: Vec<_> = ont
        .i()
        .equivalent_class()
        .filter(|ax| ax.0.iter().any(|ce| class_expr_mentions(ce, entity_iri)))
        .cloned()
        .collect();
    for ax in eqs {
        let _ = ont.take(&AnnotatedComponent::from(Component::EquivalentClasses(ax)));
    }

    let disj: Vec<_> = ont
        .i()
        .disjoint_class()
        .filter(|ax| ax.0.iter().any(|ce| class_expr_mentions(ce, entity_iri)))
        .cloned()
        .collect();
    for ax in disj {
        let _ = ont.take(&AnnotatedComponent::from(Component::DisjointClasses(ax)));
    }

    let domains: Vec<_> = ont
        .i()
        .object_property_domain()
        .filter(|ax| ope_mentions(&ax.ope, entity_iri) || class_expr_mentions(&ax.ce, entity_iri))
        .cloned()
        .collect();
    for ax in domains {
        let _ = ont.take(&AnnotatedComponent::from(Component::ObjectPropertyDomain(ax)));
    }

    let ranges: Vec<_> = ont
        .i()
        .object_property_range()
        .filter(|ax| ope_mentions(&ax.ope, entity_iri) || class_expr_mentions(&ax.ce, entity_iri))
        .cloned()
        .collect();
    for ax in ranges {
        let _ = ont.take(&AnnotatedComponent::from(Component::ObjectPropertyRange(ax)));
    }

    let dp_domains: Vec<_> = ont
        .i()
        .data_property_domain()
        .filter(|ax| ax.dp.to_string() == entity_iri || class_expr_mentions(&ax.ce, entity_iri))
        .cloned()
        .collect();
    for ax in dp_domains {
        let _ = ont.take(&AnnotatedComponent::from(Component::DataPropertyDomain(ax)));
    }

    let dp_ranges: Vec<_> = ont
        .i()
        .data_property_range()
        .filter(|ax| ax.dp.to_string() == entity_iri)
        .cloned()
        .collect();
    for ax in dp_ranges {
        let _ = ont.take(&AnnotatedComponent::from(Component::DataPropertyRange(ax)));
    }

    let cas: Vec<_> = ont
        .i()
        .class_assertion()
        .filter(|ax| {
            individual_mentions(&ax.i, entity_iri) || class_expr_mentions(&ax.ce, entity_iri)
        })
        .cloned()
        .collect();
    for ax in cas {
        let _ = ont.take(&AnnotatedComponent::from(Component::ClassAssertion(ax)));
    }
}

fn class_expr_mentions(ce: &ClassExpression<RcStr>, entity_iri: &str) -> bool {
    matches!(ce, ClassExpression::Class(Class(iri)) if iri.to_string() == entity_iri)
}

fn ope_mentions(ope: &ObjectPropertyExpression<RcStr>, entity_iri: &str) -> bool {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => p.to_string() == entity_iri,
        ObjectPropertyExpression::InverseObjectProperty(p) => p.to_string() == entity_iri,
    }
}

fn individual_mentions(ind: &Individual<RcStr>, entity_iri: &str) -> bool {
    ind.to_string() == entity_iri
}

fn parse_manchester_ce(
    manchester: &str,
    namespaces: &BTreeMap<String, String>,
) -> std::result::Result<ClassExpression<RcStr>, String> {
    parse_class_expression(manchester, namespaces)
        .map(|out| out.expression)
        .map_err(|e| e.to_string())
}

fn is_declared_data_property(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) -> bool {
    ont.i().declare_data_property().any(|d| d.0.to_string() == entity_iri)
}

fn is_declared_object_property(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) -> bool {
    ont.i().declare_object_property().any(|d| d.0.to_string() == entity_iri)
}

fn is_declared_datatype(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) -> bool {
    ont.i().declare_datatype().any(|d| d.0.to_string() == entity_iri)
}

fn looks_like_datatype_iri(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    iri: &str,
) -> bool {
    iri.contains("XMLSchema") || is_declared_datatype(ont, iri)
}

#[derive(Clone, Copy)]
enum CharacteristicKind {
    Functional,
    InverseFunctional,
    Transitive,
    Symmetric,
    Asymmetric,
    Reflexive,
    Irreflexive,
}

fn set_characteristic(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
    entity_iri: &str,
    value: bool,
    kind: CharacteristicKind,
) {
    remove_characteristic(ont, entity_iri, kind);
    if !value {
        return;
    }
    let ope = ObjectPropertyExpression::ObjectProperty(build.object_property(entity_iri));
    match kind {
        CharacteristicKind::Functional => {
            if is_declared_data_property(ont, entity_iri) {
                ont.insert(Component::FunctionalDataProperty(
                    horned_owl::model::FunctionalDataProperty(build.data_property(entity_iri)),
                ));
            } else {
                ont.insert(Component::FunctionalObjectProperty(
                    horned_owl::model::FunctionalObjectProperty(ope),
                ));
            }
        }
        CharacteristicKind::InverseFunctional => {
            ont.insert(Component::InverseFunctionalObjectProperty(
                horned_owl::model::InverseFunctionalObjectProperty(ope),
            ));
        }
        CharacteristicKind::Transitive => {
            ont.insert(Component::TransitiveObjectProperty(
                horned_owl::model::TransitiveObjectProperty(ope),
            ));
        }
        CharacteristicKind::Symmetric => {
            ont.insert(Component::SymmetricObjectProperty(
                horned_owl::model::SymmetricObjectProperty(ope),
            ));
        }
        CharacteristicKind::Asymmetric => {
            ont.insert(Component::AsymmetricObjectProperty(
                horned_owl::model::AsymmetricObjectProperty(ope),
            ));
        }
        CharacteristicKind::Reflexive => {
            ont.insert(Component::ReflexiveObjectProperty(
                horned_owl::model::ReflexiveObjectProperty(ope),
            ));
        }
        CharacteristicKind::Irreflexive => {
            ont.insert(Component::IrreflexiveObjectProperty(
                horned_owl::model::IrreflexiveObjectProperty(ope),
            ));
        }
    }
}

fn remove_characteristic(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    kind: CharacteristicKind,
) {
    match kind {
        CharacteristicKind::Functional => {
            let ops: Vec<_> = ont
                .i()
                .functional_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ =
                    ont.take(&AnnotatedComponent::from(Component::FunctionalObjectProperty(ax)));
            }
            let dps: Vec<_> = ont
                .i()
                .functional_data_property()
                .filter(|ax| ax.0.to_string() == entity_iri)
                .cloned()
                .collect();
            for ax in dps {
                let _ = ont.take(&AnnotatedComponent::from(Component::FunctionalDataProperty(ax)));
            }
        }
        CharacteristicKind::InverseFunctional => {
            let ops: Vec<_> = ont
                .i()
                .inverse_functional_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ = ont.take(&AnnotatedComponent::from(
                    Component::InverseFunctionalObjectProperty(ax),
                ));
            }
        }
        CharacteristicKind::Transitive => {
            let ops: Vec<_> = ont
                .i()
                .transitive_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ =
                    ont.take(&AnnotatedComponent::from(Component::TransitiveObjectProperty(ax)));
            }
        }
        CharacteristicKind::Symmetric => {
            let ops: Vec<_> = ont
                .i()
                .symmetric_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ = ont.take(&AnnotatedComponent::from(Component::SymmetricObjectProperty(ax)));
            }
        }
        CharacteristicKind::Asymmetric => {
            let ops: Vec<_> = ont
                .i()
                .asymmetric_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ =
                    ont.take(&AnnotatedComponent::from(Component::AsymmetricObjectProperty(ax)));
            }
        }
        CharacteristicKind::Reflexive => {
            let ops: Vec<_> = ont
                .i()
                .reflexive_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ = ont.take(&AnnotatedComponent::from(Component::ReflexiveObjectProperty(ax)));
            }
        }
        CharacteristicKind::Irreflexive => {
            let ops: Vec<_> = ont
                .i()
                .irreflexive_object_property()
                .filter(|ax| ope_mentions(&ax.0, entity_iri))
                .cloned()
                .collect();
            for ax in ops {
                let _ =
                    ont.take(&AnnotatedComponent::from(Component::IrreflexiveObjectProperty(ax)));
            }
        }
    }
}

fn remove_ontology_annotation(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    predicate: &str,
    value: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .ontology_annotation()
        .filter(|ax| {
            ax.0.ap.to_string() == predicate
                && match &ax.0.av {
                    AnnotationValue::Literal(Literal::Simple { literal }) => literal == value,
                    AnnotationValue::Literal(Literal::Language { literal, .. }) => literal == value,
                    AnnotationValue::Literal(Literal::Datatype { literal, .. }) => literal == value,
                    _ => false,
                }
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::OntologyAnnotation(ax)));
    }
}

fn remove_complex_subclass_of(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    ce: &ClassExpression<RcStr>,
) {
    let to_remove: Vec<_> = ont
        .i()
        .sub_class_of()
        .filter(|ax| {
            matches!(&ax.sub, ClassExpression::Class(Class(iri)) if iri.to_string() == entity_iri)
                && &ax.sup == ce
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::SubClassOf(ax)));
    }
}

fn remove_equivalent_class(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    ce: &ClassExpression<RcStr>,
) {
    let to_remove: Vec<_> = ont
        .i()
        .equivalent_class()
        .filter(|ax| {
            ax.0.iter().any(|c| {
                matches!(c, ClassExpression::Class(Class(iri)) if iri.to_string() == entity_iri)
            }) && ax.0.iter().any(|c| c == ce)
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::EquivalentClasses(ax)));
    }
}

fn remove_all_equivalent_classes_for(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .equivalent_class()
        .filter(|ax| ax.0.iter().any(|ce| class_expr_mentions(ce, entity_iri)))
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::EquivalentClasses(ax)));
    }
}

fn remove_disjoint_class(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    other_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .disjoint_class()
        .filter(|ax| {
            let has_a = ax.0.iter().any(|ce| class_expr_mentions(ce, entity_iri));
            let has_b = ax.0.iter().any(|ce| class_expr_mentions(ce, other_iri));
            has_a && has_b && ax.0.len() == 2
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::DisjointClasses(ax)));
    }
}

fn remove_domain(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    class_iri: &str,
) {
    let ops: Vec<_> = ont
        .i()
        .object_property_domain()
        .filter(|ax| ope_mentions(&ax.ope, entity_iri) && class_expr_mentions(&ax.ce, class_iri))
        .cloned()
        .collect();
    for ax in ops {
        let _ = ont.take(&AnnotatedComponent::from(Component::ObjectPropertyDomain(ax)));
    }
    let dps: Vec<_> = ont
        .i()
        .data_property_domain()
        .filter(|ax| ax.dp.to_string() == entity_iri && class_expr_mentions(&ax.ce, class_iri))
        .cloned()
        .collect();
    for ax in dps {
        let _ = ont.take(&AnnotatedComponent::from(Component::DataPropertyDomain(ax)));
    }
}

fn remove_range(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    range_iri: &str,
) {
    let ops: Vec<_> = ont
        .i()
        .object_property_range()
        .filter(|ax| ope_mentions(&ax.ope, entity_iri) && class_expr_mentions(&ax.ce, range_iri))
        .cloned()
        .collect();
    for ax in ops {
        let _ = ont.take(&AnnotatedComponent::from(Component::ObjectPropertyRange(ax)));
    }
    let dps: Vec<_> = ont
        .i()
        .data_property_range()
        .filter(|ax| {
            ax.dp.to_string() == entity_iri
                && matches!(
                    &ax.dr,
                    DataRange::Datatype(dt) if dt.to_string() == range_iri
                )
        })
        .cloned()
        .collect();
    for ax in dps {
        let _ = ont.take(&AnnotatedComponent::from(Component::DataPropertyRange(ax)));
    }
}

fn remove_property_chain(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    properties: &[String],
) {
    let to_remove: Vec<_> = ont
        .i()
        .sub_object_property_of()
        .filter(|ax| {
            matches!(
                &ax.sup,
                ObjectPropertyExpression::ObjectProperty(op) if op.to_string() == entity_iri
            ) && match &ax.sub {
                horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(chain) => {
                    property_exprs_match(chain, properties)
                }
                _ => false,
            }
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::SubObjectPropertyOf(ax)));
    }
}

fn remove_object_property_assertion(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    property_iri: &str,
    target_iri: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .object_property_assertion()
        .filter(|ax| {
            ax.from.to_string() == entity_iri
                && ax.to.to_string() == target_iri
                && matches!(
                    &ax.ope,
                    ObjectPropertyExpression::ObjectProperty(op) if op.to_string() == property_iri
                )
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::ObjectPropertyAssertion(ax)));
    }
}

fn remove_data_property_assertion(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
    property_iri: &str,
    value: &str,
) {
    let to_remove: Vec<_> = ont
        .i()
        .data_property_assertion()
        .filter(|ax| {
            ax.from.to_string() == entity_iri
                && ax.dp.to_string() == property_iri
                && match &ax.to {
                    Literal::Simple { literal } => literal == value,
                    Literal::Language { literal, .. } => literal == value,
                    Literal::Datatype { literal, .. } => literal == value,
                }
        })
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::DataPropertyAssertion(ax)));
    }
}

fn remove_datatype_definition(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    datatype_iri: &str,
    range: &DataRange<RcStr>,
) {
    let to_remove: Vec<_> = ont
        .i()
        .datatype_definition()
        .filter(|ax| ax.kind.to_string() == datatype_iri && &ax.range == range)
        .cloned()
        .collect();
    for ax in to_remove {
        let _ = ont.take(&AnnotatedComponent::from(Component::DatatypeDefinition(ax)));
    }
}

fn annotation_value_matches(av: &AnnotationValue<RcStr>, value: &str) -> bool {
    match av {
        AnnotationValue::Literal(Literal::Simple { literal }) => literal == value,
        AnnotationValue::Literal(Literal::Language { literal, .. }) => literal == value,
        AnnotationValue::Literal(Literal::Datatype { literal, .. }) => literal == value,
        AnnotationValue::IRI(iri) => iri.to_string() == value,
        _ => false,
    }
}

fn annotation_av_from_value(build: &Build<RcStr>, value: &str) -> AnnotationValue<RcStr> {
    let trimmed = value.trim();
    if let Some(inner) = trimmed.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
        return AnnotationValue::IRI(build.iri(inner.trim()));
    }
    if (trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("urn:"))
        && !trimmed.contains(' ')
    {
        return AnnotationValue::IRI(build.iri(trimmed));
    }
    AnnotationValue::Literal(Literal::Simple { literal: trimmed.to_string() })
}

const AXIOM_ANN_SUPPORTED: &str = "sub_class_of, disjoint_with, equivalent_class, domain, range, \
sub_object_property_of, sub_data_property_of, inverse_object_properties, equivalent_property, \
equivalent_object_properties, equivalent_data_properties, property_disjoint_with, \
disjoint_object_properties, disjoint_data_properties, same_individual, different_individuals";

#[allow(clippy::too_many_arguments)]
fn mutate_axiom_annotation(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
    axiom_op: &str,
    subject_iri: &str,
    related_iri: Option<&str>,
    predicate: &str,
    value: &str,
    add: bool,
) -> std::result::Result<(), String> {
    let ann = Annotation {
        ap: build.annotation_property(predicate),
        av: annotation_av_from_value(build, value),
    };
    let kind = match axiom_op {
        "sub_class_of" => ComponentKind::SubClassOf,
        "disjoint_with" | "disjoint_class" => ComponentKind::DisjointClasses,
        "equivalent_class" => ComponentKind::EquivalentClasses,
        "domain" => {
            // Prefer object property domain; fall through handled by dual search below.
            ComponentKind::ObjectPropertyDomain
        }
        "range" => ComponentKind::ObjectPropertyRange,
        "sub_object_property_of" | "sub_property_of" => ComponentKind::SubObjectPropertyOf,
        "sub_data_property_of" => ComponentKind::SubDataPropertyOf,
        "inverse_of" | "inverse_object_properties" => ComponentKind::InverseObjectProperties,
        "equivalent_property" | "equivalent_object_properties" => {
            ComponentKind::EquivalentObjectProperties
        }
        "equivalent_data_properties" => ComponentKind::EquivalentDataProperties,
        "property_disjoint_with" | "disjoint_object_properties" => {
            ComponentKind::DisjointObjectProperties
        }
        "disjoint_data_properties" => ComponentKind::DisjointDataProperties,
        "same_as" | "same_individual" => ComponentKind::SameIndividual,
        "different_from" | "different_individuals" => ComponentKind::DifferentIndividuals,
        other => {
            return Err(format!(
                "axiom annotation for axiom_op '{other}' not yet supported for XML write-back (supported: {AXIOM_ANN_SUPPORTED})"
            ));
        }
    };

    let related = related_iri.unwrap_or("");
    let matches = |ac: &AnnotatedComponent<RcStr>| -> bool {
        match &ac.component {
            Component::SubClassOf(ax) if axiom_op == "sub_class_of" => {
                matches!(
                    &ax.sub,
                    ClassExpression::Class(Class(iri)) if iri.to_string() == subject_iri
                ) && (related.is_empty()
                    || matches!(
                        &ax.sup,
                        ClassExpression::Class(Class(iri)) if iri.to_string() == related
                    ))
            }
            Component::DisjointClasses(ax)
                if matches!(axiom_op, "disjoint_with" | "disjoint_class") =>
            {
                ax.0.iter().any(|ce| class_expr_mentions(ce, subject_iri))
                    && (related.is_empty()
                        || ax.0.iter().any(|ce| class_expr_mentions(ce, related)))
            }
            Component::EquivalentClasses(ax) if axiom_op == "equivalent_class" => {
                ax.0.iter().any(|ce| class_expr_mentions(ce, subject_iri))
                    && (related.is_empty()
                        || ax.0.iter().any(|ce| class_expr_mentions(ce, related)))
            }
            Component::ObjectPropertyDomain(ax) if axiom_op == "domain" => {
                ope_mentions(&ax.ope, subject_iri)
                    && (related.is_empty() || class_expr_mentions(&ax.ce, related))
            }
            Component::DataPropertyDomain(ax) if axiom_op == "domain" => {
                ax.dp.to_string() == subject_iri
                    && (related.is_empty() || class_expr_mentions(&ax.ce, related))
            }
            Component::ObjectPropertyRange(ax) if axiom_op == "range" => {
                ope_mentions(&ax.ope, subject_iri)
                    && (related.is_empty() || class_expr_mentions(&ax.ce, related))
            }
            Component::DataPropertyRange(ax) if axiom_op == "range" => {
                ax.dp.to_string() == subject_iri
                    && (related.is_empty()
                        || data_range_display_match(&ax.dr, related)
                        || matches!(
                            &ax.dr,
                            DataRange::Datatype(dt) if dt.to_string() == related
                        ))
            }
            Component::SubObjectPropertyOf(ax)
                if matches!(axiom_op, "sub_object_property_of" | "sub_property_of") =>
            {
                match &ax.sub {
                    SubObjectPropertyExpression::ObjectPropertyExpression(sub) => {
                        ope_mentions(sub, subject_iri)
                            && (related.is_empty() || ope_mentions(&ax.sup, related))
                    }
                    _ => false,
                }
            }
            Component::SubDataPropertyOf(ax) if axiom_op == "sub_data_property_of" => {
                ax.sub.to_string() == subject_iri
                    && (related.is_empty() || ax.sup.to_string() == related)
            }
            Component::InverseObjectProperties(ax)
                if matches!(axiom_op, "inverse_of" | "inverse_object_properties") =>
            {
                let a = ax.0.to_string();
                let b = ax.1.to_string();
                (a == subject_iri || b == subject_iri)
                    && (related.is_empty() || a == related || b == related)
            }
            Component::EquivalentObjectProperties(ax)
                if matches!(axiom_op, "equivalent_property" | "equivalent_object_properties") =>
            {
                ax.0.iter().any(|ope| ope_mentions(ope, subject_iri))
                    && (related.is_empty() || ax.0.iter().any(|ope| ope_mentions(ope, related)))
            }
            Component::EquivalentDataProperties(ax)
                if matches!(axiom_op, "equivalent_property" | "equivalent_data_properties") =>
            {
                ax.0.iter().any(|dp| dp.to_string() == subject_iri)
                    && (related.is_empty() || ax.0.iter().any(|dp| dp.to_string() == related))
            }
            Component::DisjointObjectProperties(ax)
                if matches!(axiom_op, "property_disjoint_with" | "disjoint_object_properties") =>
            {
                ax.0.iter().any(|ope| ope_mentions(ope, subject_iri))
                    && (related.is_empty() || ax.0.iter().any(|ope| ope_mentions(ope, related)))
            }
            Component::DisjointDataProperties(ax)
                if matches!(axiom_op, "property_disjoint_with" | "disjoint_data_properties") =>
            {
                ax.0.iter().any(|dp| dp.to_string() == subject_iri)
                    && (related.is_empty() || ax.0.iter().any(|dp| dp.to_string() == related))
            }
            Component::SameIndividual(ax) if matches!(axiom_op, "same_as" | "same_individual") => {
                ax.0.iter().any(|i| individual_mentions(i, subject_iri))
                    && (related.is_empty() || ax.0.iter().any(|i| individual_mentions(i, related)))
            }
            Component::DifferentIndividuals(ax)
                if matches!(axiom_op, "different_from" | "different_individuals") =>
            {
                ax.0.iter().any(|i| individual_mentions(i, subject_iri))
                    && (related.is_empty() || ax.0.iter().any(|i| individual_mentions(i, related)))
            }
            _ => false,
        }
    };

    let mut kinds = vec![kind];
    // Domain / range may live on data or object properties.
    if axiom_op == "domain" {
        kinds.push(ComponentKind::DataPropertyDomain);
    } else if axiom_op == "range" {
        kinds.push(ComponentKind::DataPropertyRange);
    }
    if matches!(axiom_op, "equivalent_property") {
        kinds.push(ComponentKind::EquivalentDataProperties);
    }
    if matches!(axiom_op, "property_disjoint_with") {
        kinds.push(ComponentKind::DisjointDataProperties);
    }

    let mut targets: Vec<_> = Vec::new();
    for k in kinds {
        targets.extend(ont.i().component_for_kind(k).filter(|ac| matches(ac)).cloned());
    }
    if targets.is_empty() {
        return Err(format!(
            "no matching {axiom_op} axiom found for axiom annotation on {subject_iri}"
        ));
    }
    if targets.len() > 1 && related.is_empty() {
        return Err(format!(
            "ambiguous {axiom_op} axiom annotation on {subject_iri}: {} matches; supply related_iri",
            targets.len()
        ));
    }
    for target in targets {
        let mut updated = target.clone();
        let _ = ont.take(&target);
        if add {
            updated.ann.insert(ann.clone());
        } else {
            updated.ann.retain(|a| {
                !(a.ap.to_string() == predicate && annotation_value_matches(&a.av, value))
            });
        }
        ont.insert(updated);
    }
    Ok(())
}

fn data_range_display_match(dr: &DataRange<RcStr>, related: &str) -> bool {
    crate::manchester::data_range_to_manchester(dr, &BTreeMap::new()) == related
}

fn set_ontology_iri(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
    ontology_iri: &str,
    version_iri: Option<&str>,
) {
    // Preserve existing owl:versionIRI when SetOntologyIri does not supply one (#303).
    let preserved_viri = ont.i().the_ontology_id().and_then(|id| id.viri.clone());
    if let Some(id) = ont.i().the_ontology_id() {
        let _ = ont.take(&AnnotatedComponent::from(Component::OntologyID(id)));
    }
    let viri = match version_iri {
        Some(v) => Some(build.iri(v)),
        None => preserved_viri,
    };
    ont.insert(Component::OntologyID(OntologyID { iri: Some(build.iri(ontology_iri)), viri }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::load_rdf_xml_ontology;

    #[test]
    fn set_ontology_iri_preserves_version_iri() {
        let source = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
    <owl:Ontology rdf:about="http://example.org/ont">
        <owl:versionIRI rdf:resource="http://example.org/ont/1.0"/>
    </owl:Ontology>
    <owl:Class rdf:about="http://example.org/ont#A">
        <rdfs:label>A</rdfs:label>
    </owl:Class>
</rdf:RDF>"#;
        let (mut ont, _ns) = load_rdf_xml_ontology(source).expect("load");
        apply_patches_to_ontology(
            &mut ont,
            &[PatchOp::SetOntologyIri { ontology_iri: "http://example.org/ont-renamed".into() }],
        )
        .expect("set ontology iri");
        let id = ont.i().the_ontology_id().expect("ontology id");
        assert_eq!(
            id.iri.as_ref().map(|i| i.to_string()).as_deref(),
            Some("http://example.org/ont-renamed")
        );
        assert_eq!(
            id.viri.as_ref().map(|i| i.to_string()).as_deref(),
            Some("http://example.org/ont/1.0")
        );
    }

    #[test]
    fn delete_entity_removes_equivalents_annotations_and_domains() {
        let source = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#"
     xmlns:skos="http://www.w3.org/2004/02/skos/core#">
    <owl:Ontology rdf:about="http://example.org/ont"/>
    <owl:Class rdf:about="http://example.org/ont#A">
        <rdfs:label>A</rdfs:label>
        <skos:note>extra</skos:note>
        <owl:equivalentClass rdf:resource="http://example.org/ont#B"/>
        <rdfs:subClassOf rdf:resource="http://example.org/ont#C"/>
    </owl:Class>
    <owl:Class rdf:about="http://example.org/ont#B"/>
    <owl:Class rdf:about="http://example.org/ont#C"/>
    <owl:ObjectProperty rdf:about="http://example.org/ont#p">
        <rdfs:domain rdf:resource="http://example.org/ont#A"/>
    </owl:ObjectProperty>
</rdf:RDF>"#;
        let (mut ont, _ns) = load_rdf_xml_ontology(source).expect("load");
        apply_patches_to_ontology(
            &mut ont,
            &[PatchOp::DeleteEntity { entity_iri: "http://example.org/ont#A".into() }],
        )
        .expect("delete");
        assert_eq!(
            ont.i()
                .declare_class()
                .filter(|d| d.0.to_string() == "http://example.org/ont#A")
                .count(),
            0
        );
        assert_eq!(
            ont.i()
                .annotation_assertion()
                .filter(|ax| {
                    matches!(
                        &ax.subject,
                        AnnotationSubject::IRI(iri) if iri.to_string() == "http://example.org/ont#A"
                    )
                })
                .count(),
            0
        );
        assert_eq!(
            ont.i()
                .equivalent_class()
                .filter(|ax| ax
                    .0
                    .iter()
                    .any(|ce| class_expr_mentions(ce, "http://example.org/ont#A")))
                .count(),
            0
        );
        assert_eq!(
            ont.i()
                .object_property_domain()
                .filter(|ax| class_expr_mentions(&ax.ce, "http://example.org/ont#A"))
                .count(),
            0
        );
    }

    #[test]
    fn mutate_v022_ops_domain_disjoint_assertion_and_prefix_error() {
        let source = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
    <owl:Ontology rdf:about="http://example.org/ont"/>
    <owl:Class rdf:about="http://example.org/ont#A"/>
    <owl:Class rdf:about="http://example.org/ont#B"/>
    <owl:ObjectProperty rdf:about="http://example.org/ont#p"/>
    <owl:NamedIndividual rdf:about="http://example.org/ont#i1"/>
    <owl:NamedIndividual rdf:about="http://example.org/ont#i2"/>
</rdf:RDF>"#;
        let (mut ont, _incomplete) = load_rdf_xml_ontology(source).expect("load");
        apply_patches_to_ontology(
            &mut ont,
            &[
                PatchOp::AddDisjointClass {
                    entity_iri: "http://example.org/ont#A".into(),
                    other_iri: "http://example.org/ont#B".into(),
                },
                PatchOp::AddDomain {
                    entity_iri: "http://example.org/ont#p".into(),
                    class_iri: "http://example.org/ont#A".into(),
                },
                PatchOp::SetTransitive {
                    entity_iri: "http://example.org/ont#p".into(),
                    value: true,
                },
                PatchOp::AddObjectPropertyAssertion {
                    entity_iri: "http://example.org/ont#i1".into(),
                    property_iri: "http://example.org/ont#p".into(),
                    target_iri: "http://example.org/ont#i2".into(),
                },
                PatchOp::AddAxiomAnnotation {
                    axiom_op: "disjoint_with".into(),
                    subject_iri: "http://example.org/ont#A".into(),
                    related_iri: Some("http://example.org/ont#B".into()),
                    predicate: "http://www.w3.org/2000/01/rdf-schema#comment".into(),
                    value: "annotated disjoint".into(),
                },
            ],
        )
        .expect("apply v0.22 ops");
        assert_eq!(ont.i().disjoint_class().count(), 1);
        assert_eq!(ont.i().object_property_domain().count(), 1);
        assert_eq!(ont.i().transitive_object_property().count(), 1);
        assert_eq!(ont.i().object_property_assertion().count(), 1);
        assert!(
            ont.i().component_for_kind(ComponentKind::DisjointClasses).any(|ac| !ac.ann.is_empty()),
            "expected axiom annotation on disjoint"
        );

        let err = apply_patches_to_ontology(
            &mut ont,
            &[PatchOp::AddPrefix {
                prefix: "ex".into(),
                namespace_iri: "http://example.org/".into(),
            }],
        );
        assert!(err.is_err());
    }

    #[test]
    fn axiom_annotation_on_domain_and_datatype_facets() {
        let source = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#"
     xmlns:xsd="http://www.w3.org/2001/XMLSchema#">
    <owl:Ontology rdf:about="http://example.org/ont"/>
    <owl:Class rdf:about="http://example.org/ont#A"/>
    <owl:ObjectProperty rdf:about="http://example.org/ont#p">
        <rdfs:domain rdf:resource="http://example.org/ont#A"/>
    </owl:ObjectProperty>
</rdf:RDF>"#;
        let (mut ont, _incomplete) = load_rdf_xml_ontology(source).expect("load");
        let mut namespaces = BTreeMap::new();
        namespaces.insert("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into());
        apply_patches_to_ontology_with_ns(
            &mut ont,
            &[
                PatchOp::AddAxiomAnnotation {
                    axiom_op: "domain".into(),
                    subject_iri: "http://example.org/ont#p".into(),
                    related_iri: Some("http://example.org/ont#A".into()),
                    predicate: "http://www.w3.org/2000/01/rdf-schema#comment".into(),
                    value: "domain note".into(),
                },
                PatchOp::AddDatatypeDefinition {
                    datatype_iri: "http://example.org/ont#NonNegInt".into(),
                    manchester: "xsd:integer[>= 0]".into(),
                },
            ],
            &namespaces,
        )
        .expect("apply");
        assert!(
            ont.i()
                .component_for_kind(ComponentKind::ObjectPropertyDomain)
                .any(|ac| !ac.ann.is_empty()),
            "domain axiom should carry annotation"
        );
        assert_eq!(ont.i().datatype_definition().count(), 1);
        let def = ont.i().datatype_definition().next().expect("def");
        assert!(matches!(def.range, DataRange::DatatypeRestriction(_, _)));
    }
}
