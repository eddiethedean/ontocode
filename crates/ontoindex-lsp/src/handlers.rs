use crate::index_worker::IndexWorker;
use crate::positions::{byte_col_to_utf16, utf16_offset_to_byte};
use crate::protocol::{
    ApplyAxiomPatchParams, ApplyAxiomPatchResult, ApplyRefactorParams, ApplyRefactorResult,
    CatalogSnapshot, DiagnosticSummary, FindUsagesParams, FindUsagesResult, GetEntityParams,
    GetEntityResult, GetExplanationParams, GetExplanationResult, GetGraphResult,
    IndexWorkspaceParams, IndexWorkspaceResult, LspErrorPayload, ManchesterCompletions,
    ParseManchesterParams, ParseManchesterResult, PreviewRefactorParams, PreviewRefactorResult,
    QueryParams, RunReasonerParams, RunReasonerResult, RunRobotParams, RunRobotResult,
    SparqlParams, TabularQueryResult, UsageSummary,
};
use crate::state::{path_to_uri, resolve_workspace_for_index, ServerState};
use lsp_server::ResponseError;
use lsp_types::{
    DocumentChanges, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, Location, MarkupContent,
    MarkupKind, OneOf, Position, Range, ReferenceParams, RenameParams, ServerCapabilities,
    SymbolInformation, SymbolKind, TextDocumentEdit, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextEdit, Uri, WorkspaceEdit, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};
use ontoindex_catalog::{GraphBuilder, GraphRequest};
use ontoindex_core::{resolve_document_path, EntityKind, OntologyFormat};
use ontoindex_reasoner::{
    classify, explain, ExplanationRequest, ReasonerId, ReasonerSnapshot, WorkspaceInputLoader,
};
use ontoindex_refactor::{apply_refactor_plan_checked, find_usages, preview_refactor, preview_rename_iri};
use serde_json::Value;
use std::path::Path;
use std::str::FromStr;

