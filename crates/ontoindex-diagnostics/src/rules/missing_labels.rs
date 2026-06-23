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
