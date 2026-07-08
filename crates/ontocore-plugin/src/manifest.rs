use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Explicit permissions requested by a plugin.
///
/// These are surfaced to UI and enforced for sensitive actions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission {
    WorkspaceRead,
    WorkspaceWrite,
    FilesystemRead,
    FilesystemWrite,
    Network,
    AiInvoke,
    GitRead,
    GitWrite,
    ExternalProcess,
}

impl PluginPermission {
    pub fn parse(s: &str) -> Result<Self, ManifestValidationError> {
        match s {
            "workspace.read" => Ok(Self::WorkspaceRead),
            "workspace.write" => Ok(Self::WorkspaceWrite),
            "filesystem.read" => Ok(Self::FilesystemRead),
            "filesystem.write" => Ok(Self::FilesystemWrite),
            "network" => Ok(Self::Network),
            "ai.invoke" => Ok(Self::AiInvoke),
            "git.read" => Ok(Self::GitRead),
            "git.write" => Ok(Self::GitWrite),
            "external_process" => Ok(Self::ExternalProcess),
            other => Err(ManifestValidationError::UnknownPermission(other.to_string())),
        }
    }
}

/// Supported plugin kinds per PLUGIN_SPEC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginKind {
    Validator,
    Exporter,
    Workflow,
    Documentation,
    Build,
    Release,
    Reasoner,
    Query,
    Ui,
    Ai,
}

