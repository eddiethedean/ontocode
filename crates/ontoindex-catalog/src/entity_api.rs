use crate::OntologyCatalog;
use ontoindex_core::{Entity, EntityKind};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct SubclassEdge {
    pub child: String,
    pub parent: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ClassHierarchy {
    pub edges: Vec<SubclassEdge>,
    pub parents: BTreeMap<String, Vec<String>>,
    pub children: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceHint {
    pub path: PathBuf,
    pub line: u64,
    pub column: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityDetail {
    pub entity: Entity,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub axioms: Vec<String>,
    pub source: Option<SourceHint>,
}

impl OntologyCatalog {
    pub fn find_entity(&self, iri: &str) -> Option<&Entity> {
        self.data().entities.iter().find(|e| e.iri == iri)
    }

    pub fn entity_document(&self, iri: &str) -> Option<&ontoindex_core::OntologyDocument> {
        let entity = self.find_entity(iri)?;
        let data = self.data();

        if let Some(doc) = data.documents.iter().find(|d| document_matches_entity(entity, d)) {
            return Some(doc);
        }

        data.documents.iter().find(|doc| file_contains_entity(&doc.path, iri, &entity.short_name))
    }

    pub fn class_hierarchy(&self) -> ClassHierarchy {
        let mut edges = Vec::new();
        let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();

        let class_iris: BTreeSet<&str> = self
            .data()
            .entities
            .iter()
            .filter(|e| e.kind == EntityKind::Class)
            .map(|e| e.iri.as_str())
            .collect();

        for axiom in &self.data().axioms {
            if axiom.axiom_kind != "SubClassOf" {
                continue;
            }
            if !class_iris.contains(axiom.subject.as_str()) {
                continue;
            }
            let edge = SubclassEdge { child: axiom.subject.clone(), parent: axiom.object.clone() };
            edges.push(edge.clone());
            parents.entry(edge.child.clone()).or_default().push(edge.parent.clone());
            children.entry(edge.parent.clone()).or_default().push(edge.child.clone());
        }

        for list in parents.values_mut().chain(children.values_mut()) {
            list.sort();
            list.dedup();
        }

        ClassHierarchy { edges, parents, children }
    }

    pub fn entity_detail(&self, iri: &str) -> Option<EntityDetail> {
        let entity = self.find_entity(iri)?.clone();
        let hierarchy = self.class_hierarchy();

        let parents = hierarchy.parents.get(iri).cloned().unwrap_or_default();
        let children = hierarchy.children.get(iri).cloned().unwrap_or_default();

        let axioms: Vec<String> = self
            .data()
            .axioms
            .iter()
            .filter(|a| a.subject == iri)
            .map(|a| format!("{} {} {}", a.subject, predicate_label(&a.predicate), a.object))
            .collect();

        let source = self.find_source_location(iri);

        Some(EntityDetail { entity, parents, children, axioms, source })
    }

    pub fn find_source_location(&self, iri: &str) -> Option<SourceHint> {
        let entity = self.find_entity(iri)?;
        let doc = self.entity_document(iri)?;

        if let Some(loc) = entity.source_location.line {
            return Some(SourceHint {
                path: doc.path.clone(),
                line: loc,
                column: entity.source_location.column.unwrap_or(0),
            });
        }

        scan_file_for_iri(&doc.path, iri, &entity.short_name)
    }

    pub fn entities_in_document(&self, doc_path: &std::path::Path) -> Vec<&Entity> {
        self.data()
            .entities
            .iter()
            .filter(|e| self.entity_document(&e.iri).map(|d| d.path == doc_path).unwrap_or(false))
            .collect()
    }
}

fn predicate_label(predicate: &str) -> &'static str {
    if predicate.contains("subClassOf") || predicate.ends_with("#subClassOf") {
        "SubClassOf"
    } else {
        "predicate"
    }
}

fn normalize_iri_prefix(iri: &str) -> String {
    iri.trim_end_matches('#').trim_end_matches('/').to_string()
}

fn document_matches_entity(entity: &Entity, doc: &ontoindex_core::OntologyDocument) -> bool {
    if entity.ontology_id == doc.id {
        return true;
    }
    if let Some(base) = &doc.base_iri {
        if normalize_iri_prefix(base) == normalize_iri_prefix(&entity.ontology_id) {
            return true;
        }
        if entity.iri.starts_with(base) {
            return true;
        }
    }
    doc.path.to_string_lossy().contains(&entity.ontology_id)
}

fn file_contains_entity(path: &std::path::Path, iri: &str, short_name: &str) -> bool {
    fs::read_to_string(path).ok().is_some_and(|content| {
        content.contains(iri)
            || content.contains(&format!(":{short_name}"))
            || content.contains(&format!(" {short_name} "))
    })
}

fn scan_file_for_iri(path: &std::path::Path, iri: &str, short_name: &str) -> Option<SourceHint> {
    let content = fs::read_to_string(path).ok()?;
    let local_name = iri.rsplit(['#', '/']).next().unwrap_or(short_name);

    let needles = [iri.to_string(), format!("<{iri}>"), format!("{local_name}:")];

    for (line_idx, line) in content.lines().enumerate() {
        for needle in &needles {
            if let Some(col) = line.find(needle) {
                return Some(SourceHint {
                    path: path.to_path_buf(),
                    line: (line_idx + 1) as u64,
                    column: col as u64,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IndexBuilder;
    use std::path::PathBuf;

    fn fixture_workspace() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
    }

    fn fixture_catalog() -> OntologyCatalog {
        IndexBuilder::new().workspace(fixture_workspace()).build().expect("build catalog")
    }

    #[test]
    fn find_entity_by_iri() {
        let catalog = fixture_catalog();
        let entity =
            catalog.find_entity("http://example.org/people#Person").expect("Person entity");
        assert_eq!(entity.short_name, "Person");
        assert_eq!(entity.kind, EntityKind::Class);
    }

    #[test]
    fn class_hierarchy_includes_subclass_axiom() {
        let catalog = fixture_catalog();
        let hierarchy = catalog.class_hierarchy();
        assert!(!hierarchy.edges.is_empty());
        assert!(hierarchy
            .parents
            .get("http://example.org/people#Person")
            .is_some_and(|p| p.contains(&"http://example.org/people#Thing".to_string())));
    }

    #[test]
    fn entity_detail_includes_labels_and_parents() {
        let catalog = fixture_catalog();
        let detail =
            catalog.entity_detail("http://example.org/people#Person").expect("Person detail");
        assert!(!detail.entity.labels.is_empty());
        assert!(!detail.parents.is_empty());
    }

    #[test]
    fn find_source_location_in_fixture() {
        let catalog = fixture_catalog();
        let source = catalog
            .find_source_location("http://example.org/people#Person")
            .expect("source location");
        assert!(source.path.ends_with("example.ttl"));
        assert!(source.line > 0);
    }
}