#[allow(deprecated)]
pub fn handle_initialize(state: &ServerState, params: InitializeParams) -> InitializeResult {
    let workspace_uri = params
        .workspace_folders
        .and_then(|folders| folders.into_iter().next().map(|f| f.uri))
        .or(params.root_uri);

    if let Some(uri) = workspace_uri {
        if let Ok(path) = resolve_workspace_for_index(uri.as_str(), None) {
            let _ = state.set_workspace_root(path);
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
            ..Default::default()
        },
        server_info: Some(lsp_types::ServerInfo {
            name: "ontoindex-lsp".to_string(),
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

    let (stats, indexed_at) =
        index_worker.enqueue_sync(workspace).map_err(LspErrorPayload::index_failed)?;

    Ok(IndexWorkspaceResult { stats, indexed_at })
}

pub fn build_catalog_snapshot(catalog: &ontoindex_catalog::OntologyCatalog) -> CatalogSnapshot {
    CatalogSnapshot {
        documents: catalog.data().documents.clone(),
        entities: catalog.data().entities.clone(),
        hierarchy: catalog.class_hierarchy(),
        diagnostics: catalog.data().diagnostics.iter().map(DiagnosticSummary::from).collect(),
        reasoner: None,
    }
}

pub fn build_catalog_snapshot_with_reasoner(
    catalog: &ontoindex_catalog::OntologyCatalog,
    reasoner: Option<ontoindex_reasoner::ReasonerSnapshot>,
) -> CatalogSnapshot {
    let mut snapshot = build_catalog_snapshot(catalog);
    snapshot.reasoner = reasoner;
    snapshot
}

pub fn handle_get_catalog_snapshot(
    state: &ServerState,
) -> Result<CatalogSnapshot, LspErrorPayload> {
    let reasoner = state.reasoner_snapshot();
    state
        .with_catalog(|catalog| build_catalog_snapshot_with_reasoner(catalog, reasoner))
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
    let inferred_edges = if params.include_inferred {
        state.reasoner_snapshot().map(|s| s.inferred.edges.clone())
    } else {
        None
    };
    state
        .with_catalog(|catalog| {
            let mut builder = GraphBuilder::new(catalog);
            if let Some(ref edges) = inferred_edges {
                builder = builder.with_inferred_edges(edges);
            }
            let graph = builder.build(&params).map_err(LspErrorPayload::graph_failed)?;
            Ok(GetGraphResult { graph })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_run_robot(
    index_worker: &IndexWorker,
    params: RunRobotParams,
) -> Result<RunRobotResult, LspErrorPayload> {
    let mut args = vec![params.subcommand];
    args.extend(params.args);
    index_worker.run_robot_sync(params.robot_path, args).map_err(LspErrorPayload::robot_failed)
}

pub fn handle_apply_axiom_patch(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: ApplyAxiomPatchParams,
) -> Result<ApplyAxiomPatchResult, LspErrorPayload> {
    let workspace = state.effective_index_root().ok_or_else(LspErrorPayload::not_indexed)?;
    let document_path = ontoindex_core::resolve_lsp_document_path(&params.document_uri, &workspace)
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

    let source = state
        .document_text(&document_path)
        .ok_or_else(|| LspErrorPayload::patch_invalid("cannot read document".to_string()))?;

    let mut patch_result = ontoindex_owl::apply_patches_to_text(
        &source,
        &params.patches,
        params.preview_only,
        &namespaces,
    )
    .map_err(|e| match e {
        ontoindex_owl::OwlError::UnsupportedFormat(m) => LspErrorPayload::unsupported_format(m),
        _ => LspErrorPayload::patch_invalid(e.to_string()),
    })?;
    patch_result.document_path = Some(document_path.display().to_string());

    if !patch_result.applied && !patch_result.diagnostics.is_empty() {
        return Ok(ApplyAxiomPatchResult {
            patch: patch_result,
            entity_detail: None,
            reindex_warning: None,
        });
    }

    let entity_iri = params.patches.first().map(|p| match p {
        ontoindex_owl::PatchOp::CreateEntity { entity_iri, .. }
        | ontoindex_owl::PatchOp::DeleteEntity { entity_iri }
        | ontoindex_owl::PatchOp::SetLabel { entity_iri, .. }
        | ontoindex_owl::PatchOp::AddLabel { entity_iri, .. }
        | ontoindex_owl::PatchOp::RemoveLabel { entity_iri, .. }
        | ontoindex_owl::PatchOp::SetComment { entity_iri, .. }
        | ontoindex_owl::PatchOp::AddComment { entity_iri, .. }
        | ontoindex_owl::PatchOp::RemoveComment { entity_iri, .. }
        | ontoindex_owl::PatchOp::AddSubClassOf { entity_iri, .. }
        | ontoindex_owl::PatchOp::RemoveSubClassOf { entity_iri, .. }
        | ontoindex_owl::PatchOp::AddComplexSubClassOf { entity_iri, .. }
        | ontoindex_owl::PatchOp::RemoveComplexSubClassOf { entity_iri, .. }
        | ontoindex_owl::PatchOp::AddEquivalentClass { entity_iri, .. }
        | ontoindex_owl::PatchOp::RemoveEquivalentClass { entity_iri, .. }
        | ontoindex_owl::PatchOp::SetEquivalentClass { entity_iri, .. }
        | ontoindex_owl::PatchOp::SetDeprecated { entity_iri, .. }
        | ontoindex_owl::PatchOp::AddDisjointClass { entity_iri, .. }
        | ontoindex_owl::PatchOp::RemoveDisjointClass { entity_iri, .. } => entity_iri.clone(),
    });

    let mut reindex_warning = None;

    if patch_result.applied && !params.preview_only {
        if let Some(text) = &patch_result.preview_text {
            std::fs::write(&document_path, text).map_err(|e| {
                LspErrorPayload::patch_invalid(format!("failed to write document: {e}"))
            })?;
            state
                .set_document_text(document_path.clone(), text.clone())
                .map_err(LspErrorPayload::patch_invalid)?;
        }
        if let Err(msg) = index_worker.enqueue_sync(workspace) {
            reindex_warning = Some(msg);
        }
    }

    let entity_detail = entity_iri
        .and_then(|iri| state.with_catalog(|catalog| catalog.entity_detail(&iri)))
        .flatten();

    Ok(ApplyAxiomPatchResult { patch: patch_result, entity_detail, reindex_warning })
}

pub fn handle_query(
    state: &ServerState,
    params: QueryParams,
) -> Result<TabularQueryResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let result = ontoindex_query::query_catalog(catalog, &params.sql).map_err(
                |e: ontoindex_query::QueryError| LspErrorPayload::query_failed(e.to_string()),
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
            let result = ontoindex_query::sparql_catalog(catalog, &params.query).map_err(
                |e: ontoindex_query::QueryError| LspErrorPayload::query_failed(e.to_string()),
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

    let parsed = ontoindex_owl::parse_class_expression(&params.expression, &namespaces)
        .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;

    let turtle_predicate = match params.axiom_kind.as_str() {
        "equivalent_class" => "owl:equivalentClass",
        _ => "rdfs:subClassOf",
    };

    let turtle_fragment = ontoindex_owl::class_expression_to_turtle_fragment(
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
            .map(|d| ontoindex_owl::PatchDiagnostic {
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
    classification: &ontoindex_reasoner::ClassificationResult,
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
) -> Result<ontoindex_reasoner::ReasonerInput, LspErrorPayload> {
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
        let workspace = state.workspace_root().ok_or_else(LspErrorPayload::not_indexed)?;
        let path = ontoindex_core::resolve_lsp_document_path(uri, &workspace)
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
    state.with_catalog(|catalog| {
        let entities = catalog.entities_in_document(&path);
        if entities.is_empty() {
            return None;
        }
        let symbols: Vec<DocumentSymbol> = entities
            .into_iter()
            .map(|e| {
                let range = entity_source_range(state, &path, e);
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
    })?
}

#[allow(deprecated)]
pub fn handle_workspace_symbol(
    state: &ServerState,
    params: WorkspaceSymbolParams,
) -> Option<WorkspaceSymbolResponse> {
    let query = params.query.to_ascii_lowercase();
    state.with_catalog(|catalog| {
        let symbols: Vec<SymbolInformation> = catalog
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
                let uri = path_to_lsp_uri(&doc.path)?;
                let range = entity_source_range(state, &doc.path, e);
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
    })?
}

pub fn handle_goto_definition(
    state: &ServerState,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let path = lsp_document_path(state, &params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;

    state.with_catalog(|catalog| {
        let source = catalog.find_source_location(&iri)?;
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
    })?
}

pub fn handle_find_usages(
    state: &ServerState,
    params: FindUsagesParams,
) -> Result<FindUsagesResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let usages = find_usages(catalog, &params.iri);
            Ok(FindUsagesResult {
                usages: usages.into_iter().map(usage_to_summary).collect(),
            })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_preview_refactor(
    state: &ServerState,
    params: PreviewRefactorParams,
) -> Result<PreviewRefactorResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let plan = preview_refactor(catalog, &params.request)
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
    let files_written = apply_refactor_plan_checked(&params.plan, params.preview_only)
        .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;
    if params.preview_only {
        return Ok(ApplyRefactorResult { files_written: 0, reindex_warning: None });
    }
    let reindex_warning = match state.effective_index_root() {
        Some(root) => match index_worker.enqueue_sync(root) {
            Ok(_) => None,
            Err(e) => Some(format!("refactor applied but reindex failed: {e}")),
        },
        None => Some("refactor applied but workspace root unknown".to_string()),
    };
    Ok(ApplyRefactorResult { files_written, reindex_warning })
}

pub fn handle_references(
    state: &ServerState,
    params: ReferenceParams,
) -> Option<Vec<Location>> {
    let path = lsp_document_path(state, &params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;
    state.with_catalog(|catalog| {
        let locations: Vec<Location> = find_usages(catalog, &iri)
            .into_iter()
            .filter_map(|u| {
                let uri = path_to_lsp_uri(&u.file)?;
                let line_idx = u.line.unwrap_or(1).saturating_sub(1) as u32;
                let byte_col = u.column.unwrap_or(0) as usize;
                let line_text = state
                    .document_text(&u.file)
                    .and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()));
                let character = line_text
                    .as_deref()
                    .map(|l| byte_col_to_utf16(l, byte_col))
                    .unwrap_or(byte_col as u32);
                let range = Range {
                    start: Position { line: line_idx, character },
                    end: Position {
                        line: line_idx,
                        character: character.saturating_add(1),
                    },
                };
                Some(Location { uri, range })
            })
            .collect();
        Some(locations)
    })?
}

pub fn handle_rename(
    state: &ServerState,
    params: RenameParams,
) -> Option<WorkspaceEdit> {
    let path = lsp_document_path(state, &params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position;
    let content = state.document_text(&path)?;
    let from_iri = iri_at_position(&content, position)?;
    let to_iri = derive_renamed_iri(&from_iri, &params.new_name);
    state.with_catalog(|catalog| {
        let plan = preview_rename_iri(catalog, &from_iri, &to_iri).ok()?;
        plan_to_workspace_edit(&plan)
    })?
}

fn usage_to_summary(u: ontoindex_refactor::Usage) -> UsageSummary {
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

fn derive_renamed_iri(from_iri: &str, new_name: &str) -> String {
    if new_name.contains("://") {
        return new_name.to_string();
    }
    if let Some((base, _)) = from_iri.rsplit_once('#') {
        return format!("{base}#{new_name}");
    }
    if let Some((base, _)) = from_iri.rsplit_once('/') {
        return format!("{base}/{new_name}");
    }
    new_name.to_string()
}

fn plan_to_workspace_edit(plan: &ontoindex_refactor::RefactorPlan) -> Option<WorkspaceEdit> {
    let mut document_changes = Vec::new();
    for change in &plan.changes {
        if change.preview_text == change.original_text {
            continue;
        }
        let uri = path_to_lsp_uri(&change.path)?;
        document_changes.push(TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier {
                uri,
                version: None,
            },
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
        "ontoindex/indexWorkspace" => {
            let params: IndexWorkspaceParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_index_workspace(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/getCatalogSnapshot" => {
            let result = handle_get_catalog_snapshot(state)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/getEntity" => {
            let params: GetEntityParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_get_entity(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/getGraph" => {
            let params: GraphRequest = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_get_graph(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/applyAxiomPatch" => {
            let params: ApplyAxiomPatchParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_apply_axiom_patch(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/query" => {
            let params: QueryParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_query(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/sparql" => {
            let params: SparqlParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_sparql(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/parseManchester" => {
            let params: ParseManchesterParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_parse_manchester(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "ontoindex/runReasoner" => {
            let params: RunReasonerParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_run_reasoner(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))
        }
        "ontoindex/getExplanation" => {
            let params: GetExplanationParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_get_explanation(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))
        }
        "ontoindex/runRobot" => {
            let params: RunRobotParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_run_robot(index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::robot_failed(e.to_string()))
        }
        "ontoindex/findUsages" => {
            let params: FindUsagesParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_find_usages(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "ontoindex/previewRefactor" => {
            let params: PreviewRefactorParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_preview_refactor(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "ontoindex/applyRefactor" => {
            let params: ApplyRefactorParams = serde_json::from_value(params.unwrap_or(Value::Null))
                .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))?;
            let result = handle_apply_refactor(state, index_worker, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        _ => Err(LspErrorPayload::invalid_params(format!("unknown method: {method}"))),
    }
}

#[derive(Debug)]
pub enum StandardRequestOutcome {
    Ok(Value),
    MethodNotFound,
    InvalidParams(ResponseError),
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
                Some(edit) => serde_json::to_value(edit)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        _ => StandardRequestOutcome::MethodNotFound,
    }
}

fn invalid_params(method: &str) -> ResponseError {
    ResponseError { code: -32602, message: format!("invalid params for {method}"), data: None }
}

fn entity_source_range(state: &ServerState, path: &Path, entity: &ontoindex_core::Entity) -> Range {
    let line_idx = entity.source_location.line.unwrap_or(1).saturating_sub(1) as u32;
    let byte_col = entity.source_location.column.unwrap_or(0) as usize;
    let line_text = state
        .document_text(path)
        .and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()));
    let character =
        line_text.as_deref().map(|l| byte_col_to_utf16(l, byte_col)).unwrap_or(byte_col as u32);
    Range {
        start: Position { line: line_idx, character },
        end: Position { line: line_idx, character: character.saturating_add(1) },
    }
}

fn lsp_document_path(state: &ServerState, uri: &Uri) -> Option<std::path::PathBuf> {
    let root = state.workspace_root()?;
    resolve_document_path(uri.as_str(), &root).ok()
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
        let ns = &prefix_line[start..end];
        return Some(format!("{ns}{local}"));
    }

    Some(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_prefixed_iri() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class .";
        let iri = expand_iri_token(content, "ex:Person").expect("expanded");
        assert_eq!(iri, "http://example.org/people#Person");
    }

    #[test]
    fn escape_markdown_neutralizes_links() {
        let escaped = escape_markdown("[click](https://evil.example)");
        assert!(!escaped.contains("](https://"));
        assert!(escaped.contains("\\["));
    }
}
