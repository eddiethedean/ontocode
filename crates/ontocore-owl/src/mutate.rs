//! Apply PatchOp mutations onto a Horned ontology (v0.21 XML write-back).

use crate::error::{OwlError, Result};
use crate::patch::{PatchDiagnostic, PatchEntityKind, PatchOp};
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue, Build,
    Class, ClassAssertion, ClassExpression, Component, DeclareClass, DeclareDataProperty,
    DeclareNamedIndividual, DeclareObjectProperty, Import, Individual, Literal, MutableOntology,
    OntologyID, RcAnnotatedComponent, RcStr, SubClassOf,
};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;

const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";

/// Apply inspector-oriented patches to a Horned ontology.
///
/// Unsupported ops append error diagnostics and leave the ontology unchanged for that op.
pub fn apply_patches_to_ontology(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    patches: &[PatchOp],
) -> Result<Vec<PatchDiagnostic>> {
    let build = Build::new_rc();
    let mut diagnostics = Vec::new();
    for patch in patches {
        if let Err(msg) = apply_one(ont, &build, patch) {
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

fn apply_one(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
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
                PatchEntityKind::AnnotationProperty => {
                    return Err(format!(
                        "CreateEntity AnnotationProperty not supported for XML write-back yet: {entity_iri}"
                    ));
                }
                PatchEntityKind::Individual => Component::DeclareNamedIndividual(
                    DeclareNamedIndividual(build.named_individual(entity_iri.as_str())),
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
        other => Err(format!(
            "patch op {:?} not yet supported for RDF/XML or OWL/XML write-back",
            patch_op_name(other)
        )),
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

fn remove_entity_components(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) {
    // Remove declaration + annotation assertions + subclass axioms mentioning the entity.
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
    remove_annotation_assertions(ont, entity_iri, RDFS_LABEL, None);
    remove_annotation_assertions(ont, entity_iri, RDFS_COMMENT, None);
    let sco: Vec<_> = ont
        .i()
        .sub_class_of()
        .filter(|ax| {
            matches!(&ax.sub, ClassExpression::Class(Class(iri)) if iri.to_string() == entity_iri)
                || matches!(
                    &ax.sup,
                    ClassExpression::Class(Class(iri)) if iri.to_string() == entity_iri
                )
        })
        .cloned()
        .collect();
    for ax in sco {
        let _ = ont.take(&AnnotatedComponent::from(Component::SubClassOf(ax)));
    }
}

fn set_ontology_iri(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    build: &Build<RcStr>,
    ontology_iri: &str,
    version_iri: Option<&str>,
) {
    // Drop existing OntologyID components then insert the new one.
    if let Some(id) = ont.i().the_ontology_id() {
        let _ = ont.take(&AnnotatedComponent::from(Component::OntologyID(id)));
    }
    ont.insert(Component::OntologyID(OntologyID {
        iri: Some(build.iri(ontology_iri)),
        viri: version_iri.map(|v| build.iri(v)),
    }));
}
