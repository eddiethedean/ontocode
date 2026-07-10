use crate::error::{ReasonerError, Result};
use crate::result::{ExplanationResult, ExplanationStep};
use ontologos_core::{
    EntityId, EntityKind, InferenceTrace, Ontology, TraceConclusion, TracePremise, TraceStep,
};
use ontologos_dl::DlClassifier;
use ontologos_el::ElClassifier;
use ontologos_explain::{build_proof_graph, render_text, ProofGraph};
use ontologos_rl::rdfs::RdfsEngine;
use ontologos_rl::RlEngine;
use std::collections::{HashSet, VecDeque};

pub fn explain_unsatisfiable_alternatives(
    profile: crate::adapter::ReasonerId,
    ontology: &Ontology,
    class_iri: &str,
    max_justifications: usize,
) -> Result<Vec<ExplanationResult>> {
    let max = max_justifications.clamp(1, 8);
    let mut results = match profile {
        crate::adapter::ReasonerId::El => {
            explain_unsatisfiable_el_alternatives(ontology, class_iri, max)?
        }
        crate::adapter::ReasonerId::Rl => vec![explain_unsatisfiable_rl(ontology, class_iri)?],
        crate::adapter::ReasonerId::Rdfs => vec![explain_unsatisfiable_rdfs(ontology, class_iri)?],
        crate::adapter::ReasonerId::Dl => vec![explain_unsatisfiable_dl(ontology, class_iri)?],
        crate::adapter::ReasonerId::Auto => {
            // Match CLI AutoAdapter::explain: use the concrete engine Auto classify selects.
            let concrete = crate::auto::resolve_auto_reasoner_id(ontology)
                .map_err(|e| ReasonerError::Explain(e.to_string()))?;
            return explain_unsatisfiable_alternatives(concrete, ontology, class_iri, max);
        }
    };

    // Basic de-duplication by rendered text.
    let mut seen = std::collections::HashSet::new();
    results.retain(|r| seen.insert(r.text.clone()));
    if results.is_empty() {
        return Err(ReasonerError::ExplanationUnavailable(class_iri.to_string()));
    }
    Ok(results)
}

pub fn explain_unsatisfiable_el(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;

    let report = ElClassifier::new()
        .classify_with_options(ontology, true)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;

    let bottom = find_bottom_subsumption(ontology, class_id, &report.trace)
        .ok_or_else(|| ReasonerError::ExplanationUnavailable(class_iri.to_string()))?;

    let graph = explain_unsatisfiable_trace(ontology, class_id, bottom, &report.trace)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;
    map_proof_graph(ontology, class_iri, graph)
}

pub fn explain_unsatisfiable_el_alternatives(
    ontology: &Ontology,
    class_iri: &str,
    max: usize,
) -> Result<Vec<ExplanationResult>> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;

    let report = ElClassifier::new()
        .classify_with_options(ontology, true)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;

    let bottom = find_bottom_subsumption(ontology, class_id, &report.trace)
        .ok_or_else(|| ReasonerError::ExplanationUnavailable(class_iri.to_string()))?;

    let target_idxs: Vec<usize> = report
        .trace
        .steps
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            if conclusion_matches_subsumption(ontology, &s.conclusion, class_id, bottom) {
                Some(idx)
            } else {
                None
            }
        })
        .take(max)
        .collect();

    let mut out = Vec::new();
    for idx in target_idxs {
        let subgraph = InferenceTrace { steps: hst_prune(&report.trace, idx) };
        let graph = build_proof_graph(ontology, &subgraph)
            .map_err(|e| ReasonerError::Explain(e.to_string()))?;
        out.push(map_proof_graph(ontology, class_iri, graph)?);
    }
    if out.is_empty() {
        out.push(explain_unsatisfiable_el(ontology, class_iri)?);
    }
    Ok(out)
}

pub fn explain_unsatisfiable_rl(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    let mut mutable = ontology.clone();
    let trace = RlEngine::try_new(1)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?
        .with_traces(true)
        .saturate(&mut mutable)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?
        .trace;
    let bottom = find_bottom_subsumption(&mutable, class_id, &trace)
        .ok_or_else(|| ReasonerError::ExplanationUnavailable(class_iri.to_string()))?;
    let graph = explain_unsatisfiable_trace(&mutable, class_id, bottom, &trace)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;
    map_proof_graph(ontology, class_iri, graph)
}

pub fn explain_unsatisfiable_rdfs(
    ontology: &Ontology,
    class_iri: &str,
) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    let mut mutable = ontology.clone();
    let trace = RdfsEngine::new()
        .with_traces(true)
        .materialize(&mut mutable)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?
        .trace;
    let bottom = find_bottom_subsumption(&mutable, class_id, &trace)
        .ok_or_else(|| ReasonerError::ExplanationUnavailable(class_iri.to_string()))?;
    let graph = explain_unsatisfiable_trace(&mutable, class_id, bottom, &trace)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;
    map_proof_graph(ontology, class_iri, graph)
}

