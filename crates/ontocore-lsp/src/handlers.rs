use crate::index_worker::IndexWorker;
use crate::positions::{byte_col_to_utf16, utf16_offset_to_byte};
use crate::protocol::{
    ApplyAxiomPatchParams, ApplyAxiomPatchResult, ApplyRefactorParams, ApplyRefactorResult,
    CatalogSnapshot, DiagnosticSummary, FindUsagesParams, FindUsagesResult, GetEntityParams,
    GetEntityResult, GetExplanationParams, GetExplanationResult, GetGraphResult,
    IndexWorkspaceParams, IndexWorkspaceResult, LspErrorPayload, ManchesterCompletions,
    ParseManchesterParams, ParseManchesterResult, PreviewRefactorParams, PreviewRefactorResult,
    QueryParams, RunReasonerParams, RunReasonerResult, RunRobotParams, RunRobotResult,
    SemanticDiffParams, SemanticDiffResult, SparqlParams, TabularQueryResult, UsageSummary,
};
use crate::state::{path_to_uri, resolve_workspace_for_index, ServerState};
use lsp_server::ResponseError;
use lsp_types::{
    DocumentChanges, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, Location, MarkupContent,
    MarkupKind, OneOf, Position, Range, ReferenceParams, RenameParams, ServerCapabilities,
    SymbolInformation, SymbolKind, TextDocumentEdit, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextEdit, Uri, WorkspaceEdit, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities, WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use ontocore_catalog::{GraphBuilder, GraphRequest};
use ontocore_core::{EntityKind, OntologyFormat};
use ontocore_diff::{diff_catalogs, diff_directories, diff_git_refs};
use ontocore_reasoner::{
    classify, explain, ExplanationRequest, ReasonerId, ReasonerSnapshot, WorkspaceInputLoader,
};
use ontocore_refactor::{
    apply_refactor_plan_checked_with_overrides, find_usages_with_overrides, plans_equivalent,
    preview_refactor, preview_rename_iri, validate_refactor_plan_paths,
};
use serde_json::Value;
use std::path::Path;
use std::str::FromStr;

#[allow(deprecated)]
pub fn handle_initialize(state: &ServerState, params: InitializeParams) -> InitializeResult {
    let mut roots: Vec<std::path::PathBuf> = Vec::new();
    if let Some(folders) = params.workspace_folders {
        for folder in folders {
            match resolve_workspace_for_index(folder.uri.as_str(), None) {
                Ok(path) => roots.push(path),
                Err(err) => eprintln!("ontocore-lsp: failed to resolve workspace folder: {err}"),
            }
        }
    } else if let Some(uri) = params.root_uri {
        match resolve_workspace_for_index(uri.as_str(), None) {
            Ok(path) => roots.push(path),
            Err(err) => eprintln!("ontocore-lsp: failed to resolve workspace root: {err}"),
        }
    }

    if !roots.is_empty() {
        if let Err(err) = state.set_workspace_roots(roots) {
            eprintln!("ontocore-lsp: failed to set workspace roots: {err}");
        }
    }

    InitializeResult {
        capabilities: ServerCapabilities {
            definition_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            rename_provider: Some(OneOf::Left(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        server_info: Some(lsp_types::ServerInfo {
            name: "ontocore-lsp".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    }
}

pub fn handle_index_workspace(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: IndexWorkspaceParams,
) -> Result<IndexWorkspaceResult, LspErrorPayload> {
    let workspace = match params.workspace_uri.as_deref() {
        Some(uri) => resolve_workspace_for_index(uri, state.workspace_root().as_deref())
            .map_err(LspErrorPayload::index_failed)?,
        None => state.effective_index_root().ok_or_else(|| {
            LspErrorPayload::index_failed("no workspace URI provided".to_string())
        })?,
    };

    state.set_index_disk_cache(params.disk_cache);

    let (stats, indexed_at) =
        index_worker.enqueue_sync(workspace).map_err(LspErrorPayload::index_failed)?;

    Ok(IndexWorkspaceResult { stats, indexed_at })
}

pub fn build_catalog_snapshot(catalog: &ontocore_catalog::OntologyCatalog) -> CatalogSnapshot {
    CatalogSnapshot {
        documents: catalog.data().documents.clone(),
        entities: catalog.data().entities.clone(),
        hierarchy: catalog.class_hierarchy(),
        diagnostics: catalog.data().diagnostics.iter().map(DiagnosticSummary::from).collect(),
        reasoner: None,
    }
}

pub fn build_catalog_snapshot_with_reasoner(
    catalog: &ontocore_catalog::OntologyCatalog,
    reasoner: Option<ontocore_reasoner::ReasonerSnapshot>,
) -> CatalogSnapshot {
    let mut snapshot = build_catalog_snapshot(catalog);
    snapshot.reasoner = reasoner;
    snapshot
}

pub fn handle_get_catalog_snapshot(
    state: &ServerState,
) -> Result<CatalogSnapshot, LspErrorPayload> {
    state
        .with_catalog_and_reasoner(|catalog, reasoner| {
            build_catalog_snapshot_with_reasoner(catalog, reasoner.cloned())
        })
        .ok_or_else(LspErrorPayload::not_indexed)
}

pub fn handle_get_entity(
    state: &ServerState,
    params: GetEntityParams,
) -> Result<GetEntityResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            catalog
                .entity_detail(&params.iri)
                .map(|detail| GetEntityResult { detail })
                .ok_or_else(|| LspErrorPayload::not_found(&params.iri))
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_get_graph(
    state: &ServerState,
    params: GraphRequest,
) -> Result<GetGraphResult, LspErrorPayload> {
    state
        .with_catalog_and_reasoner(|catalog, reasoner| {
            let mut builder = GraphBuilder::new(catalog);
            if params.include_inferred {
                if let Some(snapshot) = reasoner {
                    builder = builder.with_inferred_edges(&snapshot.inferred.edges);
                }
            }
            let graph = builder.build(&params).map_err(LspErrorPayload::graph_failed)?;
            Ok(GetGraphResult { graph })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_semantic_diff(
    state: &ServerState,
    params: SemanticDiffParams,
) -> Result<SemanticDiffResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    let repo_root = roots
        .first()
        .cloned()
        .ok_or_else(|| LspErrorPayload::invalid_params("workspace not initialized".to_string()))?;

    let diff = if let (Some(left), Some(right)) = (&params.left_path, &params.right_path) {
        diff_directories(Path::new(left), Path::new(right))
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?
    } else {
        let left_ref = params.left_ref.as_deref().unwrap_or("HEAD");
        let right_ref = params.right_ref.as_deref().unwrap_or("WORKTREE");
        if left_ref.eq_ignore_ascii_case("WORKSPACE") {
            let diff = state
                .with_catalog(|head| {
                    if right_ref.eq_ignore_ascii_case("WORKTREE")
                        || right_ref.eq_ignore_ascii_case("WORKSPACE")
                    {
                        Ok(diff_catalogs(head, head))
                    } else {
                        let base = ontocore_diff::catalog_at_git_ref(&repo_root, right_ref)
                            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
                        Ok(diff_catalogs(&base, head))
                    }
                })
                .ok_or_else(LspErrorPayload::not_indexed)??;
            return Ok(SemanticDiffResult { diff });
        }
        if right_ref.eq_ignore_ascii_case("WORKSPACE") {
            let diff = state
                .with_catalog(|head| {
                    let base = ontocore_diff::catalog_at_git_ref(&repo_root, left_ref)
                        .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
                    Ok(diff_catalogs(&base, head))
                })
                .ok_or_else(LspErrorPayload::not_indexed)??;
            return Ok(SemanticDiffResult { diff });
        }
        diff_git_refs(&repo_root, left_ref, right_ref)
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?
    };

    Ok(SemanticDiffResult { diff })
}

pub fn handle_run_robot(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: RunRobotParams,
) -> Result<RunRobotResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::robot_failed("workspace not initialized".to_string()));
    }
    let mut args = vec![params.subcommand];
    args.extend(params.args);
    jail_robot_path_args(&roots, &args).map_err(LspErrorPayload::robot_failed)?;
    index_worker.run_robot_sync(params.robot_path, args).map_err(LspErrorPayload::robot_failed)
}

/// Reject ROBOT file operands that escape the workspace jail.
///
/// Handles separate `--input path`, `--input=path`, and attached short forms (`-i/path`).
fn jail_robot_path_args(
    workspace_roots: &[std::path::PathBuf],
    args: &[String],
) -> Result<(), String> {
    let long_path_flags = ["--input", "--output", "--report"];
    let short_path_flags = ["-i", "-o"];
    let mut expect_path = false;
    for arg in args.iter().skip(1) {
        if expect_path {
            expect_path = false;
            ontocore_core::validate_workspace_scope_any(
                std::path::Path::new(arg),
                workspace_roots,
            )?;
            continue;
        }
        if let Some((flag, value)) = arg.split_once('=') {
            if long_path_flags.contains(&flag) {
                ontocore_core::validate_workspace_scope_any(
                    std::path::Path::new(value),
                    workspace_roots,
                )?;
                continue;
            }
        }
        if long_path_flags.contains(&arg.as_str()) || short_path_flags.contains(&arg.as_str()) {
            expect_path = true;
            continue;
        }
        let attached_short = short_path_flags.iter().find_map(|flag| {
            if arg.starts_with(flag)
                && arg.len() > flag.len()
                && !arg[flag.len()..].starts_with('-')
            {
                Some(&arg[flag.len()..])
            } else {
                None
            }
        });
        if let Some(value) = attached_short {
            ontocore_core::validate_workspace_scope_any(
                std::path::Path::new(value),
                workspace_roots,
            )?;
            continue;
        }
        // Positional path-like args (contain / or end with ontology extensions).
        let looks_like_path = arg.contains('/')
            || arg.contains('\\')
            || arg.ends_with(".ttl")
            || arg.ends_with(".owl")
            || arg.ends_with(".obo")
            || arg.ends_with(".rdf");
        if looks_like_path && !arg.starts_with('-') {
            ontocore_core::validate_workspace_scope_any(
                std::path::Path::new(arg),
                workspace_roots,
            )?;
        }
    }
    if expect_path {
        return Err("ROBOT path flag is missing a path argument".to_string());
    }
    Ok(())
}

pub fn handle_apply_axiom_patch(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: ApplyAxiomPatchParams,
) -> Result<ApplyAxiomPatchResult, LspErrorPayload> {
    state.with_catalog(|_| ()).ok_or_else(LspErrorPayload::not_indexed)?;
    let workspace_root = state
        .workspace_root()
        .ok_or_else(|| LspErrorPayload::patch_invalid("workspace not initialized".to_string()))?;
    let document_path = state
        .resolve_lsp_document_uri(&params.document_uri)
        .map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;

    let namespaces = state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| {
                    d.path.canonicalize().ok().as_ref()
                        == document_path.canonicalize().ok().as_ref()
                        || d.path == document_path
                })
                .map(|d| d.namespaces.clone())
        })
        .flatten()
        .unwrap_or_default();

    let format = OntologyFormat::from_extension(
        document_path.extension().and_then(|e| e.to_str()).unwrap_or(""),
    );
    if format != OntologyFormat::Turtle {
        return Err(LspErrorPayload::unsupported_format(format!(
            "write-back supports Turtle only, got {}",
            format.as_str()
        )));
    }

    let entity_iri = params.patches.first().map(|p| match p {
        ontocore_owl::PatchOp::CreateEntity { entity_iri, .. }
        | ontocore_owl::PatchOp::DeleteEntity { entity_iri }
        | ontocore_owl::PatchOp::SetLabel { entity_iri, .. }
        | ontocore_owl::PatchOp::AddLabel { entity_iri, .. }
        | ontocore_owl::PatchOp::RemoveLabel { entity_iri, .. }
        | ontocore_owl::PatchOp::SetComment { entity_iri, .. }
        | ontocore_owl::PatchOp::AddComment { entity_iri, .. }
        | ontocore_owl::PatchOp::RemoveComment { entity_iri, .. }
        | ontocore_owl::PatchOp::AddSubClassOf { entity_iri, .. }
        | ontocore_owl::PatchOp::RemoveSubClassOf { entity_iri, .. }
        | ontocore_owl::PatchOp::AddComplexSubClassOf { entity_iri, .. }
        | ontocore_owl::PatchOp::RemoveComplexSubClassOf { entity_iri, .. }
        | ontocore_owl::PatchOp::AddEquivalentClass { entity_iri, .. }
        | ontocore_owl::PatchOp::RemoveEquivalentClass { entity_iri, .. }
        | ontocore_owl::PatchOp::SetEquivalentClass { entity_iri, .. }
        | ontocore_owl::PatchOp::SetDeprecated { entity_iri, .. }
        | ontocore_owl::PatchOp::AddDisjointClass { entity_iri, .. }
        | ontocore_owl::PatchOp::RemoveDisjointClass { entity_iri, .. } => entity_iri.clone(),
    });

    // Serialize applies without holding ops_lock across enqueue_sync (index worker needs it).
    let (patch_result, workspace_edit, needs_reindex) = {
        let ops_lock = state.ops_lock();
        let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;

        let source = state
            .document_text(&document_path)
            .ok_or_else(|| LspErrorPayload::patch_invalid("cannot read document".to_string()))?;

        let mut patch_result = ontocore_owl::apply_patches_to_text(
            &source,
            &params.patches,
            params.preview_only,
            &namespaces,
        )
        .map_err(|e| match e {
            ontocore_owl::OwlError::UnsupportedFormat(m) => LspErrorPayload::unsupported_format(m),
            _ => LspErrorPayload::patch_invalid(e.to_string()),
        })?;
        patch_result.document_path = Some(document_path.display().to_string());

        if !patch_result.applied && !patch_result.diagnostics.is_empty() {
            return Ok(ApplyAxiomPatchResult {
                patch: patch_result,
                entity_detail: None,
                reindex_warning: None,
                workspace_edit: None,
            });
        }

        let mut workspace_edit = None;
        let mut needs_reindex = false;
        if patch_result.applied && !params.preview_only {
            if let Some(text) = &patch_result.preview_text {
                if text.len() as u64 > ontocore_core::MAX_FILE_BYTES {
                    return Err(LspErrorPayload::patch_invalid(format!(
                        "patched document exceeds maximum size of {} bytes",
                        ontocore_core::MAX_FILE_BYTES
                    )));
                }
                // Disk first so a failed write never leaves a divergent LSP buffer.
                ontocore_owl::atomic_write(&document_path, text).map_err(|e| {
                    LspErrorPayload::patch_invalid(format!("failed to write document: {e}"))
                })?;
                if state.is_document_open(&document_path) {
                    if let Err(e) = state.set_document_text(document_path.clone(), text.clone()) {
                        // Roll back disk so the client never sees a half-applied patch.
                        let _ = ontocore_owl::atomic_write(&document_path, &source);
                        return Err(LspErrorPayload::patch_invalid(e));
                    }
                }
                workspace_edit = full_document_workspace_edit(state, &document_path, text);
                needs_reindex = true;
            }
        }
        (patch_result, workspace_edit, needs_reindex)
    };

    let mut reindex_warning = None;
    if needs_reindex {
        let index_root = state.effective_index_root().unwrap_or_else(|| workspace_root.clone());
        if let Err(msg) = index_worker.enqueue_sync(index_root) {
            reindex_warning = Some(format!("patch applied but reindex failed: {msg}"));
        }
    }

    let entity_detail = entity_iri
        .and_then(|iri| state.with_catalog(|catalog| catalog.entity_detail(&iri)))
        .flatten();

    Ok(ApplyAxiomPatchResult {
        patch: patch_result,
        entity_detail,
        reindex_warning,
        workspace_edit,
    })
}

pub fn handle_query(
    state: &ServerState,
    params: QueryParams,
) -> Result<TabularQueryResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let result = ontocore_query::query_catalog(catalog, &params.sql).map_err(
                |e: ontocore_query::QueryError| LspErrorPayload::query_failed(e.to_string()),
            )?;
            let truncated = if result.truncated { Some(true) } else { None };
            Ok(TabularQueryResult { columns: result.columns, rows: result.rows, truncated })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_sparql(
    state: &ServerState,
    params: SparqlParams,
) -> Result<TabularQueryResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let result = ontocore_query::sparql_catalog(catalog, &params.query).map_err(
                |e: ontocore_query::QueryError| LspErrorPayload::query_failed(e.to_string()),
            )?;
            let truncated = if result.truncated { Some(true) } else { None };
            Ok(TabularQueryResult { columns: result.columns, rows: result.rows, truncated })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_parse_manchester(
    state: &ServerState,
    params: ParseManchesterParams,
) -> Result<ParseManchesterResult, LspErrorPayload> {
    let namespaces = resolve_namespaces_for_manchester(state, &params)?;
    let completions = build_manchester_completions(state);

    let parsed = ontocore_owl::parse_class_expression(&params.expression, &namespaces)
        .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;

    let turtle_predicate = match params.axiom_kind.as_str() {
        "equivalent_class" => "owl:equivalentClass",
        "disjoint_class" => {
            return Err(LspErrorPayload::manchester_invalid(
                "disjoint_class axioms use IRI patch ops (add_disjoint_class), not Manchester expressions"
                    .to_string(),
            ));
        }
        _ => "rdfs:subClassOf",
    };

    let turtle_fragment = ontocore_owl::class_expression_to_turtle_fragment(
        &parsed.expression,
        turtle_predicate,
        &namespaces,
    )
    .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;

    Ok(ParseManchesterResult {
        normalized: parsed.normalized,
        turtle_fragment,
        tree: parsed.tree,
        diagnostics: parsed
            .diagnostics
            .into_iter()
            .map(|d| ontocore_owl::PatchDiagnostic {
                severity: "warning".to_string(),
                message: d.message,
            })
            .collect(),
        completions,
    })
}

pub fn handle_run_reasoner(
    state: &ServerState,
    params: RunReasonerParams,
) -> Result<RunReasonerResult, LspErrorPayload> {
    let ops_lock = state.ops_lock();
    let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;

    let profile = ReasonerId::parse(&params.profile)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;

    let input = load_reasoner_input(state)?;

    if let Some(cached) = state
        .with_reasoner_cache(|cache| cache.get(&input.content_hash, profile).cloned())
        .flatten()
    {
        let snapshot = cached.snapshot.clone();
        state.set_reasoner_snapshot(snapshot.clone());
        return Ok(run_reasoner_result_from_classification(&cached.classification, snapshot));
    }

    let classification = classify(profile, &input, params.auto_detect)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;

    let snapshot = state
        .reasoner_cache_mut(|cache| {
            cache.store_classification(input, profile, classification.clone())
        })
        .unwrap_or_else(|| ReasonerSnapshot::from(classification.clone()));
    state.set_reasoner_snapshot(snapshot.clone());

    Ok(run_reasoner_result_from_classification(&classification, snapshot))
}

pub fn handle_get_explanation(
    state: &ServerState,
    params: GetExplanationParams,
) -> Result<GetExplanationResult, LspErrorPayload> {
    let ops_lock = state.ops_lock();
    let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))?;

    let profile = ReasonerId::parse(&params.profile)
        .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))?;

    let fresh_input = load_reasoner_input(state)?;
    let input = state
        .with_reasoner_cache(|cache| {
            cache.get(&fresh_input.content_hash, profile).map(|c| c.input.clone())
        })
        .flatten()
        .unwrap_or(fresh_input);

    let result =
        explain(profile, &input, &ExplanationRequest { class_iri: params.class_iri.clone() })
            .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))?;

    Ok(GetExplanationResult { class_iri: result.class_iri, steps: result.steps, text: result.text })
}

