//! Remap an entity IRI through a Horned component-mapped ontology (v0.24 multi-format refactor).

use crate::error::{OwlError, Result};
use crate::serialize::{
    load_owl_xml_ontology, load_rdf_xml_ontology, serialize_owl_xml, serialize_rdf_xml,
};
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue, Build,
    Class, ClassAssertion, ClassExpression, Component, DataPropertyAssertion, DataRange,
    DeclareClass, DeclareDataProperty, DeclareNamedIndividual, DeclareObjectProperty, Individual,
    MutableOntology, NamedIndividual, ObjectPropertyAssertion, ObjectPropertyExpression,
    RcAnnotatedComponent, RcStr, SubClassOf,
};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use std::collections::BTreeMap;

/// Remap every occurrence of `from_iri` to `to_iri` in `ont`.
///
/// Returns the number of annotated components rewritten.
pub fn remap_entity_iri(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    from_iri: &str,
    to_iri: &str,
) -> Result<usize> {
    if from_iri == to_iri {
        return Ok(0);
    }
    let build = Build::new_rc();
    let components: Vec<_> = ont.i().iter().cloned().collect();
    let mut rewritten = 0usize;
    for ac in components {
        let remapped = remap_annotated(&ac, from_iri, to_iri, &build);
        if remapped.component != ac.component || remapped.ann != ac.ann {
            let _ = ont.take(&ac);
            ont.insert(remapped);
            rewritten += 1;
        }
    }
    Ok(rewritten)
}

/// Load RDF/XML or OWL/XML text, remap an entity IRI, and re-serialize.
///
/// `format` is `"rdfxml"` or `"owlxml"`. Incomplete RDF/XML loads return an error
/// (same contract as patch XML write-back).
pub fn remap_entity_iri_in_xml_text(
    source: &str,
    format: &str,
    from_iri: &str,
    to_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    match format {
        "rdfxml" | "rdf" | "owl" => {
            let (mut ont, incomplete) = load_rdf_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "RDF/XML parse incomplete; refusing IRI remap write-back".into(),
                ));
            }
            remap_entity_iri(&mut ont, from_iri, to_iri)?;
            serialize_rdf_xml(&ont)
        }
        "owlxml" | "owx" => {
            let (mut ont, mut ns, incomplete) = load_owl_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "OWL/XML parse incomplete; refusing IRI remap write-back".into(),
                ));
            }
            for (k, v) in namespaces {
                ns.entry(k.clone()).or_insert_with(|| v.clone());
            }
            remap_entity_iri(&mut ont, from_iri, to_iri)?;
            serialize_owl_xml(&ont, &ns)
        }
        other => Err(OwlError::SerializeFailed(format!("unsupported XML format: {other}"))),
    }
}

