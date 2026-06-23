use crate::input::DiagnosticInput;
use crate::location::find_in_source;
use ontoindex_core::{Diagnostic, DiagnosticCode, DiagnosticSeverity, ParseStatus};
use std::collections::BTreeSet;
use std::path::Path;

pub fn broken_imports(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let known: BTreeSet<String> = data
        .documents
        .iter()
        .filter(|d| d.parse_status != ParseStatus::Error)
        .flat_map(|d| {
            let mut iris = Vec::new();
            if let Some(base) = &d.base_iri {
                iris.push(base.clone());
            }
            iris.push(format!("file://{}", d.path.display()));
            iris
        })
        .collect();

    let mut diagnostics = Vec::new();
    for imp in data.imports {
        if known.contains(&imp.import_iri) {
            continue;
        }
        let doc = data.documents.iter().find(|d| d.id == imp.ontology_id);
        let file = doc.map(|d| d.path.clone()).unwrap_or_else(|| Path::new(".").to_path_buf());

        let text = source(&file);
        let range =
            find_in_source(&text, &[imp.import_iri.clone(), format!("<{}>", imp.import_iri)]);

        diagnostics.push(Diagnostic {
            code: DiagnosticCode::BrokenImport,
            severity: DiagnosticSeverity::Error,
            message: format!("import not found in workspace: {}", imp.import_iri),
            file,
            range,
            entity_iri: Some(imp.ontology_id.clone()),
            quick_fix: None,
        });
    }
    diagnostics
}
