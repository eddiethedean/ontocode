use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

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
    Ok(PluginManifest {
        name: file.plugin.name,
        version: file.plugin.version,
        kind,
        id: file.plugin.id,
        api_version: file.plugin.api_version,
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

[[ui.commands]]
id = "naming.check"
title = "Check naming conventions"

[[ui.inspector_cards]]
id = "naming-summary"
title = "Naming"
applies_to = ["class"]
"#;
        let manifest = parse_manifest(text).expect("parse");
        assert_eq!(manifest.ui.commands.len(), 1);
        assert_eq!(manifest.ui.inspector_cards.len(), 1);
    }
}
