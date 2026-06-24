use crate::state::ServerState;
use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Message, Notification};
use lsp_types::{
    notification::{Notification as _, ShowMessage},
    MessageType, ShowMessageParams,
};
use ontoindex_catalog::CatalogStats;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

struct IndexJob {
    workspace: PathBuf,
    reply: Option<Sender<Result<(CatalogStats, u64), String>>>,
}

/// Background worker that runs workspace reindex off the LSP message thread.
#[derive(Clone)]
pub struct IndexWorker {
    job_tx: Sender<IndexJob>,
}

impl IndexWorker {
    pub fn spawn(
        state: ServerState,
        pending_diagnostic_publish: Arc<AtomicBool>,
        lsp_sender: Sender<Message>,
    ) -> Self {
        let (job_tx, job_rx) = crossbeam_channel::unbounded();
        thread::spawn(move || run_worker(state, job_rx, pending_diagnostic_publish, lsp_sender));
        Self { job_tx }
    }

    /// Queue a debounced background reindex (no result returned to caller).
    pub fn enqueue(&self, workspace: PathBuf) {
        let _ = self.job_tx.send(IndexJob { workspace, reply: None });
    }

    /// Queue a reindex and block until the worker finishes (used by `ontoindex/indexWorkspace`).
    pub fn enqueue_sync(&self, workspace: PathBuf) -> Result<(CatalogStats, u64), String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.job_tx
            .send(IndexJob { workspace, reply: Some(tx) })
            .map_err(|e| format!("index worker unavailable: {e}"))?;
        rx.recv().map_err(|e| format!("index worker dropped reply: {e}"))?
    }
}

fn run_worker(
    state: ServerState,
    job_rx: Receiver<IndexJob>,
    pending_diagnostic_publish: Arc<AtomicBool>,
    lsp_sender: Sender<Message>,
) {
    while let Ok(mut job) = job_rx.recv() {
        let mut replies = Vec::new();
        if let Some(reply) = job.reply.take() {
            replies.push(reply);
        }
        while let Ok(next) = job_rx.try_recv() {
            if let Some(reply) = next.reply {
                replies.push(reply);
            }
            job.workspace = next.workspace;
        }

        let result = state.index_workspace(job.workspace);
        match &result {
            Ok(_) => pending_diagnostic_publish.store(true, Ordering::SeqCst),
            Err(message) => notify_index_failure(&lsp_sender, message),
        }
        for reply in replies {
            let _ = reply.send(result.clone());
        }
    }
}

fn notify_index_failure(sender: &Sender<Message>, message: &str) {
    let params = ShowMessageParams {
        typ: MessageType::ERROR,
        message: format!("OntoIndex reindex failed: {message}"),
    };
    let notif = Notification {
        method: ShowMessage::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    let _ = sender.send(Message::Notification(notif));
}
