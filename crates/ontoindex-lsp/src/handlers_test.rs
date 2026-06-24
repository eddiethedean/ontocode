use crate::handlers::{
    handle_get_catalog_snapshot, handle_get_entity, handle_goto_definition, handle_hover,
    handle_standard_request, handle_workspace_symbol, StandardRequestOutcome,
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

    assert_eq!(snapshot.documents.len(), 2);
    assert!(snapshot.entities.iter().any(|e| e.iri == "http://example.org/people#Person"));
    assert!(!snapshot.hierarchy.edges.is_empty());
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
    state.index_workspace(dir.path().to_path_buf()).expect("initial index");

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

    state.set_document_text(
        doc_path,
        format!("{base}ex:Ok a owl:Class .\nun:Bad a owl:Class .\n"),
    );
    state.index_workspace(dir.path().to_path_buf()).expect("reindex with buffer");

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
