//! `textDocument/codeAction` from diagnostic quick fixes.

use crate::state::ServerState;
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, Range, TextEdit, Uri,
    WorkspaceEdit,
};
use ontocore_core::QuickFix;
use ontocore_owl::patch::{apply_patches_to_text, PatchOp};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn handle_code_action(
    state: &ServerState,
    params: CodeActionParams,
) -> Option<Vec<CodeActionOrCommand>> {
    let path = state.resolve_lsp_document_uri(params.text_document.uri.as_str()).ok()?;
    let content = state.document_text(&path)?;
    let namespaces = namespaces_for_path(state, &path);
    let mut actions = Vec::new();

    for diag in params.context.diagnostics {
        let fix = diag.data.as_ref().and_then(|v| {
            if let Some(s) = v.as_str() {
                QuickFix::decode(s)
            } else {
                serde_json::from_value::<String>(v.clone()).ok().and_then(|s| QuickFix::decode(&s))
            }
        });

        let Some(fix) = fix else { continue };

        if let Some(action) = quick_fix_to_action(&fix, &path, &content, &namespaces) {
            actions.push(CodeActionOrCommand::CodeAction(action));
        }
    }

    if actions.is_empty() {
        None
    } else {
        Some(actions)
    }
}

fn namespaces_for_path(state: &ServerState, path: &PathBuf) -> BTreeMap<String, String> {
    state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| d.path == *path)
                .map(|d| d.namespaces.clone())
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

fn quick_fix_to_action(
    fix: &QuickFix,
    path: &Path,
    content: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<CodeAction> {
    match fix {
        QuickFix::InsertText { label, line, column, text } => {
            let line_idx = line.saturating_sub(1) as u32;
            let line_str = content.lines().nth(line_idx as usize)?;
            let character = crate::positions::byte_col_to_utf16(line_str, column.saturating_sub(1));
            let edit = TextEdit {
                range: Range {
                    start: lsp_types::Position { line: line_idx, character },
                    end: lsp_types::Position { line: line_idx, character },
                },
                new_text: text.clone(),
            };
            Some(code_action_with_edit(label, path, vec![edit]))
        }
        QuickFix::RemoveLine { label, line } => {
            let line_idx = line.saturating_sub(1) as u32;
            let mut lines: Vec<&str> = content.lines().collect();
            if line_idx as usize >= lines.len() {
                return None;
            }
            lines.remove(line_idx as usize);
            let new_text = if lines.is_empty() { String::new() } else { lines.join("\n") + "\n" };
            Some(code_action_with_edit(label, path, vec![full_document_edit(content, &new_text)]))
        }
        QuickFix::ApplyPatch { label, document_path, patches } => {
            let patch_ops: Vec<PatchOp> =
                patches.iter().filter_map(|v| serde_json::from_value(v.clone()).ok()).collect();
            if patch_ops.is_empty() {
                return None;
            }
            let doc_path = PathBuf::from(document_path);
            let result = apply_patches_to_text(content, &patch_ops, true, namespaces).ok()?;
            let new_text = result.preview_text?;
            Some(code_action_with_edit(
                label,
                &doc_path,
                vec![full_document_edit(content, &new_text)],
            ))
        }
    }
}

fn full_document_edit(old: &str, new_text: &str) -> TextEdit {
    let end_line = old.lines().count().saturating_sub(1) as u32;
    let last_len =
        old.lines().last().map(|l| crate::positions::byte_col_to_utf16(l, l.len())).unwrap_or(0);
    TextEdit {
        range: Range {
            start: lsp_types::Position { line: 0, character: 0 },
            end: lsp_types::Position { line: end_line, character: last_len },
        },
        new_text: new_text.to_string(),
    }
}

fn code_action_with_edit(label: &str, path: &Path, edits: Vec<TextEdit>) -> CodeAction {
    CodeAction {
        title: label.to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            changes: Some(std::collections::HashMap::from([(
                Uri::from_str(&format!("file://{}", path.display()))
                    .unwrap_or_else(|_| Uri::from_str("file:///").expect("file uri")),
                edits,
            )])),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontocore_core::QuickFix;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn insert_text_quick_fix_becomes_code_action() {
        let path = PathBuf::from("/tmp/live.ttl");
        let content = "ex:Bad a owl:Class .\n";
        let fix = QuickFix::InsertText {
            label: "Declare @prefix ex:".to_string(),
            line: 1,
            column: 1,
            text: "@prefix ex: <http://example.org/ex#> .\n".to_string(),
        };
        let action = quick_fix_to_action(&fix, &path, content, &BTreeMap::new()).expect("action");
        assert_eq!(action.title, "Declare @prefix ex:");
        assert!(action.edit.is_some());
    }
}
