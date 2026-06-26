use crate::error::{RefactorError, Result};
use crate::model::{FileChange, Hunk, RefactorPlan};
use crate::text::{normalize_namespace_base, remap_iri, replace_iri_in_text};
use ontoindex_catalog::OntologyCatalog;
use ontoindex_core::{OntologyFormat, ParseStatus};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub fn preview_rename_iri(
    catalog: &OntologyCatalog,
    from_iri: &str,
    to_iri: &str,
) -> Result<RefactorPlan> {
    if from_iri == to_iri {
        return Err(RefactorError::Invalid("from and to IRI must differ".to_string()));
    }
    if catalog.find_entity(from_iri).is_none() && find_usages_in_catalog(catalog, from_iri).is_empty() {
        return Err(RefactorError::EntityNotFound(from_iri.to_string()));
    }

    let mut file_changes: BTreeMap<PathBuf, FileChange> = BTreeMap::new();
    let mut warnings = Vec::new();

    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, from_iri) {
                warnings.push(format!(
                    "skipping non-Turtle or errored file: {}",
                    doc.path.display()
                ));
            }
            continue;
        }
        let original = std::fs::read_to_string(&doc.path)?;
        if !original.contains(from_iri) && !contains_prefixed_ref(&original, from_iri, &doc.namespaces) {
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
            FileChange {
                path: doc.path.clone(),
                preview_text,
                original_text: original,
                hunks,
            },
        );
    }

    if file_changes.is_empty() {
        warnings.push(format!("no Turtle files changed for IRI {from_iri}"));
    }

    Ok(RefactorPlan {
        changes: file_changes.into_values().collect(),
        warnings,
    })
}

pub fn preview_migrate_namespace(
    catalog: &OntologyCatalog,
    from_base: &str,
    to_base: &str,
) -> Result<RefactorPlan> {
    let from = normalize_namespace_base(from_base);
    let to = normalize_namespace_base(to_base);
    if from == to {
        return Err(RefactorError::Invalid("from and to namespace must differ".to_string()));
    }

    let mut merged = RefactorPlan { changes: Vec::new(), warnings: Vec::new() };

    let all_iris: Vec<String> = catalog
        .data()
        .entities
        .iter()
        .filter(|e| remap_iri(&e.iri, &from, &to).is_some())
        .map(|e| e.iri.clone())
        .chain(
            catalog.data().axioms.iter().flat_map(|a| {
                let mut v = Vec::new();
                if remap_iri(&a.subject, &from, &to).is_some() {
                    v.push(a.subject.clone());
                }
                if a.object.starts_with(&from) {
                    v.push(a.object.clone());
                }
                v
            }),
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    for old_iri in all_iris {
        let new_iri = remap_iri(&old_iri, &from, &to).unwrap_or(old_iri.clone());
        let sub = preview_rename_iri(catalog, &old_iri, &new_iri)?;
        merge_plans(&mut merged, sub);
    }

    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle {
            continue;
        }
        let original = std::fs::read_to_string(&doc.path).unwrap_or_default();
        for (prefix, ns) in &doc.namespaces {
            if normalize_namespace_base(ns) == from {
                let new_ns = if to.ends_with('#') || to.ends_with('/') {
                    to.clone()
                } else {
                    format!("{to}#")
                };
                let (preview, hunks) = replace_prefix_uri(&original, prefix, ns, &new_ns);
                if preview != original {
                    let entry = merged
                        .changes
                        .iter_mut()
                        .find(|c| c.path == doc.path)
                        .map(|c| c.path.clone());
                    if let Some(path) = entry {
                        if let Some(change) = merged.changes.iter_mut().find(|c| c.path == path) {
                            change.preview_text = preview;
                            change.hunks.extend(hunks.into_iter().map(|(s, e, o, n)| Hunk {
                                start_byte: s as u64,
                                end_byte: e as u64,
                                old_text: o,
                                new_text: n,
                            }));
                        }
                    } else {
                        merged.changes.push(FileChange {
                            path: doc.path.clone(),
                            preview_text: preview.clone(),
                            original_text: original.clone(),
                            hunks: hunks
                                .into_iter()
                                .map(|(s, e, o, n)| Hunk {
                                    start_byte: s as u64,
                                    end_byte: e as u64,
                                    old_text: o,
                                    new_text: n,
                                })
                                .collect(),
                        });
                    }
                }
            }
        }
    }

    Ok(merged)
}

