use crate::change::SemanticChange;
use crate::error::{EditError, Result};
use ontocore_obo::OboPatchOp;
use ontocore_owl::PatchOp;

pub fn invert_patch_op(op: &PatchOp) -> Result<PatchOp> {
    Ok(match op {
        PatchOp::AddPrefix { prefix, namespace_iri: _ } => PatchOp::RemovePrefix { prefix: prefix.clone() },
        PatchOp::RemovePrefix { prefix } => PatchOp::AddPrefix {
            prefix: prefix.clone(),
            namespace_iri: String::new(),
        },
        PatchOp::SetPrefix { .. } => {
            return Err(EditError::NotInvertible("set_prefix requires prior value".into()));
        }
        PatchOp::SetOntologyIri { .. } | PatchOp::SetVersionIri { .. } => {
            return Err(EditError::NotInvertible("ontology IRI set requires prior value".into()));
        }
        PatchOp::AddOntologyAnnotation { ontology_iri, predicate, value } => {
            PatchOp::RemoveOntologyAnnotation {
                ontology_iri: ontology_iri.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
            }
        }
        PatchOp::RemoveOntologyAnnotation { ontology_iri, predicate, value } => {
            PatchOp::AddOntologyAnnotation {
                ontology_iri: ontology_iri.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
            }
        }
        PatchOp::CreateEntity { entity_iri, kind: _ } => PatchOp::DeleteEntity { entity_iri: entity_iri.clone() },
        PatchOp::DeleteEntity { entity_iri: _ } => {
            return Err(EditError::NotInvertible(
                "delete_entity inverse requires entity kind".into(),
            ));
        }
        PatchOp::SetLabel { .. } | PatchOp::SetComment { .. } | PatchOp::SetEquivalentClass { .. } => {
            return Err(EditError::NotInvertible("set_* requires prior value".into()));
        }
        PatchOp::AddLabel { entity_iri, value } => PatchOp::RemoveLabel {
            entity_iri: entity_iri.clone(),
            value: value.clone(),
        },
        PatchOp::RemoveLabel { entity_iri, value } => PatchOp::AddLabel {
            entity_iri: entity_iri.clone(),
            value: value.clone(),
        },
        PatchOp::AddComment { entity_iri, value } => PatchOp::RemoveComment {
            entity_iri: entity_iri.clone(),
            value: value.clone(),
        },
        PatchOp::RemoveComment { entity_iri, value } => PatchOp::AddComment {
            entity_iri: entity_iri.clone(),
            value: value.clone(),
        },
        PatchOp::AddSubClassOf { entity_iri, parent_iri } => PatchOp::RemoveSubClassOf {
            entity_iri: entity_iri.clone(),
            parent_iri: parent_iri.clone(),
        },
        PatchOp::RemoveSubClassOf { entity_iri, parent_iri } => PatchOp::AddSubClassOf {
            entity_iri: entity_iri.clone(),
            parent_iri: parent_iri.clone(),
        },
        PatchOp::AddComplexSubClassOf { entity_iri, manchester } => PatchOp::RemoveComplexSubClassOf {
            entity_iri: entity_iri.clone(),
            manchester: manchester.clone(),
        },
        PatchOp::RemoveComplexSubClassOf { entity_iri, manchester } => PatchOp::AddComplexSubClassOf {
            entity_iri: entity_iri.clone(),
            manchester: manchester.clone(),
        },
        PatchOp::AddEquivalentClass { entity_iri, manchester } => PatchOp::RemoveEquivalentClass {
            entity_iri: entity_iri.clone(),
            manchester: manchester.clone(),
        },
        PatchOp::RemoveEquivalentClass { entity_iri, manchester } => PatchOp::AddEquivalentClass {
            entity_iri: entity_iri.clone(),
            manchester: manchester.clone(),
        },
        PatchOp::SetDeprecated { entity_iri, value } => PatchOp::SetDeprecated {
            entity_iri: entity_iri.clone(),
            value: !value,
        },
        PatchOp::AddDisjointClass { entity_iri, other_iri } => PatchOp::RemoveDisjointClass {
            entity_iri: entity_iri.clone(),
            other_iri: other_iri.clone(),
        },
        PatchOp::RemoveDisjointClass { entity_iri, other_iri } => PatchOp::AddDisjointClass {
            entity_iri: entity_iri.clone(),
            other_iri: other_iri.clone(),
        },
        PatchOp::AddImport { ontology_iri, import_iri } => PatchOp::RemoveImport {
            ontology_iri: ontology_iri.clone(),
            import_iri: import_iri.clone(),
        },
        PatchOp::RemoveImport { ontology_iri, import_iri } => PatchOp::AddImport {
            ontology_iri: ontology_iri.clone(),
            import_iri: import_iri.clone(),
        },
        PatchOp::AddDomain { entity_iri, class_iri } => PatchOp::RemoveDomain {
            entity_iri: entity_iri.clone(),
            class_iri: class_iri.clone(),
        },
        PatchOp::RemoveDomain { entity_iri, class_iri } => PatchOp::AddDomain {
            entity_iri: entity_iri.clone(),
            class_iri: class_iri.clone(),
        },
        PatchOp::AddRange { entity_iri, range_iri } => PatchOp::RemoveRange {
            entity_iri: entity_iri.clone(),
            range_iri: range_iri.clone(),
        },
        PatchOp::RemoveRange { entity_iri, range_iri } => PatchOp::AddRange {
            entity_iri: entity_iri.clone(),
            range_iri: range_iri.clone(),
        },
        PatchOp::SetFunctional { .. }
        | PatchOp::SetInverseFunctional { .. }
        | PatchOp::SetTransitive { .. }
        | PatchOp::SetSymmetric { .. }
        | PatchOp::SetAsymmetric { .. }
        | PatchOp::SetReflexive { .. }
        | PatchOp::SetIrreflexive { .. } => match op {
            PatchOp::SetFunctional { entity_iri, value } => PatchOp::SetFunctional {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            PatchOp::SetInverseFunctional { entity_iri, value } => PatchOp::SetInverseFunctional {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            PatchOp::SetTransitive { entity_iri, value } => PatchOp::SetTransitive {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            PatchOp::SetSymmetric { entity_iri, value } => PatchOp::SetSymmetric {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            PatchOp::SetAsymmetric { entity_iri, value } => PatchOp::SetAsymmetric {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            PatchOp::SetReflexive { entity_iri, value } => PatchOp::SetReflexive {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            PatchOp::SetIrreflexive { entity_iri, value } => PatchOp::SetIrreflexive {
                entity_iri: entity_iri.clone(),
                value: !value,
            },
            _ => unreachable!(),
        },
        PatchOp::AddPropertyChain { entity_iri, properties } => PatchOp::RemovePropertyChain {
            entity_iri: entity_iri.clone(),
            properties: properties.clone(),
        },
        PatchOp::RemovePropertyChain { entity_iri, properties } => PatchOp::AddPropertyChain {
            entity_iri: entity_iri.clone(),
            properties: properties.clone(),
        },
        PatchOp::AddClassAssertion { entity_iri, class_iri } => PatchOp::RemoveClassAssertion {
            entity_iri: entity_iri.clone(),
            class_iri: class_iri.clone(),
        },
        PatchOp::RemoveClassAssertion { entity_iri, class_iri } => PatchOp::AddClassAssertion {
            entity_iri: entity_iri.clone(),
            class_iri: class_iri.clone(),
        },
        PatchOp::AddObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            PatchOp::RemoveObjectPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                target_iri: target_iri.clone(),
            }
        }
        PatchOp::RemoveObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            PatchOp::AddObjectPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                target_iri: target_iri.clone(),
            }
        }
        PatchOp::AddDataPropertyAssertion { entity_iri, property_iri, value } => {
            PatchOp::RemoveDataPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                value: value.clone(),
            }
        }
        PatchOp::RemoveDataPropertyAssertion { entity_iri, property_iri, value } => {
            PatchOp::AddDataPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                value: value.clone(),
            }
        }
        PatchOp::AddAnnotation { entity_iri, predicate, value } => PatchOp::RemoveAnnotation {
            entity_iri: entity_iri.clone(),
            predicate: predicate.clone(),
            value: value.clone(),
        },
        PatchOp::RemoveAnnotation { entity_iri, predicate, value } => PatchOp::AddAnnotation {
            entity_iri: entity_iri.clone(),
            predicate: predicate.clone(),
            value: value.clone(),
        },
    })
}