fn run_reasoner_result_from_classification(
    classification: &ontocore_reasoner::ClassificationResult,
    snapshot: ReasonerSnapshot,
) -> RunReasonerResult {
    RunReasonerResult {
        profile_used: classification.profile_used.clone(),
        consistent: classification.consistent,
        unsatisfiable: classification.unsatisfiable.clone(),
        inferred_edge_count: classification.inferred.edges.len(),
        new_inferences: classification.new_inferences.clone(),
        warnings: classification.warnings.clone(),
        duration_ms: classification.duration_ms,
        snapshot,
    }
}

fn load_reasoner_input(
    state: &ServerState,
) -> Result<ontocore_reasoner::ReasonerInput, LspErrorPayload> {
    let workspace = state
        .indexed_workspace()
        .or_else(|| state.workspace_root())
        .ok_or_else(LspErrorPayload::not_indexed)?;
    let asserted = state
        .with_catalog(|catalog| catalog.class_hierarchy())
        .ok_or_else(LspErrorPayload::not_indexed)?;
    let overrides = state.open_documents_for_reasoner();
    WorkspaceInputLoader::new(&workspace)
        .document_overrides(overrides)
        .load(asserted)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))
}

fn resolve_namespaces_for_manchester(
    state: &ServerState,
    params: &ParseManchesterParams,
) -> Result<std::collections::BTreeMap<String, String>, LspErrorPayload> {
    if let Some(uri) = &params.document_uri {
        let path = state
            .resolve_lsp_document_uri(uri)
            .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;
        if let Some(ns) = state
            .with_catalog(|catalog| {
                catalog
                    .data()
                    .documents
                    .iter()
                    .find(|d| {
                        d.path.canonicalize().ok().as_ref() == path.canonicalize().ok().as_ref()
                            || d.path == path
                    })
                    .map(|d| d.namespaces.clone())
            })
            .flatten()
        {
            return Ok(ns);
        }
    }
    state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| {
                    params.entity_iri.as_ref().is_some_and(|iri| {
                        catalog.entity_document(iri).is_some_and(|ed| ed.id == d.id)
                    })
                })
                .map(|d| d.namespaces.clone())
        })
        .flatten()
        .ok_or_else(LspErrorPayload::not_indexed)
}

