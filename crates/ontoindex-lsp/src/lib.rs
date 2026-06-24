//! Language server for OntoCode (stdio). Custom methods under `ontoindex/*`.
//!
//! See [docs/lsp-api.md](https://github.com/eddiethedean/ontocode/blob/main/docs/lsp-api.md).
//!
//! # API stability
//!
//! The LSP wire format and custom `ontoindex/*` methods are **pre-1.0** and may change
//! between minor releases. See the repository README for semver policy.

pub(crate) mod diagnostics;
pub(crate) mod handlers;
pub(crate) mod index_worker;
pub(crate) mod positions;
pub mod protocol;
pub(crate) mod state;

use handlers::build_catalog_snapshot;
use ontoindex_catalog::OntologyCatalog;

/// Serialize the LSP `ontoindex/getCatalogSnapshot` payload for a built catalog.
pub fn catalog_snapshot_json(
    catalog: &OntologyCatalog,
) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(build_catalog_snapshot(catalog))
}

#[cfg(test)]
mod handlers_test;

use crossbeam_channel::Sender;
use diagnostics::publish_diagnostics_for_state;
use handlers::{
    handle_custom_request, handle_initialize, handle_standard_request, StandardRequestOutcome,
};
use index_worker::IndexWorker;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response, ResponseError};
use lsp_types::{
    notification::Notification as _,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Exit, Initialized,
    },
    InitializeParams,
};
use serde_json::Value;
use state::ServerState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct PendingReindex {
    workspace: std::path::PathBuf,
    scheduled_at: Instant,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, io_threads) = Connection::stdio();
    let state = ServerState::new();
    let pending_reindex: Arc<Mutex<Option<PendingReindex>>> = Arc::new(Mutex::new(None));
    let pending_diagnostic_publish = Arc::new(AtomicBool::new(false));

    let index_worker = IndexWorker::spawn(state.clone(), connection.sender.clone());

    let timer_worker = index_worker.clone();
    let timer_pending = Arc::clone(&pending_reindex);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(workspace) = take_due_reindex_workspace(&timer_pending) {
            timer_worker.enqueue(workspace);
        }
    });

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                if let Some(resp) =
                    handle_lsp_request(&state, &index_worker, &pending_diagnostic_publish, req)
                {
                    connection.sender.send(Message::Response(resp))?;
                }
                publish_pending_diagnostics(
                    &connection.sender,
                    &state,
                    &pending_diagnostic_publish,
                );
            }
            Message::Notification(notif) => {
                if notif.method == Exit::METHOD {
                    break;
                }
                handle_notification(&state, &pending_reindex, notif);
                publish_pending_diagnostics(
                    &connection.sender,
                    &state,
                    &pending_diagnostic_publish,
                );
            }
            Message::Response(_) => {}
        }
    }

    drop(connection);
    io_threads.join()?;
    Ok(())
}

fn handle_lsp_request(
    state: &ServerState,
    index_worker: &IndexWorker,
    pending_diagnostic_publish: &Arc<AtomicBool>,
    req: Request,
) -> Option<Response> {
    let id = req.id.clone();

    if req.method == "initialize" {
        let params: InitializeParams = match parse_params(Some(req.params)) {
            Ok(p) => p,
            Err(e) => return Some(error_response(id, e)),
        };
        let result = handle_initialize(state, params);
        if state.with_catalog(|_| ()).is_some() {
            pending_diagnostic_publish.store(true, Ordering::SeqCst);
        }
        return Some(ok_response(id, result));
    }

    if req.method == "shutdown" {
        return Some(ok_response(id, Value::Null));
    }

    if req.method.starts_with("ontoindex/") {
        return match handle_custom_request(state, index_worker, &req.method, Some(req.params)) {
            Ok(result) => {
                if req.method == "ontoindex/indexWorkspace" {
                    pending_diagnostic_publish.store(true, Ordering::SeqCst);
                }
                Some(ok_response(id, result))
            }
            Err(err) => Some(ontoindex_error_response(id, err)),
        };
    }

    match handle_standard_request(state, &req.method, Some(req.params)) {
        StandardRequestOutcome::Ok(result) => Some(ok_response(id, result)),
        StandardRequestOutcome::MethodNotFound => Some(error_response(
            id,
            ResponseError {
                code: -32601,
                message: format!("Method not found: {}", req.method),
                data: None,
            },
        )),
        StandardRequestOutcome::InvalidParams(err) => Some(error_response(id, err)),
    }
}

fn handle_notification(
    state: &ServerState,
    pending_reindex: &Arc<Mutex<Option<PendingReindex>>>,
    notif: Notification,
) {
    match notif.method.as_str() {
        Initialized::METHOD => {
            if let Some(workspace) = state.workspace_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(500));
            }
        }
        "workspace/didChangeWatchedFiles" => {
            if let Some(workspace) = state.workspace_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(500));
            }
        }
        DidOpenTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    if let Err(err) = state.set_document_text(path, params.text_document.text) {
                        eprintln!("ontoindex-lsp: rejected open document: {err}");
                    }
                }
            }
            if let Some(workspace) = state.workspace_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(750));
            }
        }
        DidChangeTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    if let Some(text) = merged_document_text(state, &path, &params) {
                        if let Err(err) = state.set_document_text(path, text) {
                            eprintln!("ontoindex-lsp: rejected document change: {err}");
                        } else if let Some(workspace) = state.workspace_root() {
                            schedule_reindex(
                                pending_reindex,
                                workspace,
                                Duration::from_millis(750),
                            );
                        }
                    } else {
                        eprintln!(
                            "ontoindex-lsp: rejected document change: invalid edit range for {}",
                            params.text_document.uri.as_str()
                        );
                    }
                }
            }
        }
        DidCloseTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    state.remove_document(&path);
                }
            }
            if let Some(workspace) = state.workspace_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(750));
            }
        }
        _ => {}
    }
}

