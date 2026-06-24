use crate::error::{OwlError, Result};
use crate::span::{entity_block_range, short_name_from_iri, ByteRange};
use ontoindex_core::{EntityKind, OntologyFormat, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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
    SetDeprecated { entity_iri: String, value: bool },
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

    let source = fs::read_to_string(document_path)?;
    let mut result = apply_patches_to_text(&source, patches, preview_only, namespaces)?;
    result.document_path = Some(document_path.display().to_string());

    if result.applied && !preview_only {
        if let Some(text) = &result.preview_text {
            fs::write(document_path, text)?;
        }
    }
    Ok(result)
}

/// Apply patches to in-memory Turtle text.
pub fn apply_patches_to_text(
    source: &str,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    let mut text = source.to_string();
    let mut diagnostics = Vec::new();

    for patch in patches {
        match apply_one_patch(&mut text, patch, namespaces) {
            Ok(()) => {}
            Err(e) => {
                diagnostics.push(PatchDiagnostic {
                    severity: "error".to_string(),
                    message: e.to_string(),
                });
            }
        }
    }

    if !diagnostics.is_empty() {
        return Ok(ApplyPatchResult {
            applied: false,
            preview_text: Some(text),
            diagnostics,
            document_path: None,
        });
    }

    let changed = text != source;
    Ok(ApplyPatchResult {
        applied: changed && !preview_only,
        preview_text: if changed { Some(text) } else { None },
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
            remove_predicate_triples(text, entity_iri, "rdfs:label", namespaces)?;
            add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::AddLabel { entity_iri, value } => {
            add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::RemoveLabel { entity_iri, value } => {
            remove_matching_predicate(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::SetComment { entity_iri, value } => {
            remove_predicate_triples(text, entity_iri, "rdfs:comment", namespaces)?;
            add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::AddComment { entity_iri, value } => {
            add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::RemoveComment { entity_iri, value } => {
            remove_matching_predicate(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::AddSubClassOf { entity_iri, parent_iri } => {
            add_subclass_triple(text, entity_iri, parent_iri, namespaces)
        }
        PatchOp::RemoveSubClassOf { entity_iri, parent_iri } => {
            remove_subclass_triple(text, entity_iri, parent_iri, namespaces)
        }
        PatchOp::SetDeprecated { entity_iri, value } => {
            if *value {
                add_object_triple(text, entity_iri, "owl:deprecated", "true", namespaces)
            } else {
                remove_predicate_triples(text, entity_iri, "owl:deprecated", namespaces)
            }
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
    let entity = ontoindex_core::Entity {
        iri: entity_iri.to_string(),
        short_name: short_name_from_iri(entity_iri),
        kind: EntityKind::Class,
        ontology_id: String::new(),
        source_location: find_entity_location(text, entity_iri, namespaces),
        labels: Vec::new(),
        comments: Vec::new(),
        deprecated: false,
    };
    let range = entity_block_range(text, &entity)
        .ok_or_else(|| OwlError::EntityNotFound(entity_iri.to_string()))?;
    replace_range(text, range, "");
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
    let escaped = value.replace('"', "\\\"");
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
    let parent_short = short_name_from_iri(parent_iri);
    remove_line_matching(
        text,
        entity_iri,
        |line| {
            line.contains("subClassOf")
                && (line.contains(parent_iri) || line.contains(&parent_short))
        },
        namespaces,
    )
}

fn remove_predicate_triples(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    remove_line_matching(text, entity_iri, |line| line.contains(predicate), namespaces)
}

fn remove_matching_predicate(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let needle = value.trim_matches('"');
    remove_line_matching(
        text,
        entity_iri,
        |line| line.contains(predicate) && line.contains(needle),
        namespaces,
    )
}

fn remove_line_matching(
    text: &mut String,
    entity_iri: &str,
    pred: impl Fn(&str) -> bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let entity = ontoindex_core::Entity {
        iri: entity_iri.to_string(),
        short_name: short_name_from_iri(entity_iri),
        kind: EntityKind::Class,
        ontology_id: String::new(),
        source_location: find_entity_location(text, entity_iri, namespaces),
        labels: Vec::new(),
        comments: Vec::new(),
        deprecated: false,
    };
    let range = entity_block_range(text, &entity)
        .ok_or_else(|| OwlError::EntityNotFound(entity_iri.to_string()))?;
    let block = &text[range.start as usize..range.end as usize];
    let mut new_block = String::new();
    for line in block.lines() {
        if pred(line) {
            continue;
        }
        new_block.push_str(line);
        new_block.push('\n');
    }
    replace_range(text, range, &new_block);
    Ok(())
}

fn insert_into_entity_block(
    text: &mut String,
    entity_iri: &str,
    insertion: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let entity = ontoindex_core::Entity {
        iri: entity_iri.to_string(),
        short_name: short_name_from_iri(entity_iri),
        kind: EntityKind::Class,
        ontology_id: String::new(),
        source_location: find_entity_location(text, entity_iri, namespaces),
        labels: Vec::new(),
        comments: Vec::new(),
        deprecated: false,
    };
    let range = entity_block_range(text, &entity)
        .ok_or_else(|| OwlError::EntityNotFound(entity_iri.to_string()))?;
    let block = &text[range.start as usize..range.end as usize];
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
    if !new_block.contains(insertion.trim()) {
        if let Some(pos) = new_block.rfind('.') {
            new_block.insert_str(pos, &format!("\n{insertion}"));
        } else {
            new_block.push_str(insertion);
        }
    }
    replace_range(text, range, &new_block);
    Ok(())
}

fn replace_range(text: &mut String, range: ByteRange, replacement: &str) {
    let start = range.start as usize;
    let end = range.end.min(text.len() as u64) as usize;
    text.replace_range(start..end, replacement);
}

fn text_contains_entity(
    text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    let short = short_name_from_iri(entity_iri);
    text.contains(entity_iri)
        || text.contains(&format!("<{entity_iri}>"))
        || namespaces.iter().any(|(prefix, ns)| {
            entity_iri.starts_with(ns) && text.contains(&format!("{prefix}:{short}"))
        })
}

fn find_entity_location(
    text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> SourceLocation {
    crate::span::find_entity_block(text, entity_iri, &short_name_from_iri(entity_iri), namespaces)
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
}
