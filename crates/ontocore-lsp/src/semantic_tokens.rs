//! Lightweight semantic tokenization for Turtle and OBO sources.

use lsp_types::{
    SemanticToken, SemanticTokens, SemanticTokensLegend, SemanticTokensParams, SemanticTokensResult,
};
use serde_json::Value;

const TOKEN_NAMESPACE: u32 = 0;
const TOKEN_IRI: u32 = 1;
const TOKEN_KEYWORD: u32 = 2;
const TOKEN_COMMENT: u32 = 3;
const TOKEN_STRING: u32 = 4;

pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            "namespace".into(),
            "iri".into(),
            "keyword".into(),
            "comment".into(),
            "string".into(),
        ],
        token_modifiers: vec![],
    }
}

pub fn handle_semantic_tokens_full(
    params: SemanticTokensParams,
    doc_text: Option<String>,
) -> Option<SemanticTokensResult> {
    let uri = params.text_document.uri.as_str();
    let text = doc_text?;
    let is_obo = uri.ends_with(".obo");
    let tokens = if is_obo { tokenize_obo(&text) } else { tokenize_turtle(&text) };
    Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data: tokens }))
}

pub fn handle_semantic_tokens_full_from_value(
    params: Value,
    doc_text: Option<String>,
) -> Option<Value> {
    let params: SemanticTokensParams = serde_json::from_value(params).ok()?;
    handle_semantic_tokens_full(params, doc_text)
        .and_then(|r| serde_json::to_value(r).ok())
}

fn tokenize_turtle(text: &str) -> Vec<SemanticToken> {
    let mut out = Vec::new();
    let mut line = 0u32;
    let mut col = 0u32;
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            line += 1;
            col = 0;
            i += 1;
            continue;
        }
        if b == b'#' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            push_token(&mut out, line, col, (i - start) as u32, TOKEN_COMMENT);
            col += (i - start) as u32;
            continue;
        }
        if b == b'"' || b == b'\'' {
            let len = scan_string(&bytes[i..]);
            push_token(&mut out, line, col, len as u32, TOKEN_STRING);
            i += len;
            col += len as u32;
            continue;
        }
        if b == b'<' {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i] != b'>' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            push_token(&mut out, line, col, (i - start) as u32, TOKEN_IRI);
            col += (i - start) as u32;
            continue;
        }
        if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let word = &text[start..i];
            let kind = if TURTLE_KEYWORDS.contains(&word) {
                TOKEN_KEYWORD
            } else if word.contains(':') {
                TOKEN_NAMESPACE
            } else {
                TOKEN_KEYWORD
            };
            push_token(&mut out, line, col, (i - start) as u32, kind);
            col += (i - start) as u32;
            continue;
        }
        i += 1;
        col += 1;
    }
    out
}

fn tokenize_obo(text: &str) -> Vec<SemanticToken> {
    let mut out = Vec::new();
    let mut line = 0u32;
    let mut col = 0u32;
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            line += 1;
            col = 0;
            i += 1;
            continue;
        }
        if b == b'!' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            push_token(&mut out, line, col, (i - start) as u32, TOKEN_COMMENT);
            col += (i - start) as u32;
            continue;
        }
        if b == b'"' {
            let len = scan_string(&bytes[i..]);
            push_token(&mut out, line, col, len as u32, TOKEN_STRING);
            i += len;
            col += len as u32;
            continue;
        }
        if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) && bytes[i] != b':' {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b':' {
                i += 1;
                while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'\t' && bytes[i] != b'\n' {
                    i += 1;
                }
                push_token(&mut out, line, col, (i - start) as u32, TOKEN_KEYWORD);
            } else {
                push_token(&mut out, line, col, (i - start) as u32, TOKEN_NAMESPACE);
            }
            col += (i - start) as u32;
            continue;
        }
        i += 1;
        col += 1;
    }
    out
}

const TURTLE_KEYWORDS: &[&str] = &["@prefix", "@base", "a", "true", "false", "PREFIX", "BASE"];

fn scan_string(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        return 0;
    }
    let quote = bytes[0];
    let mut i = 1;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == quote {
            i += 1;
            break;
        }
        i += 1;
    }
    i
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'@'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b':'
}

fn push_token(out: &mut Vec<SemanticToken>, line: u32, start_char: u32, length: u32, kind: u32) {
    if length == 0 {
        return;
    }
    let delta_line = if out.is_empty() {
        line
    } else {
        let prev = out.last().unwrap();
        line.saturating_sub(prev.delta_line)
    };
    let delta_start = if out.is_empty() || delta_line > 0 {
        start_char
    } else {
        start_char.saturating_sub(out.last().unwrap().delta_start)
    };
    out.push(SemanticToken {
        delta_line,
        delta_start,
        length,
        token_type: kind,
        token_modifiers_bitset: 0,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turtle_tokens_include_comment_and_prefix() {
        let text = "@prefix ex: <http://ex/> .\n# comment\nex:s a ex:C .";
        let tokens = tokenize_turtle(text);
        assert!(!tokens.is_empty());
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_COMMENT));
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_NAMESPACE));
    }
}
