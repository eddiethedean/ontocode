mod discovery;
mod host;
mod manifest;
mod protocol;
mod subprocess;
mod traits;

pub use discovery::{discover_plugins, PluginDiscoveryError, PLUGIN_DIR};
pub use host::{
    manifest_for_builtin, merge_plugin_diagnostics, PluginDescriptor, PluginHost, PluginHostError,
    RunPluginResult,
};
pub use manifest::{
    parse_manifest, DiscoveredPlugin, ManifestValidationError, PluginCapabilities,
    PluginCommandContribution, PluginConfig, PluginContextActionContribution, PluginInspectorCard,
    PluginKind, PluginManifest, PluginPermission, PluginPreferencePageContribution,
    PluginUiContributions, PluginViewContribution,
};
pub use protocol::{plugin_diagnostic, PluginDiagnosticWire, PluginOutput};
pub use traits::{
    ExporterPlugin, ValidatorPlugin, WorkflowPlugin, WorkflowRequest, WorkflowResult,
};
