use crate::handlers::{
    handle_apply_axiom_patch, handle_apply_refactor, handle_find_usages,
    handle_get_catalog_snapshot, handle_get_entity, handle_goto_definition, handle_hover,
    handle_query, handle_references, handle_sparql, handle_standard_request,
    handle_workspace_symbol, StandardRequestOutcome,
};
use crate::index_worker::IndexWorker;
use crate::protocol::{
    ApplyAxiomPatchParams, ApplyRefactorParams, FindUsagesParams, GetEntityParams, QueryParams,
    SparqlParams,
};
use crate::state::{path_to_uri, ServerState};
use crossbeam_channel::unbounded;
use lsp_server::Message;
use lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, HoverContents, HoverParams, Position,
    ReferenceContext, ReferenceParams, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
    WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use ontoindex_refactor::{preview_rename_iri, RefactorRequest};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn indexed_state() -> ServerState {
    let state = ServerState::new();
    let ws = fixture_workspace();
    state.set_workspace_root(ws.clone()).expect("set workspace");
    state.index_workspace(ws).expect("index fixture workspace");
    state
}

fn fixture_ttl_uri() -> Uri {
    let path = fixture_workspace().join("example.ttl");
    Uri::from_str(&path_to_uri(&path)).expect("fixture uri")
}

#[test]
fn get_catalog_snapshot_before_index_returns_not_indexed() {
    let state = ServerState::new();
    let err = handle_get_catalog_snapshot(&state).unwrap_err();
    assert_eq!(err.code, "NOT_INDEXED");
}

#[test]
fn get_entity_returns_person_detail() {
    let state = indexed_state();
    let result = handle_get_entity(
        &state,
        GetEntityParams { iri: "http://example.org/people#Person".into() },
    )
    .expect("getEntity");

    assert_eq!(result.detail.entity.short_name, "Person");
    assert!(!result.detail.parents.is_empty());
    assert!(result.detail.source.is_some());
}

#[test]
fn get_entity_unknown_iri_returns_not_found() {
    let state = indexed_state();
    let err = handle_get_entity(
        &state,
        GetEntityParams { iri: "http://example.org/people#Missing".into() },
    )
    .unwrap_err();
    assert_eq!(err.code, "ENTITY_NOT_FOUND");
}

#[test]
fn hover_rejects_document_outside_workspace() {
    let state = indexed_state();
    let outside = tempfile::tempdir().unwrap();
    let ttl = outside.path().join("other.ttl");
    std::fs::write(&ttl, "@prefix ex: <http://ex/> .\n").unwrap();
    let uri = Uri::from_str(&path_to_uri(&ttl)).expect("uri");

    let hover = handle_hover(
        &state,
        HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position::new(0, 0),
            },
            work_done_progress_params: Default::default(),
        },
    );
    assert!(hover.is_none());
}

#[test]
fn get_catalog_snapshot_returns_fixture_entities() {
    let state = indexed_state();
    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");

    assert_eq!(snapshot.documents.len(), 6);
    assert!(snapshot.entities.iter().any(|e| e.iri == "http://example.org/people#Person"));
    assert!(!snapshot.hierarchy.edges.is_empty());
}

#[test]
fn find_usages_returns_person_references() {
    let state = indexed_state();
    let result = handle_find_usages(
        &state,
        FindUsagesParams { iri: "http://example.org/people#Person".to_string() },
    )
    .expect("find usages");
    assert!(!result.usages.is_empty());
}

#[test]
fn references_span_covers_token_not_single_character() {
    let state = indexed_state();
    let content = std::fs::read_to_string(fixture_workspace().join("example.ttl")).unwrap();
    let person_line =
        content.lines().position(|l| l.contains("ex:Person")).expect("Person line") as u32;
    let refs = handle_references(
        &state,
        ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: fixture_ttl_uri() },
                position: Position { line: person_line, character: 0 },
            },
            context: ReferenceContext { include_declaration: true },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .expect("references");
    assert!(!refs.is_empty());
    let first = &refs[0];
    assert!(
        first.range.end.character > first.range.start.character.saturating_add(1),
        "reference range should span the token, got {:?}",
        first.range
    );
}

