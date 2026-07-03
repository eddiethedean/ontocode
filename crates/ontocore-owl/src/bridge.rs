use crate::manchester::class_expression_to_manchester;
use crate::span::{annotate_spans, find_entity_block, short_name_from_iri};
use horned_owl::io::rdf::reader::ConcreteRDFOntology;
use horned_owl::model::{
    AnnotationSubject, AnnotationValue, ClassExpression, RcAnnotatedComponent, RcStr,
};
use horned_owl::model::{ObjectPropertyExpression, SubObjectPropertyExpression};
use horned_owl::ontology::component_mapped::{ComponentMappedIndex, ComponentMappedOntology};
use ontocore_core::{
    Annotation, Axiom, Entity, EntityKind, Import, Namespace, SourceLocation,
    AXIOM_KIND_DISJOINT_CLASS, AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS,
    AXIOM_KIND_PROPERTY_CHAIN, AXIOM_KIND_RANGE, AXIOM_KIND_SUB_CLASS_OF,
};
use std::collections::BTreeMap;

const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
const RDFS_SUB_CLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const OWL_DEPRECATED: &str = "http://www.w3.org/2002/07/owl#deprecated";

/// Catalog-shaped extraction from a Horned-OWL ontology.
#[derive(Debug, Clone, Default)]
pub struct OwlBridgeResult {
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub imports: Vec<Import>,
    pub namespace_rows: Vec<Namespace>,
    pub base_iri: Option<String>,
}

