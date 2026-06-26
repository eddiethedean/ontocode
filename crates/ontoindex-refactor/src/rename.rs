use crate::error::{RefactorError, Result};
use crate::model::{FileChange, Hunk, RefactorPlan};
use crate::source::read_source_text;
use crate::text::{normalize_namespace_base, remap_iri, replace_iri_in_text};
use ontoindex_catalog::OntologyCatalog;
use ontoindex_core::{EntityKind, OntologyFormat, ParseStatus};
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
                let new_ns = if to.ends_with('#') || to.ends_with('/') {
                    to.clone()
                } else {
                    format!("{to}#")
                };
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

fn find_usages_in_catalog(
    catalog: &OntologyCatalog,
    iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Vec<()> {
    let u = crate::usages::find_usages_with_overrides(catalog, iri, document_overrides);
    vec![(); u.len()]
}

fn text_contains_iri(
    doc: &ontoindex_core::OntologyDocument,
    iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> bool {
    read_source_text(&doc.path, document_overrides).map(|t| t.contains(iri)).unwrap_or(false)
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

fn prefixed_curie(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    let short = ontoindex_owl::short_name_from_iri(iri);
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
        EntityKind::Ontology | EntityKind::Other => "owl:Class",
    }
}

struct EntityRemoval {
    path: PathBuf,
    start: u64,
    end: u64,
    replacement: String,
}

pub fn preview_refactor(
    catalog: &OntologyCatalog,
    request: &crate::model::RefactorRequest,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    match request {
        crate::model::RefactorRequest::RenameIri { from_iri, to_iri } => {
            preview_rename_iri(catalog, from_iri, to_iri, document_overrides)
        }
        crate::model::RefactorRequest::MigrateNamespace { from_base, to_base } => {
            preview_migrate_namespace(catalog, from_base, to_base, document_overrides)
        }
        crate::model::RefactorRequest::MoveEntity { entity_iri, target_file } => {
            preview_move_entity(catalog, entity_iri, target_file, document_overrides)
        }
        crate::model::RefactorRequest::ExtractModule { entity_iris, output_file, leave_stub } => {
            preview_extract_module(
                catalog,
                entity_iris,
                output_file,
                *leave_stub,
                document_overrides,
            )
        }
    }
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub fn preview_move_entity(
    catalog: &OntologyCatalog,
    entity_iri: &str,
    target_file: &Path,
    document_overrides: &HashMap<PathBuf, String>,
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

    let source_canon = canonical_path(&source_doc.path);
    let target_canon = canonical_path(target_file);
    if source_canon == target_canon {
        return Err(RefactorError::Invalid(
            "target file must differ from source document".to_string(),
        ));
    }

    let source_text = read_source_text(&source_doc.path, document_overrides)?;
    let namespaces = ontoindex_owl::namespaces_for_text(&source_text, &source_doc.namespaces);
    let block_range = ontoindex_owl::entity_block_range(&source_text, entity, &namespaces)
        .ok_or_else(|| {
            RefactorError::Invalid(format!("entity block not found for {entity_iri}"))
        })?;

    let block_text = source_text[block_range.start as usize..block_range.end as usize].to_string();
    let mut source_without = source_text.clone();
    source_without.replace_range(block_range.start as usize..block_range.end as usize, "");

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
                hunks: vec![Hunk {
                    start_byte: block_range.start,
                    end_byte: block_range.end,
                    old_text: block_text.clone(),
                    new_text: String::new(),
                }],
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
) -> Result<RefactorPlan> {
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
            if line.trim_start().starts_with("@prefix") {
                prefix_lines.insert(line.trim().to_string());
            }
        }
        let namespaces = ontoindex_owl::namespaces_for_text(&text, &doc.namespaces);
        let block_range = ontoindex_owl::entity_block_range(&text, entity, &namespaces)
            .ok_or_else(|| RefactorError::Invalid(format!("block not found for {iri}")))?;
        let block = text[block_range.start as usize..block_range.end as usize].to_string();
        blocks.push(block.clone());

        let replacement = if leave_stub {
            let owl_type = owl_type_for_kind(entity.kind);
            format!(
                "{} a {owl_type} ;\n    owl:deprecated true ;\n    rdfs:comment \"Moved to {}\" .\n",
                prefixed_curie(iri, &namespaces),
                output_file.display()
            )
        } else {
            String::new()
        };
        removals.push(EntityRemoval {
            path: doc.path.clone(),
            start: block_range.start,
            end: block_range.end,
            replacement,
        });
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

    Ok(RefactorPlan { changes, warnings })
}
