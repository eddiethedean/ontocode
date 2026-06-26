use ontoindex_catalog::{IndexBuilder, OntologyCatalog};
use ontoindex_core::{
    canonical_workspace_root,
    limits::{MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS},
    validate_workspace_scope, Diagnostic, OntologyDocument,
};
use ontoindex_reasoner::{ReasonerCacheStore, ReasonerSnapshot};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct ServerState {
    inner: Arc<RwLock<InnerState>>,
    /// Serializes catalog rebuilds and reasoner runs against the same workspace state.
    ops_lock: Arc<Mutex<()>>,
}

struct InnerState {
    catalog: Option<OntologyCatalog>,
    /// LSP workspace root for path jail (set at initialize).
    workspace: Option<PathBuf>,
    /// Last successfully indexed directory (may be a subdirectory of [`InnerState::workspace`]).
    indexed_workspace: Option<PathBuf>,
    indexed_at: Option<u64>,
    /// Open document text keyed by canonical file path (unsaved buffer overrides disk).
    open_documents: HashMap<PathBuf, String>,
    reasoner_cache: ReasonerCacheStore,
    reasoner_snapshot: Option<ReasonerSnapshot>,
    /// URIs that received `publishDiagnostics` (for stale clears).
    published_diagnostic_uris: BTreeSet<String>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerState {
                catalog: None,
                workspace: None,
                indexed_workspace: None,
                indexed_at: None,
                open_documents: HashMap::new(),
                reasoner_cache: ReasonerCacheStore::new(),
                reasoner_snapshot: None,
                published_diagnostic_uris: BTreeSet::new(),
            })),
            ops_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn ops_lock(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.ops_lock)
    }

    pub fn set_workspace_root(&self, workspace: PathBuf) -> Result<(), String> {
        let workspace = canonical_workspace_root(&workspace)?;
        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        guard.workspace = Some(workspace);
        Ok(())
    }

    pub fn index_workspace(
        &self,
        workspace: PathBuf,
    ) -> Result<(ontoindex_catalog::CatalogStats, u64), String> {
        let _guard = self.ops_lock.lock().map_err(|e| e.to_string())?;
        let workspace = canonical_workspace_root(&workspace)?;

        let root = self.workspace_root().ok_or_else(|| "workspace not initialized".to_string())?;
        validate_workspace_scope(&workspace, &root)?;

        let overrides = self.open_documents_snapshot();
        let catalog = IndexBuilder::new()
            .workspace(&workspace)
            .document_overrides(overrides)
            .build()
            .map_err(|e| e.to_string())?;

        let stats = catalog.data().stats();
        let indexed_at = now_epoch_secs();

        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        guard.catalog = Some(catalog);
        guard.indexed_workspace = Some(workspace);
        guard.indexed_at = Some(indexed_at);
        guard.reasoner_cache.invalidate();
        guard.reasoner_snapshot = None;

        Ok((stats, indexed_at))
    }

    pub fn reasoner_snapshot(&self) -> Option<ReasonerSnapshot> {
        self.inner.read().ok()?.reasoner_snapshot.clone()
    }

    pub fn set_reasoner_snapshot(&self, snapshot: ReasonerSnapshot) {
        if let Ok(mut guard) = self.inner.write() {
            guard.reasoner_snapshot = Some(snapshot);
        }
    }

    pub fn reasoner_cache_mut<R>(&self, f: impl FnOnce(&mut ReasonerCacheStore) -> R) -> Option<R> {
        let mut guard = self.inner.write().ok()?;
        Some(f(&mut guard.reasoner_cache))
    }

    pub fn with_reasoner_cache<R>(&self, f: impl FnOnce(&ReasonerCacheStore) -> R) -> Option<R> {
        let guard = self.inner.read().ok()?;
        Some(f(&guard.reasoner_cache))
    }

    pub fn open_documents_for_reasoner(&self) -> HashMap<PathBuf, String> {
        self.open_documents_snapshot()
    }

    /// Clone catalog documents and diagnostics for publishing outside the read lock.
    pub fn catalog_diagnostic_snapshot(&self) -> Option<CatalogDiagnosticSnapshot> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_ref()?;
        Some(CatalogDiagnosticSnapshot {
            documents: catalog.data().documents.clone(),
            diagnostics: catalog.data().diagnostics.clone(),
        })
    }

    pub fn indexed_workspace(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.indexed_workspace.clone()
    }

    /// Directory used for reindex and reasoner input: last indexed path, else workspace root.
    pub fn effective_index_root(&self) -> Option<PathBuf> {
        let guard = self.inner.read().ok()?;
        guard.indexed_workspace.clone().or_else(|| guard.workspace.clone())
    }

    pub fn published_diagnostic_uris(&self) -> BTreeSet<String> {
        self.inner.read().ok().map(|g| g.published_diagnostic_uris.clone()).unwrap_or_default()
    }

    pub fn set_published_diagnostic_uris(&self, uris: BTreeSet<String>) {
        if let Ok(mut guard) = self.inner.write() {
            guard.published_diagnostic_uris = uris;
        }
    }

    fn open_documents_snapshot(&self) -> HashMap<PathBuf, String> {
        self.inner.read().ok().map(|g| g.open_documents.clone()).unwrap_or_default()
    }

    pub fn open_document_overrides(&self) -> HashMap<PathBuf, String> {
        self.open_documents_snapshot()
    }

    pub fn with_catalog<T>(&self, f: impl FnOnce(&OntologyCatalog) -> T) -> Option<T> {
        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("ontoindex-lsp: catalog read lock poisoned: {e}");
                return None;
            }
        };
        guard.catalog.as_ref().map(f)
    }

    pub fn workspace_root(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.workspace.clone()
    }

    /// Store unsaved buffer text for a workspace document.
    ///
    /// Returns an error when the buffer exceeds [`MAX_FILE_BYTES`] or when
    /// [`MAX_OPEN_DOCUMENTS`] would be exceeded for a newly opened path.
    pub fn set_document_text(&self, path: PathBuf, text: String) -> Result<(), String> {
        if text.len() as u64 > MAX_FILE_BYTES {
            return Err(format!("document exceeds maximum size of {MAX_FILE_BYTES} bytes"));
        }
        let path = path.canonicalize().unwrap_or(path);
        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        if !guard.open_documents.contains_key(&path)
            && guard.open_documents.len() >= MAX_OPEN_DOCUMENTS
        {
            return Err(format!("open document limit of {MAX_OPEN_DOCUMENTS} reached"));
        }
        guard.open_documents.insert(path, text);
        Ok(())
    }

    pub fn remove_document(&self, path: &Path) {
        let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        match self.inner.write() {
            Ok(mut guard) => {
                guard.open_documents.remove(&path);
            }
            Err(e) => eprintln!("ontoindex-lsp: failed to remove open document: {e}"),
        }
    }

    /// Prefer unsaved LSP buffer text; fall back to disk.
    pub fn document_text(&self, path: &Path) -> Option<String> {
        let guard = self.inner.read().ok()?;
        if let Some(text) = guard.open_documents.get(path) {
            return Some(text.clone());
        }
        if let Ok(canonical) = path.canonicalize() {
            if let Some(text) = guard.open_documents.get(&canonical) {
                return Some(text.clone());
            }
        }
        ontoindex_core::read_to_string_capped(path, MAX_FILE_BYTES).ok()
    }
}

