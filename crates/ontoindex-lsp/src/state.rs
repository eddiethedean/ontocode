use ontoindex_catalog::{IndexBuilder, OntologyCatalog};
use std::path::PathBuf;
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

    pub fn workspace(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.workspace.clone()
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn uri_to_path(uri: &str) -> Result<PathBuf, String> {
    let url = url::Url::parse(uri).map_err(|e| e.to_string())?;
    url.to_file_path().map_err(|_| format!("invalid file URI: {uri}"))
}

pub fn path_to_uri(path: &std::path::Path) -> String {
    let abs = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    url::Url::from_file_path(&abs)
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("file://{}", abs.display()))
}

fn now_epoch_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
