//! Apply PatchOp mutations onto a Horned ontology (v0.21 XML write-back).

use crate::error::{OwlError, Result};
use crate::patch::{PatchDiagnostic, PatchEntityKind, PatchOp};
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue, Build,
    Class, ClassAssertion, ClassExpression, Component, DeclareClass, DeclareDataProperty,
    DeclareNamedIndividual, DeclareObjectProperty, Import, Individual, Literal, MutableOntology,
    ObjectPropertyExpression, OntologyID, RcAnnotatedComponent, RcStr, SubClassOf,
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
        .filter(|ax| {
            ope_mentions(&ax.ope, entity_iri) || class_expr_mentions(&ax.ce, entity_iri)
        })
        .cloned()
        .collect();
    for ax in domains {
        let _ = ont.take(&AnnotatedComponent::from(Component::ObjectPropertyDomain(ax)));
    }

    let ranges: Vec<_> = ont
        .i()
        .object_property_range()
        .filter(|ax| {
            ope_mentions(&ax.ope, entity_iri) || class_expr_mentions(&ax.ce, entity_iri)
        })
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
    ont.insert(Component::OntologyID(OntologyID {
        iri: Some(build.iri(ontology_iri)),
        viri,
    }));
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
            &[PatchOp::SetOntologyIri {
                ontology_iri: "http://example.org/ont-renamed".into(),
            }],
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
            &[PatchOp::DeleteEntity {
                entity_iri: "http://example.org/ont#A".into(),
            }],
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
                .filter(|ax| ax.0.iter().any(|ce| class_expr_mentions(ce, "http://example.org/ont#A")))
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
}