fn replace_prefix_uri(
    text: &str,
    prefix: &str,
    old_uri: &str,
    new_uri: &str,
) -> (String, Vec<(usize, usize, String, String)>) {
    let old_decl = format!("@prefix {prefix}: <{old_uri}>");
    let new_decl = format!("@prefix {prefix}: <{new_uri}>");
    let result = text.replacements(old_decl.as_str(), new_decl.as_str());
    let mut hunks = Vec::new();
    if result != text {
        if let Some(pos) = text.find(&old_decl) {
            hunks.push((pos, pos + old_decl.len(), old_decl, new_decl.clone()));
        }
    }
    (result, hunks)
}

trait Replacements {
    fn replacements(&self, from: &str, to: &str) -> String;
}

impl Replacements for str {
    fn replacements(&self, from: &str, to: &str) -> String {
        self.replace(from, to)
    }
}

fn merge_plans(target: &mut RefactorPlan, other: RefactorPlan) {
    target.warnings.extend(other.warnings);
    for change in other.changes {
        if let Some(existing) = target.changes.iter_mut().find(|c| c.path == change.path) {
            if change.preview_text != existing.original_text {
                existing.preview_text = change.preview_text;
                existing.hunks.extend(change.hunks);
            }
        } else {
            target.changes.push(change);
        }
    }
}

fn find_usages_in_catalog(catalog: &OntologyCatalog, iri: &str) -> Vec<()> {
    let u = crate::usages::find_usages(catalog, iri);
    vec![(); u.len()]
}

fn text_contains_iri(doc: &ontoindex_core::OntologyDocument, iri: &str) -> bool {
    std::fs::read_to_string(&doc.path)
        .map(|t| t.contains(iri))
        .unwrap_or(false)
}

fn contains_prefixed_ref(text: &str, iri: &str, namespaces: &BTreeMap<String, String>) -> bool {
    let short = ontoindex_owl::short_name_from_iri(iri);
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && text.contains(&format!("{prefix}:{short}")) {
            return true;
        }
    }
    false
}

pub fn preview_refactor(catalog: &OntologyCatalog, request: &crate::model::RefactorRequest) -> Result<RefactorPlan> {
    match request {
        crate::model::RefactorRequest::RenameIri { from_iri, to_iri } => {
            preview_rename_iri(catalog, from_iri, to_iri)
        }
        crate::model::RefactorRequest::MigrateNamespace { from_base, to_base } => {
            preview_migrate_namespace(catalog, from_base, to_base)
        }
        crate::model::RefactorRequest::MoveEntity { entity_iri, target_file } => {
            preview_move_entity(catalog, entity_iri, target_file)
        }
        crate::model::RefactorRequest::ExtractModule {
            entity_iris,
            output_file,
            leave_stub,
        } => preview_extract_module(catalog, entity_iris, output_file, *leave_stub),
    }
}

