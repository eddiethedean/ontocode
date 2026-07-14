//! Protégé-style DL Query over a workspace reasoner input.

use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{ClassificationResult, RealizationResult};
use crate::{classify, realize, ReasonerId};
use horned_owl::model::ClassExpression;
use ontocore_catalog::ClassHierarchy;
use ontocore_owl::{class_expression_to_turtle_value, parse_class_expression};
use ontologos_bridge::{core_to_triples_all, merge_triples_into_ontology};
use ontologos_parser::load_ontology;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

/// Temporary named class used to materialize anonymous Manchester expressions.
pub const DL_QUERY_CLASS_IRI: &str = "urn:ontocode:dl-query#Q";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DlQueryMode {
    #[default]
    Inferred,
    Asserted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlQueryResult {
    pub expression: String,
    pub normalized: String,
    pub query_class_iri: String,
    pub subclasses: Vec<String>,
    pub superclasses: Vec<String>,
    pub equivalents: Vec<String>,
    pub instances: Vec<String>,
    pub profile: String,
    pub mode: DlQueryMode,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<String>,
}

/// Run a Manchester class-expression DL Query against `input`.
///
/// Named class expressions use the hierarchy (asserted or inferred) and realization
/// directly. Anonymous expressions are evaluated by injecting a temporary
/// `EquivalentClasses(Q, CE)` axiom into a cloned ontology (never written to disk).
pub fn run_dl_query(
    profile: ReasonerId,
    input: &ReasonerInput,
    expression: &str,
    namespaces: &BTreeMap<String, String>,
    mode: DlQueryMode,
) -> Result<DlQueryResult> {
    let started = Instant::now();
    let parsed = parse_class_expression(expression, namespaces)
        .map_err(|e| ReasonerError::Classify(format!("Manchester parse failed: {e}")))?;
    let diagnostics: Vec<String> = parsed.diagnostics.iter().map(|d| d.message.clone()).collect();

    let named_iri = named_class_iri(&parsed.expression);
    let is_named = named_iri.is_some();
    let (query_iri, query_input, mut warnings) = if let Some(iri) = named_iri {
        (iri, input.clone_shallow(), Vec::new())
    } else {
        let (augmented, w) = inject_temp_equivalent(input, &parsed.expression, namespaces)?;
        (DL_QUERY_CLASS_IRI.to_string(), augmented, w)
    };

    let (hierarchy, profile_used, realization) = match mode {
        DlQueryMode::Asserted => {
            (query_input.asserted_hierarchy.clone(), "asserted".to_string(), None)
        }
        DlQueryMode::Inferred => {
            let classification = classify(profile, &query_input, true)?;
            let realization = realize(profile, &query_input).ok();
            collect_inferred(&classification, realization, profile)
        }
    };

    let mut subclasses = collect_descendants(&hierarchy, &query_iri);
    let mut superclasses = collect_ancestors(&hierarchy, &query_iri);
    let mut equivalents = collect_equivalents(&hierarchy, &query_iri);
    subclasses.retain(|iri| iri != &query_iri && iri != DL_QUERY_CLASS_IRI);
    superclasses.retain(|iri| {
        iri != &query_iri
            && iri != DL_QUERY_CLASS_IRI
            && iri != "http://www.w3.org/2002/07/owl#Thing"
    });
    equivalents.retain(|iri| iri != &query_iri && iri != DL_QUERY_CLASS_IRI);

    let mut instances = Vec::new();
    if let Some(realization) = realization {
        for entry in realization.individuals {
            if entry.types.iter().any(|t| t == &query_iri) {
                instances.push(entry.individual_iri);
            }
        }
    } else if mode == DlQueryMode::Asserted {
        if is_named {
            instances = asserted_instances_of_class(&query_input, &hierarchy, &query_iri);
        } else {
            warnings.push(
                "asserted mode cannot materialize instances for anonymous class expressions; use inferred mode"
                    .to_string(),
            );
        }
    }

    instances.sort();
    instances.dedup();

    Ok(DlQueryResult {
        expression: expression.to_string(),
        normalized: parsed.normalized,
        query_class_iri: query_iri,
        subclasses,
        superclasses,
        equivalents,
        instances,
        profile: profile_used,
        mode,
        duration_ms: started.elapsed().as_millis() as u64,
        warnings,
        diagnostics,
    })
}

impl ReasonerInput {
    /// Clone ontology + hierarchy without re-scanning the workspace.
    fn clone_shallow(&self) -> Self {
        Self {
            workspace: self.workspace.clone(),
            content_hash: self.content_hash.clone(),
            ontology: self.ontology.clone(),
            asserted_hierarchy: self.asserted_hierarchy.clone(),
            document_overrides: self.document_overrides.clone(),
        }
    }
}

fn collect_inferred(
    classification: &ClassificationResult,
    realization: Option<RealizationResult>,
    _profile: ReasonerId,
) -> (ClassHierarchy, String, Option<RealizationResult>) {
    (classification.inferred.combined.clone(), classification.profile_used.clone(), realization)
}

/// Collect individuals with an asserted type of `class_iri` or any asserted descendant.
fn asserted_instances_of_class(
    input: &ReasonerInput,
    hierarchy: &ClassHierarchy,
    class_iri: &str,
) -> Vec<String> {
    let mut class_iris: BTreeSet<String> = collect_descendants(hierarchy, class_iri)
        .into_iter()
        .collect();
    class_iris.insert(class_iri.to_string());

    let mut out = BTreeSet::new();
    for iri in &class_iris {
        let Some(class_id) = input.ontology.lookup_entity(iri) else {
            continue;
        };
        for ind_id in input.ontology.individuals_of(class_id) {
            if let Ok(ind_iri) = crate::result::entity_iri(&input.ontology, *ind_id) {
                out.insert(ind_iri);
            }
        }
    }

    // Fallback: walk ClassAssertion axioms (including load-marked "inferred" ones).
    if out.is_empty() {
        use ontologos_core::Axiom;
        for (_id, axiom) in input.ontology.axioms().iter() {
            let Axiom::ClassAssertion { individual, class } = axiom else {
                continue;
            };
            let Ok(class_s) = crate::result::entity_iri(&input.ontology, *class) else {
                continue;
            };
            if !class_iris.contains(&class_s) {
                continue;
            }
            if let Ok(ind_s) = crate::result::entity_iri(&input.ontology, *individual) {
                out.insert(ind_s);
            }
        }
    }

    out.into_iter().collect()
}

fn named_class_iri(expr: &ClassExpression<horned_owl::model::RcStr>) -> Option<String> {
    match expr {
        ClassExpression::Class(c) => {
            let iri = c.to_string();
            if iri == "http://www.w3.org/2002/07/owl#Thing"
                || iri == "http://www.w3.org/2002/07/owl#Nothing"
            {
                Some(iri)
            } else {
                Some(iri)
            }
        }
        _ => None,
    }
}

fn inject_temp_equivalent(
    input: &ReasonerInput,
    expr: &ClassExpression<horned_owl::model::RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Result<(ReasonerInput, Vec<String>)> {
    let ce_turtle = class_expression_to_turtle_value(expr, namespaces, 0)
        .map_err(|e| ReasonerError::Classify(format!("failed to serialize class expression: {e}")))?;
    let supplement = build_query_supplement(&ce_turtle, namespaces);
    let loaded = load_ontology_from_temp_text(&supplement)?;
    let mut ontology = input.ontology.clone();
    let triples =
        core_to_triples_all(&loaded).map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    merge_triples_into_ontology(&mut ontology, &triples, &[])
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    let asserted_hierarchy = crate::hierarchy::asserted_hierarchy_from_ontology(&ontology);
    Ok((
        ReasonerInput {
            workspace: input.workspace.clone(),
            content_hash: format!("{}:dl-query", input.content_hash),
            ontology,
            asserted_hierarchy,
            document_overrides: input.document_overrides.clone(),
        },
        vec!["evaluated via temporary equivalent class (not written to disk)".to_string()],
    ))
}

fn build_query_supplement(ce_turtle: &str, namespaces: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    out.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    out.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
    out.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    out.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n");
    for (prefix, iri) in namespaces {
        if matches!(prefix.as_str(), "owl" | "rdf" | "rdfs" | "xsd") {
            continue;
        }
        out.push_str(&format!("@prefix {prefix}: <{iri}> .\n"));
    }
    out.push_str(&format!(
        "<{DL_QUERY_CLASS_IRI}> a owl:Class ;\n  owl:equivalentClass {ce_turtle} .\n"
    ));
    out
}

fn load_ontology_from_temp_text(text: &str) -> Result<ontologos_core::Ontology> {
    let tmp = tempfile::Builder::new()
        .suffix(".ttl")
        .tempfile()
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    std::fs::write(tmp.path(), text).map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    load_ontology(tmp.path()).map_err(|e| ReasonerError::Ontology(e.to_string()))
}

fn collect_descendants(hierarchy: &ClassHierarchy, root: &str) -> Vec<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root.to_string()];
    let mut seen = BTreeSet::new();
    while let Some(current) = stack.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        if let Some(children) = hierarchy.children.get(&current) {
            for child in children {
                if child != root {
                    out.insert(child.clone());
                }
                stack.push(child.clone());
            }
        }
    }
    out.into_iter().collect()
}

fn collect_ancestors(hierarchy: &ClassHierarchy, root: &str) -> Vec<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root.to_string()];
    let mut seen = BTreeSet::new();
    while let Some(current) = stack.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        if let Some(parents) = hierarchy.parents.get(&current) {
            for parent in parents {
                if parent != root {
                    out.insert(parent.clone());
                }
                stack.push(parent.clone());
            }
        }
    }
    out.into_iter().collect()
}

fn collect_equivalents(hierarchy: &ClassHierarchy, root: &str) -> Vec<String> {
    let descendants: BTreeSet<_> = collect_descendants(hierarchy, root).into_iter().collect();
    let ancestors: BTreeSet<_> = collect_ancestors(hierarchy, root).into_iter().collect();
    descendants.intersection(&ancestors).cloned().collect()
}
