use clap::Parser;
use ontocore_catalog::IndexBuilder;
use ontocore_docs::ExportOptions;
use ontocore_plugin::ExporterPlugin;
use ontocore_plugin::PluginOutput;
use ontocore_plugin_markdown_export::MarkdownExportPlugin;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ontocore-plugin-markdown-export")]
struct Cli {
    #[arg(default_value = "export")]
    action: String,
    #[arg(long)]
    workspace: PathBuf,
    #[arg(long, default_value = ".ontocore/plugin-out")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if cli.action != "export" {
        eprintln!("only export action is supported");
        std::process::exit(2);
    }
    let catalog = IndexBuilder::new().workspace(&cli.workspace).build().expect("index");
    let plugin = MarkdownExportPlugin;
    let options = ExportOptions::markdown(&cli.output);
    match plugin.export(&catalog, &cli.workspace, options) {
        Ok(_) => {
            let out = PluginOutput {
                output_paths: vec![cli.output.display().to_string()],
                ..Default::default()
            };
            println!("{}", serde_json::to_string(&out).unwrap());
        }
        Err(e) => {
            let out = PluginOutput { exit_message: Some(e.to_string()), ..Default::default() };
            println!("{}", serde_json::to_string(&out).unwrap());
            std::process::exit(1);
        }
    }
}
