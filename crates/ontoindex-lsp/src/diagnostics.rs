use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::notification::Notification as _;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Uri};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use crate::state::path_to_uri;

pub fn publish_catalog_diagnostics(
    sender: &Sender<Message>,
    documents: &[ontoindex_core::OntologyDocument],
    diagnostics: &[ontoindex_core::Diagnostic],
) {
    let mut by_file: BTreeMap<&Path, Vec<&ontoindex_core::Diagnostic>> = BTreeMap::new();
    for diag in diagnostics {
        by_file.entry(diag.file.as_path()).or_default().push(diag);
    }

    for doc in documents {
        let uri = path_to_lsp_uri(&doc.path);
        let file_diags = by_file.remove(doc.path.as_path()).unwrap_or_default();
        send_publish(sender, &uri, file_diags);
    }

    for (path, diags) in by_file {
        let uri = path_to_lsp_uri(path);
        send_publish(sender, &uri, diags);
    }
}

fn send_publish(
    sender: &Sender<Message>,
    uri: &str,
    diagnostics: Vec<&ontoindex_core::Diagnostic>,
) {
    let lsp_uri = Uri::from_str(uri).unwrap_or_else(|_| Uri::from_str("file:///").expect("uri"));
    let params = lsp_types::PublishDiagnosticsParams {
        uri: lsp_uri,
        diagnostics: diagnostics.into_iter().map(to_lsp_diagnostic).collect(),
        version: None,
    };
    let notif = lsp_server::Notification {
        method: PublishDiagnostics::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    let _ = sender.send(Message::Notification(notif));
}

fn to_lsp_diagnostic(diag: &ontoindex_core::Diagnostic) -> Diagnostic {
    let line = diag.range.line.unwrap_or(1).saturating_sub(1) as u32;
    let column = diag.range.column.unwrap_or(0) as u32;
    Diagnostic {
        range: Range {
            start: Position { line, character: column },
            end: Position { line, character: column.saturating_add(1) },
        },
        severity: Some(match diag.severity {
            ontoindex_core::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
            ontoindex_core::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
            ontoindex_core::DiagnosticSeverity::Info => DiagnosticSeverity::INFORMATION,
        }),
        code: Some(NumberOrString::String(diag.code.as_str().to_string())),
        source: Some("ontoindex".to_string()),
        message: diag.message.clone(),
        ..Default::default()
    }
}

fn path_to_lsp_uri(path: &Path) -> String {
    path_to_uri(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontoindex_core::{DiagnosticCode, DiagnosticSeverity, SourceLocation};
    use std::path::PathBuf;

    #[test]
    fn maps_diagnostic_code_to_lsp() {
        let diag = ontoindex_core::Diagnostic {
            code: DiagnosticCode::BrokenImport,
            severity: DiagnosticSeverity::Error,
            message: "test".to_string(),
            file: PathBuf::from("a.ttl"),
            range: SourceLocation { line: Some(2), column: Some(4) },
            entity_iri: None,
            quick_fix: None,
        };
        let lsp = to_lsp_diagnostic(&diag);
        assert_eq!(lsp.range.start.line, 1);
        assert_eq!(lsp.range.start.character, 4);
        assert_eq!(lsp.code, Some(NumberOrString::String("broken_import".to_string())));
    }
}
