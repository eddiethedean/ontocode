use crate::error::{OboError, Result};
use crate::patch::OboPatchOp;
use ontocore_core::{read_to_string_capped, MAX_FILE_BYTES};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDiagnostic {
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyPatchResult {
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<PatchDiagnostic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_path: Option<String>,
}

pub fn apply_patches(
    document_path: &Path,
    patches: &[OboPatchOp],
    preview_only: bool,
) -> Result<ApplyPatchResult> {
    let source = read_to_string_capped(document_path, MAX_FILE_BYTES).map_err(OboError::Core)?;
    let mut result = apply_patches_to_text(&source, patches, preview_only)?;
    result.document_path = Some(document_path.display().to_string());
    if result.applied && !preview_only {
        if let Some(text) = &result.preview_text {
            atomic_write(document_path, text)?;
        }
    }
    Ok(result)
}

pub fn apply_patches_to_text(
    source: &str,
    patches: &[OboPatchOp],
    preview_only: bool,
) -> Result<ApplyPatchResult> {
    let mut working = source.to_string();
    let mut diagnostics = Vec::new();
    for patch in patches {
        if let Err(e) = apply_one(&mut working, patch) {
            diagnostics
                .push(PatchDiagnostic { severity: "error".to_string(), message: e.to_string() });
            return Ok(ApplyPatchResult {
                applied: false,
                preview_text: Some(source.to_string()),
                diagnostics,
                document_path: None,
            });
        }
    }
    validate_obo(&working)?;
    let changed = working != source;
    Ok(ApplyPatchResult {
        applied: changed && !preview_only,
        preview_text: if changed { Some(working) } else { None },
        diagnostics,
        document_path: None,
    })
}

fn validate_obo(text: &str) -> Result<()> {
    fastobo::from_str(text).map_err(|e| OboError::Parse(e.to_string()))?;
    Ok(())
}

fn apply_one(text: &mut String, patch: &OboPatchOp) -> Result<()> {
    match patch {
        OboPatchOp::SetName { term_id, value } => {
            set_single_line(text, term_id, "name:", value, false)
        }
        OboPatchOp::AddSynonym { term_id, value, scope } => {
            let line = format!("synonym: \"{value}\" {scope} []");
            add_line_in_term(text, term_id, &line)
        }
        OboPatchOp::RemoveSynonym { term_id, value } => remove_synonym_line(text, term_id, value),
        OboPatchOp::AddDef { term_id, value } => {
            let escaped = value.replace('"', "\\\"");
            let line = format!("def: \"{escaped}\" []");
            set_or_add_line(text, term_id, "def:", &line)
        }
        OboPatchOp::RemoveDef { term_id } => remove_lines_with_prefix(text, term_id, "def:"),
        OboPatchOp::AddXref { term_id, xref } => {
            let line = format!("xref: {xref}");
            add_line_in_term(text, term_id, &line)
        }
        OboPatchOp::RemoveXref { term_id, xref } => remove_xref_line(text, term_id, xref),
        OboPatchOp::SetNamespace { term_id, namespace } => {
            set_single_line(text, term_id, "namespace:", namespace, false)
        }
        OboPatchOp::SetDeprecated { term_id, value } => {
            let val = if *value { "true" } else { "false" };
            set_single_line(text, term_id, "is_obsolete:", val, false)
        }
        OboPatchOp::AddIsA { term_id, parent_id } => {
            let line = format!("is_a: {parent_id} ! {parent_id}");
            add_line_in_term(text, term_id, &line)
        }
        OboPatchOp::RemoveIsA { term_id, parent_id } => remove_is_a_line(text, term_id, parent_id),
    }
}

fn term_block_range(text: &str, term_id: &str) -> Result<(usize, usize)> {
    let id_line = format!("id: {term_id}");
    let start = text.find(&id_line).ok_or_else(|| OboError::TermNotFound(term_id.to_string()))?;
    let block_start = text[..start].rfind("[Term]").unwrap_or(start);
    let rest = &text[start..];
    let next_term = rest[1..].find("\n[Term]").map(|i| start + 1 + i).unwrap_or(text.len());
    Ok((block_start, next_term))
}

fn set_single_line(
    text: &mut String,
    term_id: &str,
    prefix: &str,
    value: &str,
    _allow_multiple: bool,
) -> Result<()> {
    let line = format!("{prefix} {value}");
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    if let Some(idx) = block.lines().position(|l| l.trim_start().starts_with(prefix)) {
        let lines: Vec<&str> = block.lines().collect();
        let mut out = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == idx {
                out.push_str(&line);
            } else {
                out.push_str(l);
            }
            out.push('\n');
        }
        text.replace_range(start..end, &out);
        Ok(())
    } else {
        add_line_in_term(text, term_id, &line)
    }
}

