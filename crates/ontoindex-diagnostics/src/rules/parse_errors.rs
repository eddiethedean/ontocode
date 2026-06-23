use crate::input::DiagnosticInput;
use ontoindex_core::{
    Diagnostic, DiagnosticCode, DiagnosticSeverity, ParseStatus, SourceLocation,
};
use std::path::Path;

pub fn parse_errors(
    data: &DiagnosticInput<'_>,
    _source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for doc in data.documents {
        if doc.parse_status != ParseStatus::Error {
            continue;
        }
        let message = doc
            .parse_message
            .clone()
            .unwrap_or_else(|| "parse error".to_string());
        let range = doc.parse_error_location.clone().unwrap_or(SourceLocation {
            line: Some(1),
            column: Some(0),
        });
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::ParseError,
            severity: DiagnosticSeverity::Error,
            message,
            file: doc.path.clone(),
            range,
            entity_iri: None,
            quick_fix: None,
        });
    }
    diagnostics
}
