use crate::handlers::{
    handle_get_catalog_snapshot, handle_get_entity, handle_goto_definition, handle_hover,
    handle_workspace_symbol,
};
use crate::protocol::GetEntityParams;
use crate::state::{path_to_uri, ServerState};
use lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, HoverContents, HoverParams, Position,
    TextDocumentIdentifier, TextDocumentPositionParams, Uri, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};
use std::path::PathBuf;
use std::str::FromStr;

fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn indexed_state() -> ServerState {
    let state = ServerState::new();
    state.index_workspace(fixture_workspace()).expect("index fixture workspace");
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
fn get_catalog_snapshot_returns_fixture_entities() {
    let state = indexed_state();
    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");

    assert_eq!(snapshot.documents.len(), 2);
    assert!(snapshot.entities.iter().any(|e| e.iri == "http://example.org/people#Person"));
    assert!(!snapshot.hierarchy.edges.is_empty());
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
