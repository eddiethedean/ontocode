use crate::error::{RefactorError, Result};
use crate::model::{FileChange, Hunk, RefactorPlan};
use crate::source::read_source_text;
use crate::text::{normalize_namespace_base, remap_iri, replace_iri_in_text};
use ontocore_catalog::OntologyCatalog;
use ontocore_core::{validate_workspace_scope_any, EntityKind, OntologyFormat, ParseStatus};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

pub fn preview_rename_iri(
    catalog: &OntologyCatalog,
    from_iri: &str,
    to_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    if from_iri == to_iri {
        return Err(RefactorError::Invalid("from and to IRI must differ".to_string()));
    }
    if catalog.find_entity(from_iri).is_none()
        && find_usages_in_catalog(catalog, from_iri, document_overrides).is_empty()
    {
        return Err(RefactorError::EntityNotFound(from_iri.to_string()));
    }

    let mut file_changes: BTreeMap<PathBuf, FileChange> = BTreeMap::new();
    let mut warnings = Vec::new();

    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, from_iri, document_overrides) {
                warnings
                    .push(format!("skipping non-Turtle or errored file: {}", doc.path.display()));
            }
            continue;
        }
        let original = read_source_text(&doc.path, document_overrides)?;
        if !original.contains(from_iri)
            && !contains_prefixed_ref(&original, from_iri, &doc.namespaces)
        {
            continue;
        }
        let (preview_text, raw_hunks) =
            replace_iri_in_text(&original, from_iri, to_iri, &doc.namespaces);
        if preview_text == original {
            continue;
        }
        let hunks: Vec<Hunk> = raw_hunks
            .into_iter()
            .map(|(start, end, old_text, new_text)| Hunk {
                start_byte: start as u64,
                end_byte: end as u64,
                old_text,
                new_text,
            })
            .collect();
        file_changes.insert(
            doc.path.clone(),
            FileChange { path: doc.path.clone(), preview_text, original_text: original, hunks },
        );
    }

    if file_changes.is_empty() {
        warnings.push(format!("no Turtle files changed for IRI {from_iri}"));
    }

    Ok(RefactorPlan { changes: file_changes.into_values().collect(), warnings })
}

pub fn preview_merge_entities(
    catalog: &OntologyCatalog,
    keep_iri: &str,
    merge_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    if keep_iri == merge_iri {
        return Err(RefactorError::Invalid("keep and merge IRI must differ".to_string()));
    }

    let mut warnings = Vec::new();
    if catalog.find_entity(keep_iri).is_none() {
        warnings.push(format!("keep entity not found: {keep_iri}"));
    }
    if catalog.find_entity(merge_iri).is_none() {
        warnings.push(format!("merge entity not found: {merge_iri}"));
    }

    let mut changes = BTreeMap::new();
    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, merge_iri, document_overrides) {
                warnings
                    .push(format!("skipping non-Turtle or errored file: {}", doc.path.display()));
            }
            continue;
        }

        let original = read_source_text(&doc.path, document_overrides)?;
        if !original.contains(merge_iri)
            && !contains_prefixed_ref(&original, merge_iri, &doc.namespaces)
        {
            continue;
        }

        let namespaces = ontocore_owl::namespaces_for_text(&original, &doc.namespaces);
        let short = ontocore_owl::short_name_from_iri(merge_iri);
        let mut declaration_ranges =
            ontocore_owl::all_entity_statement_ranges(&original, merge_iri, &short, &namespaces);
        declaration_ranges.sort_by_key(|range| std::cmp::Reverse(range.start));

        let mut without_merge_declaration = original.clone();
        for range in declaration_ranges {
            without_merge_declaration.replace_range(range.start as usize..range.end as usize, "");
        }
        let (preview_text, _) =
            replace_iri_in_text(&without_merge_declaration, merge_iri, keep_iri, &doc.namespaces);
        if preview_text == original {
            continue;
        }

        changes
            .insert(doc.path.clone(), whole_file_change(doc.path.clone(), original, preview_text));
    }

    if changes.is_empty() {
        warnings
            .push(format!("no Turtle files changed for entity merge {merge_iri} -> {keep_iri}"));
    }

    Ok(RefactorPlan { changes: changes.into_values().collect(), warnings })
}

