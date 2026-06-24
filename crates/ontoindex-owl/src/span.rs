use ontoindex_core::{Annotation, Axiom, Entity, SourceLocation};
use std::collections::BTreeMap;

pub fn short_name_from_iri(iri: &str) -> String {
    if let Some((_, name)) = iri.rsplit_once('#') {
        return name.to_string();
    }
    if let Some((_, name)) = iri.rsplit_once('/') {
        return name.to_string();
    }
    iri.to_string()
}

/// Locate the first occurrence of an entity subject in Turtle source.
pub fn find_entity_block(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> SourceLocation {
    let mut needles = vec![iri.to_string(), format!("<{iri}>"), format!(":{short_name}")];
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            needles.push(format!("{prefix}:{short_name}"));
        }
    }

    for (line_idx, line) in source_text.lines().enumerate() {
        for needle in &needles {
            if let Some(col) = line.find(needle) {
                let start_byte = line_start_byte(source_text, line_idx);
                return SourceLocation {
                    line: Some((line_idx + 1) as u64),
                    column: Some(col as u64),
                    start_byte: Some(start_byte + col as u64),
                    end_byte: None,
                };
            }
        }
    }
    SourceLocation::default()
}

/// Fill byte spans for labels, comments, and subclass triples.
pub fn annotate_spans(
    source_text: &str,
    entities: &mut [Entity],
    annotations: &mut [Annotation],
    axioms: &mut [Axiom],
) {
    for entity in entities.iter_mut() {
        if let Some(block) = entity_block_range(source_text, entity) {
            if entity.source_location.end_byte.is_none() {
                entity.source_location.end_byte = Some(block.end);
            }
        }
    }

    for ann in annotations.iter_mut() {
        if let Some(span) = find_predicate_literal_span(
            source_text,
            &ann.subject,
            predicate_local_name(&ann.predicate),
            &ann.object,
        ) {
            ann.source_location = span;
        }
    }

    for axiom in axioms.iter_mut() {
        if axiom.axiom_kind == ontoindex_core::AXIOM_KIND_SUB_CLASS_OF {
            if let Some(span) = find_subclass_span(source_text, &axiom.subject, &axiom.object) {
                axiom.source_location = span;
            }
        }
    }
}

pub fn entity_block_range(source_text: &str, entity: &Entity) -> Option<ByteRange> {
    let start = entity.source_location.start_byte?;
    let mut end = start;
    let bytes = source_text.as_bytes();
    let mut i = start as usize;
    let mut subject_seen = false;
    while i < bytes.len() {
        if bytes[i] == b'.' {
            let line_start =
                bytes[..=i].iter().rposition(|&b| b == b'\n').map(|p| p + 1).unwrap_or(0);
            let line = &source_text[line_start..=i];
            if subject_seen || line_contains_subject(line, entity) {
                end = (i + 1) as u64;
                if !line.trim_end().ends_with(';') {
                    break;
                }
                subject_seen = true;
            }
        }
        i += 1;
    }
    if end > start {
        Some(ByteRange { start, end })
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ByteRange {
    pub start: u64,
    pub end: u64,
}

fn line_start_byte(source_text: &str, line_idx: usize) -> u64 {
    source_text.lines().take(line_idx).map(|l| l.len() as u64 + 1).sum()
}

fn line_contains_subject(line: &str, entity: &Entity) -> bool {
    let trimmed = line.trim();
    trimmed.contains(&entity.iri)
        || trimmed.contains(&format!("<{}>", entity.iri))
        || trimmed.contains(&format!(":{}", entity.short_name))
}

fn predicate_local_name(predicate: &str) -> &str {
    if predicate.contains("label") {
        "label"
    } else if predicate.contains("comment") {
        "comment"
    } else if predicate.contains("subClassOf") {
        "subClassOf"
    } else {
        predicate
    }
}

fn find_predicate_literal_span(
    source_text: &str,
    subject: &str,
    predicate: &str,
    object: &str,
) -> Option<SourceLocation> {
    let object_needle = object.trim_matches('"');
    for (line_idx, line) in source_text.lines().enumerate() {
        if !line.contains(subject) && !line.contains(&short_name_from_iri(subject)) {
            continue;
        }
        if line.contains(predicate) && line.contains(object_needle) {
            let col = line.find(predicate).unwrap_or(0);
            let start_byte = line_start_byte(source_text, line_idx) + col as u64;
            return Some(SourceLocation {
                line: Some((line_idx + 1) as u64),
                column: Some(col as u64),
                start_byte: Some(start_byte),
                end_byte: Some(start_byte + line.len() as u64),
            });
        }
    }
    None
}

fn find_subclass_span(source_text: &str, subject: &str, parent: &str) -> Option<SourceLocation> {
    let parent_short = short_name_from_iri(parent);
    for (line_idx, line) in source_text.lines().enumerate() {
        if !line.contains("subClassOf") {
            continue;
        }
        if (line.contains(subject) || line.contains(&short_name_from_iri(subject)))
            && (line.contains(parent) || line.contains(&parent_short))
        {
            let start_byte = line_start_byte(source_text, line_idx);
            return Some(SourceLocation {
                line: Some((line_idx + 1) as u64),
                column: Some(0),
                start_byte: Some(start_byte),
                end_byte: Some(start_byte + line.len() as u64),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_entity_line() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let loc = find_entity_block(
            ttl,
            "http://example.org/people#Person",
            "Person",
            &BTreeMap::from([("ex".into(), "http://example.org/people#".into())]),
        );
        assert!(loc.line.is_some());
    }
}
