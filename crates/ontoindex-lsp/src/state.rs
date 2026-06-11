use ontoindex_catalog::{IndexBuilder, OntologyCatalog};
use ontoindex_core::{canonical_workspace_root, validate_workspace_scope};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ServerState {
    inner: Arc<RwLock<InnerState>>,
}

struct InnerState {
    catalog: Option<OntologyCatalog>,
    workspace: Option<PathBuf>,
    indexed_at: Option<u64>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerState {
                catalog: None,
                workspace: None,
                indexed_at: None,
            })),
        }
    }

    pub fn index_workspace(
        &self,
        workspace: PathBuf,
    ) -> Result<(ontoindex_catalog::CatalogStats, u64), String> {
        let workspace = canonical_workspace_root(&workspace)?;

        if let Some(existing) = self.workspace_root() {
            validate_workspace_scope(&workspace, &existing)?;
        }

        let catalog =
            IndexBuilder::new().workspace(&workspace).build().map_err(|e| e.to_string())?;

        let stats = catalog.data().stats();
        let indexed_at = now_epoch_secs();

        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        guard.catalog = Some(catalog);
        guard.workspace = Some(workspace);
        guard.indexed_at = Some(indexed_at);

        Ok((stats, indexed_at))
    }

    pub fn with_catalog<T>(&self, f: impl FnOnce(&OntologyCatalog) -> T) -> Option<T> {
        let guard = self.inner.read().ok()?;
        guard.catalog.as_ref().map(f)
    }

    pub fn workspace_root(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.workspace.clone()
    }
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