pub fn preview_replace_entity(
    catalog: &OntologyCatalog,
    from_iri: &str,
    to_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    if catalog.find_entity(to_iri).is_none() {
        return preview_rename_iri(catalog, from_iri, to_iri, document_overrides);
    }
    if from_iri == to_iri {
        return Err(RefactorError::Invalid("from and to IRI must differ".to_string()));
    }
    if catalog.find_entity(from_iri).is_none()
        && find_usages_in_catalog(catalog, from_iri, document_overrides).is_empty()
    {
        return Err(RefactorError::EntityNotFound(from_iri.to_string()));
    }

    let mut changes = BTreeMap::new();
    let mut warnings = Vec::new();
    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, from_iri, document_overrides) {
                warnings
                    .push(format!("skipping non-Turtle or errored file: {}", doc.path.display()));
            }
            continue;
        }

        let original = read_source_text(&doc.path, document_overrides)?;
        if !original.contains(from_iri)
            && !contains_prefixed_ref(&original, from_iri, &doc.namespaces)
        {
            continue;
        }

        let namespaces = ontocore_owl::namespaces_for_text(&original, &doc.namespaces);
        let declaration_start =
            ontocore_owl::entity_primary_block_range(&original, from_iri, &namespaces)
                .map(|range| range.start as usize);
        let preview_text = replace_iri_preserving_subject(
            &original,
            from_iri,
            to_iri,
            &doc.namespaces,
            declaration_start,
        );
        if preview_text == original {
            continue;
        }

        changes
            .insert(doc.path.clone(), whole_file_change(doc.path.clone(), original, preview_text));
    }

    if changes.is_empty() {
        warnings
            .push(format!("no Turtle files changed for entity replacement {from_iri} -> {to_iri}"));
    }

    Ok(RefactorPlan { changes: changes.into_values().collect(), warnings })
}

fn replace_iri_preserving_subject(
    text: &str,
    from_iri: &str,
    to_iri: &str,
    namespaces: &BTreeMap<String, String>,
    subject_start: Option<usize>,
) -> String {
    let Some(subject_start) = subject_start else {
        return replace_iri_in_text(text, from_iri, to_iri, namespaces).0;
    };
    let subject_text = &text[subject_start..];
    let subject_end = if subject_text.starts_with('<') {
        subject_text.find('>').map(|offset| subject_start + offset + 1).unwrap_or(text.len())
    } else {
        subject_text
            .find(|c: char| c.is_whitespace() || c == ';')
            .map(|offset| subject_start + offset)
            .unwrap_or(text.len())
    };

    let before = replace_iri_in_text(&text[..subject_start], from_iri, to_iri, namespaces).0;
    let after = replace_iri_in_text(&text[subject_end..], from_iri, to_iri, namespaces).0;
    format!("{before}{}{after}", &text[subject_start..subject_end])
}

fn whole_file_change(path: PathBuf, original_text: String, preview_text: String) -> FileChange {
    FileChange {
        path,
        hunks: vec![Hunk {
            start_byte: 0,
            end_byte: original_text.len() as u64,
            old_text: original_text.clone(),
            new_text: preview_text.clone(),
        }],
        preview_text,
        original_text,
    }
}