fn build_manchester_completions(state: &ServerState) -> ManchesterCompletions {
    const DATATYPES: &[&str] = &[
        "xsd:string",
        "xsd:integer",
        "xsd:decimal",
        "xsd:boolean",
        "xsd:dateTime",
        "xsd:float",
        "xsd:double",
    ];

    state
        .with_catalog(|catalog| {
            let mut classes = Vec::new();
            let mut object_properties = Vec::new();
            let mut data_properties = Vec::new();
            for entity in &catalog.data().entities {
                match entity.kind {
                    EntityKind::Class => classes.push(entity.iri.clone()),
                    EntityKind::ObjectProperty => object_properties.push(entity.iri.clone()),
                    EntityKind::DataProperty => data_properties.push(entity.iri.clone()),
                    _ => {}
                }
            }
            classes.sort();
            object_properties.sort();
            data_properties.sort();
            ManchesterCompletions {
                classes,
                object_properties,
                data_properties,
                datatypes: DATATYPES.iter().map(|s| s.to_string()).collect(),
            }
        })
        .unwrap_or(ManchesterCompletions {
            classes: Vec::new(),
            object_properties: Vec::new(),
            data_properties: Vec::new(),
            datatypes: DATATYPES.iter().map(|s| s.to_string()).collect(),
        })
}

