use crate::manifest::DiscoveredPlugin;
use crate::protocol::PluginOutput;
use std::io::Read;
use std::path::{Component, Path};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

// Conservative defaults to prevent hangs/DoS from untrusted subprocess plugins.
// These can be made configurable later once a stable plugin settings surface exists.
const DEFAULT_TIMEOUT_SECS: u64 = 60;
const MAX_STDIO_BYTES: usize = 1024 * 1024; // 1 MiB per stream

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
    #[error("plugin subprocess output exceeded {0} bytes")]
    OutputTooLarge(usize),
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
    if let Some(root) = infer_workspace_root(&plugin.manifest_path) {
        // Prevent invoking binaries outside the workspace root.
        let root = root.canonicalize().unwrap_or(root);
        let resolved = path.canonicalize().unwrap_or(path.clone());
        if !ontocore_core::is_path_within(&root, &resolved) {
            return Err(SubprocessError::PathNotAllowed(path.display().to_string()));
        }
    }
    if !path.exists() {
        return Err(SubprocessError::NotFound(path.display().to_string()));
    }
    Ok(path)
}

fn infer_workspace_root(manifest_path: &Path) -> Option<std::path::PathBuf> {
    // We discover manifests under `{workspace}/.ontocore/plugins/*.toml`.
    // Infer `{workspace}` so we can jail subprocess entry paths.
    let plugins_dir = manifest_path.parent()?;
    let ontocore_dir = plugins_dir.parent()?;
    if ontocore_dir.file_name().and_then(|n| n.to_str()) != Some(".ontocore") {
        return None;
    }
    ontocore_dir.parent().map(|p| p.to_path_buf())
}

fn validate_binary_path(path: &Path) -> Result<(), SubprocessError> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(SubprocessError::PathNotAllowed(path.display().to_string()));
    }
    Ok(())
}

fn read_capped(mut reader: impl Read, cap: usize) -> Result<Vec<u8>, SubprocessError> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let n = reader.read(&mut chunk).map_err(|e| SubprocessError::RunFailed(e.to_string()))?;
        if n == 0 {
            break;
        }
        if buf.len().saturating_add(n) > cap {
            return Err(SubprocessError::OutputTooLarge(cap));
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(buf)
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
        .env("ONTOCORE_PLUGIN_ACTION", request.action)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(step) = request.step {
        cmd.arg("--step").arg(step);
    }
    for arg in request.extra_args {
        cmd.arg(arg);
    }
    let mut child = cmd.spawn().map_err(|e| SubprocessError::RunFailed(e.to_string()))?;
    let mut stdout = child.stdout.take().expect("stdout piped");
    let mut stderr = child.stderr.take().expect("stderr piped");

    let out_handle = thread::spawn(move || read_capped(&mut stdout, MAX_STDIO_BYTES));
    let err_handle = thread::spawn(move || read_capped(&mut stderr, MAX_STDIO_BYTES));

    let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = out_handle.join().map_err(|_| {
                    SubprocessError::RunFailed("stdout reader panicked".to_string())
                })??;
                let err = err_handle.join().map_err(|_| {
                    SubprocessError::RunFailed("stderr reader panicked".to_string())
                })??;

                if !status.success() {
                    let stderr = String::from_utf8_lossy(&err);
                    return Err(SubprocessError::RunFailed(format!(
                        "exit {}: {}",
                        status.code().unwrap_or(-1),
                        stderr.trim()
                    )));
                }

                let stdout = String::from_utf8_lossy(&out);
                if stdout.trim().is_empty() {
                    return Ok(PluginOutput::default());
                }
                return serde_json::from_str(stdout.trim()).map_err(|e| {
                    SubprocessError::InvalidOutput(format!("{e}: {}", stdout.trim()))
                });
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(SubprocessError::TimedOut(DEFAULT_TIMEOUT_SECS));
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(e) => return Err(SubprocessError::RunFailed(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_capped_rejects_oversize() {
        let data = vec![b'a'; 33];
        let err = read_capped(Cursor::new(data), 32).unwrap_err();
        matches!(err, SubprocessError::OutputTooLarge(32));
    }
}
