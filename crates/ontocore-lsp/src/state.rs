use ontocore_catalog::{IndexBuilder, OntologyCatalog};
use ontocore_core::{
    canonical_workspace_root, is_path_within_any,
    limits::{MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS},
    validate_workspace_scope, validate_workspace_scope_any, Diagnostic, OntologyDocument,
};
use ontocore_reasoner::{ReasonerCacheStore, ReasonerSnapshot};
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
    /// All LSP workspace folder roots (multi-root).
    workspace_roots: Vec<PathBuf>,
    /// Primary workspace root (first folder) for backward-compatible APIs.
    workspace: Option<PathBuf>,
    /// Last successfully indexed directory (may be a subdirectory of [`InnerState::workspace`]).
    indexed_workspace: Option<PathBuf>,
    indexed_at: Option<u64>,
    /// Open document text keyed by canonical file path (unsaved buffer overrides disk).
    open_documents: HashMap<PathBuf, String>,
    /// LSP textDocument versions for open buffers (for versioned WorkspaceEdits).
    document_versions: HashMap<PathBuf, i32>,
    reasoner_cache: ReasonerCacheStore,
    reasoner_snapshot: Option<ReasonerSnapshot>,
    explanation_cache: HashMap<(String, String, String), ontocore_reasoner::ExplanationResult>,
    /// Diagnostics from workspace plugins (merged at publish time).
    plugin_diagnostics: Vec<Diagnostic>,
    /// URIs that received `publishDiagnostics` (for stale clears).
    published_diagnostic_uris: BTreeSet<String>,
    /// Persist `.ontocore/cache/` during indexing when enabled via `ontocore/indexWorkspace`.
    index_disk_cache: bool,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerState {
                catalog: None,
                workspace_roots: Vec::new(),
                workspace: None,
                indexed_workspace: None,
                indexed_at: None,
                open_documents: HashMap::new(),
                document_versions: HashMap::new(),
                reasoner_cache: ReasonerCacheStore::new(),
                reasoner_snapshot: None,
                explanation_cache: HashMap::new(),
                plugin_diagnostics: Vec::new(),
                published_diagnostic_uris: BTreeSet::new(),
                index_disk_cache: false,
            })),
            ops_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn ops_lock(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.ops_lock)
    }

    pub fn set_workspace_roots(&self, roots: Vec<PathBuf>) -> Result<(), String> {
        let _ops = self.ops_lock.lock().map_err(|e| e.to_string())?;
        let mut canonical = Vec::new();
        for root in roots {
            canonical.push(canonical_workspace_root(&root)?);
        }
        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        if canonical.is_empty() {
            clear_workspace_state_inner(&mut guard);
            return Ok(());
        }
        let primary = canonical[0].clone();
        on_workspace_roots_changed_inner(&mut guard, &canonical);
        guard.workspace_roots = canonical;
        guard.workspace = Some(primary);
        Ok(())
    }

    /// Drop all indexed state and open buffers (e.g. when every workspace folder is removed).
    pub fn clear_workspace_state(&self) {
        let _ops = self.ops_lock.lock().ok();
        if let Ok(mut guard) = self.inner.write() {
            clear_workspace_state_inner(&mut guard);
        }
    }

    /// Invalidate catalog/reasoner and prune buffers outside the current roots.
    #[allow(dead_code)]
    pub fn on_workspace_roots_changed(&self) {
        let _ops = self.ops_lock.lock().ok();
        if let Ok(mut guard) = self.inner.write() {
            let roots = guard.workspace_roots.clone();
            if roots.is_empty() {
                clear_workspace_state_inner(&mut guard);
            } else {
                on_workspace_roots_changed_inner(&mut guard, &roots);
            }
        }
    }

    pub fn workspace_roots(&self) -> Vec<PathBuf> {
        self.inner.read().ok().map(|g| g.workspace_roots.clone()).unwrap_or_default()
    }

    pub fn set_index_disk_cache(&self, enabled: bool) {
        if let Ok(mut guard) = self.inner.write() {
            guard.index_disk_cache = enabled;
        }
    }

    pub fn index_workspace(
        &self,
        workspace: PathBuf,
    ) -> Result<(ontocore_catalog::CatalogStats, u64), String> {
        let _guard = self.ops_lock.lock().map_err(|e| e.to_string())?;
        let workspace = canonical_workspace_root(&workspace)?;

        let roots = self.workspace_roots();
        if roots.is_empty() {
            return Err("workspace not initialized".to_string());
        }
        validate_workspace_scope_any(&workspace, &roots)?;

        let disk_cache = self.inner.read().map(|g| g.index_disk_cache).unwrap_or(false);
        let catalog = {
            let overrides = self.open_documents_snapshot();
            let builder = IndexBuilder::new()
                .workspace(workspace.clone())
                .scan_roots(roots.clone())
                .document_overrides(overrides)
                .disk_cache(disk_cache);
            let guard = self.inner.read().map_err(|e| e.to_string())?;
            if let Some(prev) = guard.catalog.as_ref() {
                builder.build_incremental(prev).map_err(|e| e.to_string())?
            } else {
                drop(guard);
                builder.build().map_err(|e| e.to_string())?
            }
        };

        let stats = catalog.data().stats();
        let indexed_at = now_epoch_secs();

        let plugin_diags = ontocore_plugin_builtins::load_plugin_host(&workspace)
            .map(|host| host.run_all_validators(&catalog))
            .unwrap_or_default();

        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        guard.catalog = Some(catalog);
        guard.indexed_workspace = Some(workspace);
        guard.indexed_at = Some(indexed_at);
        guard.reasoner_cache.invalidate();
        guard.reasoner_snapshot = None;
        guard.explanation_cache.clear();
        guard.plugin_diagnostics = plugin_diags;

        Ok((stats, indexed_at))
    }

    pub fn get_cached_explanation(
        &self,
        content_hash: &str,
        profile: &str,
        class_iri: &str,
    ) -> Option<ontocore_reasoner::ExplanationResult> {
        let guard = self.inner.read().ok()?;
        guard
            .explanation_cache
            .get(&(content_hash.to_string(), profile.to_string(), class_iri.to_string()))
            .cloned()
    }

    pub fn put_cached_explanation(
        &self,
        content_hash: &str,
        profile: &str,
        class_iri: &str,
        result: ontocore_reasoner::ExplanationResult,
    ) {
        if let Ok(mut guard) = self.inner.write() {
            guard.explanation_cache.insert(
                (content_hash.to_string(), profile.to_string(), class_iri.to_string()),
                result,
            );
        }
    }

    pub fn indexed_at(&self) -> Option<u64> {
        self.inner.read().ok().and_then(|g| g.indexed_at)
    }

    pub fn set_reasoner_snapshot(&self, snapshot: ReasonerSnapshot) {
        if let Ok(mut guard) = self.inner.write() {
            guard.reasoner_snapshot = Some(snapshot);
        }
    }

    /// Read catalog and reasoner snapshot under one lock (avoids TOCTOU mismatch).
    pub fn with_catalog_and_reasoner<T>(
        &self,
        f: impl FnOnce(&OntologyCatalog, Option<&ReasonerSnapshot>) -> T,
    ) -> Option<T> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_ref()?;
        Some(f(catalog, guard.reasoner_snapshot.as_ref()))
    }

    /// Read catalog and open-document overrides under one lock (avoids TOCTOU mismatch).
    pub fn with_catalog_and_overrides<T>(
        &self,
        f: impl FnOnce(&OntologyCatalog, &HashMap<PathBuf, String>) -> T,
    ) -> Option<T> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_ref()?;
        Some(f(catalog, &guard.open_documents))
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

    pub fn plugin_diagnostics(&self) -> Vec<Diagnostic> {
        self.inner.read().ok().map(|g| g.plugin_diagnostics.clone()).unwrap_or_default()
    }

    /// Clone catalog documents and diagnostics for publishing outside the read lock.
    pub fn catalog_diagnostic_snapshot(&self) -> Option<CatalogDiagnosticSnapshot> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_ref()?;
        Some(CatalogDiagnosticSnapshot {
            documents: catalog.data().documents.clone(),
            diagnostics: {
                let mut all = catalog.data().diagnostics.clone();
                all.extend(guard.plugin_diagnostics.clone());
                all
            },
        })
    }

    pub fn indexed_workspace(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.indexed_workspace.clone()
    }

    /// Directory used for reindex and reasoner input: last indexed path, else workspace root.
    pub fn effective_index_root(&self) -> Option<PathBuf> {
        let guard = self.inner.read().ok()?;
        let roots = &guard.workspace_roots;
        if let Some(ref indexed) = guard.indexed_workspace {
            if is_path_within_any(roots, indexed) {
                return Some(indexed.clone());
            }
        }
        guard.workspace.clone()
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
                eprintln!("ontocore-lsp: catalog read lock poisoned: {e}");
                return None;
            }
        };
        guard.catalog.as_ref().map(f)
    }

    pub fn workspace_root(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.workspace.clone()
    }

    pub fn resolve_lsp_document_uri(&self, uri: &str) -> Result<PathBuf, String> {
        let roots = self.workspace_roots();
        if roots.is_empty() {
            return Err("workspace not initialized".to_string());
        }
        ontocore_core::resolve_lsp_document_path_any(uri, &roots)
    }

    /// Store unsaved buffer text for a workspace document.
    ///
    /// Returns an error when the buffer exceeds [`MAX_FILE_BYTES`] or when
    /// [`MAX_OPEN_DOCUMENTS`] would be exceeded for a newly opened path.
    pub fn set_document_text(&self, path: PathBuf, text: String) -> Result<(), String> {
        if text.len() as u64 > MAX_FILE_BYTES {
            return Err(format!("document exceeds maximum size of {MAX_FILE_BYTES} bytes"));
        }
        let roots = self.workspace_roots();
        if roots.is_empty() {
            return Err("workspace not initialized".to_string());
        }
        let path = validate_workspace_scope_any(&path, &roots)?;
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
                guard.document_versions.remove(&path);
            }
            Err(e) => eprintln!("ontocore-lsp: failed to remove open document: {e}"),
        }
    }

    pub fn set_document_version(&self, path: PathBuf, version: i32) {
        let path = path.canonicalize().unwrap_or(path);
        if let Ok(mut guard) = self.inner.write() {
            guard.document_versions.insert(path, version);
        }
    }

    pub fn document_version(&self, path: &Path) -> Option<i32> {
        let guard = self.inner.read().ok()?;
        if let Some(v) = guard.document_versions.get(path) {
            return Some(*v);
        }
        path.canonicalize().ok().and_then(|c| guard.document_versions.get(&c).copied())
    }

    /// Atomically replace published diagnostic URIs and return URIs that are now stale.
    pub fn replace_published_diagnostic_uris(&self, current: BTreeSet<String>) -> BTreeSet<String> {
        let Ok(mut guard) = self.inner.write() else {
            return BTreeSet::new();
        };
        let stale: BTreeSet<String> =
            guard.published_diagnostic_uris.difference(&current).cloned().collect();
        guard.published_diagnostic_uris = current;
        stale
    }

    /// True when the path has an unsaved LSP buffer (not merely on disk).
    pub fn is_document_open(&self, path: &Path) -> bool {
        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(_) => return false,
        };
        if guard.open_documents.contains_key(path) {
            return true;
        }
        if let Ok(canonical) = path.canonicalize() {
            return guard.open_documents.contains_key(&canonical);
        }
        false
    }

    /// Prefer unsaved LSP buffer text; fall back to disk.
    pub fn document_text(&self, path: &Path) -> Option<String> {
        let roots = {
            let guard = self.inner.read().ok()?;
            if let Some(text) = guard.open_documents.get(path) {
                return Some(text.clone());
            }
            if let Ok(canonical) = path.canonicalize() {
                if let Some(text) = guard.open_documents.get(&canonical) {
                    return Some(text.clone());
                }
            }
            guard.workspace_roots.clone()
        };
        if roots.is_empty() {
            return None;
        }
        validate_workspace_scope_any(path, &roots).ok()?;
        ontocore_core::read_to_string_capped(path, MAX_FILE_BYTES).ok()
    }
}

