use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::notification::Notification as _;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Uri};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;

use crate::positions::byte_col_to_utf16;
use crate::state::path_to_uri;

static PUBLISHED_URIS: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

pub fn publish_catalog_diagnostics(
    sender: &Sender<Message>,
    documents: &[ontoindex_core::OntologyDocument],
    diagnostics: &[ontoindex_core::Diagnostic],
    document_text: &dyn Fn(&Path) -> Option<String>,
) {
    let mut by_file: BTreeMap<&Path, Vec<&ontoindex_core::Diagnostic>> = BTreeMap::new();
    for diag in diagnostics {
        by_file.entry(diag.file.as_path()).or_default().push(diag);
    }

    let mut current_uris = BTreeSet::new();

    for doc in documents {
        let uri = path_to_uri(&doc.path);
        current_uris.insert(uri.clone());
        let file_diags = by_file.remove(doc.path.as_path()).unwrap_or_default();
        send_publish(sender, &uri, file_diags, document_text);
    }

    for (path, diags) in by_file {
        let uri = path_to_uri(path);
        current_uris.insert(uri.clone());
        send_publish(sender, &uri, diags, document_text);
    }

    let stale: Vec<String> = {
        let mut published = PUBLISHED_URIS.lock().unwrap_or_else(|e| e.into_inner());
        let stale: Vec<String> = published.difference(&current_uris).cloned().collect();
        *published = current_uris;
        stale
    };

    for uri in stale {
        send_empty_publish(sender, &uri);
    }
}

fn send_publish(
    sender: &Sender<Message>,
    uri: &str,
    diagnostics: Vec<&ontoindex_core::Diagnostic>,
    document_text: &dyn Fn(&Path) -> Option<String>,
) {
    let Ok(lsp_uri) = Uri::from_str(uri) else {
        eprintln!("ontoindex-lsp: skip diagnostics for invalid URI: {uri}");
        return;
    };
    let params = lsp_types::PublishDiagnosticsParams {
        uri: lsp_uri,
        diagnostics: diagnostics.into_iter().map(|d| to_lsp_diagnostic(d, document_text)).collect(),
        version: None,
    };
    let notif = lsp_server::Notification {
        method: PublishDiagnostics::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    if sender.send(Message::Notification(notif)).is_err() {
        eprintln!("ontoindex-lsp: failed to send publishDiagnostics");
    }
}

fn send_empty_publish(sender: &Sender<Message>, uri: &str) {
    let Ok(lsp_uri) = Uri::from_str(uri) else {
        return;
    };
    let params =
        lsp_types::PublishDiagnosticsParams { uri: lsp_uri, diagnostics: vec![], version: None };
    let notif = lsp_server::Notification {
        method: PublishDiagnostics::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    let _ = sender.send(Message::Notification(notif));
}

fn to_lsp_diagnostic(
    diag: &ontoindex_core::Diagnostic,
    document_text: &dyn Fn(&Path) -> Option<String>,
) -> Diagnostic {
    let line_idx = diag.range.line.unwrap_or(1).saturating_sub(1) as u32;
    let byte_col = diag.range.column.unwrap_or(0) as usize;
    let line_text = document_text(&diag.file)
        .and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()));
    let character =
        line_text.as_deref().map(|l| byte_col_to_utf16(l, byte_col)).unwrap_or(byte_col as u32);
    Diagnostic {
        range: Range {
            start: Position { line: line_idx, character },
            end: Position { line: line_idx, character: character.saturating_add(1) },
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
        let lsp = to_lsp_diagnostic(&diag, &|_| None);
        assert_eq!(lsp.range.start.line, 1);
        assert_eq!(lsp.range.start.character, 4);
        assert_eq!(lsp.code, Some(NumberOrString::String("broken_import".to_string())));
    }

    #[test]
    fn publish_diagnostics_clears_stale_uris() {
        use crossbeam_channel::unbounded;
        use lsp_server::Message;
        use lsp_types::notification::PublishDiagnostics;
        use ontoindex_core::{
            OntologyDocument, OntologyFormat, ParseStatus, SourceLocation,
        };
        use std::collections::BTreeMap;
        use std::time::Duration;

        let dir = tempfile::tempdir().unwrap();
        let path_a = dir.path().join("a.ttl");
        let path_b = dir.path().join("b.ttl");
        std::fs::write(&path_a, "@prefix ex: <http://ex/> .\n").unwrap();
        std::fs::write(&path_b, "@prefix ex: <http://ex/> .\n").unwrap();

        let doc = |path: &std::path::Path| OntologyDocument {
            id: "doc-1".to_string(),
            path: path.to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        };

        let diagnostic = |path: &std::path::Path| ontoindex_core::Diagnostic {
            code: DiagnosticCode::UndefinedPrefix,
            severity: DiagnosticSeverity::Error,
            message: "undefined prefix: un:".to_string(),
            file: path.to_path_buf(),
            range: SourceLocation { line: Some(1), column: Some(0) },
            entity_iri: None,
            quick_fix: None,
        };

        let (tx, rx) = unbounded();
        publish_catalog_diagnostics(
            &tx,
            &[doc(&path_a), doc(&path_b)],
            &[diagnostic(&path_a), diagnostic(&path_b)],
            &|_| None,
        );

        let mut publish_count = 0usize;
        while let Ok(Message::Notification(notif)) =
            rx.recv_timeout(Duration::from_millis(100))
        {
            if notif.method == PublishDiagnostics::METHOD {
                publish_count += 1;
            }
        }
        assert_eq!(publish_count, 2);

        publish_catalog_diagnostics(&tx, &[doc(&path_a)], &[diagnostic(&path_a)], &|_| None);

        let mut final_notifications = Vec::new();
        while let Ok(Message::Notification(notif)) =
            rx.recv_timeout(Duration::from_millis(100))
        {
            if notif.method == PublishDiagnostics::METHOD {
                final_notifications.push(notif);
            }
        }
        assert_eq!(final_notifications.len(), 2);
        let cleared = final_notifications
            .iter()
            .any(|n| n.params.get("diagnostics").and_then(|d| d.as_array()).is_some_and(|a| a.is_empty()));
        assert!(cleared, "expected empty diagnostics publish for stale URI");
    }
}
