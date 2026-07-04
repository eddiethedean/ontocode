use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use ontocore_catalog::{CatalogStats, IndexBuilder, OntologyCatalog};
use ontocore_diff::{
    diff_directories, diff_git_refs, format_diff_json, format_diff_markdown, format_diff_text,
    parse_git_range,
};
use ontocore_query::{
    query_catalog,
    sparql::to_json as sparql_to_json,
    sparql_catalog,
    sql::{to_csv as sql_to_csv, to_json as sql_to_json},
};
use ontocore_reasoner::{classify, explain, ExplanationRequest, ReasonerId, WorkspaceInputLoader};
use ontocore_refactor::{
    apply_refactor_plan_checked, find_usages, preview_extract_module, preview_migrate_namespace,
    preview_move_entity, preview_rename_iri, RefactorPlan,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "ontocore",
    version,
    about = "Local-first ontology index and query engine (OntoCode v0.10)"
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
    /// Classify ontologies in a workspace with a reasoner profile
    Classify {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Reasoner profile: el, rl, rdfs, dl, auto
        #[arg(long, default_value = "el")]
        profile: String,
        /// Emit profile-detection warnings
        #[arg(long, default_value_t = true)]
        auto_profile: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Explain unsatisfiability for a class IRI
    Explain {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Class IRI to explain
        #[arg(long)]
        class: String,
        /// Reasoner profile
        #[arg(long, default_value = "el")]
        profile: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run ROBOT CLI subcommands (validate, merge, report)
    Robot {
        #[command(subcommand)]
        command: RobotCommands,
    },
    /// Workspace refactoring (rename, migrate, move, extract)
    Refactor {
        #[command(subcommand)]
        command: RefactorCommands,
    },
    /// Semantic diff between git refs, directories, or indexed snapshots
    Diff {
        /// Git range (`main..feature`), single ref vs working tree, or omitted with `--left`/`--right`
        #[arg(value_name = "GIT_RANGE")]
        git_range: Option<String>,
        /// Left directory or git ref
        #[arg(long)]
        left: Option<PathBuf>,
        /// Right directory or git ref (`WORKTREE` for working tree)
        #[arg(long)]
        right: Option<PathBuf>,
        /// Git repository root (defaults to `--left` parent or current directory)
        #[arg(long)]
        repo: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = DiffFormat::Text)]
        format: DiffFormat,
        #[arg(long)]
        breaking_only: bool,
    },
}