pub fn handle_hover(state: &ServerState, params: HoverParams) -> Option<Hover> {
    let path = lsp_document_path(state, &params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;

    state.with_catalog(|catalog| {
        let detail = catalog.entity_detail(&iri)?;
        let mut md = format!(
            "**{}** (`{}`)\n\n",
            escape_markdown(&detail.entity.short_name),
            escape_markdown(detail.entity.kind.as_str())
        );
        if !detail.entity.labels.is_empty() {
            md.push_str(&format!(
                "Labels: {}\n\n",
                detail
                    .entity
                    .labels
                    .iter()
                    .map(|l| escape_markdown(l))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !detail.entity.comments.is_empty() {
            md.push_str(&format!(
                "Comments: {}\n\n",
                detail
                    .entity
                    .comments
                    .iter()
                    .map(|c| escape_markdown(c))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !detail.parents.is_empty() {
            md.push_str(&format!(
                "Parents: {}\n",
                detail.parents.iter().map(|p| escape_markdown(p)).collect::<Vec<_>>().join(", ")
            ));
        }
        if detail.entity.deprecated {
            md.push_str("\n*deprecated*");
        }
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: md,
            }),
            range: None,
        })
    })?
}

#[allow(deprecated)]
pub fn handle_document_symbol(
    state: &ServerState,
    params: DocumentSymbolParams,
) -> Option<DocumentSymbolResponse> {
    let path = lsp_document_path(state, &params.text_document.uri)?;
    // Clone under catalog lock; never call document_text while holding it (non-reentrant RwLock).
    let entities: Vec<ontocore_core::Entity> = state.with_catalog(|catalog| {
        catalog.entities_in_document(&path).into_iter().cloned().collect()
    })?;
    if entities.is_empty() {
        return None;
    }
    let doc_text = state.document_text(&path);
    let symbols: Vec<DocumentSymbol> = entities
        .into_iter()
        .map(|e| {
            let range = entity_source_range(doc_text.as_deref(), &e);
            DocumentSymbol {
                name: e.short_name.clone(),
                detail: Some(e.iri.clone()),
                kind: entity_kind_to_symbol_kind(e.kind),
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }
        })
        .collect();
    Some(DocumentSymbolResponse::Nested(symbols))
}

#[allow(deprecated)]
pub fn handle_workspace_symbol(
    state: &ServerState,
    params: WorkspaceSymbolParams,
) -> Option<WorkspaceSymbolResponse> {
    let query = params.query.to_ascii_lowercase();
    let entries: Vec<(ontocore_core::Entity, std::path::PathBuf)> =
        state.with_catalog(|catalog| {
            catalog
                .data()
                .entities
                .iter()
                .filter(|e| {
                    query.is_empty()
                        || e.short_name.to_ascii_lowercase().contains(&query)
                        || e.iri.to_ascii_lowercase().contains(&query)
                        || e.labels.iter().any(|l| l.to_ascii_lowercase().contains(&query))
                })
                .filter_map(|e| {
                    let doc = catalog.entity_document(&e.iri)?;
                    Some((e.clone(), doc.path.clone()))
                })
                .collect()
        })?;
    let symbols: Vec<SymbolInformation> = entries
        .into_iter()
        .filter_map(|(e, path)| {
            let uri = path_to_lsp_uri(&path)?;
            let doc_text = state.document_text(&path);
            let range = entity_source_range(doc_text.as_deref(), &e);
            Some(SymbolInformation {
                name: e.short_name.clone(),
                kind: entity_kind_to_symbol_kind(e.kind),
                tags: None,
                deprecated: None,
                location: Location { uri, range },
                container_name: None,
            })
        })
        .collect();
    Some(WorkspaceSymbolResponse::Flat(symbols))
}

pub fn handle_goto_definition(
    state: &ServerState,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let path = lsp_document_path(state, &params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;

    let source = state.with_catalog(|catalog| catalog.find_source_location(&iri))??;
    let uri = path_to_lsp_uri(&source.path)?;
    let line_text = state.document_text(&source.path).and_then(|text| {
        text.lines().nth(source.line.saturating_sub(1) as usize).map(|s| s.to_string())
    });
    let character = line_text
        .as_deref()
        .map(|l| byte_col_to_utf16(l, source.column as usize))
        .unwrap_or(source.column as u32);
    let range = Range {
        start: Position { line: (source.line.saturating_sub(1)) as u32, character },
        end: Position {
            line: (source.line.saturating_sub(1)) as u32,
            character: character.saturating_add(1),
        },
    };
    Some(GotoDefinitionResponse::Scalar(Location { uri, range }))
}

pub fn handle_find_usages(
    state: &ServerState,
    params: FindUsagesParams,
) -> Result<FindUsagesResult, LspErrorPayload> {
    let overrides = state.open_document_overrides();
    state
        .with_catalog(|catalog| {
            let usages = find_usages_with_overrides(catalog, &params.iri, &overrides);
            Ok(FindUsagesResult { usages: usages.into_iter().map(usage_to_summary).collect() })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_preview_refactor(
    state: &ServerState,
    params: PreviewRefactorParams,
) -> Result<PreviewRefactorResult, LspErrorPayload> {
    let workspace = state
        .workspace_root()
        .ok_or_else(|| LspErrorPayload::refactor_failed("workspace not initialized".to_string()))?;
    let overrides = state.open_document_overrides();
    state
        .with_catalog(|catalog| {
            let plan = preview_refactor(catalog, &params.request, &overrides, &workspace)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;
            Ok(PreviewRefactorResult { plan })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_apply_refactor(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: ApplyRefactorParams,
) -> Result<ApplyRefactorResult, LspErrorPayload> {
    let workspace = state
        .workspace_root()
        .ok_or_else(|| LspErrorPayload::refactor_failed("workspace not initialized".to_string()))?;

    let (files_written, server_plan) = {
        let ops_lock = state.ops_lock();
        let _guard =
            ops_lock.lock().map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;

        let overrides = state.open_document_overrides();
        let server_plan = state
            .with_catalog(|catalog| {
                preview_refactor(catalog, &params.request, &overrides, &workspace)
                    .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
            })
            .ok_or_else(LspErrorPayload::not_indexed)??;

        if !plans_equivalent(&server_plan, &params.plan) {
            return Err(LspErrorPayload::refactor_failed(
                "submitted plan does not match server preview".to_string(),
            ));
        }

        validate_refactor_plan_paths(&workspace, &server_plan)
            .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;

        let files_written = apply_refactor_plan_checked_with_overrides(
            &server_plan,
            params.preview_only,
            Some(workspace.as_path()),
            Some(&overrides),
        )
        .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;

        if !params.preview_only {
            for change in &server_plan.changes {
                if change.preview_text != change.original_text
                    && state.is_document_open(&change.path)
                {
                    // Best-effort buffer sync; disk is already committed.
                    let _ =
                        state.set_document_text(change.path.clone(), change.preview_text.clone());
                }
            }
        }
        (files_written, server_plan)
    };

    if params.preview_only {
        return Ok(ApplyRefactorResult {
            files_written: 0,
            reindex_warning: None,
            workspace_edit: None,
        });
    }

    let reindex_warning = match state.effective_index_root() {
        Some(root) => match index_worker.enqueue_sync(root) {
            Ok(_) => None,
            Err(e) => Some(format!("refactor applied but reindex failed: {e}")),
        },
        None => Some("refactor applied but workspace root unknown".to_string()),
    };
    let workspace_edit = plan_to_workspace_edit(state, &server_plan);
    Ok(ApplyRefactorResult { files_written, reindex_warning, workspace_edit })
}

pub fn handle_references(state: &ServerState, params: ReferenceParams) -> Option<Vec<Location>> {
    let path = lsp_document_path(state, &params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;
    let overrides = state.open_document_overrides();
    let usages =
        state.with_catalog(|catalog| find_usages_with_overrides(catalog, &iri, &overrides))?;
    let locations: Vec<Location> =
        usages.into_iter().filter_map(|u| usage_to_location(state, &u)).collect();
    Some(locations)
}

pub fn handle_rename(
    state: &ServerState,
    params: RenameParams,
) -> Result<Option<WorkspaceEdit>, LspErrorPayload> {
    let path = lsp_document_path(state, &params.text_document_position.text_document.uri)
        .ok_or_else(LspErrorPayload::not_indexed)?;
    let position = params.text_document_position.position;
    let content = state
        .document_text(&path)
        .ok_or_else(|| LspErrorPayload::not_found("document not available"))?;
    let from_iri = iri_at_position(&content, position)
        .ok_or_else(|| LspErrorPayload::not_found("no IRI at cursor"))?;
    let to_iri = derive_renamed_iri(&from_iri, &params.new_name, &content);
    let overrides = state.open_document_overrides();
    let plan = state
        .with_catalog(|catalog| {
            preview_rename_iri(catalog, &from_iri, &to_iri, &overrides)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        })
        .ok_or_else(LspErrorPayload::not_indexed)??;
    let edit = plan_to_workspace_edit(state, &plan).ok_or_else(|| {
        LspErrorPayload::refactor_failed("rename produced no file changes".to_string())
    })?;
    Ok(Some(edit))
}

fn usage_to_summary(u: ontocore_refactor::Usage) -> UsageSummary {
    UsageSummary {
        iri: u.iri,
        referenced_iri: u.referenced_iri,
        file: u.file.display().to_string(),
        line: u.line,
        column: u.column,
        kind: format!("{:?}", u.kind).to_ascii_lowercase(),
        context: u.context,
    }
}

fn derive_renamed_iri(from_iri: &str, new_name: &str, document_content: &str) -> String {
    if new_name.contains("://") {
        return new_name.to_string();
    }
    if let Some(expanded) = expand_iri_token(document_content, new_name) {
        if expanded.starts_with("http://") || expanded.starts_with("https://") {
            return expanded;
        }
    }
    if let Some((base, _)) = from_iri.rsplit_once('#') {
        return format!("{base}#{new_name}");
    }
    if let Some((base, _)) = from_iri.rsplit_once('/') {
        return format!("{base}/{new_name}");
    }
    new_name.to_string()
}

fn usage_to_location(state: &ServerState, usage: &ontocore_refactor::Usage) -> Option<Location> {
    let uri = path_to_lsp_uri(&usage.file)?;
    let line_idx = usage.line.unwrap_or(1).saturating_sub(1) as u32;
    let line_text = state
        .document_text(&usage.file)
        .and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()))?;
    let line = line_text.as_str();
    let byte_col = usage.column.unwrap_or(0) as usize;
    let (start_byte, end_byte) = if byte_col < line.len() {
        let (start, end) = token_byte_range_at(line, byte_col);
        (start, end)
    } else {
        (byte_col, byte_col.saturating_add(1))
    };
    let start_char = byte_col_to_utf16(line, start_byte);
    let end_char = byte_col_to_utf16(line, end_byte);
    Some(Location {
        uri,
        range: Range {
            start: Position { line: line_idx, character: start_char },
            end: Position { line: line_idx, character: end_char.max(start_char.saturating_add(1)) },
        },
    })
}

fn token_byte_range_at(line: &str, byte_col: usize) -> (usize, usize) {
    let bytes = line.as_bytes();
    let mut start = byte_col.min(bytes.len());
    let mut end = byte_col.min(bytes.len());
    while start > 0 && is_iri_char(bytes[start - 1]) {
        start -= 1;
    }
    while end < bytes.len() && is_iri_char(bytes[end]) {
        end += 1;
    }
    if start == end && byte_col < bytes.len() {
        end = (byte_col + 1).min(bytes.len());
    }
    (start, end)
}

fn full_document_workspace_edit(
    state: &ServerState,
    path: &std::path::Path,
    new_text: &str,
) -> Option<WorkspaceEdit> {
    let uri = path_to_lsp_uri(path)?;
    let version = state.document_version(path);
    Some(WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier { uri, version },
            edits: vec![OneOf::Left(TextEdit {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: u32::MAX, character: 0 },
                },
                new_text: new_text.to_string(),
            })],
        }])),
        change_annotations: None,
    })
}

fn plan_to_workspace_edit(
    state: &ServerState,
    plan: &ontocore_refactor::RefactorPlan,
) -> Option<WorkspaceEdit> {
    let mut document_changes = Vec::new();
    for change in &plan.changes {
        if change.preview_text == change.original_text {
            continue;
        }
        let uri = path_to_lsp_uri(&change.path)?;
        let version = state.document_version(&change.path);
        document_changes.push(TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier { uri, version },
            edits: vec![OneOf::Left(TextEdit {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: u32::MAX, character: 0 },
                },
                new_text: change.preview_text.clone(),
            })],
        });
    }
    if document_changes.is_empty() {
        return None;
    }
    Some(WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits(document_changes)),
        change_annotations: None,
    })
}

