use crate::error::{RefactorError, Result};
use crate::model::RefactorPlan;
use std::fs;
use std::path::Path;

/// Apply a refactor plan to disk. When `preview_only` is true, no files are written.
pub fn apply_refactor_plan(plan: &RefactorPlan, preview_only: bool) -> Result<()> {
    if preview_only {
        return Ok(());
    }
    for change in &plan.changes {
        let parent = change.path.parent();
        if let Some(dir) = parent {
            if !dir.as_os_str().is_empty() {
                fs::create_dir_all(dir)?;
            }
        }
        fs::write(&change.path, &change.preview_text)?;
    }
    Ok(())
}

/// Apply plan and return count of files written.
pub fn apply_refactor_plan_checked(plan: &RefactorPlan, preview_only: bool) -> Result<usize> {
    if preview_only {
        return Ok(0);
    }
    let mut written = 0usize;
    for change in &plan.changes {
        if change.preview_text == change.original_text {
            continue;
        }
        let parent = change.path.parent();
        if let Some(dir) = parent {
            if !dir.as_os_str().is_empty() && !dir.exists() {
                fs::create_dir_all(dir)?;
            }
        }
        fs::write(&change.path, &change.preview_text)?;
        written += 1;
    }
    if written == 0 && !plan.changes.is_empty() {
        return Err(RefactorError::Invalid("no files changed".to_string()));
    }
    Ok(written)
}

pub fn plan_touches_path(plan: &RefactorPlan, path: &Path) -> bool {
    plan.changes.iter().any(|c| c.path == path)
}