fn set_or_add_line(text: &mut String, term_id: &str, prefix: &str, full_line: &str) -> Result<()> {
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    if block.lines().any(|l| l.trim_start().starts_with(prefix)) {
        remove_lines_with_prefix(text, term_id, prefix)?;
    }
    add_line_in_term(text, term_id, full_line)
}

fn add_line_in_term(text: &mut String, term_id: &str, line: &str) -> Result<()> {
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    let mut new_block = block.trim_end().to_string();
    if !new_block.ends_with('\n') {
        new_block.push('\n');
    }
    new_block.push_str(line);
    new_block.push('\n');
    text.replace_range(start..end, &new_block);
    Ok(())
}

fn remove_lines_where(
    text: &mut String,
    term_id: &str,
    should_remove: impl Fn(&str) -> bool,
    not_found: Option<&str>,
) -> Result<()> {
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    let lines: Vec<&str> = block.lines().collect();
    let removed_any = lines.iter().any(|l| should_remove(l));
    if let Some(message) = not_found {
        if !removed_any {
            return Err(OboError::PatchInvalid(message.to_string()));
        }
    }
    let filtered: String =
        lines.into_iter().filter(|l| !should_remove(l)).collect::<Vec<_>>().join("\n");
    let mut new_block = filtered;
    if !new_block.ends_with('\n') {
        new_block.push('\n');
    }
    text.replace_range(start..end, &new_block);
    Ok(())
}

fn remove_lines_with_prefix(text: &mut String, term_id: &str, prefix: &str) -> Result<()> {
    remove_lines_where(text, term_id, |l| l.trim_start().starts_with(prefix), None)
}

fn remove_is_a_line(text: &mut String, term_id: &str, parent_id: &str) -> Result<()> {
    let parent_id = parent_id.to_string();
    let not_found = format!("is_a parent not found: {parent_id}");
    remove_lines_where(
        text,
        term_id,
        move |l| is_is_a_parent_line(l, &parent_id),
        Some(&not_found),
    )
}

fn remove_xref_line(text: &mut String, term_id: &str, xref: &str) -> Result<()> {
    let xref = xref.to_string();
    let not_found = format!("xref not found: {xref}");
    remove_lines_where(
        text,
        term_id,
        move |l| is_xref_line(l, &xref),
        Some(&not_found),
    )
}

fn remove_synonym_line(text: &mut String, term_id: &str, value: &str) -> Result<()> {
    let value = value.to_string();
    let not_found = format!("synonym not found: {value}");
    remove_lines_where(
        text,
        term_id,
        move |l| is_synonym_value_line(l, &value),
        Some(&not_found),
    )
}

fn obo_field_token(rest: &str) -> Option<&str> {
    let end = rest.find(|c: char| c.is_whitespace() || c == '!' || c == '{').unwrap_or(rest.len());
    let token = rest[..end].trim();
    if token.is_empty() { None } else { Some(token) }
}

fn is_is_a_parent_line(line: &str, parent_id: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("is_a:") {
        return false;
    }
    obo_field_token(trimmed["is_a:".len()..].trim_start()) == Some(parent_id)
}