fn publish_pending_diagnostics(
    sender: &Sender<Message>,
    state: &ServerState,
    pending: &Arc<AtomicBool>,
) {
    if !pending.swap(false, Ordering::SeqCst) {
        return;
    }
    publish_diagnostics_for_state(sender, state);
}

fn document_path_from_uri(
    state: &ServerState,
    uri: &lsp_types::Uri,
) -> Result<std::path::PathBuf, String> {
    let root = state.workspace_root().ok_or_else(|| "workspace not initialized".to_string())?;
    ontoindex_core::resolve_lsp_document_path(uri.as_str(), &root)
}

fn merged_document_text(
    state: &ServerState,
    path: &std::path::Path,
    params: &lsp_types::DidChangeTextDocumentParams,
) -> Option<String> {
    let mut text = state.document_text(path)?;
    for change in &params.content_changes {
        if let Some(range) = &change.range {
            text = apply_text_change(&text, range, &change.text)?;
        } else {
            text = change.text.clone();
        }
    }
    Some(text)
}

fn apply_text_change(text: &str, range: &lsp_types::Range, new_text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let start_line = range.start.line as usize;
    let end_line = range.end.line as usize;
    if start_line >= lines.len() {
        eprintln!(
            "ontoindex-lsp: text change start line {} out of range ({} lines)",
            start_line,
            lines.len()
        );
        return None;
    }
    let start_line_str = lines.get(start_line).copied().unwrap_or("");
    let end_line_str = lines.get(end_line).copied().unwrap_or("");
    let start_col = utf16_offset_to_byte(start_line_str, range.start.character);
    let end_col = utf16_offset_to_byte(end_line_str, range.end.character);

    if end_line >= lines.len() && end_line != start_line {
        eprintln!(
            "ontoindex-lsp: text change end line {} out of range ({} lines)",
            end_line,
            lines.len()
        );
        return None;
    }

    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i < start_line {
            result.push_str(line);
            result.push('\n');
        } else if i == start_line {
            result.push_str(&line[..start_col.min(line.len())]);
            result.push_str(new_text);
            if i == end_line {
                result.push_str(&line[end_col.min(line.len())..]);
            }
            result.push('\n');
        } else if i > start_line && i < end_line {
            continue;
        } else if i == end_line && end_line != start_line {
            result.push_str(&line[end_col.min(line.len())..]);
            result.push('\n');
        } else if i > end_line {
            result.push_str(line);
            result.push('\n');
        }
    }
    if result.ends_with('\n') {
        result.pop();
    }
    Some(result)
}

fn utf16_offset_to_byte(line: &str, utf16_col: u32) -> usize {
    let mut utf16_seen = 0u32;
    for (byte_idx, ch) in line.char_indices() {
        if utf16_seen >= utf16_col {
            return byte_idx;
        }
        utf16_seen += ch.len_utf16() as u32;
    }
    line.len()
}

fn schedule_reindex(
    pending: &Arc<Mutex<Option<PendingReindex>>>,
    workspace: std::path::PathBuf,
    delay: Duration,
) {
    if let Ok(mut guard) = pending.lock() {
        *guard = Some(PendingReindex { workspace, scheduled_at: Instant::now() + delay });
    }
}

fn take_due_reindex_workspace(
    pending: &Arc<Mutex<Option<PendingReindex>>>,
) -> Option<std::path::PathBuf> {
    let Ok(mut guard) = pending.lock() else {
        return None;
    };
    let entry = guard.as_ref()?;
    if Instant::now() < entry.scheduled_at {
        return None;
    }
    let workspace = entry.workspace.clone();
    *guard = None;
    Some(workspace)
}

fn parse_params<T: serde::de::DeserializeOwned>(params: Option<Value>) -> Result<T, ResponseError> {
    serde_json::from_value(params.unwrap_or(Value::Null)).map_err(|e| ResponseError {
        code: -32602,
        message: format!("invalid params: {e}"),
        data: None,
    })
}

fn ok_response(id: RequestId, result: impl serde::Serialize) -> Response {
    Response { id, result: Some(serde_json::to_value(result).unwrap_or(Value::Null)), error: None }
}

fn error_response(id: RequestId, error: ResponseError) -> Response {
    Response { id, result: None, error: Some(error) }
}

fn ontoindex_error_response(id: RequestId, err: protocol::LspErrorPayload) -> Response {
    Response {
        id,
        result: None,
        error: Some(ResponseError {
            code: -32000,
            message: err.message.clone(),
            data: Some(serde_json::to_value(err).unwrap_or(Value::Null)),
        }),
    }
}

#[cfg(test)]
mod debounce_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn take_due_clears_pending_after_delay() {
        let pending: Arc<Mutex<Option<PendingReindex>>> = Arc::new(Mutex::new(None));
        let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        schedule_reindex(&pending, ws.clone(), Duration::from_millis(30));
        assert!(pending.lock().unwrap().is_some());
        thread::sleep(Duration::from_millis(60));
        let taken = take_due_reindex_workspace(&pending);
        assert_eq!(taken, Some(ws));
        assert!(pending.lock().unwrap().is_none());
    }
}