/// Snapshot of catalog data needed to publish LSP diagnostics.
pub struct CatalogDiagnosticSnapshot {
    pub documents: Vec<OntologyDocument>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve a workspace URI for indexing (must be a `file://` directory).
pub fn resolve_workspace_for_index(
    uri: &str,
    existing_root: Option<&Path>,
) -> Result<PathBuf, String> {
    let path = ontoindex_core::workspace_uri_to_path(uri)?;
    if let Some(root) = existing_root {
        validate_workspace_scope(&path, root)
    } else {
        Ok(path)
    }
}

pub fn path_to_uri(path: &Path) -> String {
    let abs = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    url::Url::from_file_path(&abs)
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("file://{}", abs.display()))
}

fn now_epoch_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontoindex_core::{
        is_path_within,
        limits::{MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS},
    };
    use std::path::PathBuf;

    #[test]
    fn subdirectory_index_does_not_shrink_workspace_root() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir_all(&sub).unwrap();
        let root = dir.path().canonicalize().unwrap();

        let state = ServerState::new();
        state.set_workspace_root(root.clone()).expect("set workspace");
        state.index_workspace(sub.clone()).expect("index subdirectory");

        assert_eq!(state.workspace_root().as_deref(), Some(root.as_path()));
        let indexed = state.indexed_workspace().expect("indexed workspace");
        assert!(is_path_within(&root, &indexed));
        assert_eq!(indexed.file_name(), sub.file_name());
    }

    #[test]
    fn rejects_oversized_buffer() {
        let state = ServerState::new();
        let path = PathBuf::from("/tmp/oversized.ttl");
        let text = "x".repeat((MAX_FILE_BYTES + 1) as usize);
        let err = state.set_document_text(path, text).unwrap_err();
        assert!(err.contains("exceeds maximum size"));
    }

    #[test]
    fn rejects_when_open_document_limit_reached() {
        let state = ServerState::new();
        for i in 0..MAX_OPEN_DOCUMENTS {
            let path = PathBuf::from(format!("/tmp/doc-{i}.ttl"));
            state.set_document_text(path, "@prefix ex: <http://ex#> .".to_string()).unwrap();
        }
        let err =
            state.set_document_text(PathBuf::from("/tmp/extra.ttl"), "x".to_string()).unwrap_err();
        assert!(err.contains("open document limit"));
    }
}
