use crate::adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
use crate::error::{ReasonerError, Result};
use crate::explain::explain_unsatisfiable_rdfs;
use crate::hierarchy::subclass_edges_from_ontology;
use crate::input::ReasonerInput;
use crate::result::{
    build_inferred_hierarchy, new_inferences, ClassificationResult, ExplanationRequest,
    ExplanationResult,
};
use ontologos_rdfs::RdfsEngine;
use std::time::Instant;

pub struct RdfsAdapter;

impl ReasonerAdapter for RdfsAdapter {
    fn id(&self) -> ReasonerId {
        ReasonerId::Rdfs
    }

    fn profile(&self) -> ReasonerProfile {
        ReasonerProfile::Rdfs
    }

    fn classify(&self, input: &ReasonerInput) -> Result<ClassificationResult> {
        let started = Instant::now();
        let mut ontology = input.ontology.clone();
        let report = RdfsEngine::new()
            .materialize(&mut ontology)
            .map_err(|e| ReasonerError::Classify(e.to_string()))?;

        let iri_edges = subclass_edges_from_ontology(&ontology, &input.asserted_hierarchy);
        let inferred = build_inferred_hierarchy(&iri_edges, &[], &input.asserted_hierarchy);
        let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

        Ok(ClassificationResult {
            profile_used: "rdfs".to_string(),
            consistent: true,
            unsatisfiable: Vec::new(),
            inferred,
            new_inferences,
            warnings: Vec::new(),
            duration_ms: started.elapsed().as_millis() as u64,
            subsumption_count: iri_edges.len(),
            inferred_axiom_count: report.inferred_total(),
        })
    }

    fn explain(
        &self,
        input: &ReasonerInput,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult> {
        explain_unsatisfiable_rdfs(&input.ontology, &request.class_iri)
    }
}
