//! Register built-in reference plugins on a [`PluginHost`].
use ontocore_plugin::PluginHost;
use ontocore_plugin_markdown_export::MarkdownExportPlugin;
use ontocore_plugin_naming::NamingValidatorPlugin;
use ontocore_plugin_shacl::ShaclValidatorPlugin;
use std::path::Path;

pub fn load_plugin_host(
    workspace: impl AsRef<Path>,
) -> Result<PluginHost, ontocore_plugin::PluginHostError> {
    let mut host = PluginHost::new(workspace.as_ref());
    host.discover()?;
    register_builtins(&mut host);
    Ok(host)
}

pub fn register_builtins(host: &mut PluginHost) {
    for plugin in host.discovered().to_vec() {
        let id = plugin.plugin_id().to_string();
        match id.as_str() {
            "ontocode.naming-validator" => {
                let cfg = &plugin.manifest.config;
                host.register_validator(Box::new(NamingValidatorPlugin::from_config(
                    cfg.require_label || plugin.manifest.capabilities.validate,
                    cfg.iri_prefix.clone(),
                )));
            }
            "ontocode.markdown-export" => {
                host.register_exporter(Box::new(MarkdownExportPlugin));
            }
            "ontocode.shacl-validator" => {
                let shapes = plugin.manifest.config.shapes_dir.clone();
                host.register_validator(Box::new(ShaclValidatorPlugin::new(
                    host.workspace(),
                    shapes.as_deref(),
                )));
            }
            _ => {}
        }
    }
}

/// Ensure built-in manifests are discoverable when workspace has no `.ontocore/plugins/`.
pub fn ensure_builtin_manifests(workspace: &Path) -> std::io::Result<()> {
    let dir = workspace.join(ontocore_plugin::PLUGIN_DIR);
    std::fs::create_dir_all(&dir)?;
    let naming = dir.join("naming-validator.toml");
    if !naming.exists() {
        std::fs::write(naming, include_str!("../fixtures/plugins/naming-validator.toml"))?;
    }
    Ok(())
}
