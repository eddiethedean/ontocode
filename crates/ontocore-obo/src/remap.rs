//! Rewrite OBO term/typedef identifiers across a document (v0.24 multi-format refactor).

use crate::error::{OboError, Result};
use fastobo;

/// Remap `from_id` → `to_id` in an OBO document (id lines and common reference tags).
///
/// Handles `id:`, `is_a:`, `intersection_of:`, `union_of:`, `disjoint_from:`,
/// `relationship:` (second token), and `replaced_by:` / `consider:` fields.
/// Validates the result with fastobo.
pub fn remap_obo_id_in_text(text: &str, from_id: &str, to_id: &str) -> Result<String> {
    if from_id == to_id {
        return Ok(text.to_string());
    }
    if from_id.is_empty() || to_id.is_empty() {
        return Err(OboError::PatchInvalid("empty OBO id".into()));
    }
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        out.push_str(&remap_obo_line(line, from_id, to_id));
        out.push('\n');
    }
    // Preserve final newline absence when source had none.
    if !text.ends_with('\n') && out.ends_with('\n') {
        out.pop();
    }
    fastobo::from_str(&out).map_err(|e| OboError::Parse(e.to_string()))?;
    Ok(out)
}

fn remap_obo_line(line: &str, from: &str, to: &str) -> String {
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];

    let Some((tag, rest)) = trimmed.split_once(':') else {
        return line.to_string();
    };
    let tag_l = tag.trim().to_ascii_lowercase();
    let rest = rest.trim_start();

    let rewritten = match tag_l.as_str() {
        "id" | "is_a" | "disjoint_from" | "replaced_by" | "consider" | "union_of" => {
            remap_first_id_token(rest, from, to)
        }
        "intersection_of" => remap_intersection_of(rest, from, to),
        "relationship" => remap_relationship(rest, from, to),
        _ => None,
    };

    match rewritten {
        Some(new_rest) => format!("{indent}{tag}: {new_rest}"),
        None => line.to_string(),
    }
}

fn remap_first_id_token(rest: &str, from: &str, to: &str) -> Option<String> {
    let (token, rem) = split_obo_token(rest)?;
    if token != from {
        return None;
    }
    Some(if rem.is_empty() { to.to_string() } else { format!("{to}{rem}") })
}

fn remap_intersection_of(rest: &str, from: &str, to: &str) -> Option<String> {
    // Forms: "ID" or "REL ID" optionally followed by " ! comment"
    let (first, after_first) = split_obo_token(rest)?;
    if first == from {
        return Some(if after_first.is_empty() {
            to.to_string()
        } else {
            format!("{to}{after_first}")
        });
    }
    let trimmed = after_first.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('!') {
        return None;
    }
    let (second, after_second) = split_obo_token(trimmed)?;
    if second != from {
        return None;
    }
    Some(format!(
        "{first} {to}{}",
        if after_second.is_empty() { String::new() } else { after_second.to_string() }
    ))
}

fn remap_relationship(rest: &str, from: &str, to: &str) -> Option<String> {
    // relationship: REL ID ! comment
    let (rel, after_rel) = split_obo_token(rest)?;
    let trimmed = after_rel.trim_start();
    let (id, after_id) = split_obo_token(trimmed)?;
    // Remap either relation type or target id when they match `from`.
    let new_rel = if rel == from { to } else { rel.as_str() };
    let new_id = if id == from { to } else { id.as_str() };
    if new_rel == rel && new_id == id {
        return None;
    }
    Some(format!(
        "{new_rel} {new_id}{}",
        if after_id.is_empty() { String::new() } else { after_id.to_string() }
    ))
}

fn split_obo_token(s: &str) -> Option<(String, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    let end = s.find(|c: char| c.is_whitespace() || c == '!').unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    Some((s[..end].to_string(), &s[end..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaps_id_and_is_a() {
        let src = "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: Person
is_a: EX:000 ! root

[Term]
id: EX:002
name: Child
is_a: EX:001 ! Person
";
        let out = remap_obo_id_in_text(src, "EX:001", "EX:010").expect("remap");
        assert!(out.contains("id: EX:010"));
        assert!(out.contains("is_a: EX:010"));
        assert!(!out.contains("id: EX:001"));
        assert!(out.contains("id: EX:002"));
    }
}