fn explain_unsatisfiable_trace(
    ontology: &Ontology,
    class: EntityId,
    bottom: EntityId,
    trace: &InferenceTrace,
) -> std::result::Result<ProofGraph, ontologos_explain::Error> {
    let subgraph = minimal_subsumption_trace(ontology, trace, class, bottom)?;
    build_proof_graph(ontology, &subgraph)
}

fn minimal_subsumption_trace(
    ontology: &Ontology,
    trace: &InferenceTrace,
    sub: EntityId,
    sup: EntityId,
) -> std::result::Result<InferenceTrace, ontologos_explain::Error> {
    let step_idx = trace
        .steps
        .iter()
        .position(|s| conclusion_matches_subsumption(ontology, &s.conclusion, sub, sup))
        .ok_or(ontologos_explain::Error::Core(ontologos_core::Error::Message(format!(
            "no inference step concludes {sub:?} ⊑ {sup:?}"
        ))))?;

    Ok(InferenceTrace { steps: hst_prune(trace, step_idx) })
}

fn hst_prune(trace: &InferenceTrace, target_idx: usize) -> Vec<TraceStep> {
    let mut needed = HashSet::new();
    let mut queue = VecDeque::from([target_idx]);

    while let Some(idx) = queue.pop_front() {
        if !needed.insert(idx) {
            continue;
        }
        let step = &trace.steps[idx];
        if step.premises.is_empty() {
            continue;
        }
        if let Some(premise_idx) = find_premise_step(trace, &step.premises[0]) {
            queue.push_back(premise_idx);
        }
        for premise in step.premises.iter().skip(1) {
            if let Some(premise_idx) = find_premise_step(trace, premise) {
                queue.push_back(premise_idx);
            }
        }
    }

    let mut ordered = needed.into_iter().collect::<Vec<_>>();
    ordered.sort_unstable();
    ordered.into_iter().map(|idx| trace.steps[idx].clone()).collect()
}

fn find_premise_step(trace: &InferenceTrace, premise: &TracePremise) -> Option<usize> {
    trace.steps.iter().position(|step| match (&step.conclusion, premise) {
        (
            TraceConclusion::SubClassOf { sub, sup },
            TracePremise::SubClassOf { sub: psub, sup: psup },
        ) => sub == psub && sup == psup,
        (TraceConclusion::Axiom { id }, TracePremise::Axiom { id: pid }) => id == pid,
        _ => false,
    })
}

fn conclusion_matches_subsumption(
    ontology: &Ontology,
    conclusion: &TraceConclusion,
    sub: EntityId,
    sup: EntityId,
) -> bool {
    match conclusion {
        TraceConclusion::SubClassOf { sub: s, sup: p } => *s == sub && *p == sup,
        TraceConclusion::Axiom { id } => ontology.axiom(*id).ok().is_some_and(|axiom| {
            matches!(
                axiom,
                ontologos_core::Axiom::SubClassOf { subclass, superclass }
                    if *subclass == sub && *superclass == sup
            )
        }),
        _ => false,
    }
}

fn find_bottom_subsumption(
    ontology: &Ontology,
    class: EntityId,
    trace: &InferenceTrace,
) -> Option<EntityId> {
    trace.steps.iter().find_map(|step| {
        let TraceConclusion::SubClassOf { sub, sup } = step.conclusion else {
            return None;
        };
        if sub != class {
            return None;
        }
        let record = ontology.entity(sup).ok()?;
        if record.kind != EntityKind::Class {
            return None;
        }
        let iri = ontology.resolve_iri(record.iri).ok()?;
        if is_owl_nothing(iri) {
            Some(sup)
        } else {
            None
        }
    })
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

fn entity_iri_opt(ontology: &Ontology, id: EntityId) -> Option<String> {
    let entity = ontology.entity(id).ok()?;
    ontology.resolve_iri(entity.iri).ok().map(|s| s.to_string())
}

/// True for the OWL bottom class (`owl:Nothing`), not IRIs that merely contain "Nothing".
fn is_owl_nothing(iri: &str) -> bool {
    iri == "http://www.w3.org/2002/07/owl#Nothing" || iri == "owl:Nothing"
}

pub fn explain_unsatisfiable_dl(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let taxonomy = DlClassifier::new()
        .classify(ontology)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    if !taxonomy.unsatisfiable.contains(&class_id) {
        return Err(ReasonerError::ExplanationUnavailable(class_iri.to_string()));
    }
    let mut result = explain_unsatisfiable_el(ontology, class_iri)
        .or_else(|_| explain_unsatisfiable_rl(ontology, class_iri))
        .or_else(|_| explain_unsatisfiable_rdfs(ontology, class_iri))?;
    result.text = format!("DL profile justification (via Ontologos traces)\n\n{}", result.text);
    Ok(result)
}
