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
    AddImport { ontology_iri: String, import_iri: String },
    RemoveImport { ontology_iri: String, import_iri: String },
    AddDomain { entity_iri: String, class_iri: String },
    RemoveDomain { entity_iri: String, class_iri: String },
    AddRange { entity_iri: String, range_iri: String },
    RemoveRange { entity_iri: String, range_iri: String },
    SetFunctional { entity_iri: String, value: bool },
    SetInverseFunctional { entity_iri: String, value: bool },
    SetTransitive { entity_iri: String, value: bool },
    SetSymmetric { entity_iri: String, value: bool },
    SetAsymmetric { entity_iri: String, value: bool },
    SetReflexive { entity_iri: String, value: bool },
    SetIrreflexive { entity_iri: String, value: bool },
    AddPropertyChain { entity_iri: String, properties: Vec<String> },
    RemovePropertyChain { entity_iri: String, properties: Vec<String> },
    AddClassAssertion { entity_iri: String, class_iri: String },
    RemoveClassAssertion { entity_iri: String, class_iri: String },
    AddObjectPropertyAssertion { entity_iri: String, property_iri: String, target_iri: String },
    RemoveObjectPropertyAssertion { entity_iri: String, property_iri: String, target_iri: String },
    AddDataPropertyAssertion { entity_iri: String, property_iri: String, value: String },
    RemoveDataPropertyAssertion { entity_iri: String, property_iri: String, value: String },
    AddAnnotation { entity_iri: String, predicate: String, value: String },
    RemoveAnnotation { entity_iri: String, predicate: String, value: String },
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
    let stem = path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
    let tmp_path = parent.join(format!(".ontocode-{stem}-{nanos}.tmp"));
    {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
    }
    replace_file(&tmp_path, path)?;
    Ok(())
}

