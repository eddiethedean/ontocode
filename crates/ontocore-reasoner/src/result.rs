use ontocore_catalog::{ClassHierarchy, SubclassEdge};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerWarning {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferredHierarchy {
    pub edges: Vec<SubclassEdge>,
    pub unsatisfiable: Vec<String>,
    pub combined: ClassHierarchy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub profile_used: String,
    /// `true` when no **named class** is unsatisfiable (⊑ `owl:Nothing`).
    /// Does not detect all ontology inconsistencies (e.g. some ABox clashes).
    pub consistent: bool,
    pub unsatisfiable: Vec<String>,
    pub inferred: InferredHierarchy,
    pub new_inferences: Vec<SubclassEdge>,
    pub warnings: Vec<ReasonerWarning>,
    pub duration_ms: u64,
    pub subsumption_count: usize,
    pub inferred_axiom_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyResult {
    /// Class-level consistency only (see [`ClassificationResult::consistent`]).
    pub consistent: bool,
    pub unsatisfiable: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerSnapshot {
    pub profile_used: String,
    pub consistent: bool,
    pub unsatisfiable: Vec<String>,
    pub inferred: InferredHierarchy,
    pub new_inferences: Vec<SubclassEdge>,
    pub warnings: Vec<ReasonerWarning>,
    pub duration_ms: u64,
    pub classified_at: u64,
}

impl From<ClassificationResult> for ReasonerSnapshot {
    fn from(result: ClassificationResult) -> Self {
        Self {
            profile_used: result.profile_used,
            consistent: result.consistent,
            unsatisfiable: result.unsatisfiable,
            inferred: result.inferred,
            new_inferences: result.new_inferences,
            warnings: result.warnings,
            duration_ms: result.duration_ms,
            classified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationRequest {
    pub class_iri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationStep {
    pub index: usize,
    pub rule: String,
    pub display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_iri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_iri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationResult {
    pub class_iri: String,
    pub steps: Vec<ExplanationStep>,
    pub text: String,
}

const OWL_NOTHING: &str = "http://www.w3.org/2002/07/owl#Nothing";

/// Expand taxonomy unsatisfiable IRIs to **named** classes that are ⊑ ⊥.
///
/// Ontologos sometimes reports only `owl:Nothing` itself. User-facing consistency
/// (see [`ClassificationResult::consistent`]) must reflect named classes that are
/// unsatisfiable via asserted/inferred ⊑ `owl:Nothing` (and descendants of any
/// unsatisfiable class).
pub fn expand_named_unsatisfiable(reported: &[String], hierarchy: &ClassHierarchy) -> Vec<String> {
    let mut unsat: BTreeSet<String> = reported.iter().cloned().collect();
    for edge in &hierarchy.edges {
        if edge.parent == OWL_NOTHING {
            unsat.insert(edge.child.clone());
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for edge in &hierarchy.edges {
            if unsat.contains(&edge.parent) && unsat.insert(edge.child.clone()) {
                changed = true;
            }
        }
    }
    unsat.remove(OWL_NOTHING);
    unsat.into_iter().collect()
}

pub fn build_inferred_hierarchy(
    taxonomy_edges: &[(String, String)],
    unsatisfiable: &[String],
    asserted: &ClassHierarchy,
) -> InferredHierarchy {
    let asserted_set: BTreeSet<(String, String)> =
        asserted.edges.iter().map(|e| (e.child.clone(), e.parent.clone())).collect();

    let mut inferred_edges = Vec::new();
    for (child, parent) in taxonomy_edges {
        let pair = (child.clone(), parent.clone());
        if !asserted_set.contains(&pair) {
            inferred_edges.push(SubclassEdge { child: child.clone(), parent: parent.clone() });
        }
    }

    let mut combined_edges = asserted.edges.clone();
    let mut combined_set = asserted_set;
    for edge in &inferred_edges {
        let pair = (edge.child.clone(), edge.parent.clone());
        if combined_set.insert(pair) {
            combined_edges.push(edge.clone());
        }
    }

    let combined = hierarchy_from_edges(combined_edges);
    let expanded = expand_named_unsatisfiable(unsatisfiable, &combined);
    InferredHierarchy { edges: inferred_edges, unsatisfiable: expanded, combined }
}

pub fn new_inferences(asserted: &ClassHierarchy, inferred: &[SubclassEdge]) -> Vec<SubclassEdge> {
    let asserted_set: BTreeSet<(String, String)> =
        asserted.edges.iter().map(|e| (e.child.clone(), e.parent.clone())).collect();
    inferred
        .iter()
        .filter(|e| !asserted_set.contains(&(e.child.clone(), e.parent.clone())))
        .cloned()
        .collect()
}

pub fn hierarchy_from_edges(edges: Vec<SubclassEdge>) -> ClassHierarchy {
    let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for edge in &edges {
        parents.entry(edge.child.clone()).or_default().push(edge.parent.clone());
        children.entry(edge.parent.clone()).or_default().push(edge.child.clone());
    }

    for list in parents.values_mut() {
        list.sort();
        list.dedup();
    }
    for list in children.values_mut() {
        list.sort();
        list.dedup();
    }

    ClassHierarchy { edges, parents, children }
}

pub fn taxonomy_to_iri_edges(
    ontology: &ontologos_core::Ontology,
    taxonomy: &ontologos_core::Taxonomy,
) -> Result<Vec<(String, String)>, String> {
    taxonomy
        .subsumptions
        .iter()
        .map(|(sub, sup)| {
            let child = entity_iri(ontology, *sub)?;
            let parent = entity_iri(ontology, *sup)?;
            Ok((child, parent))
        })
        .collect()
}

pub fn entity_iri(
    ontology: &ontologos_core::Ontology,
    id: ontologos_core::EntityId,
) -> Result<String, String> {
    let entity = ontology.entity(id).map_err(|e| e.to_string())?;
    ontology.resolve_iri(entity.iri).map(|s| s.to_string()).map_err(|e| e.to_string())
}

pub fn unsatisfiable_iris(
    ontology: &ontologos_core::Ontology,
    taxonomy: &ontologos_core::Taxonomy,
) -> Result<Vec<String>, String> {
    taxonomy.unsatisfiable.iter().map(|id| entity_iri(ontology, *id)).collect()
}

/// Run EL classification to detect unsatisfiable classes (used after RL/RDFS saturation).
pub fn detect_unsatisfiable_classes(
    ontology: &ontologos_core::Ontology,
) -> Result<Vec<String>, String> {
    use ontologos_el::ElClassifier;
    let taxonomy =
        ElClassifier::new().classify(ontology).map_err(|e| format!("unsat detection: {e}"))?;
    unsatisfiable_iris(ontology, &taxonomy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontocore_catalog::SubclassEdge;

    #[test]
    fn expand_named_unsatisfiable_includes_descendants_of_nothing() {
        let hierarchy = hierarchy_from_edges(vec![
            SubclassEdge { child: "http://ex/B".into(), parent: OWL_NOTHING.into() },
            SubclassEdge { child: "http://ex/Invalid".into(), parent: "http://ex/B".into() },
        ]);
        let expanded = expand_named_unsatisfiable(&[OWL_NOTHING.to_string()], &hierarchy);
        assert!(expanded.iter().any(|i| i == "http://ex/B"));
        assert!(expanded.iter().any(|i| i == "http://ex/Invalid"));
        assert!(!expanded.iter().any(|i| i == OWL_NOTHING));
    }
}