pub fn bridge_ontology(
    ontology: ConcreteRDFOntology<RcStr, RcAnnotatedComponent>,
    ontology_id: &str,
    source_text: &str,
    namespaces: &BTreeMap<String, String>,
) -> OwlBridgeResult {
    let mapped = ComponentMappedOntology::<RcStr, RcAnnotatedComponent>::from(ontology);
    let idx: &ComponentMappedIndex<RcStr, RcAnnotatedComponent> = mapped.i();

    let mut result = OwlBridgeResult::default();
    let ont_id = idx
        .the_ontology_id()
        .and_then(|id| id.iri.as_ref().map(|iri| iri.to_string()))
        .unwrap_or_else(|| ontology_id.to_string());
    result.base_iri = Some(ont_id.clone());

    let mut entity_map: BTreeMap<String, Entity> = BTreeMap::new();

    for decl in idx.declare_class() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::Class, &ont_id);
    }
    for decl in idx.declare_object_property() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::ObjectProperty, &ont_id);
    }
    for decl in idx.declare_data_property() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::DataProperty, &ont_id);
    }
    for decl in idx.declare_annotation_property() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::AnnotationProperty, &ont_id);
    }
    for decl in idx.declare_named_individual() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::Individual, &ont_id);
    }

    for ann_ax in idx.annotation_assertion() {
        let subject = annotation_subject_iri(&ann_ax.subject);
        let predicate = ann_ax.ann.ap.to_string();
        let object = annotation_value_string(&ann_ax.ann.av);
        if predicate == RDFS_LABEL {
            entity_map.entry(subject.clone()).and_modify(|e| e.labels.push(object.clone()));
        } else if predicate == RDFS_COMMENT {
            entity_map.entry(subject.clone()).and_modify(|e| e.comments.push(object.clone()));
        } else if predicate == OWL_DEPRECATED
            && ontocore_core::parse_boolean_literal(&object) == Some(true)
        {
            entity_map.entry(subject.clone()).and_modify(|e| e.deprecated = true);
        }
        result.annotations.push(Annotation {
            subject,
            predicate,
            object,
            ontology_id: ont_id.clone(),
            source_location: SourceLocation::default(),
        });
    }

    let mut axiom_counter = 0usize;
    for ax in idx.sub_class_of() {
        if let ClassExpression::Class(sub) = &ax.sub {
            if let Some(sup) = class_expr_display(&ax.sup, namespaces) {
                axiom_counter += 1;
                result.axioms.push(Axiom {
                    id: format!("{ontology_id}#axiom-{axiom_counter}"),
                    ontology_id: ont_id.clone(),
                    subject: sub.to_string(),
                    predicate: RDFS_SUB_CLASS_OF.to_string(),
                    object: sup,
                    axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
                    source_location: SourceLocation::default(),
                });
            }
        }
    }

    for eq in idx.equivalent_class() {
        for ce in &eq.0 {
            if let ClassExpression::Class(sub) = ce {
                for other in &eq.0 {
                    if other == ce {
                        continue;
                    }
                    if let Some(obj) = class_expr_display(other, namespaces) {
                        axiom_counter += 1;
                        result.axioms.push(Axiom {
                            id: format!("{ontology_id}#axiom-{axiom_counter}"),
                            ontology_id: ont_id.clone(),
                            subject: sub.to_string(),
                            predicate: "http://www.w3.org/2002/07/owl#equivalentClass".to_string(),
                            object: obj,
                            axiom_kind: AXIOM_KIND_EQUIVALENT_CLASS.to_string(),
                            source_location: SourceLocation::default(),
                        });
                    }
                }
            }
        }
    }

    for disj in idx.disjoint_class() {
        for ce in &disj.0 {
            if let ClassExpression::Class(sub) = ce {
                for other in &disj.0 {
                    if other == ce {
                        continue;
                    }
                    if let ClassExpression::Class(obj) = other {
                        axiom_counter += 1;
                        result.axioms.push(Axiom {
                            id: format!("{ontology_id}#axiom-{axiom_counter}"),
                            ontology_id: ont_id.clone(),
                            subject: sub.to_string(),
                            predicate: "http://www.w3.org/2002/07/owl#disjointWith".to_string(),
                            object: obj.to_string(),
                            axiom_kind: AXIOM_KIND_DISJOINT_CLASS.to_string(),
                            source_location: SourceLocation::default(),
                        });
                    } else if let Some(obj) = class_expr_display(other, namespaces) {
                        axiom_counter += 1;
                        result.axioms.push(Axiom {
                            id: format!("{ontology_id}#axiom-{axiom_counter}"),
                            ontology_id: ont_id.clone(),
                            subject: sub.to_string(),
                            predicate: "http://www.w3.org/2002/07/owl#disjointWith".to_string(),
                            object: obj,
                            axiom_kind: AXIOM_KIND_DISJOINT_CLASS.to_string(),
                            source_location: SourceLocation::default(),
                        });
                    }
                }
            }
        }
    }

    for dom in idx.object_property_domain() {
        let prop = ope_to_iri(&dom.ope);
        if let ClassExpression::Class(cls) = &dom.ce {
            push_class_binding_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                &ont_id,
                &prop,
                cls.to_string().as_str(),
                AXIOM_KIND_DOMAIN,
                "http://www.w3.org/2000/01/rdf-schema#domain",
            );
        }
    }
    for rng in idx.object_property_range() {
        let prop = ope_to_iri(&rng.ope);
        if let ClassExpression::Class(cls) = &rng.ce {
            push_class_binding_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                &ont_id,
                &prop,
                cls.to_string().as_str(),
                AXIOM_KIND_RANGE,
                "http://www.w3.org/2000/01/rdf-schema#range",
            );
        }
    }
    for dom in idx.data_property_domain() {
        let prop = dom.dp.to_string();
        if let ClassExpression::Class(cls) = &dom.ce {
            push_class_binding_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                &ont_id,
                &prop,
                cls.to_string().as_str(),
                AXIOM_KIND_DOMAIN,
                "http://www.w3.org/2000/01/rdf-schema#domain",
            );
        }
    }
    for rng in idx.data_property_range() {
        let prop = rng.dp.to_string();
        let filler = data_range_display(&rng.dr);
        push_class_binding_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            &ont_id,
            &prop,
            &filler,
            AXIOM_KIND_RANGE,
            "http://www.w3.org/2000/01/rdf-schema#range",
        );
    }

    for sub_prop in idx.sub_object_property_of() {
        if let SubObjectPropertyExpression::ObjectPropertyChain(chain) = &sub_prop.sub {
            let chain_display = chain.iter().map(ope_to_iri).collect::<Vec<_>>().join(" o ");
            axiom_counter += 1;
            result.axioms.push(Axiom {
                id: format!("{ontology_id}#axiom-{axiom_counter}"),
                ontology_id: ont_id.clone(),
                subject: ope_to_iri(&sub_prop.sup),
                predicate: "http://www.w3.org/2002/07/owl#propertyChainAxiom".to_string(),
                object: chain_display,
                axiom_kind: AXIOM_KIND_PROPERTY_CHAIN.to_string(),
                source_location: SourceLocation::default(),
            });
        }
    }

    for imp in idx.import() {
        result.imports.push(Import { ontology_id: ont_id.clone(), import_iri: imp.0.to_string() });
    }

    for (prefix, iri) in namespaces {
        result.namespace_rows.push(Namespace {
            prefix: prefix.clone(),
            iri: iri.clone(),
            ontology_id: ont_id.clone(),
        });
    }

    result.entities = entity_map.into_values().collect();
    for entity in &mut result.entities {
        entity.source_location =
            find_entity_block(source_text, &entity.iri, &entity.short_name, namespaces);
    }
    annotate_spans(source_text, &mut result.entities, &mut result.annotations, &mut result.axioms);
    result
}