pub fn preview_move_entity(
    catalog: &OntologyCatalog,
    entity_iri: &str,
    target_file: &Path,
) -> Result<RefactorPlan> {
    let entity = catalog
        .find_entity(entity_iri)
        .ok_or_else(|| RefactorError::EntityNotFound(entity_iri.to_string()))?;
    let source_doc = catalog
        .entity_document(entity_iri)
        .ok_or_else(|| RefactorError::Invalid(format!("no document for {entity_iri}")))?;
    if source_doc.format != OntologyFormat::Turtle {
        return Err(RefactorError::UnsupportedFormat(source_doc.format.as_str().to_string()));
    }

    let source_text = std::fs::read_to_string(&source_doc.path)?;
    let namespaces = ontoindex_owl::namespaces_for_text(&source_text, &source_doc.namespaces);
    let block_range = ontoindex_owl::entity_block_range(&source_text, entity, &namespaces)
        .ok_or_else(|| RefactorError::Invalid(format!("entity block not found for {entity_iri}")))?;

    let block_text = source_text[block_range.start as usize..block_range.end as usize].to_string();
    let mut source_without = source_text.clone();
    source_without.replace_range(block_range.start as usize..block_range.end as usize, "");

    let target_original = if target_file.exists() {
        std::fs::read_to_string(target_file)?
    } else {
        String::new()
    };
    let target_preview = if target_original.is_empty() {
        format!("{block_text}\n")
    } else {
        format!("{target_original}\n\n{block_text}")
    };

    let changes = vec![
        FileChange {
            path: source_doc.path.clone(),
            preview_text: source_without,
            original_text: source_text,
            hunks: vec![Hunk {
                start_byte: block_range.start,
                end_byte: block_range.end,
                old_text: block_text.clone(),
                new_text: String::new(),
            }],
        },
        FileChange {
            path: target_file.to_path_buf(),
            preview_text: target_preview.clone(),
            original_text: target_original,
            hunks: vec![Hunk {
                start_byte: 0,
                end_byte: 0,
                old_text: String::new(),
                new_text: block_text.to_string(),
            }],
        },
    ];

    if source_doc.path == target_file {
        return Err(RefactorError::Invalid(
            "target file must differ from source document".to_string(),
        ));
    }

    Ok(RefactorPlan {
        changes,
        warnings: Vec::new(),
    })
}

pub fn preview_extract_module(
    catalog: &OntologyCatalog,
    entity_iris: &[String],
    output_file: &Path,
    leave_stub: bool,
) -> Result<RefactorPlan> {
    if entity_iris.is_empty() {
        return Err(RefactorError::Invalid("no entities selected".to_string()));
    }

    let mut blocks = Vec::new();
    let mut source_changes: BTreeMap<PathBuf, (String, String, Vec<Hunk>)> = BTreeMap::new();
    let mut prefix_lines = BTreeSet::new();
    let mut warnings = Vec::new();

    for iri in entity_iris {
        let entity = catalog
            .find_entity(iri)
            .ok_or_else(|| RefactorError::EntityNotFound(iri.clone()))?;
        let doc = catalog
            .entity_document(iri)
            .ok_or_else(|| RefactorError::Invalid(format!("no document for {iri}")))?;
        if doc.format != OntologyFormat::Turtle {
            return Err(RefactorError::UnsupportedFormat(doc.format.as_str().to_string()));
        }
        let text = std::fs::read_to_string(&doc.path)?;
        for line in text.lines() {
            if line.trim_start().starts_with("@prefix") {
                prefix_lines.insert(line.trim().to_string());
            }
        }
        let namespaces = ontoindex_owl::namespaces_for_text(&text, &doc.namespaces);
        let block_range = ontoindex_owl::entity_block_range(&text, entity, &namespaces)
            .ok_or_else(|| RefactorError::Invalid(format!("block not found for {iri}")))?;
        let block = text[block_range.start as usize..block_range.end as usize].to_string();
        blocks.push(block.clone());

        let entry = source_changes.entry(doc.path.clone()).or_insert_with(|| {
            (text.clone(), text.clone(), Vec::new())
        });
        if leave_stub {
            let stub = format!(
                "{} a owl:Class ;\n    owl:deprecated true ;\n    rdfs:comment \"Moved to {}\" .\n",
                ontoindex_owl::short_name_from_iri(iri),
                output_file.display()
            );
            entry.1.replace_range(block_range.start as usize..block_range.end as usize, &stub);
            entry.2.push(Hunk {
                start_byte: block_range.start,
                end_byte: block_range.end,
                old_text: block,
                new_text: stub,
            });
        } else {
            entry.1.replace_range(block_range.start as usize..block_range.end as usize, "");
            entry.2.push(Hunk {
                start_byte: block_range.start,
                end_byte: block_range.end,
                old_text: block,
                new_text: String::new(),
            });
        }
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

    changes.push(FileChange {
        path: output_file.to_path_buf(),
        preview_text: module_text.clone(),
        original_text: String::new(),
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

    Ok(RefactorPlan {
        changes,
        warnings,
    })
}
