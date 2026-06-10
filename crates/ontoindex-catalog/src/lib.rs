mod builder;
mod entity_api;

pub use builder::{IndexBuilder, OntologyCatalog};
pub use entity_api::{ClassHierarchy, EntityDetail, SourceHint, SubclassEdge};

use ontoindex_core::{Annotation, Axiom, Entity, Import, Namespace, OntologyDocument, ParseStatus};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CatalogStats {
    pub ontology_count: usize,
    pub class_count: usize,
    pub object_property_count: usize,
    pub data_property_count: usize,
    pub annotation_property_count: usize,
    pub individual_count: usize,
    pub axiom_count: usize,
    pub annotation_count: usize,
    pub triple_count: usize,
    pub error_count: usize,
}

#[derive(Debug, Clone)]
pub struct OntologyCatalogData {
    pub documents: Vec<OntologyDocument>,
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub namespaces: Vec<Namespace>,
    pub imports: Vec<Import>,
    pub triple_count: usize,
}

impl OntologyCatalogData {
    pub fn stats(&self) -> CatalogStats {
        CatalogStats {
            ontology_count: self.documents.len(),
            class_count: self
                .entities
                .iter()
                .filter(|e| e.kind == ontoindex_core::EntityKind::Class)
                .count(),
            object_property_count: self
                .entities
                .iter()
                .filter(|e| e.kind == ontoindex_core::EntityKind::ObjectProperty)
                .count(),
            data_property_count: self
                .entities
                .iter()
                .filter(|e| e.kind == ontoindex_core::EntityKind::DataProperty)
                .count(),
            annotation_property_count: self
                .entities
                .iter()
                .filter(|e| e.kind == ontoindex_core::EntityKind::AnnotationProperty)
                .count(),
            individual_count: self
                .entities
                .iter()
                .filter(|e| e.kind == ontoindex_core::EntityKind::Individual)
                .count(),
            axiom_count: self.axioms.len(),
            annotation_count: self.annotations.len(),
            triple_count: self.triple_count,
            error_count: self
                .documents
                .iter()
                .filter(|d| d.parse_status == ParseStatus::Error)
                .count(),
        }
    }
}