#[derive(Subcommand)]
enum RefactorCommands {
    /// List usages of an entity IRI across the workspace
    Usages {
        workspace: PathBuf,
        iri: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Rename an entity IRI across Turtle files
    Rename {
        workspace: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Migrate a namespace base IRI across the workspace
    MigrateNamespace {
        workspace: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Move an entity block to another Turtle file
    Move {
        workspace: PathBuf,
        iri: String,
        #[arg(long)]
        to: PathBuf,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Extract selected entities into a new module file
    Extract {
        workspace: PathBuf,
        #[arg(long, value_delimiter = ',')]
        entities: Vec<String>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        leave_stub: bool,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum RobotCommands {
    /// Run `robot validate`
    Validate {
        /// Ontology file or directory
        path: PathBuf,
        #[arg(long)]
        robot_path: Option<String>,
    },
    /// Run `robot merge`
    Merge {
        #[arg(long, required = true)]
        inputs: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        robot_path: Option<String>,
    },
    /// Run `robot report`
    Report {
        /// Ontology file or directory
        path: PathBuf,
        #[arg(long)]
        report: PathBuf,
        #[arg(long)]
        robot_path: Option<String>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum DiffFormat {
    Text,
    Json,
    Markdown,
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
                    ontocore_core::DiagnosticSeverity::Error => {
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
                    ontocore_core::DiagnosticSeverity::Warning => {
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
                    ontocore_core::DiagnosticSeverity::Info => {
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
            let patches: Vec<ontocore_owl::PatchOp> =
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
            let result = ontocore_owl::apply_patches(&document, &patches, preview, &namespaces)
                .context("patch failed")?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            let has_errors = result.diagnostics.iter().any(|d| d.severity == "error");
            if has_errors || (!preview && !patches.is_empty() && !result.applied) {
                bail!("patch failed with {} diagnostic(s)", result.diagnostics.len().max(1));
            }
            if !preview && result.applied {
                println!("applied");
            }
        }
        Commands::Classify { workspace, profile, auto_profile, format } => {
            let result = run_classify(&workspace, &profile, auto_profile)?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!("profile: {}", result.profile_used);
                    println!("consistent: {}", result.consistent);
                    println!("unsatisfiable: {}", result.unsatisfiable.len());
                    println!("inferred_edges: {}", result.inferred.edges.len());
                    println!("new_inferences: {}", result.new_inferences.len());
                    println!("duration_ms: {}", result.duration_ms);
                    for iri in &result.unsatisfiable {
                        println!("UNSAT {iri}");
                    }
                    for edge in &result.new_inferences {
                        println!("INFERRED {} SubClassOf {}", edge.child, edge.parent);
                    }
                }
            }
            if !result.consistent {
                bail!(
                    "classification found {} unsatisfiable class(es)",
                    result.unsatisfiable.len()
                );
            }
        }
        Commands::Explain { workspace, class, profile, format } => {
            let result = run_explain(&workspace, &class, &profile)?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!("class: {}", result.class_iri);
                    println!("{}", result.text);
                }
            }
        }
        Commands::Robot { command } => {
            use ontocore_robot::{robot_merge, robot_report, robot_validate};
            let output = match command {
                RobotCommands::Validate { path, robot_path } => {
                    robot_validate(robot_path.as_deref(), &path)?
                }
                RobotCommands::Merge { inputs, output, robot_path } => {
                    let input_strs: Vec<String> =
                        inputs.iter().map(|p| p.display().to_string()).collect();
                    robot_merge(robot_path.as_deref(), &input_strs, &output)?
                }
                RobotCommands::Report { path, report, robot_path } => {
                    robot_report(robot_path.as_deref(), &path, &report)?
                }
            };
            if !output.stdout.is_empty() {
                print!("{}", output.stdout);
            }
            if !output.stderr.is_empty() {
                eprint!("{}", output.stderr);
            }
            if output.exit_code != 0 {
                std::process::exit(output.exit_code);
            }
        }
        Commands::Refactor { command } => match command {
            RefactorCommands::Usages { workspace, iri, format } => {
                let catalog = build_catalog(&workspace)?;
                let usages = find_usages(&catalog, &iri);
                match format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&usages)?);
                    }
                    _ => {
                        for u in usages {
                            println!(
                                "{}:{}:{} {:?} {}",
                                u.file.display(),
                                u.line.unwrap_or(0),
                                u.column.unwrap_or(0),
                                u.kind,
                                u.context
                            );
                        }
                    }
                }
            }
            RefactorCommands::Rename { workspace, from, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_rename_iri(&catalog, &from, &to, &HashMap::new())?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::MigrateNamespace { workspace, from, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_migrate_namespace(&catalog, &from, &to, &HashMap::new())?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Move { workspace, iri, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_move_entity(&catalog, &iri, &to, &HashMap::new(), &workspace)?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Extract { workspace, entities, out, leave_stub, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_extract_module(
                    &catalog,
                    &entities,
                    &out,
                    leave_stub,
                    &HashMap::new(),
                    &workspace,
                )?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
        },
        Commands::Diff { git_range, left, right, repo, format, breaking_only } => {
            let diff =
                run_diff(git_range.as_deref(), left.as_deref(), right.as_deref(), repo.as_deref())?;
            let output = match format {
                DiffFormat::Json => format_diff_json(&diff),
                DiffFormat::Markdown => format_diff_markdown(&diff, breaking_only),
                DiffFormat::Text => format_diff_text(&diff, breaking_only),
            };
            println!("{output}");
        }
    }
    Ok(())
}

fn run_refactor_plan(
    plan: &RefactorPlan,
    preview: bool,
    format: OutputFormat,
    workspace: &Path,
) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(plan)?),
        _ => {
            for change in &plan.changes {
                println!("{}: {} byte(s) changed", change.path.display(), change.hunks.len());
            }
            for w in &plan.warnings {
                eprintln!("WARN: {w}");
            }
        }
    }
    let files_written = apply_refactor_plan_checked(plan, preview, Some(workspace))?;
    if !preview {
        println!("applied {files_written} file(s)");
    }
    Ok(())
}

fn run_diff(
    git_range: Option<&str>,
    left: Option<&Path>,
    right: Option<&Path>,
    repo: Option<&Path>,
) -> Result<ontocore_diff::DiffResult> {
    if let Some(spec) = git_range {
        let repo_root = repo
            .map(Path::to_path_buf)
            .or_else(|| std::env::current_dir().ok())
            .context("could not determine git repository root")?;
        let (left_ref, right_ref) = parse_git_range(spec).map_err(|e| anyhow::anyhow!(e))?;
        return diff_git_refs(&repo_root, &left_ref, &right_ref).map_err(|e| anyhow::anyhow!(e));
    }
    match (left, right) {
        (Some(a), Some(b)) => diff_directories(a, b).map_err(|e| anyhow::anyhow!(e)),
        (Some(a), None) => {
            let (left_ref, right_ref) = parse_git_range("HEAD").map_err(|e| anyhow::anyhow!(e))?;
            let repo_root = repo.unwrap_or(a);
            diff_git_refs(repo_root, &left_ref, &right_ref).map_err(|e| anyhow::anyhow!(e))
        }
        _ => bail!(
            "provide a git range (`main..feature`), or both --left and --right directory paths"
        ),
    }
}

fn build_catalog(workspace: &PathBuf) -> Result<OntologyCatalog> {
    IndexBuilder::new()
        .workspace(workspace)
        .build()
        .with_context(|| format!("failed to index workspace {}", workspace.display()))
}

fn load_reasoner_input(workspace: &PathBuf) -> Result<ontocore_reasoner::ReasonerInput> {
    let catalog = build_catalog(workspace)?;
    WorkspaceInputLoader::new(workspace)
        .load(catalog.class_hierarchy())
        .map_err(|e| anyhow::anyhow!(e))
}

fn run_classify(
    workspace: &PathBuf,
    profile: &str,
    auto_profile: bool,
) -> Result<ontocore_reasoner::ClassificationResult> {
    let profile_id = ReasonerId::parse(profile).map_err(|e| anyhow::anyhow!(e))?;
    let input = load_reasoner_input(workspace)?;
    classify(profile_id, &input, auto_profile).map_err(|e| anyhow::anyhow!(e))
}

fn run_explain(
    workspace: &PathBuf,
    class: &str,
    profile: &str,
) -> Result<ontocore_reasoner::ExplanationResult> {
    let profile_id = ReasonerId::parse(profile).map_err(|e| anyhow::anyhow!(e))?;
    let input = load_reasoner_input(workspace)?;
    explain(profile_id, &input, &ExplanationRequest { class_iri: class.to_string() })
        .map_err(|e| anyhow::anyhow!(e))
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
    let result = ontocore_query::sql::QueryResult {
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