pub fn handle_custom_request(
    state: &ServerState,
    index_worker: &IndexWorker,
    method: &str,
    params: Option<Value>,
) -> Result<Value, LspErrorPayload> {
    match method {
        "ontocore/indexWorkspace" => {
            let params: IndexWorkspaceParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_index_workspace(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/getCatalogSnapshot" => {
            let result = handle_get_catalog_snapshot(state)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/getEntity" => {
            let params: GetEntityParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_get_entity(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/getGraph" => {
            let params: GraphRequest = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_get_graph(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/applyAxiomPatch" => {
            let params: ApplyAxiomPatchParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_apply_axiom_patch(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/query" => {
            let params: QueryParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_query(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/sparql" => {
            let params: SparqlParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_sparql(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/parseManchester" => {
            let params: ParseManchesterParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_parse_manchester(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontocore/runReasoner" => {
            let params: RunReasonerParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_run_reasoner(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))
        }
        "ontocore/getExplanation" => {
            let params: GetExplanationParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_get_explanation(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))
        }
        "ontocore/runRobot" => {
            let params: RunRobotParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_run_robot(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::robot_failed(e.to_string()))
        }
        "ontocore/findUsages" => {
            let params: FindUsagesParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_find_usages(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "ontocore/previewRefactor" => {
            let params: PreviewRefactorParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_preview_refactor(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "ontocore/applyRefactor" => {
            let params: ApplyRefactorParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_apply_refactor(state, index_worker, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "ontocore/semanticDiff" | "ontocore/getSemanticDiff" => {
            let params: SemanticDiffParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_semantic_diff(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::invalid_params(e.to_string()))
        }
        _ => Err(LspErrorPayload::invalid_params(format!("unknown method: {method}"))),
    }
}

#[derive(Debug)]
pub enum StandardRequestOutcome {
    Ok(Value),
    MethodNotFound,
    InvalidParams(ResponseError),
    LspError(LspErrorPayload),
}

pub fn handle_standard_request(
    state: &ServerState,
    method: &str,
    params: Option<Value>,
) -> StandardRequestOutcome {
    match method {
        "textDocument/hover" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("hover"));
            };
            match handle_hover(state, params) {
                Some(hover) => serde_json::to_value(hover)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "textDocument/documentSymbol" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("documentSymbol"));
            };
            match handle_document_symbol(state, params) {
                Some(symbols) => serde_json::to_value(symbols)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "workspace/symbol" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("workspace/symbol"));
            };
            match handle_workspace_symbol(state, params) {
                Some(symbols) => serde_json::to_value(symbols)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Array(vec![]))),
                None => StandardRequestOutcome::Ok(Value::Array(vec![])),
            }
        }
        "textDocument/definition" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("definition"));
            };
            match handle_goto_definition(state, params) {
                Some(def) => serde_json::to_value(def)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "textDocument/references" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("references"));
            };
            match handle_references(state, params) {
                Some(refs) => serde_json::to_value(refs)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Array(vec![]))),
                None => StandardRequestOutcome::Ok(Value::Array(vec![])),
            }
        }
        "textDocument/rename" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("rename"));
            };
            match handle_rename(state, params) {
                Ok(Some(edit)) => serde_json::to_value(edit)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                Ok(None) => StandardRequestOutcome::Ok(Value::Null),
                Err(err) => StandardRequestOutcome::LspError(err),
            }
        }
        _ => StandardRequestOutcome::MethodNotFound,
    }
}

