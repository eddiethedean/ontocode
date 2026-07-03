use std::path::{Component, Path, PathBuf};

/// Canonicalize a workspace root directory.
pub fn canonical_workspace_root(path: &Path) -> Result<PathBuf, String> {
    path.canonicalize().map_err(|e| format!("workspace path invalid: {e}"))
}

/// Resolve a `file://` workspace URI to a canonical directory path.
pub fn workspace_uri_to_path(uri: &str) -> Result<PathBuf, String> {
    file_uri_to_path(uri).and_then(|p| canonical_workspace_root(&p))
}

/// Resolve a `file://` URI to a path (not necessarily canonical).
pub fn file_uri_to_path(uri: &str) -> Result<PathBuf, String> {
    let url = url::Url::parse(uri).map_err(|e| e.to_string())?;
    if url.scheme() != "file" {
        return Err(format!("only file:// URIs are supported, got {}", url.scheme()));
    }
    url.to_file_path().map_err(|_| format!("invalid file URI: {uri}"))
}

/// Resolve a document URI and ensure it lies under `workspace_root` (both canonical).
pub fn resolve_document_path(uri: &str, workspace_root: &Path) -> Result<PathBuf, String> {
    let path = file_uri_to_path(uri)?;
    let canonical =
        path.canonicalize().map_err(|e| format!("cannot resolve document path: {e}"))?;
    if !is_path_within(workspace_root, &canonical) {
        return Err("document path is outside the indexed workspace".to_string());
    }
    Ok(canonical)
}

/// Resolve an LSP document URI under `workspace_root` without requiring the file to exist.
///
/// Canonicalizes when the path exists on disk; otherwise resolves via the longest existing
/// path prefix so symlink parents cannot escape the workspace.
pub fn resolve_lsp_document_path(uri: &str, workspace_root: &Path) -> Result<PathBuf, String> {
    let path = file_uri_to_path(uri)?;
    resolve_path_in_workspace(
        &path,
        workspace_root,
        "document path is outside the indexed workspace",
    )
}

/// Ensure `path` is the workspace root or a subdirectory of it (after canonicalization).
pub fn validate_workspace_scope(
    requested: &Path,
    workspace_root: &Path,
) -> Result<PathBuf, String> {
    resolve_path_in_workspace(
        requested,
        workspace_root,
        "workspace URI is outside the allowed workspace root",
    )
}

/// Resolve `path` under `workspace_root`, including non-existent paths.
///
/// Walks upward to the longest existing prefix, canonicalizes that prefix, and joins only
/// the missing suffix. If any existing prefix resolves outside the root, rejects.
fn resolve_path_in_workspace(
    path: &Path,
    workspace_root: &Path,
    outside_msg: &str,
) -> Result<PathBuf, String> {
    if path_has_parent_escape(path) {
        return Err("path escapes workspace via ..".to_string());
    }
    let root = canonical_workspace_root(workspace_root)?;

    if let Ok(canonical) = path.canonicalize() {
        if path_is_under(&root, &canonical) {
            return Ok(canonical);
        }
        return Err(outside_msg.to_string());
    }

    let absolute = if path.is_absolute() { path.to_path_buf() } else { root.join(path) };
    let absolute = normalize_lexical(&absolute);

    let (existing_prefix, missing_suffix) = split_existing_prefix(&absolute);
    let canonical_prefix =
        existing_prefix.canonicalize().map_err(|e| format!("cannot resolve path prefix: {e}"))?;

    if !path_is_under(&root, &canonical_prefix) {
        return Err(outside_msg.to_string());
    }

    let candidate = if missing_suffix.as_os_str().is_empty() {
        canonical_prefix
    } else {
        canonical_prefix.join(missing_suffix)
    };

    if path_is_under(&root, &candidate) || is_path_within_lexical(&root, &candidate) {
        Ok(candidate)
    } else {
        Err(outside_msg.to_string())
    }
}

/// Split `path` into (longest existing prefix, remaining relative suffix).
fn split_existing_prefix(path: &Path) -> (PathBuf, PathBuf) {
    let mut prefix = path.to_path_buf();
    let mut missing = PathBuf::new();
    loop {
        if prefix.exists() {
            let mut suffix = PathBuf::new();
            for component in missing.components().rev() {
                suffix.push(component);
            }
            return (prefix, suffix);
        }
        match prefix.file_name() {
            Some(name) => {
                missing.push(name);
                if !prefix.pop() {
                    break;
                }
            }
            None => break,
        }
    }
    // Nothing exists; treat as relative to current dir (caller already made absolute).
    (PathBuf::from("."), path.to_path_buf())
}

