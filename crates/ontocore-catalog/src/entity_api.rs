use crate::OntologyCatalog;
use ontocore_core::{
    document_matches_entity, read_to_string_capped, Entity, EntityKind, PropertyCharacteristics,
    AXIOM_KIND_CLASS_ASSERTION, AXIOM_KIND_DATA_PROPERTY_ASSERTION, AXIOM_KIND_DISJOINT_CLASS,
    AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS, AXIOM_KIND_OBJECT_PROPERTY_ASSERTION,
    AXIOM_KIND_PROPERTY_CHAIN, AXIOM_KIND_RANGE, AXIOM_KIND_SUB_CLASS_OF, MAX_FILE_BYTES,
};
use ontocore_diagnostics::{entity_needles, find_in_source};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubclassEdge {
    pub child: String,
    pub parent: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct EntityAxiomSummary {
    pub kind: String,
    pub display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manchester: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_iri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_iri: Option<String>,
    pub editable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityAnnotationSummary {
    pub predicate: String,
    pub value: String,
    pub editable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityDetail {
    pub entity: Entity,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub axioms: Vec<EntityAxiomSummary>,
    pub annotations: Vec<EntityAnnotationSummary>,
    #[serde(skip_serializing_if = "PropertyCharacteristics::is_empty")]
    pub characteristics: PropertyCharacteristics,
    pub source: Option<SourceHint>,
    pub editable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_path: Option<String>,
}

impl OntologyCatalog {
    pub fn find_entity(&self, iri: &str) -> Option<&Entity> {
        self.data().entities.iter().find(|e| e.iri == iri)
    }

    pub fn entity_document(&self, iri: &str) -> Option<&ontocore_core::OntologyDocument> {
        if let Some(&doc_idx) = self.entity_to_document.get(iri) {
            return self.data().documents.get(doc_idx);
        }

        let entity = self.find_entity(iri)?;
        self.data().documents.iter().find(|d| document_matches_entity(entity, d))
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
            if axiom.axiom_kind != AXIOM_KIND_SUB_CLASS_OF {
                continue;
            }
            // Keep edges when the child is a known class, even if the parent is external
            // (common for OBO is_a and imports).
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
        let hierarchy = self.class_hierarchy();
        self.entity_detail_with_hierarchy(iri, &hierarchy)
    }

    pub fn entity_detail_with_hierarchy(
        &self,
        iri: &str,
        hierarchy: &ClassHierarchy,
    ) -> Option<EntityDetail> {
        let entity = self.find_entity(iri)?.clone();

        let parents = hierarchy.parents.get(iri).cloned().unwrap_or_default();
        let children = hierarchy.children.get(iri).cloned().unwrap_or_default();

        let source = self.find_source_location(iri);
        let doc = self.entity_document(iri);
        let editable = doc.is_some_and(|d| {
            matches!(
                d.format,
                ontocore_core::OntologyFormat::Turtle | ontocore_core::OntologyFormat::Obo
            ) && d.parse_status == ontocore_core::ParseStatus::Ok
        });
        let turtle_axioms = doc.is_some_and(|d| {
            d.format == ontocore_core::OntologyFormat::Turtle
                && d.parse_status == ontocore_core::ParseStatus::Ok
        });
        let document_path = doc.map(|d| d.path.display().to_string());

        let axioms: Vec<EntityAxiomSummary> = self
            .data()
            .axioms
            .iter()
            .filter(|a| a.subject == iri)
            .map(|a| axiom_summary(a, turtle_axioms))
            .collect();

        const PROMOTED: &[&str] = &[
            "http://www.w3.org/2000/01/rdf-schema#label",
            "http://www.w3.org/2000/01/rdf-schema#comment",
            "http://www.w3.org/2002/07/owl#deprecated",
        ];
        let annotations: Vec<EntityAnnotationSummary> = self
            .data()
            .annotations
            .iter()
            .filter(|a| a.subject == iri && !PROMOTED.contains(&a.predicate.as_str()))
            .map(|a| EntityAnnotationSummary {
                predicate: a.predicate.clone(),
                value: a.object.clone(),
                editable,
            })
            .collect();

        Some(EntityDetail {
            entity: entity.clone(),
            parents,
            children,
            axioms,
            annotations,
            characteristics: entity.characteristics.clone(),
            source,
            editable,
            document_path,
        })
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

        scan_file_for_iri(&doc.path, iri, &entity.short_name, &doc.namespaces)
    }

    pub fn entities_in_document(&self, doc_path: &std::path::Path) -> Vec<&Entity> {
        let doc_path = doc_path.canonicalize().unwrap_or_else(|_| doc_path.to_path_buf());
        let Some(doc_idx) = self
            .data()
            .documents
            .iter()
            .position(|d| d.path.canonicalize().unwrap_or_else(|_| d.path.clone()) == doc_path)
        else {
            return Vec::new();
        };
        self.document_entity_iris
            .get(doc_idx)
            .into_iter()
            .flatten()
            .filter_map(|iri| self.find_entity(iri))
            .collect()
    }
}

fn axiom_summary(a: &ontocore_core::Axiom, editable: bool) -> EntityAxiomSummary {
    let is_named_iri = a.object.starts_with("http://") || a.object.starts_with("https://");
    let manchester = if is_named_iri { None } else { Some(a.object.clone()) };
    let parent_iri = if a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF && is_named_iri {
        Some(a.object.clone())
    } else {
        None
    };
    let other_iri = if a.axiom_kind == AXIOM_KIND_DISJOINT_CLASS && is_named_iri {
        Some(a.object.clone())
    } else {
        None
    };
    let kind_label = match a.axiom_kind.as_str() {
        AXIOM_KIND_EQUIVALENT_CLASS => "EquivalentClasses",
        AXIOM_KIND_DISJOINT_CLASS => "DisjointClasses",
        AXIOM_KIND_DOMAIN => "Domain",
        AXIOM_KIND_RANGE => "Range",
        AXIOM_KIND_PROPERTY_CHAIN => "PropertyChain",
        AXIOM_KIND_CLASS_ASSERTION => "ClassAssertion",
        AXIOM_KIND_OBJECT_PROPERTY_ASSERTION => "ObjectPropertyAssertion",
        AXIOM_KIND_DATA_PROPERTY_ASSERTION => "DataPropertyAssertion",
        _ => "SubClassOf",
    };
    let axiom_editable = editable;
    EntityAxiomSummary {
        kind: a.axiom_kind.clone(),
        display: format!("{} {}", kind_label, a.object),
        manchester,
        parent_iri,
        other_iri,
        editable: axiom_editable,
    }
}

fn scan_file_for_iri(
    path: &std::path::Path,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<SourceHint> {
    if path.symlink_metadata().ok()?.file_type().is_symlink() {
        return None;
    }
    let content = read_to_string_capped(path, MAX_FILE_BYTES).ok()?;
    let loc = find_in_source(&content, &entity_needles(iri, short_name, namespaces));
    loc.line.map(|line| SourceHint {
        path: path.to_path_buf(),
        line,
        column: loc.column.unwrap_or(0),
    })
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

    #[test]
    fn entities_in_document_uses_build_time_index() {
        let catalog = fixture_catalog();
        let doc_path = fixture_workspace().join("example.ttl");
        let entities = catalog.entities_in_document(&doc_path);
        assert!(entities.iter().any(|e| e.short_name == "Person"));
    }

    #[test]
    fn find_source_location_skips_person_colon_in_comments() {
        let dir = tempfile::tempdir().expect("tempdir");
        let ttl_path = dir.path().join("test.ttl");
        std::fs::write(
            &ttl_path,
            concat!(
                "@prefix ex: <http://ex#> .\n",
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
                "# Note: Person: see documentation\n\n",
                "ex:Person a owl:Class .\n"
            ),
        )
        .expect("write ttl");

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build catalog");
        let source = catalog.find_source_location("http://ex#Person").expect("source location");
        let entity_line = std::fs::read_to_string(&ttl_path)
            .expect("read ttl")
            .lines()
            .position(|line| line.contains("ex:Person"))
            .expect("entity line")
            + 1;
        assert_eq!(source.line, entity_line as u64);
        assert_eq!(source.column, 0);
    }
}
