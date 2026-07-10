//! Graph export for visualization webviews.

use crate::entity_api::SubclassEdge;
use crate::OntologyCatalog;
use ontocore_core::{
    limits::{MAX_GRAPH_EDGES, MAX_GRAPH_NODES},
    EntityKind, AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS, AXIOM_KIND_RANGE,
    AXIOM_KIND_SUB_CLASS_OF,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct GraphError(String);

impl From<String> for GraphError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub type GraphResult<T> = std::result::Result<T, GraphError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphKind {
    Class,
    Property,
    Import,
    Neighborhood,
}

impl GraphKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "class" => Some(Self::Class),
            "property" => Some(Self::Property),
            "import" => Some(Self::Import),
            "neighborhood" => Some(Self::Neighborhood),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Property => "property",
            Self::Import => "import",
            Self::Neighborhood => "neighborhood",
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GraphFilters {
    pub ontology_iri: Option<String>,
    #[serde(default)]
    pub hide_deprecated: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphRequest {
    pub graph_kind: String,
    pub root_iri: Option<String>,
    #[serde(default = "default_depth")]
    pub depth: u32,
    #[serde(default)]
    pub include_inferred: bool,
    #[serde(default)]
    pub filters: GraphFilters,
}

fn default_depth() -> u32 {
    2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub inferred: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphPayload {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub truncated: bool,
    pub graph_kind: String,
}

pub struct GraphBuilder<'a> {
    catalog: &'a OntologyCatalog,
    inferred_edges: Option<&'a [SubclassEdge]>,
}

impl<'a> GraphBuilder<'a> {
    pub fn new(catalog: &'a OntologyCatalog) -> Self {
        Self { catalog, inferred_edges: None }
    }

    pub fn with_inferred_edges(mut self, edges: &'a [SubclassEdge]) -> Self {
        self.inferred_edges = Some(edges);
        self
    }

    pub fn build(&self, request: &GraphRequest) -> GraphResult<GraphPayload> {
        let kind = GraphKind::parse(&request.graph_kind)
            .ok_or_else(|| GraphError(format!("unknown graph_kind: {}", request.graph_kind)))?;
        let depth = request.depth.clamp(1, 5);

        let mut payload = match kind {
            GraphKind::Class => self.build_class_graph(request),
            GraphKind::Property => self.build_property_graph(request),
            GraphKind::Import => self.build_import_graph(request),
            GraphKind::Neighborhood => {
                let root = request.root_iri.as_deref().ok_or_else(|| {
                    GraphError("neighborhood graph requires root_iri".to_string())
                })?;
                self.build_neighborhood_graph(request, root, depth)
            }
        }?;

        payload.graph_kind = kind.as_str().to_string();
        Ok(payload)
    }

    fn entity_allowed(&self, iri: &str, filters: &GraphFilters) -> bool {
        let Some(entity) = self.catalog.find_entity(iri) else {
            // Unknown IRIs are not in any indexed ontology — exclude them when filtering
            // by ontology_iri so external/imported parents cannot bypass the filter (#117).
            if filters.ontology_iri.is_some() {
                return false;
            }
            return !filters.hide_deprecated;
        };
        if filters.hide_deprecated && entity.deprecated {
            return false;
        }
        if let Some(ref ont) = filters.ontology_iri {
            if entity.ontology_id != *ont {
                return false;
            }
        }
        true
    }

    fn add_node(
        nodes: &mut Vec<GraphNode>,
        node_ids: &mut HashSet<String>,
        truncated: &mut bool,
        id: String,
        label: String,
        kind: String,
    ) {
        if node_ids.contains(&id) {
            return;
        }
        if nodes.len() >= MAX_GRAPH_NODES {
            *truncated = true;
            return;
        }
        node_ids.insert(id.clone());
        nodes.push(GraphNode { id, label, kind });
    }

    fn add_edge(
        edges: &mut Vec<GraphEdge>,
        truncated: &mut bool,
        source: String,
        target: String,
        kind: String,
        inferred: bool,
    ) {
        if edges.len() >= MAX_GRAPH_EDGES {
            *truncated = true;
            return;
        }
        edges.push(GraphEdge { source, target, kind, inferred });
    }

    fn label_for(&self, iri: &str) -> String {
        self.catalog
            .find_entity(iri)
            .and_then(|e| e.labels.first().cloned())
            .or_else(|| self.catalog.find_entity(iri).map(|e| e.short_name.clone()))
            .unwrap_or_else(|| short_name(iri))
    }

    fn kind_for(&self, iri: &str) -> String {
        self.catalog
            .find_entity(iri)
            .map(|e| e.kind.as_str().to_string())
            .unwrap_or_else(|| "other".to_string())
    }

    fn build_class_graph(&self, request: &GraphRequest) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        let hierarchy = self.catalog.class_hierarchy();
        for edge in &hierarchy.edges {
            if !self.entity_allowed(&edge.child, &request.filters)
                || !self.entity_allowed(&edge.parent, &request.filters)
            {
                continue;
            }
            Self::add_node(
                &mut nodes,
                &mut node_ids,
                &mut truncated,
                edge.child.clone(),
                self.label_for(&edge.child),
                self.kind_for(&edge.child),
            );
            Self::add_node(
                &mut nodes,
                &mut node_ids,
                &mut truncated,
                edge.parent.clone(),
                self.label_for(&edge.parent),
                self.kind_for(&edge.parent),
            );
            Self::add_edge(
                &mut edges,
                &mut truncated,
                edge.child.clone(),
                edge.parent.clone(),
                "sub_class_of".to_string(),
                false,
            );
        }

        if request.include_inferred {
            if let Some(inferred) = self.inferred_edges {
                for edge in inferred {
                    if !self.entity_allowed(&edge.child, &request.filters)
                        || !self.entity_allowed(&edge.parent, &request.filters)
                    {
                        continue;
                    }
                    let is_new = !hierarchy
                        .edges
                        .iter()
                        .any(|e| e.child == edge.child && e.parent == edge.parent);
                    if !is_new {
                        continue;
                    }
                    Self::add_node(
                        &mut nodes,
                        &mut node_ids,
                        &mut truncated,
                        edge.child.clone(),
                        self.label_for(&edge.child),
                        self.kind_for(&edge.child),
                    );
                    Self::add_node(
                        &mut nodes,
                        &mut node_ids,
                        &mut truncated,
                        edge.parent.clone(),
                        self.label_for(&edge.parent),
                        self.kind_for(&edge.parent),
                    );
                    Self::add_edge(
                        &mut edges,
                        &mut truncated,
                        edge.child.clone(),
                        edge.parent.clone(),
                        "sub_class_of".to_string(),
                        true,
                    );
                }
            }
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }

    fn build_property_graph(&self, request: &GraphRequest) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        for entity in &self.catalog.data().entities {
            if entity.kind != EntityKind::ObjectProperty && entity.kind != EntityKind::DataProperty
            {
                continue;
            }
            if !self.entity_allowed(&entity.iri, &request.filters) {
                continue;
            }
            Self::add_node(
                &mut nodes,
                &mut node_ids,
                &mut truncated,
                entity.iri.clone(),
                entity.labels.first().cloned().unwrap_or_else(|| entity.short_name.clone()),
                entity.kind.as_str().to_string(),
            );
        }

        for axiom in &self.catalog.data().axioms {
            let edge_kind = match axiom.axiom_kind.as_str() {
                AXIOM_KIND_DOMAIN => "domain",
                AXIOM_KIND_RANGE => "range",
                _ => continue,
            };
            let Some(prop) = self.catalog.find_entity(&axiom.subject) else {
                continue;
            };
            if prop.kind != EntityKind::ObjectProperty && prop.kind != EntityKind::DataProperty {
                continue;
            }
            if !self.entity_allowed(&axiom.subject, &request.filters) {
                continue;
            }
            Self::add_node(
                &mut nodes,
                &mut node_ids,
                &mut truncated,
                axiom.object.clone(),
                self.label_for(&axiom.object),
                self.kind_for(&axiom.object),
            );
            Self::add_edge(
                &mut edges,
                &mut truncated,
                axiom.subject.clone(),
                axiom.object.clone(),
                edge_kind.to_string(),
                false,
            );
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }

    fn build_import_graph(&self, request: &GraphRequest) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        for doc in &self.catalog.data().documents {
            let ont_iri = doc.base_iri.clone().unwrap_or_else(|| doc.id.clone());
            if let Some(ref filter) = request.filters.ontology_iri {
                if &ont_iri != filter && &doc.id != filter {
                    continue;
                }
            }
            Self::add_node(
                &mut nodes,
                &mut node_ids,
                &mut truncated,
                ont_iri.clone(),
                short_name(&ont_iri),
                "ontology".to_string(),
            );
            for import in &doc.imports {
                Self::add_node(
                    &mut nodes,
                    &mut node_ids,
                    &mut truncated,
                    import.clone(),
                    short_name(import),
                    "ontology".to_string(),
                );
                Self::add_edge(
                    &mut edges,
                    &mut truncated,
                    ont_iri.clone(),
                    import.clone(),
                    "imports".to_string(),
                    false,
                );
            }
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }

    fn build_neighborhood_graph(
        &self,
        request: &GraphRequest,
        root: &str,
        depth: u32,
    ) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        let hierarchy = self.catalog.class_hierarchy();
        let mut adjacency: Vec<(String, String, String, bool)> = Vec::new();

        for edge in &hierarchy.edges {
            adjacency.push((
                edge.child.clone(),
                edge.parent.clone(),
                "sub_class_of".to_string(),
                false,
            ));
            adjacency.push((
                edge.parent.clone(),
                edge.child.clone(),
                "super_class_of".to_string(),
                false,
            ));
        }

        if request.include_inferred {
            if let Some(inferred) = self.inferred_edges {
                for edge in inferred {
                    adjacency.push((
                        edge.child.clone(),
                        edge.parent.clone(),
                        "sub_class_of".to_string(),
                        true,
                    ));
                    adjacency.push((
                        edge.parent.clone(),
                        edge.child.clone(),
                        "super_class_of".to_string(),
                        true,
                    ));
                }
            }
        }

        for axiom in &self.catalog.data().axioms {
            if axiom.axiom_kind == AXIOM_KIND_EQUIVALENT_CLASS
                && (axiom.object.starts_with("http://") || axiom.object.starts_with("https://"))
            {
                adjacency.push((
                    axiom.subject.clone(),
                    axiom.object.clone(),
                    "equivalent_class".to_string(),
                    false,
                ));
                adjacency.push((
                    axiom.object.clone(),
                    axiom.subject.clone(),
                    "equivalent_class".to_string(),
                    false,
                ));
            } else if axiom.axiom_kind == AXIOM_KIND_SUB_CLASS_OF
                && !axiom.object.starts_with("http://")
                && !axiom.object.starts_with("https://")
            {
                for filler in restriction_fillers_in_expr(&axiom.object, self.catalog) {
                    adjacency.push((
                        axiom.subject.clone(),
                        filler,
                        "some_values_from".to_string(),
                        false,
                    ));
                }
            }
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((root.to_string(), 0u32));
        visited.insert(root.to_string());

        Self::add_node(
            &mut nodes,
            &mut node_ids,
            &mut truncated,
            root.to_string(),
            self.label_for(root),
            self.kind_for(root),
        );

        while let Some((current, d)) = queue.pop_front() {
            if d >= depth {
                continue;
            }
            for (src, tgt, kind, inferred) in &adjacency {
                if src != &current {
                    continue;
                }
                if !self.entity_allowed(tgt, &request.filters) {
                    continue;
                }
                Self::add_node(
                    &mut nodes,
                    &mut node_ids,
                    &mut truncated,
                    tgt.clone(),
                    self.label_for(tgt),
                    self.kind_for(tgt),
                );
                Self::add_edge(
                    &mut edges,
                    &mut truncated,
                    src.clone(),
                    tgt.clone(),
                    kind.clone(),
                    *inferred,
                );
                if visited.insert(tgt.clone()) {
                    queue.push_back((tgt.clone(), d + 1));
                }
            }
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }
}

fn short_name(iri: &str) -> String {
    let hash = iri.rfind('#');
    let slash = iri.rfind('/');
    match (hash, slash) {
        (Some(h), Some(s)) => iri[h.max(s) + 1..].to_string(),
        (Some(h), None) => iri[h + 1..].to_string(),
        (None, Some(s)) => iri[s + 1..].to_string(),
        _ => iri.to_string(),
    }
}

fn restriction_fillers_in_expr(expr: &str, catalog: &OntologyCatalog) -> Vec<String> {
    catalog
        .data()
        .entities
        .iter()
        .filter(|e| e.kind == EntityKind::Class)
        .filter(|e| expr.contains(&e.iri) || expr.contains(&format!(":{}", e.short_name)))
        .map(|e| e.iri.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IndexBuilder;
    use std::path::Path;

    #[test]
    fn class_graph_from_fixtures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload = GraphBuilder::new(&catalog)
            .build(&GraphRequest {
                graph_kind: "class".to_string(),
                root_iri: None,
                depth: 2,
                include_inferred: false,
                filters: GraphFilters::default(),
            })
            .expect("graph");
        assert!(!payload.nodes.is_empty());
        assert!(!payload.edges.is_empty());
    }

    #[test]
    fn import_graph_from_fixtures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload = GraphBuilder::new(&catalog)
            .build(&GraphRequest {
                graph_kind: "import".to_string(),
                root_iri: None,
                depth: 2,
                include_inferred: false,
                filters: GraphFilters::default(),
            })
            .expect("graph");
        assert!(!payload.nodes.is_empty());
    }

    #[test]
    fn property_graph_includes_domain_range_from_axioms() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload = GraphBuilder::new(&catalog)
            .build(&GraphRequest {
                graph_kind: "property".to_string(),
                root_iri: None,
                depth: 2,
                include_inferred: false,
                filters: GraphFilters::default(),
            })
            .expect("graph");
        assert!(
            payload.edges.iter().any(|e| e.kind == "domain"),
            "expected domain edges from axioms"
        );
        assert!(
            payload.edges.iter().any(|e| e.kind == "range"),
            "expected range edges from axioms"
        );
    }

    #[test]
    fn neighborhood_graph_includes_restriction_fillers_from_axioms() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let patient = "http://example.org/clinic#Patient";
        let record = "http://example.org/clinic#MedicalRecord";
        let payload = GraphBuilder::new(&catalog)
            .build(&GraphRequest {
                graph_kind: "neighborhood".to_string(),
                root_iri: Some(patient.to_string()),
                depth: 2,
                include_inferred: false,
                filters: GraphFilters::default(),
            })
            .expect("graph");
        assert!(
            payload.edges.iter().any(|e| e.source == patient && e.target == record),
            "expected Patient -> MedicalRecord restriction edge"
        );
    }

    #[test]
    fn ontology_iri_filter_excludes_unknown_entities() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("local.ttl"),
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             <http://ex/onto> a owl:Ontology .\n\
             ex:Child a owl:Class ; rdfs:subClassOf <http://external.example/Parent> .\n",
        )
        .expect("write");
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let ont_id = catalog
            .data()
            .documents
            .first()
            .map(|d| d.id.clone())
            .expect("doc");
        let unfiltered = GraphBuilder::new(&catalog)
            .build(&GraphRequest {
                graph_kind: "class".to_string(),
                root_iri: None,
                depth: 2,
                include_inferred: false,
                filters: GraphFilters::default(),
            })
            .expect("unfiltered");
        assert!(
            unfiltered
                .nodes
                .iter()
                .any(|n| n.id == "http://external.example/Parent"),
            "unknown parent should appear without ontology filter"
        );

        let filtered = GraphBuilder::new(&catalog)
            .build(&GraphRequest {
                graph_kind: "class".to_string(),
                root_iri: None,
                depth: 2,
                include_inferred: false,
                filters: GraphFilters {
                    ontology_iri: Some(ont_id),
                    hide_deprecated: false,
                },
            })
            .expect("filtered");
        assert!(
            filtered
                .nodes
                .iter()
                .all(|n| n.id != "http://external.example/Parent"),
            "unknown parent must not bypass ontology_iri filter"
        );
        assert!(
            filtered.edges.iter().all(|e| {
                e.source != "http://external.example/Parent"
                    && e.target != "http://external.example/Parent"
            }),
            "edges to unknown parent must be excluded when filtering"
        );
    }
}
