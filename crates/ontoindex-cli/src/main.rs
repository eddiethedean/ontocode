use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use ontoindex_catalog::{CatalogStats, IndexBuilder, OntologyCatalog};
use ontoindex_query::{
    query_catalog,
    sparql::to_json as sparql_to_json,
    sparql_catalog,
    sql::{to_csv as sql_to_csv, to_json as sql_to_json},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "ontoindex",
    version,
    about = "Local-first ontology index and query engine (OntoCode v0.4)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan and index ontology files in a workspace
    Index {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run a SQL-like query over ontology tables
    Query {
        /// Workspace directory
        workspace: PathBuf,
        /// SQL query string
        sql: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run a SPARQL query over indexed triples
    Sparql {
        /// Workspace directory
        workspace: PathBuf,
        /// SPARQL query string
        query: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Validate ontology files in a workspace
    Validate {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
    },
    /// Inspect catalog statistics for a workspace
    Inspect {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Apply Turtle patch operations from a JSON file
    Patch {
        /// Turtle document to patch
        document: PathBuf,
        /// JSON file containing an array of patch operations
        patch_file: PathBuf,
        /// Preview changes without writing to disk
        #[arg(long)]
        preview: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Index { workspace, format } => {
            let catalog = build_catalog(&workspace)?;
            print_stats(&catalog.data().stats(), format)?;
        }
        Commands::Query { workspace, sql, format } => {
            let catalog = build_catalog(&workspace)?;
            let result = query_catalog(&catalog, &sql).context("query failed")?;
            print_query_result(&result.columns, &result.rows, format)?;
        }
        Commands::Sparql { workspace, query, format } => {
            let catalog = build_catalog(&workspace)?;
            let result = sparql_catalog(&catalog, &query).context("sparql failed")?;
            match format {
                OutputFormat::Json => println!("{}", sparql_to_json(&result)?),
                _ => print_query_result(&result.columns, &result.rows, format)?,
            }
        }
        Commands::Validate { workspace } => {
            let catalog = build_catalog(&workspace)?;
            let data = catalog.data();
            let mut error_count = 0usize;
            let mut warning_count = 0usize;

            for diag in &data.diagnostics {
                match diag.severity {
                    ontoindex_core::DiagnosticSeverity::Error => {
                        error_count += 1;
                        eprintln!(
                            "ERROR [{}] {}:{}:{}: {}",
                            diag.code.as_str(),
                            diag.file.display(),
                            diag.range.line.unwrap_or(0),
                            diag.range.column.unwrap_or(0),
                            diag.message
                        );
                    }
                    ontoindex_core::DiagnosticSeverity::Warning => {
                        warning_count += 1;
                        eprintln!(
                            "WARN  [{}] {}:{}:{}: {}",
                            diag.code.as_str(),
                            diag.file.display(),
                            diag.range.line.unwrap_or(0),
                            diag.range.column.unwrap_or(0),
                            diag.message
                        );
                    }
                    ontoindex_core::DiagnosticSeverity::Info => {
                        eprintln!(
                            "INFO  [{}] {}: {}",
                            diag.code.as_str(),
                            diag.file.display(),
                            diag.message
                        );
                    }
                }
            }

            if error_count > 0 {
                bail!("validation failed with {error_count} error(s), {warning_count} warning(s)");
            }
            println!(
                "OK: indexed {} ontology file(s), {} warning(s)",
                data.stats().ontology_count,
                warning_count
            );
        }
        Commands::Inspect { workspace, format } => {
            let catalog = build_catalog(&workspace)?;
            print_stats(&catalog.data().stats(), format)?;
        }
        Commands::Patch { document, patch_file, preview } => {
            let patches: Vec<ontoindex_owl::PatchOp> =
                serde_json::from_slice(&std::fs::read(&patch_file)?)
                    .context("failed to parse patch JSON")?;
            let catalog = IndexBuilder::new()
                .workspace(document.parent().unwrap_or(std::path::Path::new(".")))
                .build()
                .ok();
            let namespaces = catalog
                .as_ref()
                .and_then(|c| {
                    c.data().documents.iter().find(|d| {
                        d.path.canonicalize().ok().as_ref() == document.canonicalize().ok().as_ref()
                    })
                })
                .map(|d| d.namespaces.clone())
                .unwrap_or_default();
            let result = ontoindex_owl::apply_patches(&document, &patches, preview, &namespaces)
                .context("patch failed")?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            if !preview && result.applied {
                println!("applied");
            }
        }
    }
    Ok(())
}

fn build_catalog(workspace: &PathBuf) -> Result<OntologyCatalog> {
    IndexBuilder::new()
        .workspace(workspace)
        .build()
        .with_context(|| format!("failed to index workspace {}", workspace.display()))
}

fn print_stats(stats: &CatalogStats, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(stats)?),
        OutputFormat::Csv | OutputFormat::Text => {
            println!("ontologies:            {}", stats.ontology_count);
            println!("classes:               {}", stats.class_count);
            println!("object_properties:     {}", stats.object_property_count);
            println!("data_properties:       {}", stats.data_property_count);
            println!("annotation_properties: {}", stats.annotation_property_count);
            println!("individuals:           {}", stats.individual_count);
            println!("axioms:                {}", stats.axiom_count);
            println!("annotations:           {}", stats.annotation_count);
            println!("triples:               {}", stats.triple_count);
            println!("parse_errors:          {}", stats.error_count);
            println!("diagnostic_errors:     {}", stats.diagnostic_error_count);
            println!("diagnostic_warnings:   {}", stats.diagnostic_warning_count);
        }
    }
    Ok(())
}

fn print_query_result(
    columns: &[String],
    rows: &[std::collections::BTreeMap<String, String>],
    format: OutputFormat,
) -> Result<()> {
    let result = ontoindex_query::sql::QueryResult {
        columns: columns.to_vec(),
        rows: rows.to_vec(),
        truncated: false,
    };
    match format {
        OutputFormat::Json => println!("{}", sql_to_json(&result)?),
        OutputFormat::Csv => print!("{}", sql_to_csv(&result)?),
        OutputFormat::Text => {
            if columns.is_empty() {
                println!("(no columns)");
                return Ok(());
            }
            println!("{}", columns.join("\t"));
            for row in rows {
                let line: Vec<String> =
                    columns.iter().map(|c| row.get(c).cloned().unwrap_or_default()).collect();
                println!("{}", line.join("\t"));
            }
        }
    }
    Ok(())
}
