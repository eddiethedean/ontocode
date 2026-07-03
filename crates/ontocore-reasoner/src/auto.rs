use crate::adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
use crate::error::{ReasonerError, Result};
use crate::explain::explain_unsatisfiable_el;
use crate::hierarchy::subclass_edges_from_ontology;
use crate::input::ReasonerInput;
use crate::result::{
    build_inferred_hierarchy, detect_unsatisfiable_classes, new_inferences, taxonomy_to_iri_edges,
    unsatisfiable_iris, ClassificationResult, ExplanationRequest, ExplanationResult,
};
use ontologos_core::{Profile, Reasoner};
use ontologos_facade::ClassifyOutcome;
use std::time::Instant;

pub struct AutoAdapter;

impl ReasonerAdapter for AutoAdapter {
    fn id(&self) -> ReasonerId {
        ReasonerId::Auto
    }

    fn profile(&self) -> ReasonerProfile {
        ReasonerProfile::Auto
    }

    fn classify(&self, input: &ReasonerInput) -> Result<ClassificationResult> {
        let started = Instant::now();
        let mut reasoner = Reasoner::builder()
            .profile(Profile::Auto)
            .build(input.ontology.clone())
            .map_err(|e| ReasonerError::Classify(e.to_string()))?;

        match ontologos_facade::classify(&mut reasoner)
            .map_err(|e| ReasonerError::Classify(e.to_string()))?
        {
            ClassifyOutcome::Taxonomy(taxonomy) => {
                let iri_edges = taxonomy_to_iri_edges(&input.ontology, &taxonomy)
                    .map_err(ReasonerError::Classify)?;
                let unsatisfiable = unsatisfiable_iris(&input.ontology, &taxonomy)
                    .map_err(ReasonerError::Classify)?;
                let inferred =
                    build_inferred_hierarchy(&iri_edges, &unsatisfiable, &input.asserted_hierarchy);
                let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

                Ok(ClassificationResult {
                    profile_used: "auto".to_string(),
                    consistent: unsatisfiable.is_empty(),
                    unsatisfiable: unsatisfiable.clone(),
                    inferred,
                    new_inferences,
                    warnings: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                    subsumption_count: taxonomy.subsumption_count(),
                    inferred_axiom_count: taxonomy.subsumption_count(),
                })
            }
            ClassifyOutcome::Rdfs(report) => {
                let ontology = reasoner.ontology();
                let iri_edges = subclass_edges_from_ontology(ontology, &input.asserted_hierarchy);
                let unsatisfiable =
                    detect_unsatisfiable_classes(ontology).map_err(ReasonerError::Classify)?;
                let inferred =
                    build_inferred_hierarchy(&iri_edges, &unsatisfiable, &input.asserted_hierarchy);
                let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

                Ok(ClassificationResult {
                    profile_used: "rdfs".to_string(),
                    consistent: unsatisfiable.is_empty(),
                    unsatisfiable,
                    inferred,
                    new_inferences,
                    warnings: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                    subsumption_count: iri_edges.len(),
                    inferred_axiom_count: report.inferred_total(),
                })
            }
            ClassifyOutcome::Rl(report) => {
                let ontology = reasoner.ontology();
                let iri_edges = subclass_edges_from_ontology(ontology, &input.asserted_hierarchy);
                let unsatisfiable =
                    detect_unsatisfiable_classes(ontology).map_err(ReasonerError::Classify)?;
                let inferred =
                    build_inferred_hierarchy(&iri_edges, &unsatisfiable, &input.asserted_hierarchy);
                let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

                Ok(ClassificationResult {
                    profile_used: "rl".to_string(),
                    consistent: unsatisfiable.is_empty(),
                    unsatisfiable,
                    inferred,
                    new_inferences,
                    warnings: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                    subsumption_count: iri_edges.len(),
                    inferred_axiom_count: report.inferred_total(),
                })
            }
            _ => Err(ReasonerError::Classify(
                "unsupported auto classification outcome".to_string(),
            )),
        }
    }

    fn explain(
        &self,
        input: &ReasonerInput,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult> {
        explain_unsatisfiable_el(&input.ontology, &request.class_iri)
    }
}