fn normalize_lexical(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                components.pop();
            }
            Component::CurDir => {}
            c => components.push(c),
        }
    }
    components.iter().collect()
}

fn is_path_within_lexical(root: &Path, path: &Path) -> bool {
    let root = normalize_lexical(root);
    let path = normalize_lexical(path);
    path == root || path.starts_with(&root)
}

/// Component-aware containment for already-canonical (or known-absolute) paths.
fn path_is_under(root: &Path, path: &Path) -> bool {
    path == root || path.starts_with(root)
}

/// Returns true if `path` is `workspace_root` or nested under it.
pub fn is_path_within(workspace_root: &Path, path: &Path) -> bool {
    let Ok(root) = workspace_root.canonicalize() else {
        return false;
    };
    if let Ok(path) = path.canonicalize() {
        return path_is_under(&root, &path);
    }
    // For non-existent paths, resolve via longest existing prefix — never trust lexical alone
    // when a real prefix exists (symlink escape).
    let absolute = if path.is_absolute() {
        normalize_lexical(path)
    } else {
        normalize_lexical(&root.join(path))
    };
    let (existing_prefix, missing_suffix) = split_existing_prefix(&absolute);
    let Ok(canonical_prefix) = existing_prefix.canonicalize() else {
        return is_path_within_lexical(&root, &absolute);
    };
    if !path_is_under(&root, &canonical_prefix) {
        return false;
    }
    let candidate = if missing_suffix.as_os_str().is_empty() {
        canonical_prefix
    } else {
        canonical_prefix.join(missing_suffix)
    };
    path_is_under(&root, &candidate) || is_path_within_lexical(&root, &candidate)
}

/// Reject paths that escape upward via `..` before canonicalize (symlink-free check).
pub fn path_has_parent_escape(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn validate_scope_allows_nonexistent_file_under_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let new_file = dir.path().join("module.ttl");
        let resolved =
            validate_workspace_scope(&new_file, &root).expect("new file under workspace");
        assert!(is_path_within(&root, &resolved));
        assert_eq!(resolved.file_name(), new_file.file_name());
    }

    #[test]
    fn lsp_document_path_allows_nonexistent_file_under_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let new_file = dir.path().join("new.ttl");
        let uri = url::Url::from_file_path(&new_file).unwrap().to_string();
        let resolved = resolve_lsp_document_path(&uri, &root).expect("new file under workspace");
        assert!(is_path_within(&root, &resolved));
        assert_eq!(resolved.file_name(), new_file.file_name());
    }

    #[test]
    fn document_must_be_under_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("ontologies");
        fs::create_dir_all(&sub).unwrap();
        let ttl = sub.join("ex.ttl");
        fs::write(&ttl, "@prefix ex: <http://ex/> .").unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let uri = url::Url::from_file_path(&ttl).unwrap().to_string();
        let resolved = resolve_document_path(&uri, &root).expect("under workspace");
        assert_eq!(resolved, ttl.canonicalize().unwrap());
    }

    #[test]
    fn rejects_path_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let ttl = outside.path().join("secret.ttl");
        fs::write(&ttl, "@prefix ex: <http://ex/> .").unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let uri = url::Url::from_file_path(&ttl).unwrap().to_string();
        assert!(resolve_document_path(&uri, &root).is_err());
    }

    #[test]
    #[cfg(unix)]
    fn rejects_nonexistent_file_under_symlink_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let link = dir.path().join("vendor");
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let target = link.join("pwn.ttl");
        let uri = url::Url::from_file_path(&target).unwrap().to_string();
        assert!(
            resolve_lsp_document_path(&uri, &root).is_err(),
            "must reject create-through-symlink escape"
        );
        assert!(validate_workspace_scope(&target, &root).is_err());
        assert!(!is_path_within(&root, &target));
    }

    #[test]
    #[cfg(unix)]
    fn rejects_nested_missing_path_through_symlink_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let link = dir.path().join("vendor");
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let target = link.join("nested").join("pwn.ttl");
        assert!(validate_workspace_scope(&target, &root).is_err());
    }
}
