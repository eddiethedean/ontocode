use crate::protocol::{
    CatalogSnapshot, DiagnosticSummary, GetEntityParams, GetEntityResult, IndexWorkspaceParams,
    IndexWorkspaceResult, LspErrorPayload,
};
use crate::state::{path_to_uri, resolve_workspace_for_index, ServerState};
use lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
    InitializeParams, InitializeResult, Location, MarkupContent, MarkupKind, OneOf, Position,
    Range, ServerCapabilities, SymbolInformation, SymbolKind, Uri, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};
use ontoindex_core::{resolve_document_path, EntityKind};
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
            let _ = state.index_workspace(path);
        }
    }

    InitializeResult {
        capabilities: ServerCapabilities {
            definition_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
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
    params: IndexWorkspaceParams,
) -> Result<IndexWorkspaceResult, LspErrorPayload> {
    let workspace = match params.workspace_uri.as_deref() {
        Some(uri) => resolve_workspace_for_index(uri, state.workspace_root().as_deref())
            .map_err(LspErrorPayload::index_failed)?,
        None => state.workspace_root().ok_or_else(|| {
            LspErrorPayload::index_failed("no workspace URI provided".to_string())
        })?,
    };

    let (stats, indexed_at) =
        state.index_workspace(workspace).map_err(LspErrorPayload::index_failed)?;

    Ok(IndexWorkspaceResult { stats, indexed_at })
}

pub fn handle_get_catalog_snapshot(
    state: &ServerState,
) -> Result<CatalogSnapshot, LspErrorPayload> {
    state
        .with_catalog(|catalog| CatalogSnapshot {
            documents: catalog.data().documents.clone(),
            entities: catalog.data().entities.clone(),
            hierarchy: catalog.class_hierarchy(),
            diagnostics: catalog
                .data()
                .diagnostics
                .iter()
                .map(DiagnosticSummary::from)
                .collect(),
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
            .map(|e| DocumentSymbol {
                name: e.short_name.clone(),
                detail: Some(e.iri.clone()),
                kind: entity_kind_to_symbol_kind(e.kind),
                tags: None,
                deprecated: None,
                range: Range::default(),
                selection_range: Range::default(),
                children: None,
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
                Some(SymbolInformation {
                    name: e.short_name.clone(),
                    kind: entity_kind_to_symbol_kind(e.kind),
                    tags: None,
                    deprecated: None,
                    location: Location { uri, range: Range::default() },
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
        let range = Range {
            start: Position {
                line: (source.line.saturating_sub(1)) as u32,
                character: source.column as u32,
            },
            end: Position {
                line: (source.line.saturating_sub(1)) as u32,
                character: (source.column.saturating_add(1)) as u32,
            },
        };
        Some(GotoDefinitionResponse::Scalar(Location { uri, range }))
    })?
}

pub fn handle_custom_request(
    state: &ServerState,
    method: &str,
    params: Option<Value>,
) -> Result<Value, LspErrorPayload> {
    match method {
        "ontoindex/indexWorkspace" => {
            let params: IndexWorkspaceParams =
                serde_json::from_value(params.unwrap_or(Value::Null))
                    .map_err(|e| LspErrorPayload::index_failed(format!("invalid params: {e}")))?;
            let result = handle_index_workspace(state, params)?;
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
        _ => Err(LspErrorPayload::index_failed(format!("unknown method: {method}"))),
    }
}

pub fn handle_standard_request(
    state: &ServerState,
    method: &str,
    params: Option<Value>,
) -> Option<Value> {
    match method {
        "textDocument/hover" => {
            let params: HoverParams = serde_json::from_value(params?).ok()?;
            serde_json::to_value(handle_hover(state, params)?).ok()
        }
        "textDocument/documentSymbol" => {
            let params: DocumentSymbolParams = serde_json::from_value(params?).ok()?;
            serde_json::to_value(handle_document_symbol(state, params)?).ok()
        }
        "workspace/symbol" => {
            let params: WorkspaceSymbolParams = serde_json::from_value(params?).ok()?;
            serde_json::to_value(handle_workspace_symbol(state, params)?).ok()
        }
        "textDocument/definition" => {
            let params: GotoDefinitionParams = serde_json::from_value(params?).ok()?;
            serde_json::to_value(handle_goto_definition(state, params)?).ok()
        }
        _ => None,
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

fn utf16_offset_to_byte(line: &str, utf16_col: u32) -> usize {
    let mut utf16_seen = 0u32;
    for (byte_idx, ch) in line.char_indices() {
        if utf16_seen >= utf16_col {
            return byte_idx;
        }
        utf16_seen += ch.len_utf16() as u32;
    }
    line.len()
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
