use crate::compare::{diff_catalogs, DiffError, Result};
use crate::model::DiffResult;
use git2::{ObjectType, Oid, Repository, TreeWalkMode};
use ontocore_catalog::{IndexBuilder, OntologyCatalog};
use ontocore_core::{
    discover_git_repo_root, ensure_extract_path_within, limits::MAX_FILE_BYTES,
    limits::MAX_SCAN_FILES, path_jail::path_has_parent_escape,
};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const ONTOLOGY_EXTENSIONS: &[&str] =
    &["ttl", "rdf", "owl", "jsonld", "json-ld", "nt", "nq", "trig", "obo"];

const CHECKOUT_CACHE_MAX: usize = 4;

#[derive(Debug, Clone)]
pub struct GitDiffSpec {
    pub repo_path: PathBuf,
    pub left_ref: String,
    pub right_ref: String,
}

struct CachedCheckout {
    key: String,
    root: PathBuf,
    _temp: tempfile::TempDir,
}

static CHECKOUT_CACHE: Mutex<Vec<CachedCheckout>> = Mutex::new(Vec::new());

/// Parse `main..feature`, `main...feature` (merge-base), or single ref vs WORKTREE.
pub fn parse_git_range(spec: &str) -> Result<(String, String)> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err(DiffError::Message("empty git range".to_string()));
    }
    if let Some(idx) = spec.find("...") {
        let left = spec[..idx].trim();
        let right = spec[idx + 3..].trim();
        if left.is_empty() || right.is_empty() {
            return Err(DiffError::Message("invalid triple-dot git range".to_string()));
        }
        return Ok((format!("{left}...{right}"), "TRIPLE_DOT".to_string()));
    }
    if let Some((left, right)) = spec.split_once("..") {
        let left = left.trim();
        let right = right.trim();
        if left.is_empty() || right.is_empty() {
            return Err(DiffError::Message("invalid two-dot git range".to_string()));
        }
        return Ok((left.to_string(), right.to_string()));
    }
    Ok((spec.to_string(), "WORKTREE".to_string()))
}

pub fn discover_repo_root(paths: &[PathBuf]) -> Option<PathBuf> {
    discover_git_repo_root(paths)
}

pub fn diff_git_refs(repo_path: &Path, left_ref: &str, right_ref: &str) -> Result<DiffResult> {
    if right_ref == "TRIPLE_DOT" || left_ref.contains("...") {
        let (left, right) = parse_merge_base_refs(left_ref)?;
        let repo = Repository::open(repo_path)
            .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
        let left_obj = repo
            .revparse_single(&left)
            .map_err(|e| DiffError::Message(format!("git revparse {left}: {e}")))?;
        let right_obj = repo
            .revparse_single(&right)
            .map_err(|e| DiffError::Message(format!("git revparse {right}: {e}")))?;
        let merge_base = repo
            .merge_base(left_obj.id(), right_obj.id())
            .map_err(|e| DiffError::Message(format!("git merge-base: {e}")))?;
        let base = checkout_catalog_at_oid(&repo, merge_base)?;
        let head = catalog_at_git_ref(repo_path, &right)?;
        return Ok(diff_catalogs(&base, &head));
    }
    let left = catalog_at_git_ref(repo_path, left_ref)?;
    let right = if right_ref == "WORKTREE" || right_ref.is_empty() {
        catalog_at_worktree(repo_path)?
    } else {
        catalog_at_git_ref(repo_path, right_ref)?
    };
    Ok(diff_catalogs(&left, &right))
}

fn parse_merge_base_refs(spec: &str) -> Result<(String, String)> {
    let idx = spec
        .find("...")
        .ok_or_else(|| DiffError::Message("expected triple-dot range".to_string()))?;
    let left = spec[..idx].trim().to_string();
    let right = spec[idx + 3..].trim().to_string();
    if left.is_empty() || right.is_empty() {
        return Err(DiffError::Message("invalid triple-dot git range".to_string()));
    }
    Ok((left, right))
}

pub fn catalog_at_git_ref(repo_path: &Path, git_ref: &str) -> Result<OntologyCatalog> {
    let repo = Repository::open(repo_path)
        .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
    checkout_catalog(&repo, git_ref)
}

/// Build catalog from git-tracked ontology files in the working tree (aligned with git-side diffs).
pub fn catalog_at_worktree(repo_path: &Path) -> Result<OntologyCatalog> {
    let tracked = list_git_tracked_ontology_paths(repo_path)?;
    let mut builder = IndexBuilder::new().workspace(repo_path);
    if !tracked.is_empty() {
        builder = builder.only_paths(tracked);
    }
    builder.build().map_err(DiffError::from)
}

