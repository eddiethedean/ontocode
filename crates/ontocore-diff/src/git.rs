use crate::compare::{diff_catalogs, DiffError, Result};
use crate::model::DiffResult;
use git2::{ObjectType, Repository, TreeWalkMode};
use ontocore_catalog::{IndexBuilder, OntologyCatalog};
use std::path::{Path, PathBuf};

const ONTOLOGY_EXTENSIONS: &[&str] =
    &["ttl", "rdf", "owl", "jsonld", "json-ld", "nt", "nq", "trig", "obo"];

#[derive(Debug, Clone)]
pub struct GitDiffSpec {
    pub repo_path: PathBuf,
    pub left_ref: String,
    pub right_ref: String,
}

/// Parse `main..feature` or `HEAD` vs working tree when right is empty/`WORKTREE`.
pub fn parse_git_range(spec: &str) -> Result<(String, String)> {
    if let Some((left, right)) = spec.split_once("..") {
        Ok((left.to_string(), right.to_string()))
    } else {
        Ok((spec.to_string(), "WORKTREE".to_string()))
    }
}

pub fn diff_git_refs(repo_path: &Path, left_ref: &str, right_ref: &str) -> Result<DiffResult> {
    let left = catalog_at_git_ref(repo_path, left_ref)?;
    let right = if right_ref == "WORKTREE" || right_ref.is_empty() {
        IndexBuilder::new().workspace(repo_path).build()?
    } else {
        catalog_at_git_ref(repo_path, right_ref)?
    };
    Ok(diff_catalogs(&left, &right))
}

pub fn catalog_at_git_ref(repo_path: &Path, git_ref: &str) -> Result<OntologyCatalog> {
    let repo = Repository::open(repo_path)
        .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
    checkout_catalog(&repo, git_ref)
}

fn checkout_catalog(repo: &Repository, git_ref: &str) -> Result<OntologyCatalog> {
    let obj = repo
        .revparse_single(git_ref)
        .map_err(|e| DiffError::Message(format!("git revparse {git_ref}: {e}")))?;
    let commit =
        obj.peel_to_commit().map_err(|e| DiffError::Message(format!("git peel commit: {e}")))?;
    let tree = commit.tree().map_err(|e| DiffError::Message(format!("git tree: {e}")))?;

    let tmp = tempfile::Builder::new()
        .prefix("ontocore-diff-")
        .tempdir()
        .map_err(|e| DiffError::Message(e.to_string()))?;
    let root = tmp.path().to_path_buf();
    let mut walk_error: Option<String> = None;

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
        let rel = if dir.is_empty() { name.to_string() } else { format!("{dir}{name}") };
        let blob = match repo.find_blob(entry.id()) {
            Ok(b) => b,
            Err(e) => {
                walk_error = Some(format!("find blob: {e}"));
                return 1;
            }
        };
        let out_path = root.join(&rel);
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

    // Keep tempdir alive until catalog is built.
    let catalog = IndexBuilder::new().workspace(&root).build()?;
    std::mem::forget(tmp);
    Ok(catalog)
}

fn is_ontology_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| ONTOLOGY_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
}
