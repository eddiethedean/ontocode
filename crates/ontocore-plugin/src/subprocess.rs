use crate::manifest::DiscoveredPlugin;
use crate::protocol::PluginOutput;
use std::path::{Component, Path};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubprocessError {
    #[error("plugin binary not found: {0}")]
    NotFound(String),
    #[error("plugin binary path is not allowed: {0}")]
    PathNotAllowed(String),
    #[error("plugin subprocess failed: {0}")]
    RunFailed(String),
    #[error("plugin subprocess timed out after {0}s")]
    TimedOut(u64),
    #[error("plugin output is not valid JSON: {0}")]
    InvalidOutput(String),
}

pub struct SubprocessRequest<'a> {
    pub action: &'a str,
    pub workspace: &'a Path,
    pub step: Option<&'a str>,
    pub extra_args: &'a [String],
}

pub fn resolve_entry_path(
    plugin: &DiscoveredPlugin,
) -> Result<std::path::PathBuf, SubprocessError> {
    let entry = plugin
        .manifest
        .entry
        .as_deref()
        .ok_or_else(|| SubprocessError::NotFound("missing entry".to_string()))?;
    let path = if Path::new(entry).is_absolute() {
        Path::new(entry).to_path_buf()
    } else {
        plugin.manifest_path.parent().unwrap_or(Path::new(".")).join(entry)
    };
    validate_binary_path(&path)?;
    if !path.exists() {
        return Err(SubprocessError::NotFound(path.display().to_string()));
    }
    Ok(path)
}

fn validate_binary_path(path: &Path) -> Result<(), SubprocessError> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(SubprocessError::PathNotAllowed(path.display().to_string()));
    }
    Ok(())
}

pub fn run_plugin_subprocess(
    plugin: &DiscoveredPlugin,
    request: SubprocessRequest<'_>,
) -> Result<PluginOutput, SubprocessError> {
    let binary = resolve_entry_path(plugin)?;
    let mut cmd = Command::new(&binary);
    cmd.arg(request.action)
        .arg("--workspace")
        .arg(request.workspace)
        .env("ONTOCORE_PLUGIN_ACTION", request.action);
    if let Some(step) = request.step {
        cmd.arg("--step").arg(step);
    }
    for arg in request.extra_args {
        cmd.arg(arg);
    }
    let output = cmd.output().map_err(|e| SubprocessError::RunFailed(e.to_string()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SubprocessError::RunFailed(format!(
            "exit {}: {}",
            output.status.code().unwrap_or(-1),
            stderr.trim()
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return Ok(PluginOutput::default());
    }
    serde_json::from_str(stdout.trim())
        .map_err(|e| SubprocessError::InvalidOutput(format!("{e}: {}", stdout.trim())))
}
