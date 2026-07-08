use ontocore_catalog::OntologyCatalog;
use ontocore_docs::{export_workspace, ExportOptions};
use ontocore_plugin::ExporterPlugin;
use std::path::{Path, PathBuf};

pub const PLUGIN_ID: &str = "ontocode.markdown-export";

pub struct MarkdownExportPlugin;

impl ExporterPlugin for MarkdownExportPlugin {
    fn id(&self) -> &str {
        PLUGIN_ID
    }

    fn export(
        &self,
        catalog: &OntologyCatalog,
        _workspace: &Path,
        options: ExportOptions,
    ) -> Result<Vec<PathBuf>, ontocore_docs::ExportError> {
        export_workspace(catalog, options)?;
        Ok(vec![])
    }
}
