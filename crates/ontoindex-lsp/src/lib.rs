//! Language server for OntoCode (stdio). Custom methods under `ontoindex/*`.
//!
//! See [docs/lsp-api.md](https://github.com/eddiethedean/ontocode/blob/main/docs/lsp-api.md).

pub(crate) mod handlers;
pub(crate) mod protocol;
pub(crate) mod state;

#[cfg(test)]
mod handlers_test;

use handlers::{handle_custom_request, handle_initialize, handle_standard_request};
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response, ResponseError};
use lsp_types::{
    notification::Notification as _,
    notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Initialized},
    InitializeParams,
};
use serde_json::Value;
use state::ServerState;
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

    let timer_state = state.clone();
    let timer_pending = Arc::clone(&pending_reindex);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        flush_due_reindex(&timer_state, &timer_pending);
    });

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                if let Some(resp) = handle_lsp_request(&state, &pending_reindex, req) {
                    connection.sender.send(Message::Response(resp))?;
                }
            }
            Message::Notification(notif) => {
                handle_notification(&state, &pending_reindex, notif);
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
    pending_reindex: &Arc<Mutex<Option<PendingReindex>>>,
    req: Request,
) -> Option<Response> {
    flush_due_reindex(state, pending_reindex);
    let id = req.id.clone();

    if req.method == "initialize" {
        let params: InitializeParams = match parse_params(Some(req.params)) {
            Ok(p) => p,
            Err(e) => return Some(error_response(id, e)),
        };
        let result = handle_initialize(state, params);
        return Some(ok_response(id, result));
    }

    if req.method == "shutdown" {
        return Some(ok_response(id, Value::Null));
    }

    if req.method.starts_with("ontoindex/") {
        return match handle_custom_request(state, &req.method, Some(req.params)) {
            Ok(result) => Some(ok_response(id, result)),
            Err(err) => Some(ontoindex_error_response(id, err)),
        };
    }

    if let Some(result) = handle_standard_request(state, &req.method, Some(req.params)) {
        return Some(ok_response(id, result));
    }

    None
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
                flush_due_reindex(state, pending_reindex);
            }
        }
        DidOpenTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    state.set_document_text(path, params.text_document.text);
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
                        state.set_document_text(path, text);
                    }
                }
            }
            if let Some(workspace) = state.workspace_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(750));
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

fn document_path_from_uri(
    state: &ServerState,
    uri: &lsp_types::Uri,
) -> Result<std::path::PathBuf, String> {
    let root = state.workspace_root().ok_or_else(|| "workspace not initialized".to_string())?;
    ontoindex_core::resolve_document_path(uri.as_str(), &root)
}

fn merged_document_text(
    state: &ServerState,
    path: &std::path::Path,
    params: &lsp_types::DidChangeTextDocumentParams,
) -> Option<String> {
    let mut text = state.document_text(path)?;
    for change in &params.content_changes {
        if let Some(range) = &change.range {
            text = apply_text_change(&text, range, &change.text);
        } else {
            text = change.text.clone();
        }
    }
    Some(text)
}

fn apply_text_change(text: &str, range: &lsp_types::Range, new_text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start_line = range.start.line as usize;
    let end_line = range.end.line as usize;
    if start_line >= lines.len() {
        return text.to_string();
    }
    let start_line_str = lines.get(start_line).copied().unwrap_or("");
    let end_line_str = lines.get(end_line).copied().unwrap_or("");
    let start_col = utf16_offset_to_byte(start_line_str, range.start.character);
    let end_col = utf16_offset_to_byte(end_line_str, range.end.character);

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
    result
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

fn flush_due_reindex(state: &ServerState, pending: &Arc<Mutex<Option<PendingReindex>>>) {
    let due = {
        let Ok(mut guard) = pending.lock() else {
            return;
        };
        let Some(entry) = guard.as_ref() else {
            return;
        };
        if Instant::now() < entry.scheduled_at {
            return;
        }
        let workspace = entry.workspace.clone();
        *guard = None;
        workspace
    };
    let _ = state.index_workspace(due);
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
    fn flush_clears_pending_after_delay() {
        let pending: Arc<Mutex<Option<PendingReindex>>> = Arc::new(Mutex::new(None));
        let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        schedule_reindex(&pending, ws, Duration::from_millis(30));
        assert!(pending.lock().unwrap().is_some());
        thread::sleep(Duration::from_millis(60));
        let state = ServerState::new();
        flush_due_reindex(&state, &pending);
        assert!(pending.lock().unwrap().is_none());
    }
}
