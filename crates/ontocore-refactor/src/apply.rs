use crate::error::{RefactorError, Result};
use crate::model::{FileChange, RefactorPlan};
use ontocore_core::{
    canonical_workspace_root, read_to_string_capped, validate_workspace_scope, MAX_FILE_BYTES,
};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Validate every path in a refactor plan is within the workspace jail.
pub fn validate_refactor_plan_paths(workspace_root: &Path, plan: &RefactorPlan) -> Result<()> {
    let root = canonical_workspace_root(workspace_root).map_err(RefactorError::Invalid)?;
    for change in &plan.changes {
        validate_workspace_scope(&change.path, &root).map_err(RefactorError::Invalid)?;
    }
    Ok(())
}

/// Returns true when client-submitted plan matches a server re-preview byte-for-byte.
pub fn plans_equivalent(server: &RefactorPlan, client: &RefactorPlan) -> bool {
    if server.changes.len() != client.changes.len() {
        return false;
    }
    let mut server_sorted: Vec<&FileChange> = server.changes.iter().collect();
    let mut client_sorted: Vec<&FileChange> = client.changes.iter().collect();
    server_sorted.sort_by_key(|c| &c.path);
    client_sorted.sort_by_key(|c| &c.path);
    for (s, c) in server_sorted.iter().zip(client_sorted.iter()) {
        if s.path != c.path || s.preview_text != c.preview_text {
            return false;
        }
    }
    true
}

fn atomic_write(path: &Path, contents: &str) -> std::io::Result<()> {
    let parent =
        path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let tmp_path = parent.join(format!(
        ".ontocode-{}-{}.tmp",
        path.file_name().and_then(|s| s.to_str()).unwrap_or("file"),
        nanos
    ));
    {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(&tmp_path, path)?;
    Ok(())
}

struct FileBackup {
    path: PathBuf,
    previous: Option<String>,
    created: bool,
}

fn backup_file(path: &Path) -> std::io::Result<FileBackup> {
    let created = !path.exists();
    let previous = if created {
        None
    } else {
        Some(read_to_string_capped(path, MAX_FILE_BYTES).map_err(std::io::Error::other)?)
    };
    Ok(FileBackup { path: path.to_path_buf(), previous, created })
}

fn restore_backup(backup: &FileBackup) -> std::io::Result<()> {
    if backup.created {
        if backup.path.exists() {
            fs::remove_file(&backup.path)?;
        }
    } else if let Some(content) = &backup.previous {
        atomic_write(&backup.path, content)?;
    }
    Ok(())
}

/// Apply a refactor plan to disk. When `preview_only` is true, no files are written.
pub fn apply_refactor_plan(
    plan: &RefactorPlan,
    preview_only: bool,
    workspace_root: &Path,
) -> Result<()> {
    apply_refactor_plan_checked(plan, preview_only, Some(workspace_root)).map(|_| ())
}

/// Apply plan and return count of files written.
pub fn apply_refactor_plan_checked(
    plan: &RefactorPlan,
    preview_only: bool,
    workspace_root: Option<&Path>,
) -> Result<usize> {
    if let Some(root) = workspace_root {
        validate_refactor_plan_paths(root, plan)?;
    }
    if preview_only {
        return Ok(0);
    }
    let mut written = 0usize;
    let mut backups: Vec<FileBackup> = Vec::new();
    for change in &plan.changes {
        if change.preview_text == change.original_text {
            continue;
        }
        let backup = backup_file(&change.path)?;
        if let Err(e) = atomic_write(&change.path, &change.preview_text) {
            rollback_backups(&backups)?;
            return Err(RefactorError::Io(e));
        }
        backups.push(backup);
        written += 1;
    }
    if written == 0 && !plan.changes.is_empty() {
        rollback_backups(&backups)?;
        return Err(RefactorError::Invalid("no files changed".to_string()));
    }
    Ok(written)
}

fn rollback_backups(backups: &[FileBackup]) -> Result<()> {
    let mut restore_errors = Vec::new();
    for b in backups.iter().rev() {
        if let Err(e) = restore_backup(b) {
            restore_errors.push(format!("{}: {e}", b.path.display()));
        }
    }
    if restore_errors.is_empty() {
        Ok(())
    } else {
        Err(RefactorError::Invalid(format!(
            "refactor rollback failed for: {}",
            restore_errors.join("; ")
        )))
    }
}

pub fn plan_touches_path(plan: &RefactorPlan, path: &Path) -> bool {
    plan.changes.iter().any(|c| c.path == path)
}
