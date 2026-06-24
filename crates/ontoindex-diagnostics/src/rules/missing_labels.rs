use crate::input::DiagnosticInput;
use crate::location::{entity_needles, find_in_source};
use ontoindex_core::{
    document_for_entity, Diagnostic, DiagnosticCode, DiagnosticSeverity, EntityKind,
};
use std::path::Path;

pub fn missing_labels(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let kinds = [
        EntityKind::Class,
        EntityKind::ObjectProperty,
        EntityKind::DataProperty,
        EntityKind::AnnotationProperty,
    ];
    let mut diagnostics = Vec::new();
    for entity in data.entities {
        if !kinds.contains(&entity.kind) || !entity.labels.is_empty() {
            continue;
        }
        let doc = document_for_entity(data.documents, entity);
        let file = doc.map(|d| d.path.clone()).unwrap_or_else(|| Path::new(".").to_path_buf());
        let namespaces = doc.map(|d| &d.namespaces).cloned().unwrap_or_default();
        let text = source(&file);
        let needles = entity_needles(&entity.iri, &entity.short_name, &namespaces);
        let range = find_in_source(&text, &needles);
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::MissingLabel,
            severity: DiagnosticSeverity::Warning,
            message: format!("{} has no rdfs:label", entity.kind.as_str()),
            file,
            range,
            entity_iri: Some(entity.iri.clone()),
            quick_fix: None,
        });
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::DiagnosticInput;
    use ontoindex_core::{
        DiagnosticCode, DiagnosticSeverity, Entity, EntityKind, OntologyDocument, OntologyFormat,
        ParseStatus,
    };
    use std::collections::BTreeMap;
    use std::path::Path;

    fn empty_source(_: &Path) -> String {
        String::new()
    }

    #[test]
    fn flags_class_without_label() {
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
            iri: "http://ex/Unlabeled".to_string(),
            short_name: "Unlabeled".to_string(),
            kind: EntityKind::Class,
            ontology_id: "http://ex/".to_string(),
            source_location: Default::default(),
            labels: vec![],
            comments: vec![],
            deprecated: false,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = missing_labels(&input, &empty_source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagnosticCode::MissingLabel);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert!(diags[0].message.contains("rdfs:label"));
    }
}
