pub mod handlers;
pub mod protocol;
pub mod state;

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
use std::time::{Duration, Instant};

struct PendingReindex {
    workspace: std::path::PathBuf,
    scheduled_at: Instant,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, io_threads) = Connection::stdio();
    let state = ServerState::new();
    let pending_reindex: Arc<Mutex<Option<PendingReindex>>> = Arc::new(Mutex::new(None));

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
    _pending_reindex: &Arc<Mutex<Option<PendingReindex>>>,
    req: Request,
) -> Option<Response> {
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
            if let Some(workspace) = state.workspace() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(500));
                flush_due_reindex(state, pending_reindex);
            }
        }
        DidOpenTextDocument::METHOD
        | DidChangeTextDocument::METHOD
        | DidCloseTextDocument::METHOD => {
            if let Some(workspace) = state.workspace() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(750));
                flush_due_reindex(state, pending_reindex);
            }
        }
        _ => {}
    }
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

fn ontoindex_error_response(id: RequestId, err: protocol::OntoIndexError) -> Response {
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
