use crate::location::{entity_needles, find_in_source};
use crate::input::DiagnosticInput;
use ontoindex_core::{Diagnostic, DiagnosticCode, DiagnosticSeverity};
use std::collections::BTreeMap;
use std::path::Path;

pub fn duplicate_labels(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut by_label: BTreeMap<String, Vec<&ontoindex_core::Entity>> = BTreeMap::new();
    for entity in data.entities {
        for label in &entity.labels {
            let key = normalize_label(label);
            if key.is_empty() {
                continue;
            }
            by_label.entry(key).or_default().push(entity);
        }
    }

    let mut diagnostics = Vec::new();
    for (label, entities) in by_label {
        if entities.len() < 2 {
            continue;
        }
        for entity in entities {
            let doc = data.documents.iter().find(|d| d.id == entity.ontology_id);
            let file = doc
                .map(|d| d.path.clone())
                .unwrap_or_else(|| Path::new(".").to_path_buf());
            let namespaces = doc.map(|d| &d.namespaces).cloned().unwrap_or_default();
            let text = source(&file);
            let needles = entity_needles(&entity.iri, &entity.short_name, &namespaces);
            let range = find_in_source(&text, &needles);
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::DuplicateLabel,
                severity: DiagnosticSeverity::Warning,
                message: format!("duplicate label \"{label}\" (also used by other entities)"),
                file,
                range,
                entity_iri: Some(entity.iri.clone()),
                quick_fix: None,
            });
        }
    }
    diagnostics
}

fn normalize_label(label: &str) -> String {
    label.trim_matches('"').trim().to_ascii_lowercase()
}