fn invalid_params(method: &str) -> ResponseError {
    ResponseError { code: -32602, message: format!("invalid params for {method}"), data: None }
}

/// Build an LSP range from entity source metadata and optional document text.
/// Callers must not hold the catalog `RwLock` when loading `doc_text`.
fn entity_source_range(doc_text: Option<&str>, entity: &ontocore_core::Entity) -> Range {
    let line_idx = entity.source_location.line.unwrap_or(1).saturating_sub(1) as u32;
    let byte_col = entity.source_location.column.unwrap_or(0) as usize;
    let line_text =
        doc_text.and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()));
    let character =
        line_text.as_deref().map(|l| byte_col_to_utf16(l, byte_col)).unwrap_or(byte_col as u32);
    Range {
        start: Position { line: line_idx, character },
        end: Position { line: line_idx, character: character.saturating_add(1) },
    }
}

fn lsp_document_path(state: &ServerState, uri: &Uri) -> Option<std::path::PathBuf> {
    state.resolve_lsp_document_uri(uri.as_str()).ok()
}

fn path_to_lsp_uri(path: &Path) -> Option<Uri> {
    Uri::from_str(&path_to_uri(path)).ok()
}

fn escape_markdown(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('`', "\\`")
        .replace('*', "\\*")
        .replace('_', "\\_")
}

