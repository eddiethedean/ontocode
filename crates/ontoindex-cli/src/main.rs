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
    about = "Local-first ontology index and query engine (OntoCode v0.1)"
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
            let stats = catalog.data().stats();
            if stats.error_count > 0 {
                for doc in &catalog.data().documents {
                    if matches!(doc.parse_status, ontoindex_core::ParseStatus::Error) {
                        eprintln!(
                            "ERROR {}: {}",
                            doc.path.display(),
                            doc.parse_message.as_deref().unwrap_or("parse error")
                        );
                    }
                }
                bail!("validation failed with {} error(s)", stats.error_count);
            }
            println!(
                "OK: indexed {} ontology file(s), {} parse error(s)",
                stats.ontology_count, stats.error_count
            );
        }
        Commands::Inspect { workspace, format } => {
            let catalog = build_catalog(&workspace)?;
            print_stats(&catalog.data().stats(), format)?;
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
        }
    }
    Ok(())
}

fn print_query_result(
    columns: &[String],
    rows: &[std::collections::BTreeMap<String, String>],
    format: OutputFormat,
) -> Result<()> {
    let result =
        ontoindex_query::sql::QueryResult { columns: columns.to_vec(), rows: rows.to_vec() };
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
