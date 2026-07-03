use crate::diagnostics::publish_diagnostics_for_state;
use crate::protocol::RunRobotResult;
use crate::state::ServerState;
use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Message, Notification};
use lsp_types::{
    notification::{Notification as _, ShowMessage},
    MessageType, ShowMessageParams,
};
use ontocore_catalog::CatalogStats;
use std::path::PathBuf;
use std::thread;

struct IndexJob {
    workspace: PathBuf,
    reply: Option<Sender<Result<(CatalogStats, u64), String>>>,
}

struct RobotJob {
    robot_path: Option<String>,
    args: Vec<String>,
    reply: Sender<Result<RunRobotResult, String>>,
}

enum WorkerJob {
    Index(IndexJob),
    Robot(RobotJob),
}

/// Background worker that runs workspace reindex and ROBOT CLI off the LSP message thread.
#[derive(Clone)]
pub struct IndexWorker {
    job_tx: Sender<WorkerJob>,
}

impl IndexWorker {
    pub fn spawn(state: ServerState, lsp_sender: Sender<Message>) -> Self {
        let (job_tx, job_rx) = crossbeam_channel::unbounded();
        thread::spawn(move || run_worker(state, job_rx, lsp_sender));
        Self { job_tx }
    }

    /// Queue a debounced background reindex (no result returned to caller).
    pub fn enqueue(&self, workspace: PathBuf) {
        let _ = self.job_tx.send(WorkerJob::Index(IndexJob { workspace, reply: None }));
    }

    /// Queue a reindex and block until the worker finishes (used by `ontocore/indexWorkspace`).
    pub fn enqueue_sync(&self, workspace: PathBuf) -> Result<(CatalogStats, u64), String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.job_tx
            .send(WorkerJob::Index(IndexJob { workspace, reply: Some(tx) }))
            .map_err(|e| format!("index worker unavailable: {e}"))?;
        rx.recv().map_err(|e| format!("index worker dropped reply: {e}"))?
    }

    /// Run ROBOT CLI on the worker thread and block for the result.
    pub fn run_robot_sync(
        &self,
        robot_path: Option<String>,
        args: Vec<String>,
    ) -> Result<RunRobotResult, String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.job_tx
            .send(WorkerJob::Robot(RobotJob { robot_path, args, reply: tx }))
            .map_err(|e| format!("index worker unavailable: {e}"))?;
        rx.recv().map_err(|e| format!("index worker dropped reply: {e}"))?
    }
}

fn run_worker(state: ServerState, job_rx: Receiver<WorkerJob>, lsp_sender: Sender<Message>) {
    while let Ok(first) = job_rx.recv() {
        match first {
            WorkerJob::Robot(job) => {
                let result = run_robot_job(job.robot_path.as_deref(), &job.args);
                let _ = job.reply.send(result);
            }
            WorkerJob::Index(first_index) => {
                let mut batch = vec![first_index];
                while let Ok(WorkerJob::Index(next)) = job_rx.try_recv() {
                    batch.push(next);
                }
                // Drain any robot jobs that arrived while batching index work.
                let mut pending_robots = Vec::new();
                while let Ok(WorkerJob::Robot(robot)) = job_rx.try_recv() {
                    pending_robots.push(robot);
                }

                let workspace = batch.last().map(|j| j.workspace.clone()).unwrap_or_default();
                let replies: Vec<Sender<Result<(CatalogStats, u64), String>>> =
                    batch.into_iter().flat_map(|j| j.reply).collect();

                let result = state.index_workspace(workspace);
                match &result {
                    Ok(_) => {
                        publish_diagnostics_for_state(&lsp_sender, &state);
                    }
                    Err(message) => notify_index_failure(&lsp_sender, message),
                }
                for reply in replies {
                    let _ = reply.send(result.clone());
                }

                for robot in pending_robots {
                    let result = run_robot_job(robot.robot_path.as_deref(), &robot.args);
                    let _ = robot.reply.send(result);
                }
            }
        }
    }
}

fn run_robot_job(robot_path: Option<&str>, args: &[String]) -> Result<RunRobotResult, String> {
    let output = ontocore_robot::run_robot(robot_path, args).map_err(|e| e.to_string())?;
    Ok(RunRobotResult { exit_code: output.exit_code, stdout: output.stdout, stderr: output.stderr })
}

fn notify_index_failure(sender: &Sender<Message>, message: &str) {
    let params = ShowMessageParams {
        typ: MessageType::ERROR,
        message: format!("OntoCore reindex failed: {message}"),
    };
    let notif = Notification {
        method: ShowMessage::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    let _ = sender.send(Message::Notification(notif));
}