fn entity_kind_to_symbol_kind(kind: EntityKind) -> SymbolKind {
    match kind {
        EntityKind::Class => SymbolKind::CLASS,
        EntityKind::ObjectProperty | EntityKind::DataProperty | EntityKind::AnnotationProperty => {
            SymbolKind::PROPERTY
        }
        EntityKind::Individual => SymbolKind::VARIABLE,
        EntityKind::Ontology => SymbolKind::NAMESPACE,
        EntityKind::Other => SymbolKind::OBJECT,
    }
}

fn iri_at_position(content: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;
    let byte_col = utf16_offset_to_byte(line, position.character);
    if byte_col > line.len() {
        return extract_iri_from_line(line);
    }

    let token = extract_token_at(line, byte_col);
    if token.contains(':') || token.starts_with("http") {
        return expand_iri_token(content, &token);
    }
    extract_iri_from_line(line)
}

fn extract_token_at(line: &str, ch: usize) -> String {
    let bytes = line.as_bytes();
    let mut start = ch.min(bytes.len());
    let mut end = ch.min(bytes.len());

    while start > 0 && is_iri_char(bytes[start - 1]) {
        start -= 1;
    }
    while end < bytes.len() && is_iri_char(bytes[end]) {
        end += 1;
    }

    line[start..end].to_string()
}