pub fn preview_migrate_namespace(
    catalog: &OntologyCatalog,
    from_base: &str,
    to_base: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    let from = normalize_namespace_base(from_base);
    let to = normalize_namespace_base(to_base);
    if from == to {
        return Err(RefactorError::Invalid("from and to namespace must differ".to_string()));
    }

    let all_iris: Vec<String> = catalog
        .data()
        .entities
        .iter()
        .filter(|e| remap_iri(&e.iri, &from, &to).is_some())
        .map(|e| e.iri.clone())
        .chain(catalog.data().axioms.iter().flat_map(|a| {
            let mut v = Vec::new();
            if remap_iri(&a.subject, &from, &to).is_some() {
                v.push(a.subject.clone());
            }
            if remap_iri(&a.object, &from, &to).is_some() {
                v.push(a.object.clone());
            }
            v
        }))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let mut changes: BTreeMap<PathBuf, FileChange> = BTreeMap::new();
    let mut warnings = Vec::new();

    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            continue;
        }
        let original = read_source_text(&doc.path, document_overrides)?;
        let mut preview = original.clone();
        let mut hunks = Vec::new();
        let mut changed = false;

        for old_iri in &all_iris {
            let new_iri = remap_iri(old_iri, &from, &to).unwrap_or_else(|| old_iri.clone());
            if !preview.contains(old_iri)
                && !contains_prefixed_ref(&preview, old_iri, &doc.namespaces)
            {
                continue;
            }
            let (next, raw_hunks) =
                replace_iri_in_text(&preview, old_iri, &new_iri, &doc.namespaces);
            if next != preview {
                preview = next;
                changed = true;
                hunks.extend(raw_hunks.into_iter().map(|(s, e, o, n)| Hunk {
                    start_byte: s as u64,
                    end_byte: e as u64,
                    old_text: o,
                    new_text: n,
                }));
            }
        }

        for (prefix, ns) in &doc.namespaces {
            if normalize_namespace_base(ns) == from {
                let terminator = if ns.ends_with('/') { '/' } else { '#' };
                let new_ns = format!("{to}{terminator}");
                let (next, raw_hunks) = replace_prefix_uri(&preview, prefix, ns, &new_ns);
                if next != preview {
                    preview = next;
                    changed = true;
                    hunks.extend(raw_hunks.into_iter().map(|(s, e, o, n)| Hunk {
                        start_byte: s as u64,
                        end_byte: e as u64,
                        old_text: o,
                        new_text: n,
                    }));
                }
            }
        }

        if changed {
            changes.insert(
                doc.path.clone(),
                FileChange {
                    path: doc.path.clone(),
                    preview_text: preview,
                    original_text: original,
                    hunks,
                },
            );
        }
    }

    if changes.is_empty() {
        warnings.push(format!("no Turtle files changed for namespace migration {from} -> {to}"));
    }

    Ok(RefactorPlan { changes: changes.into_values().collect(), warnings })
}

fn is_prefix_declaration_line(line: &str) -> bool {
    let keyword = line.split_whitespace().next().unwrap_or("");
    keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX")
}

fn prefix_declaration_name(line: &str) -> Option<&str> {
    let mut parts = line.split_whitespace();
    let keyword = parts.next()?;
    if !(keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX")) {
        return None;
    }
    parts.next()?.strip_suffix(':')
}

fn replace_prefix_uri(
    text: &str,
    prefix: &str,
    old_uri: &str,
    new_uri: &str,
) -> (String, Vec<(usize, usize, String, String)>) {
    let old_term = format!("<{old_uri}>");
    let new_term = format!("<{new_uri}>");
    let mut out = String::with_capacity(text.len());
    let mut hunks = Vec::new();
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        let updated = if prefix_declaration_name(line) == Some(prefix) && line.contains(&old_term) {
            line.replacen(&old_term, &new_term, 1)
        } else {
            line.to_string()
        };
        if updated != line {
            hunks.push((offset, offset + line.len(), line.to_string(), updated.clone()));
        }
        out.push_str(&updated);
        offset += line.len();
    }
    (out, hunks)
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

fn find_usages_in_catalog(
    catalog: &OntologyCatalog,
    iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Vec<()> {
    let u = crate::usages::find_usages_with_overrides(catalog, iri, document_overrides);
    vec![(); u.len()]
}

fn text_contains_iri(
    doc: &ontocore_core::OntologyDocument,
    iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> bool {
    read_source_text(&doc.path, document_overrides).map(|t| t.contains(iri)).unwrap_or(false)
}

fn contains_prefixed_ref(text: &str, iri: &str, namespaces: &BTreeMap<String, String>) -> bool {
    let short = ontocore_owl::short_name_from_iri(iri);
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && text.contains(&format!("{prefix}:{short}")) {
            return true;
        }
    }
    false
}

fn prefixed_curie(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    let short = ontocore_owl::short_name_from_iri(iri);
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            return format!("{prefix}:{short}");
        }
    }
    format!("<{iri}>")
}