fn clear_workspace_state_inner(guard: &mut InnerState) {
    guard.catalog = None;
    guard.workspace_roots.clear();
    guard.workspace = None;
    guard.indexed_workspace = None;
    guard.indexed_at = None;
    guard.open_documents.clear();
    guard.document_versions.clear();
    guard.reasoner_cache.invalidate();
    guard.reasoner_snapshot = None;
    guard.plugin_diagnostics.clear();
}

fn on_workspace_roots_changed_inner(guard: &mut InnerState, new_roots: &[PathBuf]) {
    guard.catalog = None;
    guard.indexed_at = None;
    guard.reasoner_cache.invalidate();
    guard.reasoner_snapshot = None;
    guard.plugin_diagnostics.clear();
    if let Some(ref indexed) = guard.indexed_workspace {
        if !is_path_within_any(new_roots, indexed) {
            guard.indexed_workspace = None;
        }
    }
    guard.open_documents.retain(|path, _| is_path_within_any(new_roots, path));
    guard.document_versions.retain(|path, _| is_path_within_any(new_roots, path));
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
    let path = ontocore_core::workspace_uri_to_path(uri)?;
    if let Some(root) = existing_root {
        validate_workspace_scope(&path, root)
    } else {
        Ok(path)
    }
}

/// Resolve a workspace folder URI for multi-root folder add/remove events.
///
/// When `existing_roots` is non-empty, the folder must lie under at least one root
/// (same rule as document paths). Empty roots accept the first folder as initial root.
pub fn resolve_workspace_folder_uri(
    uri: &str,
    existing_roots: &[PathBuf],
) -> Result<PathBuf, String> {
    let path = ontocore_core::workspace_uri_to_path(uri)?;
    if existing_roots.is_empty() {
        return Ok(path);
    }
    validate_workspace_scope_any(&path, existing_roots)
}