fn insert_entity(
    map: &mut BTreeMap<String, Entity>,
    iri: String,
    kind: EntityKind,
    ontology_id: &str,
) {
    let short_name = short_name_from_iri(&iri);
    map.entry(iri.clone()).or_insert_with(|| Entity {
        iri,
        short_name,
        kind,
        ontology_id: ontology_id.to_string(),
        source_location: SourceLocation::default(),
        labels: Vec::new(),
        comments: Vec::new(),
        deprecated: false,
        obo_id: None,
    });
}

fn class_expr_display(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Option<String> {
    match expr {
        ClassExpression::Class(c) => Some(c.to_string()),
        _ => Some(class_expression_to_manchester(expr, namespaces)),
    }
}

fn ope_to_iri(ope: &ObjectPropertyExpression<RcStr>) -> String {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => p.to_string(),
        ObjectPropertyExpression::InverseObjectProperty(p) => {
            format!("inverse({})", p.0)
        }
    }
}

fn data_range_display(dr: &horned_owl::model::DataRange<RcStr>) -> String {
    match dr {
        horned_owl::model::DataRange::Datatype(dt) => dt.to_string(),
        other => format!("{other:?}"),
    }
}

fn push_class_binding_axiom(
    axioms: &mut Vec<Axiom>,
    counter: &mut usize,
    ontology_id: &str,
    subject: &str,
    object: &str,
    kind: &str,
    predicate: &str,
) {
    *counter += 1;
    axioms.push(Axiom {
        id: format!("{ontology_id}#axiom-{counter}"),
        ontology_id: ontology_id.to_string(),
        subject: subject.to_string(),
        predicate: predicate.to_string(),
        object: object.to_string(),
        axiom_kind: kind.to_string(),
        source_location: SourceLocation::default(),
    });
}

fn annotation_subject_iri(subject: &AnnotationSubject<RcStr>) -> String {
    match subject {
        AnnotationSubject::IRI(iri) => iri.to_string(),
        AnnotationSubject::AnonymousIndividual(a) => format!("_:{}", a.as_ref()),
    }
}

fn annotation_value_string(value: &AnnotationValue<RcStr>) -> String {
    match value {
        AnnotationValue::Literal(l) => l.literal().clone(),
        AnnotationValue::IRI(iri) => iri.to_string(),
        AnnotationValue::AnonymousIndividual(a) => format!("_:{}", a.as_ref()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load::load_turtle_text;
    use ontocore_core::{
        AXIOM_KIND_DISJOINT_CLASS, AXIOM_KIND_PROPERTY_CHAIN, AXIOM_KIND_SUB_CLASS_OF,
    };
    use oxigraph::io::RdfParser;
    use oxigraph::model::Quad;
    use std::path::Path;

    #[test]
    fn bridge_extracts_subclass_axioms() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let parser = RdfParser::from_format(oxigraph::io::RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces =
            BTreeMap::from([("ex".to_string(), "http://example.org/people#".to_string())]);
        let loaded = load_turtle_text(Path::new("example.ttl"), "doc-1", ttl, &quads, &namespaces)
            .expect("load");
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF));
    }

    #[test]
    fn bridge_extracts_disjoint_and_property_chain() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let parser = RdfParser::from_format(oxigraph::io::RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces =
            BTreeMap::from([("ex".to_string(), "http://example.org/org#".to_string())]);
        let loaded =
            load_turtle_text(Path::new("disjoint-classes.ttl"), "doc-1", ttl, &quads, &namespaces)
                .expect("load");
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_DISJOINT_CLASS));
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_PROPERTY_CHAIN));
    }
}