/// Contract test: LSP JSON must use snake_case enum strings (extension + docs/lsp-api.md).
#[test]
fn catalog_snapshot_wire_format_uses_snake_case_enums() {
    let state = indexed_state();
    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");
    let json = serde_json::to_value(&snapshot).expect("serialize snapshot");

    let person = json
        .get("entities")
        .and_then(|e| e.as_array())
        .and_then(|arr| {
            arr.iter().find(|e| {
                e.get("iri").and_then(|v| v.as_str()) == Some("http://example.org/people#Person")
            })
        })
        .expect("Person in entities");
    assert_eq!(person.get("kind").and_then(|v| v.as_str()), Some("class"));

    let doc = json
        .get("documents")
        .and_then(|d| d.as_array())
        .and_then(|arr| {
            arr.iter().find(|d| {
                d.get("path").and_then(|v| v.as_str()).is_some_and(|p| p.ends_with("example.ttl"))
            })
        })
        .expect("example.ttl document");
    assert_eq!(doc.get("parse_status").and_then(|v| v.as_str()), Some("ok"));
    assert_eq!(doc.get("format").and_then(|v| v.as_str()), Some("turtle"));
}

#[test]
fn hover_on_person_returns_markdown() {
    let state = indexed_state();
    let content = std::fs::read_to_string(fixture_workspace().join("example.ttl")).unwrap();
    let person_line =
        content.lines().position(|l| l.starts_with("ex:Person")).expect("Person line") as u32;

    let hover = handle_hover(
        &state,
        HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: fixture_ttl_uri() },
                position: Position { line: person_line, character: 2 },
            },
            work_done_progress_params: Default::default(),
        },
    )
    .expect("hover");

    let HoverContents::Markup(markup) = hover.contents else {
        panic!("expected markdown hover");
    };
    assert!(markup.value.contains("Person"));
    assert!(markup.value.contains("class"));
}

#[test]
fn goto_definition_on_person_points_to_source() {
    let state = indexed_state();
    let content = std::fs::read_to_string(fixture_workspace().join("example.ttl")).unwrap();
    let person_line =
        content.lines().position(|l| l.starts_with("ex:Person")).expect("Person line") as u32;

    let response = handle_goto_definition(
        &state,
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: fixture_ttl_uri() },
                position: Position { line: person_line, character: 2 },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .expect("definition");

    let GotoDefinitionResponse::Scalar(location) = response else {
        panic!("expected scalar location");
    };
    assert!(location.uri.as_str().contains("example.ttl"));
    assert_eq!(location.range.start.line, person_line);
}

#[test]
fn workspace_symbol_finds_person() {
    let state = indexed_state();
    let response = handle_workspace_symbol(
        &state,
        WorkspaceSymbolParams {
            query: "Person".into(),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .expect("workspace symbols");

    let WorkspaceSymbolResponse::Flat(symbols) = response else {
        panic!("expected flat symbols");
    };
    assert!(symbols.iter().any(|s| s.name == "Person"));
}

#[test]
fn hover_on_blank_line_returns_null_result() {
    let state = indexed_state();
    let uri = fixture_ttl_uri();
    let outcome = handle_standard_request(
        &state,
        "textDocument/hover",
        Some(serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": 9999, "character": 0 }
        })),
    );
    match outcome {
        StandardRequestOutcome::Ok(value) => assert!(value.is_null()),
        other => panic!("expected null hover result, got {other:?}"),
    }
}

#[test]
fn open_buffer_override_surfaces_undefined_prefix_in_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("live.ttl");
    let base = "@prefix ex: <http://example.org/live#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<http://example.org/live> a owl:Ontology .\n";
    std::fs::write(&path, format!("{base}ex:Ok a owl:Class .\n")).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_root(ws.clone()).expect("set workspace");
    state.index_workspace(ws.clone()).expect("initial index");

    let doc_path = state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| d.path.file_name() == path.file_name())
                .map(|d| d.path.clone())
        })
        .expect("indexed catalog")
        .expect("live.ttl document");

    let _ = state
        .set_document_text(doc_path, format!("{base}ex:Ok a owl:Class .\nun:Bad a owl:Class .\n"));
    state.index_workspace(ws).expect("reindex with buffer");

    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");
    assert!(
        snapshot.diagnostics.iter().any(|d| {
            d.severity == "error"
                && d.message.contains("un:")
                && (d.code == "undefined_prefix" || d.code == "parse_error")
        }),
        "expected undeclared prefix diagnostic from open buffer, got: {:?}",
        snapshot.diagnostics
    );
}

#[test]
fn index_workspace_params_accept_snake_case_and_legacy_camel_case() {
    let snake: crate::protocol::IndexWorkspaceParams =
        serde_json::from_value(serde_json::json!({ "workspace_uri": "file:///tmp/ws" }))
            .expect("workspace_uri");
    assert_eq!(snake.workspace_uri.as_deref(), Some("file:///tmp/ws"));

    let legacy: crate::protocol::IndexWorkspaceParams =
        serde_json::from_value(serde_json::json!({ "workspaceUri": "file:///tmp/ws" }))
            .expect("workspaceUri alias");
    assert_eq!(legacy.workspace_uri.as_deref(), Some("file:///tmp/ws"));
}