fn owl_type_for_kind(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Class => "owl:Class",
        EntityKind::ObjectProperty => "owl:ObjectProperty",
        EntityKind::DataProperty => "owl:DatatypeProperty",
        EntityKind::AnnotationProperty => "owl:AnnotationProperty",
        EntityKind::Individual => "owl:NamedIndividual",
        EntityKind::Ontology => "owl:Ontology",
        EntityKind::Other => "owl:Class",
    }
}

struct EntityRemoval {
    path: PathBuf,
    start: u64,
    end: u64,
    replacement: String,
}

/// Preview a refactor. Client-supplied paths (`target_file` / `output_file`) are jailed under
/// `workspace_roots` **before** any filesystem read.
pub fn preview_refactor(
    catalog: &OntologyCatalog,
    request: &crate::model::RefactorRequest,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    match request {
        crate::model::RefactorRequest::RenameIri { from_iri, to_iri } => {
            preview_rename_iri(catalog, from_iri, to_iri, document_overrides)
        }
        crate::model::RefactorRequest::MergeEntities { keep_iri, merge_iri } => {
            preview_merge_entities(catalog, keep_iri, merge_iri, document_overrides)
        }
        crate::model::RefactorRequest::ReplaceEntity { from_iri, to_iri } => {
            preview_replace_entity(catalog, from_iri, to_iri, document_overrides)
        }
        crate::model::RefactorRequest::MigrateNamespace { from_base, to_base } => {
            preview_migrate_namespace(catalog, from_base, to_base, document_overrides)
        }
        crate::model::RefactorRequest::MoveEntity { entity_iri, target_file } => {
            preview_move_entity(
                catalog,
                entity_iri,
                target_file,
                document_overrides,
                workspace_roots,
            )
        }
        crate::model::RefactorRequest::ExtractModule { entity_iris, output_file, leave_stub } => {
            preview_extract_module(
                catalog,
                entity_iris,
                output_file,
                *leave_stub,
                document_overrides,
                workspace_roots,
            )
        }
    }
}