pub(crate) fn canonical_roots_match(a: &Path, b: &Path) -> bool {
    canonical_workspace_root(a).ok().as_ref() == canonical_workspace_root(b).ok().as_ref() || a == b
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
    use ontocore_core::{
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
        state.set_workspace_roots(vec![root.clone()]).expect("set workspace");
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
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let state = ServerState::new();
        state.set_workspace_roots(vec![root.clone()]).expect("set workspace");
        for i in 0..MAX_OPEN_DOCUMENTS {
            let path = root.join(format!("doc-{i}.ttl"));
            state.set_document_text(path, "@prefix ex: <http://ex#> .".to_string()).unwrap();
        }
        let err = state.set_document_text(root.join("extra.ttl"), "x".to_string()).unwrap_err();
        assert!(err.contains("open document limit"));
    }

    #[test]
    fn catalog_and_reasoner_unavailable_before_index() {
        let state = ServerState::new();
        assert!(state.with_catalog_and_reasoner(|_, _| ()).is_none());
    }

    #[test]
    fn resolve_workspace_folder_uri_rejects_outside_existing_roots() {
        let root_dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root = root_dir.path().canonicalize().unwrap();
        let outside_uri =
            url::Url::from_file_path(outside.path().canonicalize().unwrap()).unwrap().to_string();

        assert!(resolve_workspace_folder_uri(&outside_uri, &[root]).is_err());
    }

    #[test]
    fn resolve_workspace_folder_uri_accepts_subdirectory_of_root() {
        let root_dir = tempfile::tempdir().unwrap();
        let sub = root_dir.path().join("ontologies");
        std::fs::create_dir_all(&sub).unwrap();
        let root = root_dir.path().canonicalize().unwrap();
        let sub_uri = url::Url::from_file_path(sub.canonicalize().unwrap()).unwrap().to_string();

        let resolved =
            resolve_workspace_folder_uri(&sub_uri, &[root]).expect("subfolder under root");
        assert!(is_path_within(&root_dir.path().canonicalize().unwrap(), &resolved));
    }
}
