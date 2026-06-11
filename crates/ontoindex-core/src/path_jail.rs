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
    let Ok(path) = path.canonicalize() else {
        return false;
    };
    path == root || path.starts_with(&root)
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