pub fn invert_obo_patch_op(op: &OboPatchOp) -> Result<OboPatchOp> {
    Ok(match op {
        OboPatchOp::SetName { .. } | OboPatchOp::SetNamespace { .. } => {
            return Err(EditError::NotInvertible("obo set_* requires prior value".into()));
        }
        OboPatchOp::AddSynonym { term_id, value, scope: _ } => OboPatchOp::RemoveSynonym {
            term_id: term_id.clone(),
            value: value.clone(),
        },
        OboPatchOp::RemoveSynonym { term_id, value } => OboPatchOp::AddSynonym {
            term_id: term_id.clone(),
            value: value.clone(),
            scope: "exact".into(),
        },
        OboPatchOp::AddDef { term_id, value: _ } => OboPatchOp::RemoveDef { term_id: term_id.clone() },
        OboPatchOp::RemoveDef { term_id: _ } => {
            return Err(EditError::NotInvertible("remove_def inverse requires prior def".into()));
        }
        OboPatchOp::AddXref { term_id, xref } => OboPatchOp::RemoveXref {
            term_id: term_id.clone(),
            xref: xref.clone(),
        },
        OboPatchOp::RemoveXref { term_id, xref } => OboPatchOp::AddXref {
            term_id: term_id.clone(),
            xref: xref.clone(),
        },
        OboPatchOp::SetDeprecated { term_id, value } => OboPatchOp::SetDeprecated {
            term_id: term_id.clone(),
            value: !value,
        },
        OboPatchOp::AddIsA { term_id, parent_id } => OboPatchOp::RemoveIsA {
            term_id: term_id.clone(),
            parent_id: parent_id.clone(),
        },
        OboPatchOp::RemoveIsA { term_id, parent_id } => OboPatchOp::AddIsA {
            term_id: term_id.clone(),
            parent_id: parent_id.clone(),
        },
    })
}

pub fn invert_change(change: &SemanticChange) -> Result<SemanticChange> {
    Ok(match change {
        SemanticChange::Turtle { change } => SemanticChange::Turtle { change: invert_patch_op(change)? },
        SemanticChange::Obo { change } => SemanticChange::Obo { change: invert_obo_patch_op(change)? },
    })
}
