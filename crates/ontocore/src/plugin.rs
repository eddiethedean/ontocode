//! Plugin manifest discovery (v0.14 host foundation).

pub use ontocore_plugin::{
    discover_plugins, parse_manifest, DiscoveredPlugin, PluginCapabilities, PluginDiscoveryError,
    PluginManifest, PLUGIN_DIR,
};
