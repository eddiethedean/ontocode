use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use ontocore_catalog::{CatalogStats, IndexBuilder, OntologyCatalog};
use ontocore_diff::{
    apply_unsat_diff, catalog_at_git_ref, catalog_at_worktree, diff_catalogs,
    diff_git_refs_with_catalogs, format_diff_json, format_diff_markdown, format_diff_pr_summary,
    format_diff_text, parse_git_range, DiffResult,
};
use ontocore_docs::{export_workspace, ExportFormat, ExportOptions};
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
    about = "Local-first ontology index and query engine (OntoCode v0.13)"
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
    /// Export Markdown or HTML documentation for a workspace
    Docs {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Output directory
        #[arg(long, short = 'o')]
        output: PathBuf,
        #[arg(long, value_enum, default_value = "markdown")]
        format: DocsFormat,
        /// Limit export to a single ontology id / base IRI
        #[arg(long)]
        ontology_id: Option<String>,
    },
    /// Semantic diff between git refs, directories, or indexed snapshots
    Diff {
        /// Git range (`main..feature`), single ref vs working tree, or omitted with `--left-ref`/`--right-ref`
        #[arg(value_name = "GIT_RANGE")]
        git_range: Option<String>,
        /// Left git ref or directory path
        #[arg(long)]
        left_ref: Option<String>,
        /// Right git ref, directory path, or `WORKTREE` for working tree
        #[arg(long)]
        right_ref: Option<String>,
        /// Git repository root (defaults to current directory)
        #[arg(long)]
        repo: Option<PathBuf>,
        /// Enrich diff with reasoner unsatisfiability changes (requires resolvable workspace paths)
        #[arg(long)]
        reasoner: bool,
        #[arg(long, value_enum, default_value_t = DiffFormat::Text)]
        format: DiffFormat,
        #[arg(long)]
        breaking_only: bool,
        /// Emit a Markdown summary suitable for pull request descriptions
        #[arg(long)]
        pr_summary: bool,
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
enum DocsFormat {
    Markdown,
    Html,
}

#[derive(Clone, Copy, ValueEnum)]
enum DiffFormat {
    Text,
    Json,
    Markdown,
    #[value(name = "pr-summary")]
    PrSummary,
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
            let data = catalog.data();
            let mut errors = 0usize;
            let mut warnings = 0usize;
            for diag in &data.diagnostics {
                match diag.severity {
                    ontocore_core::DiagnosticSeverity::Error => errors += 1,
                    ontocore_core::DiagnosticSeverity::Warning => warnings += 1,
                    _ => {}
                }
            }
            if errors > 0 || warnings > 0 {
                println!("\nDiagnostics: {errors} error(s), {warnings} warning(s)");
                for diag in data.diagnostics.iter().take(10) {
                    println!(
                        "  [{}] {} — {}",
                        diag.code.as_str(),
                        diag.severity.as_str(),
                        diag.message
                    );
                }
                if data.diagnostics.len() > 10 {
                    println!(
                        "  … and {} more (run `ontocore validate` for full list)",
                        data.diagnostics.len() - 10
                    );
                }
            } else {
                println!("\nNo diagnostics.");
            }
        }
        Commands::Patch { document, patch_file, preview } => {
            let patch_bytes = std::fs::read(&patch_file)?;
            let ext =
                document.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
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
            if ext == "obo" {
                let patches: Vec<ontocore_obo::OboPatchOp> =
                    serde_json::from_slice(&patch_bytes).context("failed to parse patch JSON")?;
                let result = ontocore_obo::apply_patches(&document, &patches, preview)
                    .context("patch failed")?;
                println!("{}", serde_json::to_string_pretty(&result)?);
                let has_errors = result.diagnostics.iter().any(|d| d.severity == "error");
                if has_errors || (!preview && !patches.is_empty() && !result.applied) {
                    bail!("patch failed with {} diagnostic(s)", result.diagnostics.len().max(1));
                }
                if !preview && result.applied {
                    println!("applied");
                }
            } else if ext == "ttl" {
                let patches: Vec<ontocore_owl::PatchOp> =
                    serde_json::from_slice(&patch_bytes).context("failed to parse patch JSON")?;
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
            } else {
                bail!("patch write-back supports .ttl and .obo documents only");
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
                let roots = vec![workspace.clone()];
                let plan = preview_move_entity(&catalog, &iri, &to, &HashMap::new(), &roots)?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Extract { workspace, entities, out, leave_stub, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let roots = vec![workspace.clone()];
                let plan = preview_extract_module(
                    &catalog,
                    &entities,
                    &out,
                    leave_stub,
                    &HashMap::new(),
                    &roots,
                )?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
        },
        Commands::Docs { workspace, output, format, ontology_id } => {
            let catalog = build_catalog(&workspace)?;
            let export_format = match format {
                DocsFormat::Markdown => ExportFormat::Markdown,
                DocsFormat::Html => ExportFormat::Html,
            };
            let mut options =
                ExportOptions { output_dir: output, format: export_format, ontology_id: None };
            if let Some(id) = ontology_id {
                options = options.with_ontology_id(id);
            }
            export_workspace(&catalog, options.clone()).context("docs export failed")?;
            println!("Wrote documentation to {}", options.output_dir.display());
        }
        Commands::Diff {
            git_range,
            left_ref,
            right_ref,
            repo,
            reasoner,
            format,
            breaking_only,
            pr_summary,
        } => {
            let diff = run_diff(
                git_range.as_deref(),
                left_ref.as_deref(),
                right_ref.as_deref(),
                repo.as_deref(),
                reasoner,
            )?;
            let output = if pr_summary || matches!(format, DiffFormat::PrSummary) {
                format_diff_pr_summary(&diff)
            } else {
                match format {
                    DiffFormat::Json => format_diff_json(&diff),
                    DiffFormat::Markdown => format_diff_markdown(&diff, breaking_only),
                    DiffFormat::Text | DiffFormat::PrSummary => {
                        format_diff_text(&diff, breaking_only)
                    }
                }
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
    let root = workspace.canonicalize().unwrap_or_else(|_| workspace.to_path_buf());
    let files_written =
        apply_refactor_plan_checked(plan, preview, Some(std::slice::from_ref(&root)))?;
    if !preview {
        println!("applied {files_written} file(s)");
    }
    Ok(())
}

fn run_diff(
    git_range: Option<&str>,
    left_ref: Option<&str>,
    right_ref: Option<&str>,
    repo: Option<&Path>,
    reasoner: bool,
) -> Result<DiffResult> {
    let repo_root = repo
        .map(Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok())
        .context("could not determine git repository root")?;

    let (mut diff, base_cat, head_cat) = if let Some(spec) = git_range {
        let (left, right) = parse_git_range(spec).map_err(|e| anyhow::anyhow!(e))?;
        let (diff, base, head) = diff_git_refs_with_catalogs(&repo_root, &left, &right)
            .map_err(|e| anyhow::anyhow!(e))?;
        (diff, Some(base), Some(head))
    } else {
        let left = left_ref.context("provide --left-ref or a git range")?;
        let right = right_ref.context("provide --right-ref or a git range")?;
        diff_from_refs_with_catalogs(left, right, &repo_root)?
    };

    if reasoner {
        if let (Some(base), Some(head)) = (base_cat, head_cat) {
            apply_reasoner_unsat_catalogs(&mut diff, &base, &head)?;
        } else {
            eprintln!("WARN: --reasoner skipped (could not resolve catalogs for both sides)");
        }
    }
    Ok(diff)
}

fn diff_from_refs_with_catalogs(
    left: &str,
    right: &str,
    repo: &Path,
) -> Result<(DiffResult, Option<OntologyCatalog>, Option<OntologyCatalog>)> {
    let left = left.trim();
    let right = right.trim();
    if left.is_empty() || right.is_empty() {
        bail!("empty --left-ref or --right-ref");
    }
    let left_path = Path::new(left);
    let right_path = Path::new(right);
    let left_is_dir = left_path.is_dir();
    let right_is_dir = right_path.is_dir();
    match (left_is_dir, right_is_dir) {
        (true, true) => {
            let left_cat = build_catalog(&left_path.to_path_buf())?;
            let right_cat = build_catalog(&right_path.to_path_buf())?;
            Ok((diff_catalogs(&left_cat, &right_cat), Some(left_cat), Some(right_cat)))
        }
        (false, false) => {
            let (diff, base, head) =
                diff_git_refs_with_catalogs(repo, left, right).map_err(|e| anyhow::anyhow!(e))?;
            Ok((diff, Some(base), Some(head)))
        }
        (true, false) => {
            let left_cat = build_catalog(&left_path.to_path_buf())?;
            let right_cat = resolve_git_catalog(repo, right)?;
            Ok((diff_catalogs(&left_cat, &right_cat), Some(left_cat), Some(right_cat)))
        }
        (false, true) => {
            let left_cat = resolve_git_catalog(repo, left)?;
            let right_cat = build_catalog(&right_path.to_path_buf())?;
            Ok((diff_catalogs(&left_cat, &right_cat), Some(left_cat), Some(right_cat)))
        }
    }
}

fn resolve_git_catalog(repo: &Path, git_ref: &str) -> Result<OntologyCatalog> {
    if ontocore_diff::is_indexed_catalog_ref(git_ref) {
        anyhow::bail!(
            "ref {git_ref}: indexed catalog (INDEXED) is only available via LSP semantic diff; \
             use WORKTREE or a git commit ref in the CLI"
        );
    }
    if ontocore_diff::is_worktree_ref(git_ref) {
        catalog_at_worktree(repo).map_err(|e| anyhow::anyhow!(e))
    } else {
        catalog_at_git_ref(repo, git_ref).map_err(|e| anyhow::anyhow!(e))
    }
}

fn apply_reasoner_unsat_catalogs(
    diff: &mut DiffResult,
    base: &OntologyCatalog,
    head: &OntologyCatalog,
) -> Result<()> {
    let profile = ReasonerId::El;
    let base_input =
        WorkspaceInputLoader::new(base.workspace()).load().map_err(|e| anyhow::anyhow!(e))?;
    let head_input =
        WorkspaceInputLoader::new(head.workspace()).load().map_err(|e| anyhow::anyhow!(e))?;
    let base_cls = classify(profile, &base_input, true).map_err(|e| anyhow::anyhow!(e))?;
    let head_cls = classify(profile, &head_input, true).map_err(|e| anyhow::anyhow!(e))?;
    apply_unsat_diff(diff, &base_cls.unsatisfiable, &head_cls.unsatisfiable);
    Ok(())
}

fn build_catalog(workspace: &PathBuf) -> Result<OntologyCatalog> {
    IndexBuilder::new()
        .workspace(workspace)
        .build()
        .with_context(|| format!("failed to index workspace {}", workspace.display()))
}

fn load_reasoner_input(workspace: &PathBuf) -> Result<ontocore_reasoner::ReasonerInput> {
    WorkspaceInputLoader::new(workspace).load().map_err(|e| anyhow::anyhow!(e))
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
