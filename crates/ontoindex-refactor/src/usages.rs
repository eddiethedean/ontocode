use crate::model::{Usage, UsageKind};
use ontoindex_catalog::OntologyCatalog;
use ontoindex_core::{OntologyFormat, ParseStatus};
use ontoindex_owl::{namespaces_for_text, short_name_from_iri};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// Find all usages of `target_iri` across the indexed workspace.
pub fn find_usages(catalog: &OntologyCatalog, target_iri: &str) -> Vec<Usage> {
    let mut usages = Vec::new();
    let data = catalog.data();
    let mut seen = BTreeSet::new();

    for entity in &data.entities {
        if entity.iri == target_iri {
            if let Some(doc) = catalog.entity_document(&entity.iri) {
                let key = (doc.path.clone(), UsageKind::EntityDeclaration, entity.iri.clone());
                if seen.insert(key.clone()) {
                    usages.push(Usage {
                        iri: entity.iri.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: entity.source_location.line,
                        column: entity.source_location.column,
                        start_byte: entity.source_location.start_byte,
                        end_byte: entity.source_location.end_byte,
                        kind: UsageKind::EntityDeclaration,
                        context: format!("{} declaration", entity.kind.as_str()),
                    });
                }
            }
        }
    }

    for axiom in &data.axioms {
        if axiom.subject == target_iri {
            if let Some(doc) = data.documents.iter().find(|d| d.id == axiom.ontology_id) {
                let key = (doc.path.clone(), UsageKind::AxiomSubject, axiom.id.clone());
                if seen.insert(key) {
                    usages.push(usage_from_axiom(
                        doc.path.clone(),
                        target_iri,
                        axiom,
                        UsageKind::AxiomSubject,
                    ));
                }
            }
        }
        if axiom.object == target_iri || is_named_ref(&axiom.object, target_iri) {
            if let Some(doc) = data.documents.iter().find(|d| d.id == axiom.ontology_id) {
                let key = (doc.path.clone(), UsageKind::AxiomObject, axiom.id.clone());
                if seen.insert(key) {
                    usages.push(usage_from_axiom(
                        doc.path.clone(),
                        target_iri,
                        axiom,
                        UsageKind::AxiomObject,
                    ));
                }
            }
        }
    }

    for ann in &data.annotations {
        if ann.subject == target_iri {
            if let Some(doc) = data.documents.iter().find(|d| d.id == ann.ontology_id) {
                let key = (doc.path.clone(), UsageKind::AnnotationSubject, ann.subject.clone());
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: ann.subject.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: ann.source_location.line,
                        column: ann.source_location.column,
                        start_byte: ann.source_location.start_byte,
                        end_byte: ann.source_location.end_byte,
                        kind: UsageKind::AnnotationSubject,
                        context: format!("annotation subject {}", ann.predicate),
                    });
                }
            }
        }
        if ann.object == target_iri {
            if let Some(doc) = data.documents.iter().find(|d| d.id == ann.ontology_id) {
                let key = (
                    doc.path.clone(),
                    UsageKind::AnnotationObject,
                    format!("{}-{}", ann.subject, ann.predicate),
                );
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: ann.subject.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: ann.source_location.line,
                        column: ann.source_location.column,
                        start_byte: ann.source_location.start_byte,
                        end_byte: ann.source_location.end_byte,
                        kind: UsageKind::AnnotationObject,
                        context: format!("annotation object {}", ann.predicate),
                    });
                }
            }
        }
    }

    for imp in &data.imports {
        if imp.import_iri == target_iri {
            if let Some(doc) = data.documents.iter().find(|d| d.id == imp.ontology_id) {
                let key = (doc.path.clone(), UsageKind::Import, imp.import_iri.clone());
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: imp.ontology_id.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: None,
                        column: None,
                        start_byte: None,
                        end_byte: None,
                        kind: UsageKind::Import,
                        context: format!("import {}", imp.import_iri),
                    });
                }
            }
        }
    }

    for doc in &data.documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&doc.path) {
            let namespaces = namespaces_for_text(&text, &doc.namespaces);
            let short = short_name_from_iri(target_iri);
            let needles = [format!("<{target_iri}>"), format!(":{short}")];
            for (line_idx, line) in text.lines().enumerate() {
                for needle in &needles {
                    if !line.contains(needle.as_str()) {
                        continue;
                    }
                    if let Some(col) = line.find(needle) {
                        let key = (
                            doc.path.clone(),
                            UsageKind::TextReference,
                            format!("{line_idx}-{col}-{needle}"),
                        );
                        if seen.insert(key) {
                            usages.push(Usage {
                                iri: doc.id.clone(),
                                referenced_iri: target_iri.to_string(),
                                file: doc.path.clone(),
                                line: Some((line_idx + 1) as u64),
                                column: Some(col as u64),
                                start_byte: None,
                                end_byte: None,
                                kind: UsageKind::TextReference,
                                context: line.trim().to_string(),
                            });
                        }
                    }
                }
                for (prefix, ns) in &namespaces {
                    if target_iri.starts_with(ns) && !prefix.is_empty() {
                        let token = format!("{prefix}:{short}");
                        if line.contains(&token) {
                            if let Some(col) = line.find(&token) {
                                let key = (
                                    doc.path.clone(),
                                    UsageKind::TextReference,
                                    format!("{line_idx}-{col}-{token}"),
                                );
                                if seen.insert(key) {
                                    usages.push(Usage {
                                        iri: doc.id.clone(),
                                        referenced_iri: target_iri.to_string(),
                                        file: doc.path.clone(),
                                        line: Some((line_idx + 1) as u64),
                                        column: Some(col as u64),
                                        start_byte: None,
                                        end_byte: None,
                                        kind: UsageKind::TextReference,
                                        context: line.trim().to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    usages.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.unwrap_or(0).cmp(&b.line.unwrap_or(0)))
            .then(a.column.unwrap_or(0).cmp(&b.column.unwrap_or(0)))
    });
    usages
}

fn usage_from_axiom(
    path: PathBuf,
    target_iri: &str,
    axiom: &ontoindex_core::Axiom,
    kind: UsageKind,
) -> Usage {
    Usage {
        iri: axiom.subject.clone(),
        referenced_iri: target_iri.to_string(),
        file: path,
        line: axiom.source_location.line,
        column: axiom.source_location.column,
        start_byte: axiom.source_location.start_byte,
        end_byte: axiom.source_location.end_byte,
        kind,
        context: format!("{} {}", axiom.axiom_kind, axiom.object),
    }
}

fn is_named_ref(object: &str, target_iri: &str) -> bool {
    object == target_iri || object == format!("<{target_iri}>")
}
