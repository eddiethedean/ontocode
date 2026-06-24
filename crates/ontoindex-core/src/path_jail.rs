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
/// Canonicalizes when the path exists on disk; otherwise jails via lexical normalization.
pub fn resolve_lsp_document_path(uri: &str, workspace_root: &Path) -> Result<PathBuf, String> {
    let path = file_uri_to_path(uri)?;
    if path_has_parent_escape(&path) {
        return Err("document path escapes workspace via ..".to_string());
    }
    let root = canonical_workspace_root(workspace_root)?;

    if let Ok(canonical) = path.canonicalize() {
        if is_path_within(&root, &canonical) {
            return Ok(canonical);
        }
        return Err("document path is outside the indexed workspace".to_string());
    }

    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        if let Ok(canonical_parent) = parent.canonicalize() {
            let candidate = canonical_parent.join(path.file_name().unwrap_or_default());
            if is_path_within(&root, &candidate) {
                return Ok(candidate);
            }
        }
    }

    let candidate = if path.is_absolute() {
        normalize_lexical(&path)
    } else {
        normalize_lexical(&root.join(path))
    };

    if is_path_within_lexical(&root, &candidate) {
        Ok(candidate)
    } else {
        Err("document path is outside the indexed workspace".to_string())
    }
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

/// Ensure `path` is the workspace root or a subdirectory of it (after canonicalization).
pub fn validate_workspace_scope(
    requested: &Path,
    workspace_root: &Path,
) -> Result<PathBuf, String> {
    let requested = canonical_workspace_root(requested)?;
    if is_path_within(workspace_root, &requested) {
        Ok(requested)
    } else {
        Err("workspace URI is outside the allowed workspace root".to_string())
    }
}

/// Returns true if `path` is `workspace_root` or nested under it.
pub fn is_path_within(workspace_root: &Path, path: &Path) -> bool {
    let Ok(root) = workspace_root.canonicalize() else {
        return false;
    };
    if let Ok(path) = path.canonicalize() {
        return path == root || path.starts_with(&root);
    }
    is_path_within_lexical(&root, path)
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
}
