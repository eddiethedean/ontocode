use crate::discovery::{discover_plugins, PluginDiscoveryError};
use crate::manifest::{DiscoveredPlugin, PluginKind, PluginManifest, PluginPermission};
use crate::protocol::{jail_output_paths, plugin_diagnostic, PluginOutput};
use crate::subprocess::{run_plugin_subprocess, SubprocessError, SubprocessRequest};
use crate::traits::{ExporterPlugin, ValidatorPlugin, WorkflowPlugin, WorkflowRequest};
use ontocore_catalog::OntologyCatalog;
use ontocore_core::{Diagnostic, DiagnosticSeverity};
use ontocore_docs::ExportOptions;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginHostError {
    #[error(transparent)]
    Discovery(#[from] PluginDiscoveryError),
    #[error(transparent)]
    Subprocess(#[from] SubprocessError),
    #[error("plugin not found: {0}")]
    NotFound(String),
    #[error("plugin is disabled: {0}")]
    Disabled(String),
    #[error("plugin {0} is missing required permission: {1}")]
    MissingPermission(String, String),
    #[error("plugin {0} does not support action {1}")]
    UnsupportedAction(String, String),
    #[error("export failed: {0}")]
    Export(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginDescriptor {
    pub id: String,
    pub name: String,
    pub version: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<PluginPermission>,
    pub capabilities: crate::manifest::PluginCapabilities,
    pub manifest_path: String,
    pub ui: crate::manifest::PluginUiContributions,
    pub in_process: bool,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunPluginResult {
    pub diagnostics: Vec<Diagnostic>,
    pub output_paths: Vec<String>,
    pub logs: Option<String>,
    pub view_html: Option<String>,
    pub success: bool,
}

pub struct PluginHost {
    workspace: PathBuf,
    discovered: Vec<DiscoveredPlugin>,
    disabled: std::collections::HashSet<String>,
    validators: HashMap<String, Box<dyn ValidatorPlugin>>,
    exporters: HashMap<String, Box<dyn ExporterPlugin>>,
    workflows: HashMap<String, Box<dyn WorkflowPlugin>>,
}

impl PluginHost {
    pub fn new(workspace: impl AsRef<Path>) -> Self {
        Self {
            workspace: workspace.as_ref().to_path_buf(),
            discovered: Vec::new(),
            disabled: std::collections::HashSet::new(),
            validators: HashMap::new(),
            exporters: HashMap::new(),
            workflows: HashMap::new(),
        }
    }

    pub fn discover(&mut self) -> Result<(), PluginHostError> {
        self.discovered = discover_plugins(&self.workspace)?;
        Ok(())
    }

    pub fn register_validator(&mut self, plugin: Box<dyn ValidatorPlugin>) {
        self.validators.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_exporter(&mut self, plugin: Box<dyn ExporterPlugin>) {
        self.exporters.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_workflow(&mut self, plugin: Box<dyn WorkflowPlugin>) {
        self.workflows.insert(plugin.id().to_string(), plugin);
    }

    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn discovered(&self) -> &[DiscoveredPlugin] {
        &self.discovered
    }

    pub fn disable_plugin(&mut self, plugin_id: &str) {
        self.disabled.insert(plugin_id.to_string());
        self.validators.remove(plugin_id);
        self.exporters.remove(plugin_id);
        self.workflows.remove(plugin_id);
    }

    pub fn enable_plugin(&mut self, plugin_id: &str) {
        self.disabled.remove(plugin_id);
    }

    pub fn is_disabled(&self, plugin_id: &str) -> bool {
        self.disabled.contains(plugin_id)
    }

    pub fn list_plugins(&self) -> Vec<PluginDescriptor> {
        self.discovered
            .iter()
            .map(|p| {
                let id = p.plugin_id().to_string();
                PluginDescriptor {
                    id: id.clone(),
                    name: p.manifest.name.clone(),
                    version: p.manifest.version.clone(),
                    kind: p.manifest.kind.as_str().to_string(),
                    api_version: p.manifest.api_version.clone(),
                    permissions: p.manifest.permissions.clone(),
                    capabilities: p.manifest.capabilities.clone(),
                    manifest_path: p.manifest_path.display().to_string(),
                    ui: p.manifest.ui.clone(),
                    in_process: self.validators.contains_key(&id)
                        || self.exporters.contains_key(&id)
                        || self.workflows.contains_key(&id),
                    disabled: self.is_disabled(&id),
                }
            })
            .collect()
    }

    fn find_plugin(&self, plugin_id: &str) -> Result<&DiscoveredPlugin, PluginHostError> {
        self.discovered
            .iter()
            .find(|p| p.plugin_id() == plugin_id)
            .ok_or_else(|| PluginHostError::NotFound(plugin_id.to_string()))
    }

    fn ensure_enabled(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        if self.is_disabled(plugin_id) {
            return Err(PluginHostError::Disabled(plugin_id.to_string()));
        }
        Ok(())
    }

    fn ensure_permission(
        plugin: &DiscoveredPlugin,
        plugin_id: &str,
        required: PluginPermission,
    ) -> Result<(), PluginHostError> {
        if plugin.manifest.permissions.contains(&required) {
            return Ok(());
        }
        Err(PluginHostError::MissingPermission(plugin_id.to_string(), format!("{required:?}")))
    }

    pub fn run_validate_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
    ) -> Result<Vec<Diagnostic>, PluginHostError> {
        self.ensure_enabled(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        if !plugin.manifest.capabilities.supports_validation() {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "validate".into(),
            ));
        }
        if let Some(v) = self.validators.get(plugin_id) {
            return Ok(v.validate(catalog, &self.workspace));
        }
        if plugin.manifest.entry.is_some() {
            Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
            let output = run_plugin_subprocess(
                plugin,
                SubprocessRequest {
                    action: "validate",
                    workspace: &self.workspace,
                    step: None,
                    extra_args: &[],
                },
            )?;
            let output = jail_output_paths(plugin_id, &self.workspace, output);
            return Ok(wire_to_diagnostics(plugin_id, &self.workspace, output));
        }
        Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")))
    }

    pub fn run_all_validators(&self, catalog: &OntologyCatalog) -> Vec<Diagnostic> {
        let mut all = Vec::new();
        for plugin in &self.discovered {
            if !plugin.manifest.capabilities.supports_validation() {
                continue;
            }
            let id = plugin.plugin_id();
            match self.run_validate_plugin(id, catalog) {
                Ok(mut diags) => all.append(&mut diags),
                Err(err) => all.push(plugin_diagnostic(
                    id,
                    "plugin_error",
                    DiagnosticSeverity::Error,
                    err.to_string(),
                    self.workspace.clone(),
                    None,
                )),
            }
        }
        all
    }

    pub fn run_export_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
        options: ExportOptions,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_enabled(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceWrite)?;
        if !plugin.manifest.capabilities.export {
            return Err(PluginHostError::UnsupportedAction(plugin_id.to_string(), "export".into()));
        }
        if let Some(e) = self.exporters.get(plugin_id) {
            let paths = e
                .export(catalog, &self.workspace, options)
                .map_err(|e| PluginHostError::Export(e.to_string()))?;
            return Ok(RunPluginResult {
                diagnostics: Vec::new(),
                output_paths: paths.iter().map(|p| p.display().to_string()).collect(),
                logs: None,
                view_html: None,
                success: true,
            });
        }
        if plugin.manifest.entry.is_some() {
            Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
            let output = run_plugin_subprocess(
                plugin,
                SubprocessRequest {
                    action: "export",
                    workspace: &self.workspace,
                    step: None,
                    extra_args: &[],
                },
            )?;
            let output = jail_output_paths(plugin_id, &self.workspace, output);
            let diags = wire_to_diagnostics(plugin_id, &self.workspace, output.clone());
            return Ok(RunPluginResult {
                diagnostics: diags,
                output_paths: output.output_paths,
                logs: output.logs,
                view_html: output.view_html,
                success: true,
            });
        }
        Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")))
    }

    pub fn run_workflow_plugin(
        &self,
        plugin_id: &str,
        step: &str,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_enabled(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceWrite)?;
        if plugin.manifest.kind != PluginKind::Workflow && plugin.manifest.kind != PluginKind::Build
        {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "workflow".into(),
            ));
        }
        if let Some(w) = self.workflows.get(plugin_id) {
            let result =
                w.run(&self.workspace, WorkflowRequest { step: step.to_string(), dry_run: false });
            return Ok(RunPluginResult {
                diagnostics: result.diagnostics,
                output_paths: Vec::new(),
                logs: Some(result.logs),
                view_html: None,
                success: result.success,
            });
        }
        if plugin.manifest.entry.is_some() {
            Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
            let output = run_plugin_subprocess(
                plugin,
                SubprocessRequest {
                    action: "workflow",
                    workspace: &self.workspace,
                    step: Some(step),
                    extra_args: &[],
                },
            )?;
            let output = jail_output_paths(plugin_id, &self.workspace, output);
            let diags = wire_to_diagnostics(plugin_id, &self.workspace, output.clone());
            return Ok(RunPluginResult {
                diagnostics: diags,
                output_paths: output.output_paths,
                logs: output.logs,
                view_html: output.view_html,
                success: output.exit_message.is_none(),
            });
        }
        Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")))
    }

    pub fn run_plugin_action(
        &self,
        plugin_id: &str,
        action: &str,
        catalog: Option<&OntologyCatalog>,
        export_options: Option<ExportOptions>,
        step: Option<&str>,
        view_id: Option<&str>,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_enabled(plugin_id)?;
        match action {
            "validate" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "validate (no catalog)".into(),
                    )
                })?;
                let diags = self.run_validate_plugin(plugin_id, catalog)?;
                Ok(RunPluginResult {
                    diagnostics: diags,
                    output_paths: Vec::new(),
                    logs: None,
                    view_html: None,
                    success: true,
                })
            }
            "export" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "export (no catalog)".into(),
                    )
                })?;
                let options =
                    export_options.unwrap_or_else(|| ExportOptions::markdown("plugin-out"));
                self.run_export_plugin(plugin_id, catalog, options)
            }
            "workflow" => self.run_workflow_plugin(plugin_id, step.unwrap_or("qc")),
            "ui_view" => {
                let plugin = self.find_plugin(plugin_id)?;
                Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
                Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
                let view = view_id.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "ui_view (missing view_id)".into(),
                    )
                })?;
                let extra = vec!["--view".to_string(), view.to_string()];
                let output = run_plugin_subprocess(
                    plugin,
                    SubprocessRequest {
                        action: "ui-view",
                        workspace: &self.workspace,
                        step: None,
                        extra_args: &extra,
                    },
                )?;
                let output = jail_output_paths(plugin_id, &self.workspace, output);
                let diags = wire_to_diagnostics(plugin_id, &self.workspace, output.clone());
                Ok(RunPluginResult {
                    diagnostics: diags,
                    output_paths: output.output_paths,
                    logs: output.logs,
                    view_html: output.view_html,
                    success: output.exit_message.is_none(),
                })
            }
            _ => Err(PluginHostError::UnsupportedAction(plugin_id.to_string(), action.into())),
        }
    }
}

fn wire_to_diagnostics(plugin_id: &str, workspace: &Path, output: PluginOutput) -> Vec<Diagnostic> {
    output.diagnostics.into_iter().map(|d| d.into_diagnostic(plugin_id, workspace)).collect()
}

pub fn merge_plugin_diagnostics(base: &mut Vec<Diagnostic>, plugin: Vec<Diagnostic>) {
    base.extend(plugin);
}

pub fn manifest_for_builtin(id: &str) -> Option<PluginManifest> {
    let text = match id {
        "ontocode.naming-validator" => include_str!("../fixtures/builtin-naming.toml"),
        "ontocode.markdown-export" => include_str!("../fixtures/builtin-markdown.toml"),
        "ontocode.shacl-validator" => include_str!("../fixtures/builtin-shacl.toml"),
        _ => return None,
    };
    crate::manifest::parse_manifest(text).ok()
}