impl PluginKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Validator => "validator",
            Self::Exporter => "exporter",
            Self::Workflow => "workflow",
            Self::Documentation => "documentation",
            Self::Build => "build",
            Self::Release => "release",
            Self::Reasoner => "reasoner",
            Self::Query => "query",
            Self::Ui => "ui",
            Self::Ai => "ai",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestValidationError> {
        match s {
            "validator" => Ok(Self::Validator),
            "exporter" => Ok(Self::Exporter),
            "workflow" => Ok(Self::Workflow),
            "documentation" => Ok(Self::Documentation),
            "build" => Ok(Self::Build),
            "release" => Ok(Self::Release),
            "reasoner" => Ok(Self::Reasoner),
            "query" => Ok(Self::Query),
            "ui" => Ok(Self::Ui),
            "ai" => Ok(Self::Ai),
            other => Err(ManifestValidationError::UnknownKind(other.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum ManifestValidationError {
    #[error("unknown plugin kind: {0}")]
    UnknownKind(String),
    #[error("unknown plugin permission: {0}")]
    UnknownPermission(String),
    #[error("unsupported api_version: {0} (expected \"1\")")]
    UnsupportedApiVersion(String),
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}

/// Parsed plugin manifest from workspace TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub kind: PluginKind,
    pub id: Option<String>,
    pub api_version: Option<String>,
    pub permissions: Vec<PluginPermission>,
    pub entry: Option<String>,
    pub capabilities: PluginCapabilities,
    pub config: PluginConfig,
    pub ui: PluginUiContributions,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginConfig {
    #[serde(default)]
    pub require_label: bool,
    #[serde(default)]
    pub iri_prefix: Option<String>,
    #[serde(default)]
    pub shapes_dir: Option<String>,
    #[serde(default)]
    pub output_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginCommandContribution {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginViewContribution {
    pub id: String,
    pub title: String,
    /// Optional view type hint (e.g. "dock", "panel") for UI placement.
    #[serde(default)]
    pub kind: Option<String>,
    /// Optional command to invoke when opening the view.
    #[serde(default)]
    pub command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginPreferencePageContribution {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginContextActionContribution {
    pub id: String,
    pub title: String,
    /// Context scopes like "entity", "graphNode", "workspace".
    #[serde(default)]
    pub scope: Option<String>,
    /// Optional entity kinds the action applies to (e.g. "class", "property").
    #[serde(default)]
    pub applies_to: Vec<String>,
    /// Command invoked when the action is selected.
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginInspectorCard {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub applies_to: Vec<String>,
    #[serde(default)]
    pub command: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginUiContributions {
    #[serde(default)]
    pub commands: Vec<PluginCommandContribution>,
    #[serde(default)]
    pub views: Vec<PluginViewContribution>,
    #[serde(default, rename = "preferences_pages")]
    pub preferences_pages: Vec<PluginPreferencePageContribution>,
    #[serde(default, rename = "context_actions")]
    pub context_actions: Vec<PluginContextActionContribution>,
    #[serde(default, rename = "inspector_cards")]
    pub inspector_cards: Vec<PluginInspectorCard>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginCapabilities {
    #[serde(default)]
    pub build: bool,
    #[serde(default)]
    pub validate: bool,
    #[serde(default)]
    pub release: bool,
    #[serde(default)]
    pub diagnostics: bool,
    #[serde(default)]
    pub export: bool,
}

impl PluginCapabilities {
    pub fn supports_validation(&self) -> bool {
        self.validate || self.diagnostics
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredPlugin {
    pub manifest: PluginManifest,
    pub manifest_path: PathBuf,
}

impl DiscoveredPlugin {
    pub fn plugin_id(&self) -> &str {
        self.manifest.id.as_deref().unwrap_or(&self.manifest.name)
    }
}

#[derive(Debug, Deserialize)]
struct ManifestFile {
    plugin: PluginSection,
    #[serde(default)]
    capabilities: PluginCapabilities,
    #[serde(default)]
    config: PluginConfig,
    #[serde(default)]
    ui: PluginUiContributions,
}

#[derive(Debug, Deserialize)]
struct PluginSection {
    name: String,
    version: String,
    kind: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    api_version: Option<String>,
    #[serde(default)]
    permissions: Vec<String>,
    #[serde(default)]
    entry: Option<String>,
}

/// Parse and validate a plugin manifest from TOML text.
pub fn parse_manifest(text: &str) -> Result<PluginManifest, ManifestValidationError> {
    let file: ManifestFile = toml::from_str(text)
        .map_err(|_| ManifestValidationError::MissingField("valid TOML structure"))?;
    if file.plugin.name.is_empty() {
        return Err(ManifestValidationError::MissingField("plugin.name"));
    }
    if file.plugin.version.is_empty() {
        return Err(ManifestValidationError::MissingField("plugin.version"));
    }
    if let Some(api) = &file.plugin.api_version {
        if api != "1" {
            return Err(ManifestValidationError::UnsupportedApiVersion(api.clone()));
        }
    }
    let kind = PluginKind::parse(&file.plugin.kind)?;
    let mut permissions = Vec::new();
    for p in &file.plugin.permissions {
        permissions.push(PluginPermission::parse(p)?);
    }
    // Backward-compatible defaults for v0.14 manifests that didn't declare permissions yet.
    // v0.15+ plugins should explicitly declare permissions in `[plugin].permissions`.
    if permissions.is_empty() {
        permissions.push(PluginPermission::WorkspaceRead);
        if file.plugin.entry.is_some() {
            permissions.push(PluginPermission::ExternalProcess);
        }
    }
    Ok(PluginManifest {
        name: file.plugin.name,
        version: file.plugin.version,
        kind,
        id: file.plugin.id,
        api_version: file.plugin.api_version,
        permissions,
        entry: file.plugin.entry,
        capabilities: file.capabilities,
        config: file.config,
        ui: file.ui,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_manifest() {
        let text = r#"
[plugin]
name = "example-workflow"
version = "0.1.0"
kind = "workflow"

[capabilities]
build = true
validate = true
"#;
        let manifest = parse_manifest(text).expect("parse");
        assert_eq!(manifest.name, "example-workflow");
        assert_eq!(manifest.kind, PluginKind::Workflow);
        assert!(manifest.capabilities.build);
        assert!(manifest.capabilities.validate);
    }

    #[test]
    fn rejects_unknown_kind() {
        let text = r#"
[plugin]
name = "bad"
version = "0.1.0"
kind = "not-a-kind"
"#;
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn parses_ui_contributions() {
        let text = r#"
[plugin]
name = "naming"
version = "0.1.0"
kind = "validator"
id = "ontocode.naming-validator"
permissions = ["workspace.read"]

[[ui.commands]]
id = "naming.check"
title = "Check naming conventions"

[[ui.views]]
id = "naming.view"
title = "Naming view"

[[ui.preferences_pages]]
id = "naming.prefs"
title = "Naming"

[[ui.context_actions]]
id = "naming.ctx"
title = "Check naming for class"
scope = "entity"
applies_to = ["class"]
command = "naming.check"

[[ui.inspector_cards]]
id = "naming-summary"
title = "Naming"
applies_to = ["class"]
"#;
        let manifest = parse_manifest(text).expect("parse");
        assert_eq!(manifest.ui.commands.len(), 1);
        assert_eq!(manifest.ui.views.len(), 1);
        assert_eq!(manifest.ui.preferences_pages.len(), 1);
        assert_eq!(manifest.ui.context_actions.len(), 1);
        assert_eq!(manifest.ui.inspector_cards.len(), 1);
        assert_eq!(manifest.permissions.len(), 1);
    }
}
