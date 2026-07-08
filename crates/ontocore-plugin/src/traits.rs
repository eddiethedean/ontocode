use ontocore_catalog::OntologyCatalog;
use ontocore_core::Diagnostic;
use ontocore_docs::{ExportError, ExportOptions};
use std::path::{Path, PathBuf};

pub trait ValidatorPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn validate(&self, catalog: &OntologyCatalog, workspace: &Path) -> Vec<Diagnostic>;
}

pub trait ExporterPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn export(
        &self,
        catalog: &OntologyCatalog,
        workspace: &Path,
        options: ExportOptions,
    ) -> Result<Vec<PathBuf>, ExportError>;
}

#[derive(Debug, Clone)]
pub struct WorkflowRequest {
    pub step: String,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct WorkflowResult {
    pub success: bool,
    pub logs: String,
    pub diagnostics: Vec<Diagnostic>,
}

pub trait WorkflowPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn run(&self, workspace: &Path, request: WorkflowRequest) -> WorkflowResult;
}
