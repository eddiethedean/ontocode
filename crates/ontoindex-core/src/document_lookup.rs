use crate::{Entity, OntologyDocument};

/// Normalize an ontology IRI for prefix/suffix comparison.
pub fn normalize_iri(iri: &str) -> String {
    iri.trim_end_matches('#').trim_end_matches('/').to_string()
}

/// Canonical `file://` path for workspace document matching.
pub fn file_uri_for_path(path: &std::path::Path) -> String {
    let display = path.display().to_string();
    if display.starts_with('/') {
        format!("file://{display}")
    } else {
        format!("file:///{display}")
    }
}

/// Whether `entity` belongs to `doc` (handles ontology IRI vs `doc-N` id mismatch).
pub fn document_matches_entity(entity: &Entity, doc: &OntologyDocument) -> bool {
    if entity.ontology_id == doc.id {
        return true;
    }
    if let Some(base) = &doc.base_iri {
        if normalize_iri(base) == normalize_iri(&entity.ontology_id) {
            return true;
        }
        if entity.iri.starts_with(base) {
            return true;
        }
    }
    doc.path.to_string_lossy().contains(&entity.ontology_id)
}

/// Whether an import ontology id matches a document.
pub fn document_matches_ontology_id(ontology_id: &str, doc: &OntologyDocument) -> bool {
    if ontology_id == doc.id {
        return true;
    }
    if let Some(base) = &doc.base_iri {
        if normalize_iri(base) == normalize_iri(ontology_id) {
            return true;
        }
    }
    false
}

/// Find the document that owns an entity by ontology id.
pub fn document_for_entity<'a>(
    documents: &'a [OntologyDocument],
    entity: &Entity,
) -> Option<&'a OntologyDocument> {
    documents.iter().find(|d| document_matches_entity(entity, d))
}

/// Find the document for an import's owning ontology id.
pub fn document_for_ontology_id<'a>(
    documents: &'a [OntologyDocument],
    ontology_id: &str,
) -> Option<&'a OntologyDocument> {
    documents.iter().find(|d| document_matches_ontology_id(ontology_id, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EntityKind, OntologyFormat, ParseStatus};
    use std::collections::BTreeMap;
    use std::path::Path;

    #[test]
    fn matches_entity_by_ontology_iri() {
        let doc = OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("people.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://example.org/people".to_string()),
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        };
        let entity = Entity {
            iri: "http://example.org/people#Person".to_string(),
            short_name: "Person".to_string(),
            kind: EntityKind::Class,
            ontology_id: "http://example.org/people".to_string(),
            source_location: Default::default(),
            labels: vec![],
            comments: vec![],
            deprecated: false,
            obo_id: None,
        };
        assert!(document_matches_entity(&entity, &doc));
    }
}