#[test]
fn query_returns_classes() {
    let state = indexed_state();
    let result =
        handle_query(&state, QueryParams { sql: "SELECT short_name FROM classes".to_string() })
            .expect("query");
    assert!(!result.columns.is_empty());
    assert!(!result.rows.is_empty());
}

#[test]
fn sparql_returns_triples() {
    let state = indexed_state();
    let result = handle_sparql(
        &state,
        SparqlParams { query: "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5".to_string() },
    )
    .expect("sparql");
    assert!(!result.columns.is_empty());
}

#[test]
fn apply_axiom_patch_uses_open_buffer_not_disk() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("example.ttl");
    std::fs::copy(fixture_workspace().join("example.ttl"), &path).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_root(ws.clone()).expect("set workspace");
    state.index_workspace(ws.clone()).expect("index");

    let buffer_marker = "# unsaved buffer marker\n";
    let disk = std::fs::read_to_string(&path).unwrap();
    let buffer = format!("{buffer_marker}{disk}");
    state.set_document_text(path.clone(), buffer).expect("set buffer");

    let (tx, _rx) = unbounded::<Message>();
    let worker = IndexWorker::spawn(state.clone(), tx);

    let uri = path_to_uri(&path);
    let result = handle_apply_axiom_patch(
        &state,
        &worker,
        ApplyAxiomPatchParams {
            document_uri: uri,
            patches: vec![ontoindex_owl::PatchOp::AddLabel {
                entity_iri: "http://example.org/people#Person".into(),
                value: "Human".into(),
            }],
            preview_only: false,
        },
    )
    .expect("apply patch");

    assert!(result.patch.applied);
    let updated = state.document_text(&path).expect("buffer after patch");
    assert!(
        updated.contains(buffer_marker),
        "patch should apply to open buffer, not disk-only source"
    );
    assert!(updated.contains("Human"));
}

fn refactor_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/refactor").join(path)
}

#[test]
fn apply_refactor_tracks_only_open_buffers() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("people.ttl");
    std::fs::copy(refactor_fixture("people.ttl"), &path).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_root(ws.clone()).expect("set workspace");
    state.index_workspace(ws.clone()).expect("index");

    let from = "http://example.org/org#Agent";
    let to = "http://example.org/org#Worker";
    let overrides = HashMap::new();
    let plan = state
        .with_catalog(|catalog| preview_rename_iri(catalog, from, to, &overrides).ok())
        .flatten()
        .expect("plan");

    let (tx, _rx) = unbounded::<Message>();
    let worker = IndexWorker::spawn(state.clone(), tx);

    let result = handle_apply_refactor(
        &state,
        &worker,
        ApplyRefactorParams {
            plan: plan.clone(),
            request: RefactorRequest::RenameIri {
                from_iri: from.to_string(),
                to_iri: to.to_string(),
            },
            preview_only: false,
        },
    )
    .expect("apply refactor");

    assert!(result.workspace_edit.is_some());
    assert!(
        state.open_document_overrides().is_empty(),
        "closed files must not be injected into open_documents"
    );

    let disk = std::fs::read_to_string(&path).unwrap();
    assert!(disk.contains("Worker") || disk.contains("ex:Worker"));

    let buffer_marker = "# unsaved buffer marker\n";
    let buffer = format!("{buffer_marker}{}", std::fs::read_to_string(&path).unwrap());
    state.set_document_text(path.clone(), buffer.clone()).expect("open buffer");

    let overrides = state.open_document_overrides();
    let plan2 = state
        .with_catalog(|catalog| preview_rename_iri(catalog, to, from, &overrides).ok())
        .flatten()
        .expect("plan2");

    handle_apply_refactor(
        &state,
        &worker,
        ApplyRefactorParams {
            plan: plan2,
            request: RefactorRequest::RenameIri {
                from_iri: to.to_string(),
                to_iri: from.to_string(),
            },
            preview_only: false,
        },
    )
    .expect("apply refactor to open buffer");

    let updated = state.document_text(&path).expect("buffer after refactor");
    assert!(updated.contains(buffer_marker), "refactor should update open buffer in place");
    assert!(updated.contains("Agent") || updated.contains("ex:Agent"));
}