fn is_iri_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b':' | b'#' | b'/' | b'_' | b'-')
}

fn extract_iri_from_line(line: &str) -> Option<String> {
    for token in line.split_whitespace() {
        let cleaned = token.trim_matches(|c: char| c == ';' || c == '.' || c == ',');
        if cleaned.starts_with("http://") || cleaned.starts_with("https://") {
            return Some(cleaned.to_string());
        }
        if cleaned.contains(':') && !cleaned.starts_with('@') {
            return Some(cleaned.to_string());
        }
    }
    None
}

fn expand_iri_token(content: &str, token: &str) -> Option<String> {
    if token.starts_with("http://") || token.starts_with("https://") {
        return Some(token.to_string());
    }

    if let Some((prefix, local)) = token.split_once(':') {
        let prefix_line = content.lines().find(|l| l.contains(&format!("@prefix {prefix}:")))?;
        let start = prefix_line.find('<')? + 1;
        let end = prefix_line.find('>')?;
        if end <= start {
            return None;
        }
        let ns = &prefix_line[start..end];
        return Some(format!("{ns}{local}"));
    }

    Some(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_renamed_iri_expands_prefixed_name() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class .";
        let iri = derive_renamed_iri("http://example.org/people#Person", "ex:Human", content);
        assert_eq!(iri, "http://example.org/people#Human");
    }

    #[test]
    fn expand_prefixed_iri() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class .";
        let iri = expand_iri_token(content, "ex:Person").expect("expanded");
        assert_eq!(iri, "http://example.org/people#Person");
    }

    #[test]
    fn expand_iri_token_rejects_malformed_prefix_line() {
        let content = "@prefix ex: >oops<http://example.org/people#> .\nex:Person a owl:Class .";
        assert!(expand_iri_token(content, "ex:Person").is_none());
    }

    #[test]
    fn escape_markdown_neutralizes_links() {
        let escaped = escape_markdown("[click](https://evil.example)");
        assert!(!escaped.contains("](https://"));
        assert!(escaped.contains("\\["));
    }
}
