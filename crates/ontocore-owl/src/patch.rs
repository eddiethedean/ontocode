use crate::error::{OwlError, Result};
use crate::manchester::{class_expression_to_turtle_fragment, parse_class_expression};
use crate::span::{
    all_entity_statement_ranges, entity_primary_block_range, short_name_from_iri, ByteRange,
};
use ontocore_core::{read_to_string_capped, OntologyFormat, MAX_FILE_BYTES};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single authoring patch operation (v0.4 Turtle scope).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOp {
    CreateEntity { entity_iri: String, kind: PatchEntityKind },
    DeleteEntity { entity_iri: String },
    SetLabel { entity_iri: String, value: String },
    AddLabel { entity_iri: String, value: String },
    RemoveLabel { entity_iri: String, value: String },
    SetComment { entity_iri: String, value: String },
    AddComment { entity_iri: String, value: String },
    RemoveComment { entity_iri: String, value: String },
    AddSubClassOf { entity_iri: String, parent_iri: String },
    RemoveSubClassOf { entity_iri: String, parent_iri: String },
    AddComplexSubClassOf { entity_iri: String, manchester: String },
    RemoveComplexSubClassOf { entity_iri: String, manchester: String },
    AddEquivalentClass { entity_iri: String, manchester: String },
    RemoveEquivalentClass { entity_iri: String, manchester: String },
    SetEquivalentClass { entity_iri: String, manchester: String },
    SetDeprecated { entity_iri: String, value: bool },
    AddDisjointClass { entity_iri: String, other_iri: String },
    RemoveDisjointClass { entity_iri: String, other_iri: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatchEntityKind {
    Class,
    ObjectProperty,
    DataProperty,
    AnnotationProperty,
    Individual,
}

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

/// Apply patches to a document on disk (Turtle only).
pub fn apply_patches(
    document_path: &Path,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    let format = OntologyFormat::from_extension(
        document_path.extension().and_then(|e| e.to_str()).unwrap_or(""),
    );
    if format != OntologyFormat::Turtle {
        return Err(OwlError::UnsupportedFormat(format!(
            "write-back supports Turtle (.ttl) only, got {}",
            format.as_str()
        )));
    }

    let source = read_to_string_capped(document_path, MAX_FILE_BYTES).map_err(OwlError::Core)?;
    let mut result = apply_patches_to_text(&source, patches, preview_only, namespaces)?;
    result.document_path = Some(document_path.display().to_string());

    if result.applied && !preview_only {
        if let Some(text) = &result.preview_text {
            atomic_write(document_path, text)?;
        }
    }
    Ok(result)
}

pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let parent =
        path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let tmp_path = parent.join(format!(
        ".ontocode-{}-{}.tmp",
        path.file_name().and_then(|s| s.to_str()).unwrap_or("file"),
        nanos
    ));
    {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(&tmp_path, path)?;
    Ok(())
}

/// Apply patches to in-memory Turtle text.
pub fn apply_patches_to_text(
    source: &str,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    let mut working = source.to_string();
    let mut diagnostics = Vec::new();

    for patch in patches {
        match apply_one_patch(&mut working, patch, namespaces) {
            Ok(()) => {}
            Err(e) => {
                diagnostics.push(PatchDiagnostic {
                    severity: "error".to_string(),
                    message: e.to_string(),
                });
                return Ok(ApplyPatchResult {
                    applied: false,
                    preview_text: Some(source.to_string()),
                    diagnostics,
                    document_path: None,
                });
            }
        }
    }

    let changed = working != source;
    Ok(ApplyPatchResult {
        applied: changed && !preview_only,
        preview_text: if changed { Some(working) } else { None },
        diagnostics,
        document_path: None,
    })
}

fn apply_one_patch(
    text: &mut String,
    patch: &PatchOp,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    match patch {
        PatchOp::CreateEntity { entity_iri, kind } => {
            create_entity(text, entity_iri, *kind, namespaces)
        }
        PatchOp::DeleteEntity { entity_iri } => delete_entity(text, entity_iri, namespaces),
        PatchOp::SetLabel { entity_iri, value } => {
            remove_all_predicate_any_statement(text, entity_iri, "rdfs:label", namespaces)?;
            add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::AddLabel { entity_iri, value } => {
            add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::RemoveLabel { entity_iri, value } => {
            remove_matching_predicate_any(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::SetComment { entity_iri, value } => {
            remove_all_predicate_any_statement(text, entity_iri, "rdfs:comment", namespaces)?;
            add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::AddComment { entity_iri, value } => {
            add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::RemoveComment { entity_iri, value } => {
            remove_matching_predicate_any(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::AddSubClassOf { entity_iri, parent_iri } => {
            add_subclass_triple(text, entity_iri, parent_iri, namespaces)
        }
        PatchOp::RemoveSubClassOf { entity_iri, parent_iri } => {
            remove_subclass_triple(text, entity_iri, parent_iri, namespaces)
        }
        PatchOp::AddComplexSubClassOf { entity_iri, manchester } => {
            add_complex_axiom(text, entity_iri, manchester, "rdfs:subClassOf", namespaces)
        }
        PatchOp::RemoveComplexSubClassOf { entity_iri, manchester } => {
            remove_complex_axiom(text, entity_iri, manchester, "rdfs:subClassOf", namespaces)
        }
        PatchOp::AddEquivalentClass { entity_iri, manchester } => {
            add_complex_axiom(text, entity_iri, manchester, "owl:equivalentClass", namespaces)
        }
        PatchOp::RemoveEquivalentClass { entity_iri, manchester } => {
            remove_complex_axiom(text, entity_iri, manchester, "owl:equivalentClass", namespaces)
        }
        PatchOp::SetEquivalentClass { entity_iri, manchester } => {
            remove_predicate_triples(text, entity_iri, "owl:equivalentClass", namespaces)?;
            add_complex_axiom(text, entity_iri, manchester, "owl:equivalentClass", namespaces)
        }
        PatchOp::SetDeprecated { entity_iri, value } => {
            if *value {
                add_object_triple(text, entity_iri, "owl:deprecated", "true", namespaces)
            } else {
                remove_predicate_triples(text, entity_iri, "owl:deprecated", namespaces)
            }
        }
        PatchOp::AddDisjointClass { entity_iri, other_iri } => {
            let other = iri_to_turtle_term(other_iri, namespaces);
            add_object_triple(text, entity_iri, "owl:disjointWith", &other, namespaces)
        }
        PatchOp::RemoveDisjointClass { entity_iri, other_iri } => {
            remove_disjoint_triple(text, entity_iri, other_iri, namespaces)
        }
    }
}

fn create_entity(
    text: &mut String,
    entity_iri: &str,
    kind: PatchEntityKind,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityExists(entity_iri.to_string()));
    }
    let subject = iri_to_turtle_term(entity_iri, namespaces);
    let type_term = match kind {
        PatchEntityKind::Class => "owl:Class",
        PatchEntityKind::ObjectProperty => "owl:ObjectProperty",
        PatchEntityKind::DataProperty => "owl:DatatypeProperty",
        PatchEntityKind::AnnotationProperty => "owl:AnnotationProperty",
        PatchEntityKind::Individual => "owl:NamedIndividual",
    };
    let block = format!("\n{subject} a {type_term} .\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&block);
    Ok(())
}

fn delete_entity(
    text: &mut String,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let namespaces = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    let mut ranges = all_entity_statement_ranges(text, entity_iri, &short, &namespaces);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    ranges.sort_by_key(|r| r.start);
    for range in ranges.into_iter().rev() {
        replace_range(text, range, "");
    }
    Ok(())
}

fn add_annotation_triple(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let escaped = escape_turtle_string(value);
    let triple = format!("    {predicate} \"{escaped}\" ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces)
}

fn add_object_triple(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let triple = format!("    {predicate} {object} ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces)
}

fn add_complex_axiom(
    text: &mut String,
    entity_iri: &str,
    manchester: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let parsed = parse_class_expression(manchester, namespaces)?;
    let triple = class_expression_to_turtle_fragment(&parsed.expression, predicate, namespaces)?;
    insert_multiline_into_entity_block(text, entity_iri, &triple, namespaces)
}

fn remove_complex_axiom(
    text: &mut String,
    entity_iri: &str,
    manchester: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let parsed = parse_class_expression(manchester, namespaces)?;
    let object_value =
        crate::manchester::class_expression_to_turtle_value(&parsed.expression, namespaces, 0)?;
    remove_predicate_object(text, entity_iri, predicate, &object_value, namespaces)
}

fn insert_multiline_into_entity_block(
    text: &mut String,
    entity_iri: &str,
    insertion: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let range = entity_primary_range(text, entity_iri, namespaces)?;
    let block = &text[range.start as usize..range.end as usize];
    let trimmed = block.trim_end();
    let mut new_block = block.to_string();
    if trimmed.ends_with('.') {
        if let Some(pos) = new_block.trim_end().rfind('.') {
            let base = new_block[..pos].trim_end();
            new_block = format!("{base} ;\n{insertion}.");
        }
    } else if !trimmed.ends_with(';') {
        new_block.push_str(" ;\n");
        new_block.push_str(insertion);
    } else {
        new_block.push_str(insertion);
    }
    replace_range(text, range, &new_block);
    Ok(())
}

fn add_subclass_triple(
    text: &mut String,
    entity_iri: &str,
    parent_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let parent = iri_to_turtle_term(parent_iri, namespaces);
    add_object_triple(text, entity_iri, "rdfs:subClassOf", &parent, namespaces)
}

fn remove_subclass_triple(
    text: &mut String,
    entity_iri: &str,
    parent_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let parent = iri_to_turtle_term(parent_iri, namespaces);
    remove_predicate_object_any_statement(text, entity_iri, "rdfs:subClassOf", &parent, namespaces)
}

fn remove_disjoint_triple(
    text: &mut String,
    entity_iri: &str,
    other_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let other = iri_to_turtle_term(other_iri, namespaces);
    remove_predicate_object_any_statement(text, entity_iri, "owl:disjointWith", &other, namespaces)
}

fn remove_predicate_triples(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let range = entity_primary_range(text, entity_iri, namespaces)?;
    let block = &text[range.start as usize..range.end as usize];
    let new_block = remove_all_predicate_objects(block, predicate);
    replace_range(text, range, &new_block);
    Ok(())
}

fn remove_matching_predicate_any(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let escaped = escape_turtle_string(value.trim_matches('"'));
    let object = format!("\"{escaped}\"");
    remove_predicate_object_any_statement(text, entity_iri, predicate, &object, namespaces)
}

fn remove_all_predicate_any_statement(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    loop {
        let ns = crate::span::namespaces_for_text(text, namespaces);
        let short = short_name_from_iri(entity_iri);
        let ranges = all_entity_statement_ranges(text, entity_iri, &short, &ns);
        if ranges.is_empty() {
            return Ok(());
        }
        let mut removed = false;
        for range in ranges {
            let block = &text[range.start as usize..range.end as usize];
            let new_block = remove_all_predicate_objects(block, predicate);
            if new_block != block {
                replace_range(text, range, &new_block);
                removed = true;
                break;
            }
        }
        if !removed {
            return Ok(());
        }
    }
}

fn insert_into_entity_block(
    text: &mut String,
    entity_iri: &str,
    insertion: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let range = entity_primary_range(text, entity_iri, namespaces)?;
    let block = &text[range.start as usize..range.end as usize];
    let insertion_key = insertion.trim().trim_end_matches(';').trim();
    if block.contains(insertion_key) {
        return Ok(());
    }
    let trimmed = block.trim_end();
    let ends_with_dot = trimmed.ends_with('.');
    let ends_with_semi = trimmed.ends_with(';');
    let mut new_block = block.to_string();
    if ends_with_dot {
        if let Some(pos) = new_block.trim_end().rfind('.') {
            let base = new_block[..pos].trim_end();
            new_block =
                format!("{base} ;\n{insertion}{}.", &new_block[pos..pos + 1].replace('.', ""));
        }
    } else if !ends_with_semi {
        new_block = format!("{new_block} ;\n");
    }
    if let Some(pos) = new_block.rfind('.') {
        new_block.insert_str(pos, &format!("\n{insertion}"));
    } else {
        new_block.push_str(insertion);
    }
    replace_range(text, range, &new_block);
    Ok(())
}

fn replace_range(text: &mut String, range: ByteRange, replacement: &str) {
    let start = range.start as usize;
    let end = range.end.min(text.len() as u64) as usize;
    text.replace_range(start..end, replacement);
}

fn escape_turtle_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn entity_primary_range(
    text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<ByteRange> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    entity_primary_block_range(text, entity_iri, &ns)
        .ok_or_else(|| OwlError::EntityNotFound(entity_iri.to_string()))
}

fn remove_predicate_object_any_statement(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    let ranges = all_entity_statement_ranges(text, entity_iri, &short, &ns);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    for range in ranges {
        let block = &text[range.start as usize..range.end as usize];
        if let Some(new_block) = remove_matching_predicate_object(block, predicate, object_value) {
            replace_range(text, range, &new_block);
            return Ok(());
        }
    }
    Err(OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))
}

fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn bracket_end_index(text: &str, bracket_start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(bracket_start) != Some(&b'[') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;
    let mut i = bracket_start;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn extend_removal_span(block: &str, pred_start: usize, obj_end: usize) -> (usize, usize) {
    let mut start = pred_start;
    while start > 0 && block.as_bytes()[start - 1].is_ascii_whitespace() {
        start -= 1;
    }
    if start > 0 && block.as_bytes()[start - 1] == b',' {
        start -= 1;
        while start > 0 && block.as_bytes()[start - 1].is_ascii_whitespace() {
            start -= 1;
        }
    }
    let mut end = obj_end;
    while end < block.len() && block.as_bytes()[end].is_ascii_whitespace() {
        end += 1;
    }
    if end < block.len() && (block.as_bytes()[end] == b',' || block.as_bytes()[end] == b';') {
        end += 1;
    }
    (start, end)
}

fn cleanup_block_separators(block: &str) -> String {
    let mut lines: Vec<&str> = block.lines().collect();
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    lines.join("\n").replace(";\n    ;", ";\n").replace(",\n        ,", ",\n")
}

fn remove_all_predicate_objects(block: &str, predicate: &str) -> String {
    let mut result = block.to_string();
    while let Some(next) = remove_first_predicate_object(&result, predicate) {
        result = next;
    }
    cleanup_block_separators(&result)
}

fn remove_first_predicate_object(block: &str, predicate: &str) -> Option<String> {
    let pred_pos = block.find(predicate)?;
    let (obj_start, obj_end) =
        objects_in_predicate_value(block, pred_pos, predicate).first().copied()?;
    let (remove_start, remove_end) = extend_removal_span(block, obj_start, obj_end);
    let mut out = String::new();
    out.push_str(&block[..remove_start]);
    out.push_str(&block[remove_end..]);
    Some(cleanup_block_separators(&out))
}

fn find_named_object_end(block: &str, obj_start: usize) -> Option<usize> {
    let bytes = block.as_bytes();
    let mut i = obj_start;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b',' || b == b';' || b == b'.' {
            return Some(i);
        }
        i += 1;
    }
    Some(block.len())
}

fn remove_predicate_object(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let range = entity_primary_range(text, entity_iri, namespaces)?;
    let block = &text[range.start as usize..range.end as usize];
    let new_block = remove_matching_predicate_object(block, predicate, object_value)
        .ok_or_else(|| OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))?;
    replace_range(text, range, &new_block);
    Ok(())
}

fn objects_in_predicate_value(
    block: &str,
    pred_pos: usize,
    predicate: &str,
) -> Vec<(usize, usize)> {
    let list_start = pred_pos + predicate.len();
    let mut objects = Vec::new();
    let mut i = list_start;
    loop {
        let rest = block.get(i..).unwrap_or("").trim_start();
        if rest.is_empty() || rest.starts_with(';') || rest.starts_with('.') {
            break;
        }
        i += block[i..].len() - rest.len();
        if block.as_bytes().get(i) == Some(&b'[') {
            if let Some(end) = bracket_end_index(block, i) {
                objects.push((i, end));
                i = end;
            } else {
                break;
            }
        } else {
            let end = find_named_object_end(block, i).unwrap_or(block.len());
            objects.push((i, end));
            i = end;
        }
        let rest = block.get(i..).unwrap_or("").trim_start();
        i += block[i..].len() - rest.len();
        if rest.starts_with(',') {
            i += 1;
        } else {
            break;
        }
    }
    objects
}

fn remove_matching_predicate_object(
    block: &str,
    predicate: &str,
    object_value: &str,
) -> Option<String> {
    let obj_trim = object_value.trim();
    let norm_obj = normalize_ws(obj_trim);
    let mut search_from = 0;
    while let Some(rel) = block[search_from..].find(predicate) {
        let pred_pos = search_from + rel;
        for (obj_start, obj_end) in objects_in_predicate_value(block, pred_pos, predicate) {
            let candidate = normalize_ws(block[obj_start..obj_end].trim());
            if candidate == norm_obj {
                let (remove_start, remove_end) = extend_removal_span(block, obj_start, obj_end);
                let mut out = String::new();
                out.push_str(&block[..remove_start]);
                out.push_str(&block[remove_end..]);
                return Some(cleanup_block_separators(&out));
            }
        }
        search_from = pred_pos + 1;
    }
    None
}

fn text_contains_entity(
    text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    let namespaces = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    let mut needles = vec![entity_iri.to_string(), format!("<{entity_iri}>")];
    for (prefix, ns) in &namespaces {
        if entity_iri.starts_with(ns) {
            needles.push(format!("{prefix}:{short}"));
        }
    }
    text.lines().any(|line| {
        let trimmed = line.trim_start();
        needles.iter().any(|needle| line_starts_with_subject(trimmed, needle))
    })
}

fn line_starts_with_subject(trimmed: &str, subject: &str) -> bool {
    trimmed == subject
        || trimmed.starts_with(&format!("{subject} "))
        || trimmed.starts_with(&format!("{subject}\t"))
        || trimmed.starts_with(&format!("{subject};"))
        || trimmed.starts_with(&format!("{subject}."))
}

fn iri_to_turtle_term(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    for (prefix, ns) in namespaces {
        if !prefix.is_empty() && iri.starts_with(ns) {
            let local = &iri[ns.len()..];
            return format!("{prefix}:{local}");
        }
    }
    format!("<{iri}>")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/people#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    #[test]
    fn add_label_to_existing_class() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Human".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("Human"));
    }

    #[test]
    fn create_new_class() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::CreateEntity {
            entity_iri: "http://example.org/people#Employee".to_string(),
            kind: PatchEntityKind::Class,
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(result.preview_text.unwrap().contains("ex:Employee"));
    }

    #[test]
    fn batch_failure_leaves_source_unchanged() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![
            PatchOp::AddLabel {
                entity_iri: "http://example.org/people#Person".to_string(),
                value: "Human".to_string(),
            },
            PatchOp::AddLabel {
                entity_iri: "http://example.org/people#NoSuch".to_string(),
                value: "X".to_string(),
            },
        ];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
        assert!(!result.applied);
    }

    fn clinic_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/clinic#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    #[test]
    fn remove_complex_subclass_keeps_named_parent() {
        let ttl = include_str!("../../../fixtures/complex-classes.ttl");
        let patches = vec![PatchOp::RemoveComplexSubClassOf {
            entity_iri: "http://example.org/clinic#Patient".to_string(),
            manchester: "ex:hasRecord some ex:MedicalRecord".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &clinic_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("owl:someValuesFrom"));
        assert!(preview.contains("rdfs:subClassOf ex:ClinicPerson"));
    }

    #[test]
    fn remove_subclass_from_trailing_triple() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::RemoveSubClassOf {
            entity_iri: "http://example.org/people#Person".to_string(),
            parent_iri: "http://example.org/people#Thing".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("ex:Person rdfs:subClassOf ex:Thing"));
        assert!(preview.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn delete_entity_removes_all_statements() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::DeleteEntity {
            entity_iri: "http://example.org/people#Person".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("ex:Person a owl:Class"));
        assert!(!preview.contains("ex:Person rdfs:subClassOf"));
    }

    #[test]
    fn crlf_line_offsets_match_byte_positions() {
        let ttl = "ex:Foo a owl:Class ;\r\n    rdfs:label \"Bar\" .\r\n";
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let range = entity_primary_block_range(ttl, "http://example.org/Foo", &ns).expect("range");
        let block = &ttl[range.start as usize..range.end as usize];
        assert!(block.contains("rdfs:label"));
        assert!(block.trim_end().ends_with('.'));
    }

    #[test]
    fn cleanup_preserves_literal_double_spaces() {
        let block = "ex:Foo rdfs:label \"a  b\" .";
        let cleaned = cleanup_block_separators(block);
        assert!(cleaned.contains("\"a  b\""));
    }

    #[test]
    fn add_disjoint_class_is_idempotent_when_axiom_exists() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/org#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ]);
        let patches = vec![PatchOp::AddDisjointClass {
            entity_iri: "http://example.org/org#Cat".to_string(),
            other_iri: "http://example.org/org#Dog".to_string(),
        }];
        let before = ttl.matches("owl:disjointWith").count();
        let result = apply_patches_to_text(ttl, &patches, true, &ns).expect("patch");
        let preview = result.preview_text.as_deref().unwrap_or(ttl);
        assert_eq!(before, preview.matches("owl:disjointWith").count());
    }
}