fn remap_annotated(
    ac: &AnnotatedComponent<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> AnnotatedComponent<RcStr> {
    let component = remap_component(&ac.component, from, to, build);
    let ann = ac.ann.iter().map(|a| remap_annotation(a, from, to, build)).collect();
    AnnotatedComponent { component, ann }
}

fn remap_component(
    c: &Component<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Component<RcStr> {
    match c {
        Component::DeclareClass(DeclareClass(cls)) => {
            Component::DeclareClass(DeclareClass(remap_class(cls, from, to, build)))
        }
        Component::DeclareObjectProperty(d) => Component::DeclareObjectProperty(
            DeclareObjectProperty(remap_object_property(&d.0, from, to, build)),
        ),
        Component::DeclareDataProperty(d) => Component::DeclareDataProperty(DeclareDataProperty(
            remap_data_property(&d.0, from, to, build),
        )),
        Component::DeclareNamedIndividual(d) => Component::DeclareNamedIndividual(
            DeclareNamedIndividual(remap_named_individual(&d.0, from, to, build)),
        ),
        Component::SubClassOf(ax) => Component::SubClassOf(SubClassOf {
            sub: remap_class_expression(&ax.sub, from, to, build),
            sup: remap_class_expression(&ax.sup, from, to, build),
        }),
        Component::EquivalentClasses(ax) => {
            Component::EquivalentClasses(horned_owl::model::EquivalentClasses(
                ax.0.iter().map(|ce| remap_class_expression(ce, from, to, build)).collect(),
            ))
        }
        Component::DisjointClasses(ax) => {
            Component::DisjointClasses(horned_owl::model::DisjointClasses(
                ax.0.iter().map(|ce| remap_class_expression(ce, from, to, build)).collect(),
            ))
        }
        Component::ClassAssertion(ax) => Component::ClassAssertion(ClassAssertion {
            ce: remap_class_expression(&ax.ce, from, to, build),
            i: remap_individual(&ax.i, from, to, build),
        }),
        Component::ObjectPropertyAssertion(ax) => {
            Component::ObjectPropertyAssertion(ObjectPropertyAssertion {
                ope: remap_ope(&ax.ope, from, to, build),
                from: remap_individual(&ax.from, from, to, build),
                to: remap_individual(&ax.to, from, to, build),
            })
        }
        Component::DataPropertyAssertion(ax) => {
            Component::DataPropertyAssertion(DataPropertyAssertion {
                dp: remap_data_property(&ax.dp, from, to, build),
                from: remap_individual(&ax.from, from, to, build),
                to: ax.to.clone(),
            })
        }
        Component::AnnotationAssertion(ax) => Component::AnnotationAssertion(AnnotationAssertion {
            subject: remap_annotation_subject(&ax.subject, from, to, build),
            ann: remap_annotation(&ax.ann, from, to, build),
        }),
        Component::ObjectPropertyDomain(ax) => {
            Component::ObjectPropertyDomain(horned_owl::model::ObjectPropertyDomain {
                ope: remap_ope(&ax.ope, from, to, build),
                ce: remap_class_expression(&ax.ce, from, to, build),
            })
        }
        Component::ObjectPropertyRange(ax) => {
            Component::ObjectPropertyRange(horned_owl::model::ObjectPropertyRange {
                ope: remap_ope(&ax.ope, from, to, build),
                ce: remap_class_expression(&ax.ce, from, to, build),
            })
        }
        Component::DataPropertyDomain(ax) => {
            Component::DataPropertyDomain(horned_owl::model::DataPropertyDomain {
                dp: remap_data_property(&ax.dp, from, to, build),
                ce: remap_class_expression(&ax.ce, from, to, build),
            })
        }
        Component::DataPropertyRange(ax) => {
            Component::DataPropertyRange(horned_owl::model::DataPropertyRange {
                dp: remap_data_property(&ax.dp, from, to, build),
                dr: remap_data_range(&ax.dr, from, to, build),
            })
        }
        Component::SubObjectPropertyOf(ax) => {
            Component::SubObjectPropertyOf(horned_owl::model::SubObjectPropertyOf {
                sub: match &ax.sub {
                    horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(chain) => {
                        horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(
                            chain.iter().map(|ope| remap_ope(ope, from, to, build)).collect(),
                        )
                    }
                    horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(
                        ope,
                    ) => horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(
                        remap_ope(ope, from, to, build),
                    ),
                },
                sup: remap_ope(&ax.sup, from, to, build),
            })
        }
        Component::Import(imp) => {
            let iri = imp.0.to_string();
            if iri == from {
                Component::Import(horned_owl::model::Import(build.iri(to)))
            } else {
                c.clone()
            }
        }
        other => other.clone(),
    }
}

fn remap_annotation(
    ann: &Annotation<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Annotation<RcStr> {
    Annotation {
        ap: remap_annotation_property(&ann.ap, from, to, build),
        av: match &ann.av {
            AnnotationValue::IRI(iri) if iri.to_string() == from => {
                AnnotationValue::IRI(build.iri(to))
            }
            other => other.clone(),
        },
    }
}

fn remap_annotation_subject(
    subject: &AnnotationSubject<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> AnnotationSubject<RcStr> {
    match subject {
        AnnotationSubject::IRI(iri) if iri.to_string() == from => {
            AnnotationSubject::IRI(build.iri(to))
        }
        other => other.clone(),
    }
}

fn remap_annotation_property(
    ap: &horned_owl::model::AnnotationProperty<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::AnnotationProperty<RcStr> {
    if ap.to_string() == from {
        build.annotation_property(to)
    } else {
        ap.clone()
    }
}

fn remap_class(cls: &Class<RcStr>, from: &str, to: &str, build: &Build<RcStr>) -> Class<RcStr> {
    if cls.0.to_string() == from {
        build.class(to)
    } else {
        cls.clone()
    }
}

fn remap_object_property(
    p: &horned_owl::model::ObjectProperty<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::ObjectProperty<RcStr> {
    if p.to_string() == from {
        build.object_property(to)
    } else {
        p.clone()
    }
}

fn remap_data_property(
    p: &horned_owl::model::DataProperty<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::DataProperty<RcStr> {
    if p.to_string() == from {
        build.data_property(to)
    } else {
        p.clone()
    }
}

fn remap_named_individual(
    i: &NamedIndividual<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> NamedIndividual<RcStr> {
    if i.0.to_string() == from {
        build.named_individual(to)
    } else {
        i.clone()
    }
}

fn remap_individual(
    i: &Individual<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Individual<RcStr> {
    match i {
        Individual::Named(n) => Individual::Named(remap_named_individual(n, from, to, build)),
        other => other.clone(),
    }
}

fn remap_ope(
    ope: &ObjectPropertyExpression<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> ObjectPropertyExpression<RcStr> {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => {
            ObjectPropertyExpression::ObjectProperty(remap_object_property(p, from, to, build))
        }
        ObjectPropertyExpression::InverseObjectProperty(p) => {
            ObjectPropertyExpression::InverseObjectProperty(remap_object_property(
                p, from, to, build,
            ))
        }
    }
}

fn remap_class_expression(
    ce: &ClassExpression<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> ClassExpression<RcStr> {
    match ce {
        ClassExpression::Class(c) => ClassExpression::Class(remap_class(c, from, to, build)),
        ClassExpression::ObjectIntersectionOf(v) => ClassExpression::ObjectIntersectionOf(
            v.iter().map(|e| remap_class_expression(e, from, to, build)).collect(),
        ),
        ClassExpression::ObjectUnionOf(v) => ClassExpression::ObjectUnionOf(
            v.iter().map(|e| remap_class_expression(e, from, to, build)).collect(),
        ),
        ClassExpression::ObjectComplementOf(inner) => ClassExpression::ObjectComplementOf(
            Box::new(remap_class_expression(inner, from, to, build)),
        ),
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            ClassExpression::ObjectSomeValuesFrom {
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => ClassExpression::ObjectAllValuesFrom {
            ope: remap_ope(ope, from, to, build),
            bce: Box::new(remap_class_expression(bce, from, to, build)),
        },
        ClassExpression::ObjectHasValue { ope, i } => ClassExpression::ObjectHasValue {
            ope: remap_ope(ope, from, to, build),
            i: remap_individual(i, from, to, build),
        },
        ClassExpression::ObjectHasSelf(ope) => {
            ClassExpression::ObjectHasSelf(remap_ope(ope, from, to, build))
        }
        ClassExpression::ObjectOneOf(inds) => ClassExpression::ObjectOneOf(
            inds.iter().map(|i| remap_individual(i, from, to, build)).collect(),
        ),
        ClassExpression::ObjectMinCardinality { n, ope, bce } => {
            ClassExpression::ObjectMinCardinality {
                n: *n,
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
            ClassExpression::ObjectMaxCardinality {
                n: *n,
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::ObjectExactCardinality { n, ope, bce } => {
            ClassExpression::ObjectExactCardinality {
                n: *n,
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::DataSomeValuesFrom { dp, dr } => ClassExpression::DataSomeValuesFrom {
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataAllValuesFrom { dp, dr } => ClassExpression::DataAllValuesFrom {
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataHasValue { dp, l } => ClassExpression::DataHasValue {
            dp: remap_data_property(dp, from, to, build),
            l: l.clone(),
        },
        ClassExpression::DataMinCardinality { n, dp, dr } => ClassExpression::DataMinCardinality {
            n: *n,
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataMaxCardinality { n, dp, dr } => ClassExpression::DataMaxCardinality {
            n: *n,
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataExactCardinality { n, dp, dr } => {
            ClassExpression::DataExactCardinality {
                n: *n,
                dp: remap_data_property(dp, from, to, build),
                dr: remap_data_range(dr, from, to, build),
            }
        }
    }
}

fn remap_data_range(
    dr: &DataRange<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> DataRange<RcStr> {
    match dr {
        DataRange::Datatype(dt) => {
            if dt.to_string() == from {
                DataRange::Datatype(build.datatype(to))
            } else {
                dr.clone()
            }
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaps_class_in_rdf_xml() {
        let src = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
  <owl:Class rdf:about="http://example.org#Person"/>
  <owl:Class rdf:about="http://example.org#Patient">
    <rdfs:subClassOf rdf:resource="http://example.org#Person"/>
  </owl:Class>
</rdf:RDF>
"#;
        let out = remap_entity_iri_in_xml_text(
            src,
            "rdfxml",
            "http://example.org#Person",
            "http://example.org#Human",
            &BTreeMap::new(),
        )
        .expect("remap");
        assert!(out.contains("http://example.org#Human"), "{out}");
        assert!(!out.contains("http://example.org#Person"), "{out}");
        assert!(out.contains("http://example.org#Patient"), "{out}");
    }
}