fn is_xref_line(line: &str, xref: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("xref:") {
        return false;
    }
    obo_field_token(trimmed["xref:".len()..].trim_start()) == Some(xref)
}

fn is_synonym_value_line(line: &str, value: &str) -> bool {
    parse_quoted_value_after_prefix(line, "synonym:").as_deref() == Some(value)
}

fn parse_quoted_value_after_prefix(line: &str, prefix: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with(prefix) {
        return None;
    }
    let mut rest = trimmed[prefix.len()..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    rest = &rest[1..];
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            out.push(chars.next()?);
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    None
}

pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let parent =
        path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|e| OboError::Io(e.to_string()))?;
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let stem = path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
    let tmp_path = parent.join(format!(".ontocode-{stem}-{nanos}.tmp"));
    {
        let mut file = fs::File::create(&tmp_path).map_err(|e| OboError::Io(e.to_string()))?;
        file.write_all(contents.as_bytes()).map_err(|e| OboError::Io(e.to_string()))?;
        file.sync_all().map_err(|e| OboError::Io(e.to_string()))?;
    }
    fs::rename(&tmp_path, path).map_err(|e| OboError::Io(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: example
is_a: EX:000 ! root

[Term]
id: EX:002
name: other
"#;

    #[test]
    fn set_name_and_add_synonym() {
        let result = apply_patches_to_text(
            SAMPLE,
            &[
                OboPatchOp::SetName { term_id: "EX:001".into(), value: "renamed".into() },
                OboPatchOp::AddSynonym {
                    term_id: "EX:001".into(),
                    value: "alias".into(),
                    scope: "EXACT".into(),
                },
            ],
            true,
        )
        .expect("patch");
        let text = result.preview_text.expect("preview");
        assert!(text.contains("name: renamed"));
        assert!(text.contains("synonym: \"alias\" EXACT"));
    }

    #[test]
    fn add_and_remove_is_a() {
        let added = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::AddIsA { term_id: "EX:002".into(), parent_id: "EX:001".into() }],
            true,
        )
        .expect("add");
        let with_parent = added.preview_text.expect("preview");
        assert!(with_parent.contains("is_a: EX:001"));

        let removed = apply_patches_to_text(
            &with_parent,
            &[OboPatchOp::RemoveIsA { term_id: "EX:002".into(), parent_id: "EX:001".into() }],
            true,
        )
        .expect("remove");
        let text = removed.preview_text.expect("preview");
        let (start, end) = term_block_range(&text, "EX:002").expect("term block");
        assert!(
            !text[start..end].lines().any(|l| is_is_a_parent_line(l, "EX:001")),
            "EX:001 parent must be removed from EX:002"
        );
    }

    #[test]
    fn remove_is_a_does_not_match_prefix_collision_parent_ids() {
        const COLLISION: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:002
name: child
is_a: EX:0010 ! longer parent
is_a: EX:001 ! shorter parent
"#;

        let result = apply_patches_to_text(
            COLLISION,
            &[OboPatchOp::RemoveIsA { term_id: "EX:002".into(), parent_id: "EX:001".into() }],
            true,
        )
        .expect("remove shorter parent only");

        let text = result.preview_text.expect("preview");
        assert!(
            text.lines().any(|l| l.contains("is_a: EX:0010")),
            "EX:0010 parent must remain when removing EX:001"
        );
        assert!(
            !text.lines().any(|l| is_is_a_parent_line(l, "EX:001")),
            "EX:001 parent must be removed"
        );
    }

    #[test]
    fn remove_is_a_errors_when_parent_missing() {
        let err = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::RemoveIsA { term_id: "EX:001".into(), parent_id: "EX:999".into() }],
            true,
        )
        .expect("patch result");
        assert!(!err.diagnostics.is_empty());
        assert!(err.diagnostics[0].message.contains("is_a parent not found"));
    }
}