fn require_path_in_workspace(path: &Path, workspace_roots: &[PathBuf]) -> Result<()> {
    validate_workspace_scope_any(path, workspace_roots).map_err(RefactorError::Invalid)?;
    Ok(())
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub fn preview_move_entity(
    catalog: &OntologyCatalog,
    entity_iri: &str,
    target_file: &Path,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    // Jail before any filesystem read of the client-supplied path.
    require_path_in_workspace(target_file, workspace_roots)?;
    catalog
        .find_entity(entity_iri)
        .ok_or_else(|| RefactorError::EntityNotFound(entity_iri.to_string()))?;
    let source_doc = catalog
        .entity_document(entity_iri)
        .ok_or_else(|| RefactorError::Invalid(format!("no document for {entity_iri}")))?;
    if source_doc.format != OntologyFormat::Turtle {
        return Err(RefactorError::UnsupportedFormat(source_doc.format.as_str().to_string()));
    }

    let source_canon = canonical_path(&source_doc.path);
    let target_canon = canonical_path(target_file);
    if source_canon == target_canon {
        return Err(RefactorError::Invalid(
            "target file must differ from source document".to_string(),
        ));
    }

    let source_text = read_source_text(&source_doc.path, document_overrides)?;
    let namespaces = ontocore_owl::namespaces_for_text(&source_text, &source_doc.namespaces);
    let short = ontocore_owl::short_name_from_iri(entity_iri);
    let mut ranges =
        ontocore_owl::all_entity_statement_ranges(&source_text, entity_iri, &short, &namespaces);
    if ranges.is_empty() {
        return Err(RefactorError::Invalid(format!("entity block not found for {entity_iri}")));
    }
    ranges.sort_by_key(|r| r.start);

    let mut block_parts = Vec::new();
    let mut hunks = Vec::new();
    for range in &ranges {
        let part = source_text[range.start as usize..range.end as usize].to_string();
        block_parts.push(part.clone());
        hunks.push(Hunk {
            start_byte: range.start,
            end_byte: range.end,
            old_text: part,
            new_text: String::new(),
        });
    }
    let block_text = block_parts.join("\n");
    let mut source_without = source_text.clone();
    for range in ranges.into_iter().rev() {
        source_without.replace_range(range.start as usize..range.end as usize, "");
    }

    let target_original = if target_file.exists() {
        read_source_text(target_file, document_overrides)?
    } else {
        String::new()
    };
    let target_preview = if target_original.is_empty() {
        format!("{block_text}\n")
    } else {
        format!("{target_original}\n\n{block_text}")
    };

    Ok(RefactorPlan {
        changes: vec![
            FileChange {
                path: source_doc.path.clone(),
                preview_text: source_without,
                original_text: source_text,
                hunks,
            },
            FileChange {
                path: target_file.to_path_buf(),
                preview_text: target_preview,
                original_text: target_original,
                hunks: vec![Hunk {
                    start_byte: 0,
                    end_byte: 0,
                    old_text: String::new(),
                    new_text: block_text,
                }],
            },
        ],
        warnings: Vec::new(),
    })
}

pub fn preview_extract_module(
    catalog: &OntologyCatalog,
    entity_iris: &[String],
    output_file: &Path,
    leave_stub: bool,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    // Jail before any filesystem read of the client-supplied path.
    require_path_in_workspace(output_file, workspace_roots)?;
    if entity_iris.is_empty() {
        return Err(RefactorError::Invalid("no entities selected".to_string()));
    }

    let mut blocks = Vec::new();
    let mut removals: Vec<EntityRemoval> = Vec::new();
    let mut source_texts: BTreeMap<PathBuf, String> = BTreeMap::new();
    let mut prefix_lines = BTreeSet::new();
    let mut warnings = Vec::new();

    for iri in entity_iris {
        let entity =
            catalog.find_entity(iri).ok_or_else(|| RefactorError::EntityNotFound(iri.clone()))?;
        let doc = catalog
            .entity_document(iri)
            .ok_or_else(|| RefactorError::Invalid(format!("no document for {iri}")))?;
        if doc.format != OntologyFormat::Turtle {
            return Err(RefactorError::UnsupportedFormat(doc.format.as_str().to_string()));
        }
        let text = if let Some(existing) = source_texts.get(&doc.path) {
            existing.clone()
        } else {
            read_source_text(&doc.path, document_overrides)?
        };
        source_texts.insert(doc.path.clone(), text.clone());
        for line in text.lines() {
            if is_prefix_declaration_line(line) {
                prefix_lines.insert(line.trim().to_string());
            }
        }
        let namespaces = ontocore_owl::namespaces_for_text(&text, &doc.namespaces);
        let short = ontocore_owl::short_name_from_iri(iri);
        let mut ranges = ontocore_owl::all_entity_statement_ranges(&text, iri, &short, &namespaces);
        if ranges.is_empty() {
            return Err(RefactorError::Invalid(format!("block not found for {iri}")));
        }
        ranges.sort_by_key(|r| r.start);
        let mut entity_blocks = Vec::new();
        for (idx, range) in ranges.iter().enumerate() {
            let block = text[range.start as usize..range.end as usize].to_string();
            entity_blocks.push(block);
            let replacement = if leave_stub && idx == 0 {
                let owl_type = owl_type_for_kind(entity.kind);
                let moved_path = escape_turtle_string(&output_file.display().to_string());
                format!(
                    "{} a {owl_type} ;\n    owl:deprecated true ;\n    rdfs:comment \"Moved to {moved_path}\" .\n",
                    prefixed_curie(iri, &namespaces),
                )
            } else {
                String::new()
            };
            removals.push(EntityRemoval {
                path: doc.path.clone(),
                start: range.start,
                end: range.end,
                replacement,
            });
        }
        blocks.push(entity_blocks.join("\n"));
    }

    let mut source_changes: BTreeMap<PathBuf, (String, String, Vec<Hunk>)> = BTreeMap::new();
    let mut removals_by_path: BTreeMap<PathBuf, Vec<EntityRemoval>> = BTreeMap::new();
    for removal in removals {
        removals_by_path.entry(removal.path.clone()).or_default().push(removal);
    }
    for (path, mut path_removals) in removals_by_path {
        path_removals.sort_by_key(|b| std::cmp::Reverse(b.start));
        let original = source_texts.remove(&path).ok_or_else(|| {
            RefactorError::Invalid(format!("missing source text for {}", path.display()))
        })?;
        let mut preview = original.clone();
        let mut hunks = Vec::new();
        for removal in path_removals {
            let start = removal.start as usize;
            let end = removal.end as usize;
            let old_text = preview[start..end].to_string();
            preview.replace_range(start..end, &removal.replacement);
            hunks.push(Hunk {
                start_byte: removal.start,
                end_byte: removal.end,
                old_text,
                new_text: removal.replacement.clone(),
            });
        }
        source_changes.insert(path, (original, preview, hunks));
    }

    let mut module_body = blocks.join("\n\n");
    if !module_body.ends_with('\n') {
        module_body.push('\n');
    }
    let prefix_header: String = prefix_lines.into_iter().collect::<Vec<_>>().join("\n");
    let module_text = if prefix_header.is_empty() {
        module_body
    } else {
        format!("{prefix_header}\n\n{module_body}")
    };

    let mut changes: Vec<FileChange> = source_changes
        .into_iter()
        .map(|(path, (original, preview, hunks))| FileChange {
            path: path.clone(),
            preview_text: preview,
            original_text: original,
            hunks,
        })
        .collect();

    let output_original = if output_file.exists() {
        read_source_text(output_file, document_overrides)?
    } else {
        String::new()
    };
    let output_preview = if output_original.is_empty() {
        module_text.clone()
    } else {
        format!("{output_original}\n\n{module_text}")
    };
    changes.push(FileChange {
        path: output_file.to_path_buf(),
        preview_text: output_preview,
        original_text: output_original,
        hunks: vec![Hunk {
            start_byte: 0,
            end_byte: 0,
            old_text: String::new(),
            new_text: module_text,
        }],
    });

    if leave_stub {
        warnings.push("left deprecated stubs in source files".to_string());
    }

    Ok(RefactorPlan { changes, warnings })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_prefix_uri_updates_at_prefix_uppercase() {
        let text = "@PREFIX ex: <http://example.org/org#> .\nex:Person a owl:Class .\n";
        let (out, hunks) =
            replace_prefix_uri(text, "ex", "http://example.org/org#", "http://example.org/v2/org#");
        assert!(out.contains("@PREFIX ex: <http://example.org/v2/org#>"));
        assert!(!out.contains("@PREFIX ex: <http://example.org/org#>"));
        assert!(!hunks.is_empty());
    }

    #[test]
    fn replace_prefix_uri_updates_sparql_style_prefix() {
        let text = "PREFIX ex: <http://example.org/org#>\nex:Person a owl:Class .\n";
        let (out, _) =
            replace_prefix_uri(text, "ex", "http://example.org/org#", "http://example.org/v2/org#");
        assert!(out.contains("PREFIX ex: <http://example.org/v2/org#>"));
    }

    #[test]
    fn owl_type_for_kind_maps_ontology() {
        assert_eq!(owl_type_for_kind(EntityKind::Ontology), "owl:Ontology");
        assert_eq!(owl_type_for_kind(EntityKind::Other), "owl:Class");
        assert_eq!(owl_type_for_kind(EntityKind::Class), "owl:Class");
    }

    #[test]
    fn escape_turtle_string_escapes_path_specials() {
        assert_eq!(
            escape_turtle_string(r#"C:\ontology\mod"ule.ttl"#),
            r#"C:\\ontology\\mod\"ule.ttl"#
        );
    }
}
