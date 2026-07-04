//! Per-document snapshots for content-hash incremental reindexing.

use ontocore_core::{Annotation, Axiom, Diagnostic, Entity, Import, Namespace, OntologyDocument};
use oxigraph::model::Quad;
use std::path::Path;

/// Cached parse + semantics for one ontology file at a specific content hash.
#[derive(Debug, Clone)]
pub(crate) struct DocumentSnapshot {
    pub content_hash: String,
    pub document: OntologyDocument,
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub namespace_rows: Vec<Namespace>,
    pub imports: Vec<Import>,
    pub quads: Vec<Quad>,
    pub triple_count: usize,
    pub bridge_warning: Option<Diagnostic>,
}

impl DocumentSnapshot {
    pub fn with_doc_id(&self, doc_id: &str) -> Self {
        let mut snap = self.clone();
        snap.document.id = doc_id.to_string();
        let old_id = self.document.id.clone();
        for entity in &mut snap.entities {
            if entity.ontology_id == old_id {
                entity.ontology_id = doc_id.to_string();
            }
        }
        for ann in &mut snap.annotations {
            if ann.ontology_id == old_id {
                ann.ontology_id = doc_id.to_string();
            }
        }
        for ax in &mut snap.axioms {
            if ax.ontology_id == old_id {
                ax.ontology_id = doc_id.to_string();
            }
        }
        for ns in &mut snap.namespace_rows {
            if ns.ontology_id == old_id {
                ns.ontology_id = doc_id.to_string();
            }
        }
        for imp in &mut snap.imports {
            if imp.ontology_id == old_id {
                imp.ontology_id = doc_id.to_string();
            }
        }
        snap
    }
}

pub(crate) fn effective_content_hash(disk_hash: &str, override_text: Option<&str>) -> String {
    if let Some(text) = override_text {
        content_hash_text(text)
    } else {
        disk_hash.to_string()
    }
}

pub(crate) fn content_hash_text(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

pub(crate) fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncrementalStats {
    FullBuild,
    Incremental { reused: usize, reparsed: usize, removed: usize },
}

impl IncrementalStats {
    pub fn reused_documents(&self) -> usize {
        match self {
            Self::FullBuild => 0,
            Self::Incremental { reused, .. } => *reused,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontocore_core::{OntologyFormat, ParseStatus, SourceLocation};
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn content_hash_is_stable() {
        let h1 = content_hash_text("hello");
        let h2 = content_hash_text("hello");
        assert_eq!(h1, h2);
        assert_ne!(h1, content_hash_text("world"));
    }

    #[test]
    fn remap_doc_id_updates_ontology_ids() {
        let snap = DocumentSnapshot {
            content_hash: "abc".to_string(),
            document: OntologyDocument {
                id: "doc-1".to_string(),
                path: PathBuf::from("a.ttl"),
                format: OntologyFormat::Turtle,
                base_iri: None,
                imports: vec![],
                namespaces: BTreeMap::new(),
                parse_status: ParseStatus::Ok,
                content_hash: "abc".to_string(),
                modified_time: 0,
                parse_message: None,
                parse_error_location: None,
            },
            entities: vec![Entity {
                iri: "http://ex#A".to_string(),
                kind: ontocore_core::EntityKind::Class,
                short_name: "A".to_string(),
                ontology_id: "doc-1".to_string(),
                source_location: SourceLocation::default(),
                labels: vec![],
                comments: vec![],
                deprecated: false,
                obo_id: None,
            }],
            annotations: vec![],
            axioms: vec![],
            namespace_rows: vec![],
            imports: vec![],
            quads: vec![],
            triple_count: 0,
            bridge_warning: None,
        };
        let remapped = snap.with_doc_id("doc-2");
        assert_eq!(remapped.document.id, "doc-2");
        assert_eq!(remapped.entities[0].ontology_id, "doc-2");
    }
}