/// Replace `path` with `tmp_path` (tmp is consumed). Works on Windows where `rename` cannot
/// overwrite an existing destination.
fn replace_file(tmp_path: &Path, path: &Path) -> std::io::Result<()> {
    match fs::rename(tmp_path, path) {
        Ok(()) => Ok(()),
        Err(_) if path.exists() => {
            // Windows (and some network FS): rename refuses to replace. Move the existing
            // file aside, then rename; restore on failure.
            let bak_path = tmp_path.with_extension("bak");
            fs::rename(path, &bak_path)?;
            match fs::rename(tmp_path, path) {
                Ok(()) => {
                    let _ = fs::remove_file(&bak_path);
                    Ok(())
                }
                Err(rename_err) => {
                    let _ = fs::rename(&bak_path, path);
                    let _ = fs::remove_file(tmp_path);
                    Err(rename_err)
                }
            }
        }
        Err(e) => {
            let _ = fs::remove_file(tmp_path);
            Err(e)
        }
    }
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
            let other = iri_to_turtle_term(other_iri, namespaces)?;
            add_object_triple(text, entity_iri, "owl:disjointWith", &other, namespaces)
        }
        PatchOp::RemoveDisjointClass { entity_iri, other_iri } => {
            remove_disjoint_triple(text, entity_iri, other_iri, namespaces)
        }
        PatchOp::AddImport { ontology_iri, import_iri } => {
            let import_term = iri_to_turtle_term(import_iri, namespaces)?;
            add_object_triple(text, ontology_iri, "owl:imports", &import_term, namespaces)
        }
        PatchOp::RemoveImport { ontology_iri, import_iri } => {
            let import_term = iri_to_turtle_term(import_iri, namespaces)?;
            remove_predicate_object(text, ontology_iri, "owl:imports", &import_term, namespaces)
        }
        PatchOp::AddDomain { entity_iri, class_iri } => {
            let class = iri_to_turtle_term(class_iri, namespaces)?;
            add_object_triple(text, entity_iri, "rdfs:domain", &class, namespaces)
        }
        PatchOp::RemoveDomain { entity_iri, class_iri } => {
            let class = iri_to_turtle_term(class_iri, namespaces)?;
            remove_predicate_object_any_statement(
                text,
                entity_iri,
                "rdfs:domain",
                &class,
                namespaces,
            )
        }
        PatchOp::AddRange { entity_iri, range_iri } => {
            let range = iri_to_turtle_term(range_iri, namespaces)?;
            add_object_triple(text, entity_iri, "rdfs:range", &range, namespaces)
        }
        PatchOp::RemoveRange { entity_iri, range_iri } => {
            let range = iri_to_turtle_term(range_iri, namespaces)?;
            remove_predicate_object_any_statement(
                text,
                entity_iri,
                "rdfs:range",
                &range,
                namespaces,
            )
        }
        PatchOp::SetFunctional { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:FunctionalProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetInverseFunctional { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:InverseFunctionalProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetTransitive { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:TransitiveProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetSymmetric { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:SymmetricProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetAsymmetric { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:AsymmetricProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetReflexive { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:ReflexiveProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetIrreflexive { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:IrreflexiveProperty",
            *value,
            namespaces,
        ),
        PatchOp::AddPropertyChain { entity_iri, properties } => {
            add_property_chain(text, entity_iri, properties, namespaces)
        }
        PatchOp::RemovePropertyChain { entity_iri, properties } => {
            remove_property_chain(text, entity_iri, properties, namespaces)
        }
        PatchOp::AddClassAssertion { entity_iri, class_iri } => {
            let class = iri_to_turtle_term(class_iri, namespaces)?;
            add_type_triple(text, entity_iri, &class, namespaces)
        }
        PatchOp::RemoveClassAssertion { entity_iri, class_iri } => {
            let class = iri_to_turtle_term(class_iri, namespaces)?;
            remove_type_triple(text, entity_iri, &class, namespaces)
        }
        PatchOp::AddObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            let target = iri_to_turtle_term(target_iri, namespaces)?;
            add_property_assertion_triple(text, entity_iri, &prop, &target, namespaces)
        }
        PatchOp::RemoveObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            let target = iri_to_turtle_term(target_iri, namespaces)?;
            remove_predicate_object_any_statement(text, entity_iri, &prop, &target, namespaces)
        }
        PatchOp::AddDataPropertyAssertion { entity_iri, property_iri, value } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            add_data_property_assertion(text, entity_iri, &prop, value, namespaces)
        }
        PatchOp::RemoveDataPropertyAssertion { entity_iri, property_iri, value } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            let escaped = escape_turtle_string(value);
            let object = format!("\"{escaped}\"");
            remove_predicate_object_any_statement(text, entity_iri, &prop, &object, namespaces)
        }
        PatchOp::AddAnnotation { entity_iri, predicate, value } => {
            if predicate == "rdfs:label" || predicate.ends_with("#label") {
                add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
            } else if predicate == "rdfs:comment" || predicate.ends_with("#comment") {
                add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
            } else if value.starts_with("http://") || value.starts_with("https://") {
                let pred = predicate_to_term(predicate, namespaces)?;
                let obj = iri_to_turtle_term(value, namespaces)?;
                add_object_triple(text, entity_iri, &pred, &obj, namespaces)
            } else {
                let pred = predicate_to_term(predicate, namespaces)?;
                add_annotation_triple(text, entity_iri, &pred, value, namespaces)
            }
        }
        PatchOp::RemoveAnnotation { entity_iri, predicate, value } => {
            if predicate == "rdfs:label" || predicate.ends_with("#label") {
                remove_matching_predicate_any(text, entity_iri, "rdfs:label", value, namespaces)
            } else if predicate == "rdfs:comment" || predicate.ends_with("#comment") {
                remove_matching_predicate_any(text, entity_iri, "rdfs:comment", value, namespaces)
            } else if value.starts_with("http://") || value.starts_with("https://") {
                let pred = predicate_to_term(predicate, namespaces)?;
                let obj = iri_to_turtle_term(value, namespaces)?;
                remove_predicate_object_any_statement(text, entity_iri, &pred, &obj, namespaces)
            } else {
                let pred = predicate_to_term(predicate, namespaces)?;
                remove_matching_predicate_any(text, entity_iri, &pred, value, namespaces)
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
    let subject = iri_to_turtle_term(entity_iri, namespaces)?;
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
    insert_into_entity_block(text, entity_iri, &triple, namespaces, true)
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
    insert_into_entity_block(text, entity_iri, &triple, namespaces, false)
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
    let parent = iri_to_turtle_term(parent_iri, namespaces)?;
    add_object_triple(text, entity_iri, "rdfs:subClassOf", &parent, namespaces)
}

fn remove_subclass_triple(
    text: &mut String,
    entity_iri: &str,
    parent_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let parent = iri_to_turtle_term(parent_iri, namespaces)?;
    remove_predicate_object_any_statement(text, entity_iri, "rdfs:subClassOf", &parent, namespaces)
}

fn remove_disjoint_triple(
    text: &mut String,
    entity_iri: &str,
    other_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let other = iri_to_turtle_term(other_iri, namespaces)?;
    remove_predicate_object_any_statement(text, entity_iri, "owl:disjointWith", &other, namespaces)
}

fn predicate_to_term(predicate: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    if predicate.contains(':') && !predicate.starts_with("http") {
        Ok(predicate.to_string())
    } else {
        iri_to_turtle_term(predicate, namespaces)
    }
}

fn set_property_characteristic(
    text: &mut String,
    entity_iri: &str,
    characteristic: &str,
    value: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if value {
        add_type_triple(text, entity_iri, characteristic, namespaces)
    } else {
        remove_type_triple(text, entity_iri, characteristic, namespaces)
    }
}

fn add_type_triple(
    text: &mut String,
    entity_iri: &str,
    type_term: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let subject = iri_to_turtle_term(entity_iri, namespaces)?;
    let ns = crate::span::namespaces_for_text(text, namespaces);
    if let Some(range) = entity_primary_block_range(text, entity_iri, &ns) {
        let block = &text[range.start as usize..range.end as usize];
        if block.contains(&format!("a {type_term}"))
            || block.contains(&format!("a owl:NamedIndividual, {type_term}"))
            || block.contains(&format!(", {type_term}"))
        {
            return Ok(());
        }
        let insertion = format!("    a {type_term} ;\n");
        return insert_into_entity_block(text, entity_iri, &insertion, namespaces, false);
    }
    // Trailing `subject a type .` form
    let triple = format!("\n{subject} a {type_term} .\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&triple);
    Ok(())
}

fn remove_type_triple(
    text: &mut String,
    entity_iri: &str,
    type_term: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    remove_predicate_object_any_statement(text, entity_iri, "a", type_term, namespaces)
}

fn add_property_assertion_triple(
    text: &mut String,
    entity_iri: &str,
    property_term: &str,
    target_term: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let triple = format!("    {property_term} {target_term} ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces, false)
}

fn add_data_property_assertion(
    text: &mut String,
    entity_iri: &str,
    property_term: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let escaped = escape_turtle_string(value);
    let triple = format!("    {property_term} \"{escaped}\" ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces, false)
}

fn entity_declared_as(
    text: &str,
    entity_iri: &str,
    owl_type: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return false;
    }
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
        let is_subject = needles.iter().any(|needle| line_starts_with_subject(trimmed, needle));
        is_subject
            && (trimmed.contains(" a ") || trimmed.contains("\ta"))
            && trimmed.contains(owl_type)
    })
}

fn validate_property_chain_members(
    text: &str,
    properties: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    const INVALID_TYPES: &[&str] = &[
        "owl:Class",
        "owl:NamedIndividual",
        "owl:DatatypeProperty",
        "owl:AnnotationProperty",
    ];
    for iri in properties {
        for owl_type in INVALID_TYPES {
            if entity_declared_as(text, iri, owl_type, namespaces) {
                return Err(OwlError::PatchInvalid(format!(
                    "property chain member {iri} is declared as {owl_type}, expected owl:ObjectProperty"
                )));
            }
        }
    }
    Ok(())
}

fn add_property_chain(
    text: &mut String,
    entity_iri: &str,
    properties: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if properties.is_empty() {
        return Err(OwlError::PatchInvalid(
            "property chain must have at least one property".into(),
        ));
    }
    validate_property_chain_members(text, properties, namespaces)?;
    let terms: Vec<String> =
        properties.iter().map(|p| iri_to_turtle_term(p, namespaces)).collect::<Result<Vec<_>>>()?;
    let chain_obj = format!("( {} )", terms.join(" "));
    add_object_triple(text, entity_iri, "owl:propertyChainAxiom", &chain_obj, namespaces)
}

fn remove_property_chain(
    text: &mut String,
    entity_iri: &str,
    properties: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let terms: Vec<String> =
        properties.iter().map(|p| iri_to_turtle_term(p, namespaces)).collect::<Result<Vec<_>>>()?;
    let chain_obj = format!("( {} )", terms.join(" "));
    remove_predicate_object_any_statement(
        text,
        entity_iri,
        "owl:propertyChainAxiom",
        &chain_obj,
        namespaces,
    )
}

fn remove_predicate_triples(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    remove_all_predicate_any_statement(text, entity_iri, predicate, namespaces)
}

fn remove_predicate_object(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    remove_predicate_object_any_statement(text, entity_iri, predicate, object_value, namespaces)
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
    duplicate_is_error: bool,
) -> Result<()> {
    if let Some((predicate, object)) = parse_simple_insertion(insertion) {
        if entity_has_predicate_object(text, entity_iri, &predicate, &object, namespaces) {
            if duplicate_is_error {
                return Err(OwlError::PatchInvalid(format!(
                    "duplicate {predicate} axiom already present: {object}"
                )));
            }
            return Ok(());
        }
    }
    // Same single-path insertion as multiline axioms (no double-insert).
    insert_multiline_into_entity_block(text, entity_iri, insertion, namespaces)
}

fn parse_simple_insertion(insertion: &str) -> Option<(String, String)> {
    let line = insertion.lines().next()?.trim().trim_end_matches(';').trim();
    let mut parts = line.splitn(2, char::is_whitespace);
    let predicate = parts.next()?.to_string();
    let object = parts.next()?.trim().to_string();
    if predicate.is_empty() || object.is_empty() {
        return None;
    }
    Some((predicate, object))
}

fn entity_has_predicate_object(
    text: &str,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    all_entity_statement_ranges(text, entity_iri, &short, &ns).into_iter().any(|range| {
        let block = &text[range.start as usize..range.end as usize];
        block_has_matching_predicate_object(block, predicate, object_value)
    })
}

fn block_has_matching_predicate_object(block: &str, predicate: &str, object_value: &str) -> bool {
    remove_matching_predicate_object(block, predicate, object_value).is_some()
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
    let mut in_iri = false;
    let mut in_comment = false;
    let mut escape = false;
    let mut i = bracket_start;
    while i < bytes.len() {
        let b = bytes[i];
        if in_comment {
            if b == b'\n' {
                in_comment = false;
            }
            i += 1;
            continue;
        }
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
        if in_iri {
            if b == b'>' {
                in_iri = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'#' => in_comment = true,
            b'"' => in_string = true,
            b'<' => in_iri = true,
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

/// Extend removal to cover the object, and the predicate when it would be left empty.
fn extend_removal_span(
    block: &str,
    pred_pos: usize,
    predicate: &str,
    obj_start: usize,
    obj_end: usize,
) -> (usize, usize) {
    let objects = objects_in_predicate_value(block, pred_pos, predicate);
    let is_only_object = objects.len() == 1;

    let mut start = if is_only_object { pred_pos } else { obj_start };
    while start > 0 && block.as_bytes()[start - 1].is_ascii_whitespace() {
        start -= 1;
    }
    if !is_only_object && start > 0 && block.as_bytes()[start - 1] == b',' {
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
    let pred_pos = find_predicate_token(block, 0, predicate)?;
    let (obj_start, obj_end) =
        objects_in_predicate_value(block, pred_pos, predicate).first().copied()?;
    let (remove_start, remove_end) =
        extend_removal_span(block, pred_pos, predicate, obj_start, obj_end);
    let mut out = String::new();
    out.push_str(&block[..remove_start]);
    out.push_str(&block[remove_end..]);
    Some(cleanup_block_separators(&out))
}

/// Find `predicate` as a Turtle token outside strings, IRIs, comments, and brackets.
fn find_predicate_token(block: &str, search_from: usize, predicate: &str) -> Option<usize> {
    let bytes = block.as_bytes();
    let pred_bytes = predicate.as_bytes();
    if pred_bytes.is_empty() || search_from >= bytes.len() {
        return None;
    }
    let mut i = search_from;
    let mut in_string = false;
    let mut in_iri = false;
    let mut in_comment = false;
    let mut escape = false;
    let mut bracket_depth = 0i32;
    while i + pred_bytes.len() <= bytes.len() {
        let b = bytes[i];
        if in_comment {
            if b == b'\n' {
                in_comment = false;
            }
            i += 1;
            continue;
        }
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
        if in_iri {
            if b == b'>' {
                in_iri = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'#' => {
                in_comment = true;
                i += 1;
                continue;
            }
            b'"' => {
                in_string = true;
                i += 1;
                continue;
            }
            b'<' => {
                in_iri = true;
                i += 1;
                continue;
            }
            b'[' => {
                bracket_depth += 1;
                i += 1;
                continue;
            }
            b']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                i += 1;
                continue;
            }
            _ => {}
        }
        if bracket_depth == 0 && bytes[i..].starts_with(pred_bytes) {
            let after = i + pred_bytes.len();
            let before_ok = i == 0
                || !bytes[i - 1].is_ascii_alphanumeric()
                    && bytes[i - 1] != b':'
                    && bytes[i - 1] != b'_';
            let after_ok = after >= bytes.len()
                || bytes[after].is_ascii_whitespace()
                || bytes[after] == b';'
                || bytes[after] == b'.'
                || bytes[after] == b',';
            if before_ok && after_ok {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn find_named_object_end(block: &str, obj_start: usize) -> Option<usize> {
    use crate::span::is_turtle_terminating_dot;
    let bytes = block.as_bytes();
    let mut i = obj_start;
    let mut in_string = false;
    let mut in_iri = false;
    let mut escape = false;
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
        if in_iri {
            if b == b'>' {
                in_iri = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'<' => in_iri = true,
            b',' | b';' => return Some(i),
            b'.' if is_turtle_terminating_dot(bytes, i) => return Some(i),
            _ => {}
        }
        i += 1;
    }
    Some(block.len())
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
    while let Some(pred_pos) = find_predicate_token(block, search_from, predicate) {
        for (obj_start, obj_end) in objects_in_predicate_value(block, pred_pos, predicate) {
            let candidate = normalize_ws(block[obj_start..obj_end].trim());
            if candidate == norm_obj {
                let (remove_start, remove_end) =
                    extend_removal_span(block, pred_pos, predicate, obj_start, obj_end);
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

/// Reject IRIs that would break Turtle `<...>` terms or inject syntax.
pub(crate) fn is_safe_iri(iri: &str) -> bool {
    if iri.is_empty() {
        return false;
    }
    !iri.chars().any(|c| {
        c.is_control()
            || c.is_whitespace()
            || matches!(c, '<' | '>' | '"' | '{' | '}' | '|' | '^' | '`' | '\\')
    })
}

/// True when `local` is a valid Turtle PN_LOCAL (simplified).
fn is_valid_pn_local(local: &str) -> bool {
    if local.is_empty() {
        return false;
    }
    local.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '~'))
        && !local.starts_with('.')
        && !local.ends_with('.')
}

fn iri_to_turtle_term(iri: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    iri_to_turtle_term_impl(iri, namespaces)
}

pub(crate) fn iri_to_turtle_term_impl(
    iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    if !is_safe_iri(iri) {
        return Err(OwlError::PatchInvalid(format!(
            "IRI contains characters that cannot be safely written to Turtle: {iri:?}"
        )));
    }
    if iri == "http://www.w3.org/2002/07/owl#Thing" {
        return Ok("owl:Thing".to_string());
    }

    if let Some((prefix, ns)) = best_namespace_match(iri, namespaces) {
        let local = &iri[ns.len()..];
        if is_valid_pn_local(local) {
            return Ok(format!("{prefix}:{local}"));
        }
    }
    Ok(format!("<{iri}>"))
}

pub(crate) fn best_namespace_match<'a>(
    iri: &str,
    namespaces: &'a BTreeMap<String, String>,
) -> Option<(&'a str, &'a str)> {
    let mut best: Option<(&str, &str, usize)> = None;
    for (prefix, ns) in namespaces {
        if prefix.is_empty() || !iri.starts_with(ns.as_str()) {
            continue;
        }
        let len = ns.len();
        if best.as_ref().is_none_or(|(_, _, best_len)| len > *best_len) {
            best = Some((prefix.as_str(), ns.as_str(), len));
        }
    }
    best.map(|(prefix, ns, _)| (prefix, ns))
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
    fn add_and_remove_import() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<http://example.org/people> a owl:Ontology ;
    rdfs:label "People" .
"#;
        let ns = ex_ns();
        let add = apply_patches_to_text(
            ttl,
            &[PatchOp::AddImport {
                ontology_iri: "http://example.org/people".to_string(),
                import_iri: "http://example.org/org".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add import");
        let with_import = add.preview_text.expect("preview");
        assert!(with_import.contains("owl:imports <http://example.org/org>"));

        let remove = apply_patches_to_text(
            &with_import,
            &[PatchOp::RemoveImport {
                ontology_iri: "http://example.org/people".to_string(),
                import_iri: "http://example.org/org".to_string(),
            }],
            true,
            &ns,
        )
        .expect("remove import");
        let cleaned = remove.preview_text.expect("preview");
        assert!(!cleaned.contains("owl:imports"));
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
        assert_eq!(
            preview.matches("rdfs:label \"Human\"").count(),
            1,
            "must insert label exactly once"
        );
    }

    #[test]
    fn add_label_not_blocked_by_label_text_in_long_comment() {
        let ttl = r#"@prefix ex: <http://example.org/ex#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Foo a owl:Class ;
    rdfs:comment """Documentation mentions rdfs:label \"Bar\" syntax.""" .
"#;
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/ex#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ]);
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/ex#Foo".to_string(),
            value: "Bar".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ns).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("rdfs:label \"Bar\""));
    }

    #[test]
    fn add_label_duplicate_returns_error() {
        let ttl = r#"@prefix ex: <http://example.org/ex#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Foo a owl:Class ;
    rdfs:label "Bar" .
"#;
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/ex#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ]);
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/ex#Foo".to_string(),
            value: "Bar".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ns).expect("patch");
        assert!(!result.applied);
        assert!(result.diagnostics.iter().any(|d| d.message.contains("duplicate")));
    }

    #[test]
    fn remove_subclass_does_not_leave_orphaned_predicate() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::RemoveSubClassOf {
            entity_iri: "http://example.org/people#Person".to_string(),
            parent_iri: "http://example.org/people#Thing".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("rdfs:subClassOf."));
        assert!(!preview.contains("rdfs:subClassOf ;"));
        assert!(!preview.contains("ex:Person rdfs:subClassOf"));
    }

    #[test]
    fn add_subclass_of_no_duplicate_when_trailing_triple_exists() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::AddSubClassOf {
            entity_iri: "http://example.org/people#Person".to_string(),
            parent_iri: "http://example.org/people#Thing".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(
            result.preview_text.is_none(),
            "must not duplicate subclass axiom already present as trailing triple"
        );
    }

    #[test]
    fn remove_import_from_trailing_triple() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/people> a owl:Ontology .
<http://example.org/people> owl:imports <http://example.org/other> .
"#;
        let ns = ex_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveImport {
                ontology_iri: "http://example.org/people".to_string(),
                import_iri: "http://example.org/other".to_string(),
            }],
            true,
            &ns,
        )
        .expect("remove trailing import");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("owl:imports"));
    }

    #[test]
    fn set_deprecated_false_removes_trailing_flag() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
ex:Person owl:deprecated true .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetDeprecated {
                entity_iri: "http://example.org/people#Person".to_string(),
                value: false,
            }],
            true,
            &ex_ns(),
        )
        .expect("clear deprecated");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("owl:deprecated"));
    }

    #[test]
    fn remove_comment_with_period_in_literal() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:comment "A human being." .
"#;
        let patches = vec![PatchOp::RemoveComment {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "A human being.".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("A human being."));
        assert!(!preview.contains("rdfs:comment"));
        assert!(preview.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn remove_label_ignores_predicate_name_inside_comment() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:comment "see rdfs:label usage" ;
    rdfs:label "Name" .
"#;
        let patches = vec![PatchOp::RemoveLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Name".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("see rdfs:label usage"));
        assert!(!preview.contains("rdfs:label \"Name\""));
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
    fn iri_with_angle_bracket_is_rejected_not_injected() {
        let ttl = "@prefix ex: <http://example.org/people#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Person a owl:Class .\n";
        let evil =
            "http://example.org/people#X> . ex:Pwned a owl:Class . <http://example.org/people#Y";
        let patches = vec![PatchOp::CreateEntity {
            entity_iri: evil.to_string(),
            kind: PatchEntityKind::Class,
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(!result.applied);
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
        assert!(!ttl.contains("ex:Pwned"));
    }

    #[test]
    fn iri_with_newline_is_rejected() {
        let ttl = "@prefix ex: <http://example.org/people#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Person a owl:Class .\n";
        let evil = "http://example.org/people#X\n. ex:Injected a owl:Class .\n#";
        let patches = vec![PatchOp::CreateEntity {
            entity_iri: evil.to_string(),
            kind: PatchEntityKind::Class,
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(!result.applied);
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
    }

    #[test]
    fn longest_namespace_prefix_wins() {
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/".to_string()),
            ("exfoo".to_string(), "http://example.org/foo/".to_string()),
        ]);
        let term = iri_to_turtle_term("http://example.org/foo/Bar", &ns).expect("term");
        assert_eq!(term, "exfoo:Bar");
    }

    #[test]
    fn slash_in_local_name_uses_angle_brackets() {
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org/".to_string())]);
        let term = iri_to_turtle_term("http://example.org/foo/Bar", &ns).expect("term");
        assert_eq!(term, "<http://example.org/foo/Bar>");
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

    fn org_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/org#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    #[test]
    fn add_and_remove_domain() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = org_ns();
        let add = apply_patches_to_text(
            ttl,
            &[PatchOp::AddDomain {
                entity_iri: "http://example.org/org#chases".to_string(),
                class_iri: "http://example.org/org#Cat".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add domain");
        let with_domain = add.preview_text.expect("preview");
        assert!(with_domain.contains("rdfs:domain"));

        let remove = apply_patches_to_text(
            &with_domain,
            &[PatchOp::RemoveDomain {
                entity_iri: "http://example.org/org#chases".to_string(),
                class_iri: "http://example.org/org#Dog".to_string(),
            }],
            true,
            &ns,
        )
        .expect("remove domain");
        let preview = remove.preview_text.expect("preview");
        assert!(!preview.contains("rdfs:domain ex:Dog"));
        assert!(preview.contains("rdfs:domain ex:Cat"));
    }

    #[test]
    fn add_property_chain() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = org_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddPropertyChain {
                entity_iri: "http://example.org/org#chases".to_string(),
                properties: vec![
                    "http://example.org/org#chases".to_string(),
                    "http://example.org/org#composed".to_string(),
                ],
            }],
            true,
            &ns,
        )
        .expect("add chain");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:propertyChainAxiom"));
    }

    #[test]
    fn add_property_chain_rejects_class_iris() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = org_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddPropertyChain {
                entity_iri: "http://example.org/org#chases".to_string(),
                properties: vec![
                    "http://example.org/org#Cat".to_string(),
                    "http://example.org/org#Dog".to_string(),
                ],
            }],
            true,
            &ns,
        )
        .expect("patch result");
        assert!(!result.diagnostics.is_empty());
        assert!(result.diagnostics[0].message.contains("owl:Class"));
    }

    #[test]
    fn add_class_assertion_to_individual() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Alice a owl:NamedIndividual .
ex:Person a owl:Class .
"#;
        let ns = org_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddClassAssertion {
                entity_iri: "http://example.org/org#Alice".to_string(),
                class_iri: "http://example.org/org#Person".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add class assertion");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("ex:Person"));
    }

    #[test]
    fn add_generic_annotation() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

ex:Cat a owl:Class .
"#;
        let ns = org_ns();
        let mut ns = ns;
        ns.insert("skos".to_string(), "http://www.w3.org/2004/02/skos/core#".to_string());
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddAnnotation {
                entity_iri: "http://example.org/org#Cat".to_string(),
                predicate: "skos:definition".to_string(),
                value: "A feline animal".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add annotation");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("skos:definition \"A feline animal\""));
    }

    #[test]
    fn set_functional_property() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = org_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetFunctional {
                entity_iri: "http://example.org/org#chases".to_string(),
                value: true,
            }],
            true,
            &ns,
        )
        .expect("set functional");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:FunctionalProperty"));
    }
}
