use crate::input::DiagnosticInput;
use crate::location::{entity_needles, find_in_source};
use ontoindex_core::{
    document_for_entity, Diagnostic, DiagnosticCode, DiagnosticSeverity, EntityKind,
    AXIOM_KIND_SUB_CLASS_OF,
};
use std::collections::BTreeSet;
use std::path::Path;

const BUILTIN_ROOTS: &[&str] = &[
    "http://www.w3.org/2002/07/owl#Thing",
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#Resource",
    "http://www.w3.org/2000/01/rdf-schema#Resource",
];

pub fn orphan_classes(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let classes: Vec<_> = data.entities.iter().filter(|e| e.kind == EntityKind::Class).collect();
    let entity_iris: BTreeSet<&str> = data.entities.iter().map(|e| e.iri.as_str()).collect();
    let child_set: BTreeSet<&str> = data
        .axioms
        .iter()
        .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF)
        .map(|a| a.subject.as_str())
        .collect();
    let parent_object_set: BTreeSet<&str> = data
        .axioms
        .iter()
        .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF)
        .map(|a| a.object.as_str())
        .collect();

    let mut diagnostics = Vec::new();
    for class in classes {
        if BUILTIN_ROOTS.contains(&class.iri.as_str()) {
            continue;
        }
        // Taxonomy roots: other classes subclass this entity in the workspace.
        if parent_object_set.contains(class.iri.as_str()) {
            continue;
        }
        let is_orphan = if !child_set.contains(class.iri.as_str()) {
            true
        } else {
            let parents: Vec<_> = data
                .axioms
                .iter()
                .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF && a.subject == class.iri)
                .map(|a| a.object.as_str())
                .collect();
            !parents.iter().any(|p| entity_iris.contains(p))
        };
        if !is_orphan {
            continue;
        }
        let doc = document_for_entity(data.documents, class);
        let file = doc.map(|d| d.path.clone()).unwrap_or_else(|| Path::new(".").to_path_buf());
        let namespaces = doc.map(|d| &d.namespaces).cloned().unwrap_or_default();
        let text = source(&file);
        let needles = entity_needles(&class.iri, &class.short_name, &namespaces);
        let range = find_in_source(&text, &needles);
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::OrphanClass,
            severity: DiagnosticSeverity::Warning,
            message: "class has no parent in workspace catalog".to_string(),
            file,
            range,
            entity_iri: Some(class.iri.clone()),
            quick_fix: None,
        });
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::DiagnosticInput;
    use ontoindex_core::{Entity, OntologyDocument, OntologyFormat, ParseStatus};
    use std::collections::BTreeMap;
    use std::path::Path;

    fn empty_source(_: &Path) -> String {
        String::new()
    }

    #[test]
    fn orphan_class_detected_without_catalog_parent() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("test.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let entities = vec![Entity {
            iri: "http://ex/Orphan".to_string(),
            short_name: "Orphan".to_string(),
            kind: EntityKind::Class,
            ontology_id: "doc-1".to_string(),
            source_location: Default::default(),
            labels: vec!["\"Orphan\"".to_string()],
            comments: vec![],
            deprecated: false,
            obo_id: None,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = orphan_classes(&input, &empty_source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagnosticCode::OrphanClass);
    }

    #[test]
    fn root_class_with_children_not_orphan() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("test.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let entities = vec![
            Entity {
                iri: "http://ex/Thing".to_string(),
                short_name: "Thing".to_string(),
                kind: EntityKind::Class,
                ontology_id: "http://ex/".to_string(),
                source_location: Default::default(),
                labels: vec![],
                comments: vec![],
                deprecated: false,
                obo_id: None,
            },
            Entity {
                iri: "http://ex/Person".to_string(),
                short_name: "Person".to_string(),
                kind: EntityKind::Class,
                ontology_id: "http://ex/".to_string(),
                source_location: Default::default(),
                labels: vec![],
                comments: vec![],
                deprecated: false,
                obo_id: None,
            },
        ];
        let axioms = vec![ontoindex_core::Axiom {
            id: "a1".to_string(),
            ontology_id: "http://ex/".to_string(),
            subject: "http://ex/Person".to_string(),
            predicate: "subClassOf".to_string(),
            object: "http://ex/Thing".to_string(),
            axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
            source_location: Default::default(),
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &axioms,
            namespaces: &[],
            imports: &[],
        };
        let diags = orphan_classes(&input, &empty_source);
        assert!(
            !diags.iter().any(|d| d.entity_iri.as_deref() == Some("http://ex/Thing")),
            "root with children should not be orphan"
        );
    }
}