fn list_git_tracked_ontology_paths(repo_path: &Path) -> Result<Vec<PathBuf>> {
    let repo = Repository::open(repo_path)
        .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
    let mut paths = Vec::new();
    let mut index = repo.index().map_err(|e| DiffError::Message(format!("git index: {e}")))?;
    index.read(true).map_err(|e| DiffError::Message(format!("git index read: {e}")))?;
    for entry in index.iter() {
        let path_str =
            std::str::from_utf8(&entry.path).map_err(|e| DiffError::Message(e.to_string()))?;
        if !is_ontology_file(path_str) {
            continue;
        }
        let full = repo_path.join(path_str);
        if path_has_parent_escape(full.as_path()) {
            continue;
        }
        paths.push(full);
        if paths.len() > MAX_SCAN_FILES {
            return Err(DiffError::Message(format!(
                "git tracked files exceed maximum of {MAX_SCAN_FILES}"
            )));
        }
    }
    Ok(paths)
}

fn checkout_catalog(repo: &Repository, git_ref: &str) -> Result<OntologyCatalog> {
    let obj = repo
        .revparse_single(git_ref)
        .map_err(|e| DiffError::Message(format!("git revparse {git_ref}: {e}")))?;
    let commit =
        obj.peel_to_commit().map_err(|e| DiffError::Message(format!("git peel commit: {e}")))?;
    checkout_catalog_at_oid(repo, commit.id())
}

fn checkout_catalog_at_oid(repo: &Repository, oid: Oid) -> Result<OntologyCatalog> {
    let repo_path =
        repo.path().parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
    let key = format!("{}:{oid}", repo_path.display());
    if let Some(root) = cache_get(&key) {
        return IndexBuilder::new().workspace(&root).build().map_err(DiffError::from);
    }

    let commit =
        repo.find_commit(oid).map_err(|e| DiffError::Message(format!("git find commit: {e}")))?;
    let tree = commit.tree().map_err(|e| DiffError::Message(format!("git tree: {e}")))?;

    let tmp = tempfile::Builder::new()
        .prefix("ontocore-diff-")
        .tempdir()
        .map_err(|e| DiffError::Message(e.to_string()))?;
    let root = tmp.path().to_path_buf();
    let mut walk_error: Option<String> = None;
    let mut file_count = 0usize;

    tree.walk(TreeWalkMode::PreOrder, |dir, entry| {
        if walk_error.is_some() {
            return 1;
        }
        if entry.kind() != Some(ObjectType::Blob) {
            return 0;
        }
        let name = entry.name().unwrap_or("");
        if !is_ontology_file(name) {
            return 0;
        }
        file_count += 1;
        if file_count > MAX_SCAN_FILES {
            walk_error =
                Some(format!("git tree exceeds maximum of {MAX_SCAN_FILES} ontology files"));
            return 1;
        }
        let rel = if dir.is_empty() { name.to_string() } else { format!("{dir}{name}") };
        let out_path = match ensure_extract_path_within(&root, &rel) {
            Ok(p) => p,
            Err(e) => {
                walk_error = Some(e);
                return 1;
            }
        };
        let blob = match repo.find_blob(entry.id()) {
            Ok(b) => b,
            Err(e) => {
                walk_error = Some(format!("find blob: {e}"));
                return 1;
            }
        };
        let size = blob.content().len() as u64;
        if size > MAX_FILE_BYTES {
            walk_error =
                Some(format!("blob exceeds size limit ({size} bytes > {MAX_FILE_BYTES}): {rel}",));
            return 1;
        }
        if let Some(parent) = out_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                walk_error = Some(format!("create dir {}: {e}", parent.display()));
                return 1;
            }
        }
        if let Err(e) = std::fs::write(&out_path, blob.content()) {
            walk_error = Some(format!("write {}: {e}", out_path.display()));
            return 1;
        }
        0
    })
    .map_err(|e| DiffError::Message(format!("git tree walk: {e}")))?;

    if let Some(msg) = walk_error {
        return Err(DiffError::Message(msg));
    }

    cache_insert(key, root.clone(), tmp);
    IndexBuilder::new().workspace(&root).build().map_err(DiffError::from)
}

fn cache_get(key: &str) -> Option<PathBuf> {
    let guard = CHECKOUT_CACHE.lock().ok()?;
    guard.iter().find(|e| e.key == key).map(|e| e.root.clone())
}

fn cache_insert(key: String, root: PathBuf, temp: tempfile::TempDir) {
    if let Ok(mut guard) = CHECKOUT_CACHE.lock() {
        if guard.iter().any(|e| e.key == key) {
            return;
        }
        while guard.len() >= CHECKOUT_CACHE_MAX {
            guard.remove(0);
        }
        guard.push(CachedCheckout { key, root, _temp: temp });
    }
}

fn is_ontology_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| ONTOLOGY_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_triple_dot_range() {
        let (left, right) = parse_git_range("main...feature").unwrap();
        assert_eq!(left, "main...feature");
        assert_eq!(right, "TRIPLE_DOT");
    }

    #[test]
    fn parse_two_dot_range() {
        let (left, right) = parse_git_range("main..feature").unwrap();
        assert_eq!(left, "main");
        assert_eq!(right, "feature");
    }

    #[test]
    fn reject_escape_path() {
        let dir = tempfile::tempdir().unwrap();
        assert!(ensure_extract_path_within(dir.path(), "../outside.ttl").is_err());
    }
}
