use serde::Deserialize;
use std::path::PathBuf;

/// Parsed plugin manifest from workspace TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub kind: String,
    pub id: Option<String>,
    pub api_version: Option<String>,
    pub entry: Option<String>,
    pub capabilities: PluginCapabilities,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredPlugin {
    pub manifest: PluginManifest,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ManifestFile {
    plugin: PluginSection,
    #[serde(default)]
    capabilities: PluginCapabilities,
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

/// Parse a plugin manifest from TOML text.
pub fn parse_manifest(text: &str) -> Result<PluginManifest, toml::de::Error> {
    let file: ManifestFile = toml::from_str(text)?;
    Ok(PluginManifest {
        name: file.plugin.name,
        version: file.plugin.version,
        kind: file.plugin.kind,
        id: file.plugin.id,
        api_version: file.plugin.api_version,
        entry: file.plugin.entry,
        capabilities: file.capabilities,
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
        assert_eq!(manifest.kind, "workflow");
        assert!(manifest.capabilities.build);
        assert!(manifest.capabilities.validate);
    }
}
