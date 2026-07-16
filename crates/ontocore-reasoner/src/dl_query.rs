//! Protégé-style DL Query over a workspace reasoner input.

use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{ClassificationResult, RealizationResult};
use crate::{classify, realize, ReasonerId};
use horned_owl::model::{
    ClassExpression, Individual, NamedIndividual, ObjectPropertyExpression, RcStr,
};
use ontocore_catalog::ClassHierarchy;
use ontocore_owl::parse_class_expression;
use ontologos_core::{CeId, ClassExpr, DlAxiom, EntityId, EntityKind, Ontology, RoleExpr};
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
        let (augmented, w) = inject_temp_equivalent(input, &parsed.expression)?;
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
    let mut class_iris: BTreeSet<String> =
        collect_descendants(hierarchy, class_iri).into_iter().collect();
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

fn named_class_iri(expr: &ClassExpression<RcStr>) -> Option<String> {
    match expr {
        ClassExpression::Class(c) => Some(c.to_string()),
        _ => None,
    }
}

/// Inject `EquivalentClasses(Q, CE)` directly into the Ontologos DL store (#366).
///
/// The previous Turtle → `core_to_triples_all` path only serialized flat axioms, so complex
/// equivalent-class restrictions never reached the reasoner.
fn inject_temp_equivalent(
    input: &ReasonerInput,
    expr: &ClassExpression<RcStr>,
) -> Result<(ReasonerInput, Vec<String>)> {
    let mut ontology = input.ontology.clone();
    let dl_before = ontology.dl().axiom_count();

    let q_id = ontology
        .entity_id(DL_QUERY_CLASS_IRI, EntityKind::Class)
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    let q_ce = ontology.dl_mut().intern_ce(ClassExpr::Atomic(q_id));
    let expr_ce = intern_horned_ce(&mut ontology, expr).map_err(|e| {
        ReasonerError::Classify(format!(
            "anonymous DL Query inject could not materialize class expression: {e}"
        ))
    })?;
    ontology.dl_mut().push_axiom(DlAxiom::EquivalentClasses(vec![q_ce, expr_ce]));
    ontology.reindex_dl_abox();

    if ontology.dl().axiom_count() <= dl_before {
        return Err(ReasonerError::Classify(
            "anonymous DL Query inject failed to add EquivalentClasses to the DL store".into(),
        ));
    }

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

fn intern_horned_ce(ontology: &mut Ontology, ce: &ClassExpression<RcStr>) -> Result<CeId> {
    let expr = build_class_expr(ontology, ce)?;
    Ok(ontology.dl_mut().intern_ce(expr))
}

fn build_class_expr(ontology: &mut Ontology, ce: &ClassExpression<RcStr>) -> Result<ClassExpr> {
    use ClassExpression::*;
    match ce {
        Class(class) => {
            let iri = class.0.as_ref();
            if iri == "http://www.w3.org/2002/07/owl#Nothing" {
                Ok(ClassExpr::Bottom)
            } else if iri == "http://www.w3.org/2002/07/owl#Thing" {
                Ok(ClassExpr::Top)
            } else {
                let id = ensure_entity(ontology, iri, EntityKind::Class)?;
                Ok(ClassExpr::Atomic(id))
            }
        }
        ObjectIntersectionOf(ops) => {
            let ids = map_ce_list(ontology, ops)?;
            Ok(ClassExpr::And(ids))
        }
        ObjectUnionOf(ops) => {
            let ids = map_ce_list(ontology, ops)?;
            Ok(ClassExpr::Or(ids))
        }
        ObjectComplementOf(inner) => {
            let id = intern_horned_ce(ontology, inner)?;
            Ok(ClassExpr::Not(id))
        }
        ObjectOneOf(individuals) => {
            let mut ids = Vec::with_capacity(individuals.len());
            for ind in individuals {
                ids.push(map_individual(ontology, ind)?);
            }
            Ok(ClassExpr::OneOf(ids))
        }
        ObjectSomeValuesFrom { ope, bce } => {
            let property = map_role(ontology, ope)?;
            let filler = intern_horned_ce(ontology, bce)?;
            Ok(ClassExpr::Some { property, filler })
        }
        ObjectAllValuesFrom { ope, bce } => {
            let property = map_role(ontology, ope)?;
            let filler = intern_horned_ce(ontology, bce)?;
            Ok(ClassExpr::All { property, filler })
        }
        ObjectHasValue { ope, i } => {
            let property = map_role(ontology, ope)?;
            let individual = map_individual(ontology, i)?;
            Ok(ClassExpr::HasValue { property, individual })
        }
        ObjectHasSelf(ope) => match map_role(ontology, ope)? {
            RoleExpr::Atomic(prop) => Ok(ClassExpr::HasSelf(prop)),
            RoleExpr::Inverse(_) => Err(ReasonerError::Classify(
                "ObjectHasSelf with inverse property is not supported for DL Query inject".into(),
            )),
        },
        ObjectMinCardinality { n, ope, bce } => {
            let property = map_role(ontology, ope)?;
            let filler = universal_or_filler(ontology, bce)?;
            Ok(ClassExpr::MinCardinality { n: *n, property, filler })
        }
        ObjectMaxCardinality { n, ope, bce } => {
            let property = map_role(ontology, ope)?;
            let filler = universal_or_filler(ontology, bce)?;
            Ok(ClassExpr::MaxCardinality { n: *n, property, filler })
        }
        ObjectExactCardinality { n, ope, bce } => {
            let property = map_role(ontology, ope)?;
            let filler = universal_or_filler(ontology, bce)?;
            Ok(ClassExpr::ExactCardinality { n: *n, property, filler })
        }
        DataSomeValuesFrom { .. }
        | DataAllValuesFrom { .. }
        | DataHasValue { .. }
        | DataMinCardinality { .. }
        | DataMaxCardinality { .. }
        | DataExactCardinality { .. } => Err(ReasonerError::Classify(
            "data-property class expressions are not supported for anonymous DL Query inject"
                .into(),
        )),
    }
}

fn map_ce_list(ontology: &mut Ontology, ops: &[ClassExpression<RcStr>]) -> Result<Vec<CeId>> {
    let mut ids = Vec::with_capacity(ops.len());
    for op in ops {
        ids.push(intern_horned_ce(ontology, op)?);
    }
    Ok(ids)
}

fn universal_or_filler(
    ontology: &mut Ontology,
    bce: &ClassExpression<RcStr>,
) -> Result<Option<CeId>> {
    if matches!(
        bce,
        ClassExpression::Class(c) if c.0.as_ref() == "http://www.w3.org/2002/07/owl#Thing"
    ) {
        return Ok(None);
    }
    Ok(Some(intern_horned_ce(ontology, bce)?))
}

fn map_role(ontology: &mut Ontology, ope: &ObjectPropertyExpression<RcStr>) -> Result<RoleExpr> {
    match ope {
        ObjectPropertyExpression::ObjectProperty(prop) => {
            let id = ensure_entity(ontology, prop.0.as_ref(), EntityKind::ObjectProperty)?;
            Ok(RoleExpr::Atomic(id))
        }
        ObjectPropertyExpression::InverseObjectProperty(prop) => {
            let id = ensure_entity(ontology, prop.0.as_ref(), EntityKind::ObjectProperty)?;
            Ok(RoleExpr::Inverse(id))
        }
    }
}

fn map_individual(ontology: &mut Ontology, individual: &Individual<RcStr>) -> Result<EntityId> {
    match individual {
        Individual::Named(NamedIndividual(iri)) => {
            ensure_entity(ontology, iri.as_ref(), EntityKind::Individual)
        }
        Individual::Anonymous(anon) => {
            let iri = format!("urn:ontocode:dl-query#_{}", anon.as_ref());
            ensure_entity(ontology, &iri, EntityKind::Individual)
        }
    }
}

fn ensure_entity(ontology: &mut Ontology, iri: &str, kind: EntityKind) -> Result<EntityId> {
    ontology.entity_id(iri, kind).map_err(|e| ReasonerError::Ontology(e.to_string()))
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

#[cfg(test)]
mod inject_tests {
    use super::*;
    use ontocore_owl::parse_class_expression;

    #[test]
    fn inject_temp_equivalent_adds_dl_axiom() {
        let mut ontology = Ontology::default();
        ontology
            .entity_id("http://example.org/clinic#hasRecord", EntityKind::ObjectProperty)
            .unwrap();
        ontology.entity_id("http://example.org/clinic#MedicalRecord", EntityKind::Class).unwrap();
        let input = ReasonerInput {
            workspace: std::path::PathBuf::from("."),
            content_hash: "t".into(),
            ontology,
            asserted_hierarchy: ClassHierarchy::default(),
            document_overrides: Default::default(),
        };
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
        let parsed = parse_class_expression("ex:hasRecord some ex:MedicalRecord", &namespaces)
            .expect("parse");
        let (augmented, warnings) =
            inject_temp_equivalent(&input, &parsed.expression).expect("inject");
        assert!(augmented.ontology.dl().axiom_count() >= 1);
        assert!(augmented.ontology.lookup_entity(DL_QUERY_CLASS_IRI).is_some());
        assert!(warnings.iter().any(|w| w.contains("temporary equivalent")));
    }
}
