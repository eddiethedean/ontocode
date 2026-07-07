//! Plugin host foundation for OntoCore (v0.14 MVP).
//!
//! Parses plugin manifests and discovers them under `.ontocore/plugins/`.
//! Runtime provider registration ships in v0.14 — see [PLUGIN_SPEC](https://github.com/eddiethedean/ontocode/blob/main/docs/design/PLUGIN_SPEC.md).

mod discovery;
mod manifest;

pub use discovery::{discover_plugins, PluginDiscoveryError, PLUGIN_DIR};
pub use manifest::{parse_manifest, DiscoveredPlugin, PluginCapabilities, PluginManifest};
