use crate::error::{ReasonerError, Result};
use crate::result::{ExplanationResult, ExplanationStep};
use ontologos_core::{Ontology, Profile};
use ontologos_el::ElClassifier;
use ontologos_explain::{
    explain_rdfs, explain_rl, explain_unsatisfiable, find_bottom_subsumption, render_text,
    ProofGraph,
};

pub fn explain_unsatisfiable_el(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;

    let report = ElClassifier::new()
        .classify_with_options(ontology, true)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;

    let bottom = find_bottom_subsumption(ontology, class_id, &report.trace)
        .ok_or_else(|| ReasonerError::ExplanationUnavailable(class_iri.to_string()))?;

    let graph = explain_unsatisfiable(ontology, class_id, bottom, Profile::El, &report.trace)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;
    map_proof_graph(ontology, class_iri, graph)
}

pub fn explain_unsatisfiable_rl(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let _class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    let mut mutable = ontology.clone();
    let graph = explain_rl(&mut mutable).map_err(|e| ReasonerError::Explain(e.to_string()))?;
    map_proof_graph(ontology, class_iri, graph)
}

pub fn explain_unsatisfiable_rdfs(
    ontology: &Ontology,
    class_iri: &str,
) -> Result<ExplanationResult> {
    let _class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    let mut mutable = ontology.clone();
    let graph = explain_rdfs(&mut mutable).map_err(|e| ReasonerError::Explain(e.to_string()))?;
    map_proof_graph(ontology, class_iri, graph)
}

fn map_proof_graph(
    ontology: &Ontology,
    class_iri: &str,
    graph: ProofGraph,
) -> Result<ExplanationResult> {
    if graph.nodes.is_empty() {
        return Err(ReasonerError::ExplanationUnavailable(class_iri.to_string()));
    }

    let text = render_text(ontology, &graph);
    let mut steps = Vec::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        let (subject_iri, object_iri, display) = format_node(ontology, node);
        steps.push(ExplanationStep {
            index: index + 1,
            rule: node.rule.clone(),
            display,
            subject_iri,
            object_iri,
        });
    }

    Ok(ExplanationResult { class_iri: class_iri.to_string(), steps, text })
}

fn format_node(
    ontology: &Ontology,
    node: &ontologos_explain::ProofNode,
) -> (Option<String>, Option<String>, String) {
    if let Some((sub, sup)) = node.conclusion_sub {
        let sub_iri = entity_iri_opt(ontology, sub);
        let sup_iri = entity_iri_opt(ontology, sup);
        let display = match (&sub_iri, &sup_iri) {
            (Some(a), Some(b)) => format!("{a} SubClassOf {b}"),
            _ => node.rule.clone(),
        };
        return (sub_iri, sup_iri, display);
    }
    if let Some(id) = node.conclusion_axiom {
        if let Ok(axiom) = ontology.axiom(id) {
            return (None, None, format!("{axiom:?} ({})", node.rule));
        }
    }
    (None, None, node.rule.clone())
}

fn entity_iri_opt(ontology: &Ontology, id: ontologos_core::EntityId) -> Option<String> {
    let entity = ontology.entity(id).ok()?;
    ontology.resolve_iri(entity.iri).ok().map(|s| s.to_string())
}
