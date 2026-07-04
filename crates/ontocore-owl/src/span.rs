use ontocore_core::{Annotation, Axiom, Entity, SourceLocation};
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

/// Parse `@prefix` declarations from Turtle source (skips `#` comments; requires `<iri>` form).
pub fn prefixes_from_turtle(source_text: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in source_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@prefix ") {
            let mut parts = rest.split_whitespace();
            let Some(prefix_raw) = parts.next() else { continue };
            let Some(uri_raw) = parts.next() else { continue };
            let prefix = prefix_raw.trim_end_matches(':').to_string();
            if let Some(uri) = parse_turtle_prefix_iri(uri_raw) {
                if !prefix.is_empty() {
                    map.insert(prefix, uri);
                }
            }
        } else if !trimmed.starts_with('@') {
            break;
        }
    }
    map
}

fn parse_turtle_prefix_iri(token: &str) -> Option<String> {
    let token = token.trim_end_matches('.');
    if !token.starts_with('<') || !token.ends_with('>') {
        return None;
    }
    let inner = &token[1..token.len() - 1];
    if inner.is_empty() || inner.contains('<') || inner.contains('>') {
        return None;
    }
    Some(inner.to_string())
}

pub fn namespaces_for_text(
    source_text: &str,
    declared: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut merged = declared.clone();
    merged.extend(prefixes_from_turtle(source_text));
    merged
}

fn subject_needles(
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut needles = vec![format!("<{iri}>")];
    // Only match default-prefix form when the empty prefix is bound to this IRI's namespace.
    if let Some(default_ns) = namespaces.get("") {
        if iri.starts_with(default_ns.as_str()) {
            needles.push(format!(":{short_name}"));
        }
    }
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns.as_str()) && !prefix.is_empty() {
            needles.push(format!("{prefix}:{short_name}"));
        }
    }
    needles
}

/// Byte offset immediately after a line's content (handles `\n` and `\r\n`).
fn line_byte_offset_after(text: &str, line_start: usize, line: &str) -> usize {
    let after_content = line_start + line.len();
    let bytes = text.as_bytes();
    if bytes.get(after_content) == Some(&b'\r') && bytes.get(after_content + 1) == Some(&b'\n') {
        after_content + 2
    } else if bytes.get(after_content) == Some(&b'\n') {
        after_content + 1
    } else {
        after_content
    }
}

/// Locate the byte range of an entity's subject token (first Turtle statement for that subject).
#[allow(dead_code)]
pub fn find_subject_statement(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    all_subject_statements(source_text, iri, short_name, namespaces).into_iter().next()
}

/// All statement start positions for a subject IRI in document order.
pub fn all_subject_statements(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<ByteRange> {
    let needles = subject_needles(iri, short_name, namespaces);
    let mut byte_offset = 0usize;
    let mut out = Vec::new();
    for line in source_text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('@') {
            byte_offset = line_byte_offset_after(source_text, byte_offset, line);
            continue;
        }
        if let Some(col_in_trimmed) = subject_column_at_line_start(trimmed, &needles) {
            let ws = line.len() - trimmed.len();
            let start = byte_offset + ws + col_in_trimmed;
            out.push(ByteRange { start: start as u64, end: start as u64 });
        }
        byte_offset = line_byte_offset_after(source_text, byte_offset, line);
    }
    out
}

/// Full byte ranges for every Turtle statement whose subject is `iri`.
pub fn all_entity_statement_ranges(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<ByteRange> {
    all_subject_statements(source_text, iri, short_name, namespaces)
        .into_iter()
        .filter_map(|start_range| {
            let end = statement_end_byte(source_text, start_range.start as usize)?;
            if end > start_range.start as usize {
                Some(ByteRange { start: start_range.start, end: end as u64 })
            } else {
                None
            }
        })
        .collect()
}

/// Prefer the subject statement that declares a type (` a `), e.g. `ex:Foo a owl:Class`.
fn primary_entity_statement_start(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    let starts = all_subject_statements(source_text, iri, short_name, namespaces);
    for start in &starts {
        if let Some(end) = statement_end_byte(source_text, start.start as usize) {
            let stmt = &source_text[start.start as usize..end];
            if stmt.contains(" a ") {
                return Some(*start);
            }
        }
    }
    starts.first().copied()
}

/// True when `needle` is the subject at the start of a trimmed Turtle line.
fn subject_column_at_line_start(trimmed: &str, needles: &[String]) -> Option<usize> {
    for needle in needles {
        if !trimmed.starts_with(needle) {
            continue;
        }
        let rest = &trimmed[needle.len()..];
        if rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == ';' || c == '.')
        {
            return Some(0);
        }
    }
    None
}

/// Locate the primary entity block (type declaration) in Turtle source.
pub fn find_entity_block(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> SourceLocation {
    if let Some(range) = primary_entity_statement_start(source_text, iri, short_name, namespaces) {
        let start = range.start as usize;
        let line_idx = source_text[..start].matches('\n').count();
        let line_start = source_text[..start].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let col = start - line_start;
        return SourceLocation {
            line: Some((line_idx + 1) as u64),
            column: Some(col as u64),
            start_byte: Some(range.start),
            end_byte: None,
        };
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
        if let Some(block) = entity_block_range(source_text, entity, &BTreeMap::new()) {
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
        if axiom.axiom_kind == ontocore_core::AXIOM_KIND_SUB_CLASS_OF {
            if let Some(span) = find_subclass_span(source_text, &axiom.subject, &axiom.object) {
                axiom.source_location = span;
            }
        }
    }
}

pub fn entity_block_range(
    source_text: &str,
    entity: &Entity,
    declared_namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    let namespaces = namespaces_for_text(source_text, declared_namespaces);
    let start = if let Some(s) = entity.source_location.start_byte {
        s as usize
    } else {
        primary_entity_statement_start(source_text, &entity.iri, &entity.short_name, &namespaces)
            .map(|r| r.start as usize)?
    };
    let end = statement_end_byte(source_text, start)?;
    if end > start {
        Some(ByteRange { start: start as u64, end: end as u64 })
    } else {
        None
    }
}

/// Primary declaration block for patch insertions (type statement, not trailing triples).
pub fn entity_primary_block_range(
    source_text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    let short_name = short_name_from_iri(entity_iri);
    let start = primary_entity_statement_start(source_text, entity_iri, &short_name, namespaces)?;
    let end = statement_end_byte(source_text, start.start as usize)?;
    if end > start.start as usize {
        Some(ByteRange { start: start.start, end: end as u64 })
    } else {
        None
    }
}

/// True when `b` can appear inside a Turtle PN_LOCAL / prefixed-name token.
pub(crate) fn is_turtle_name_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b':' | b'~' | b'%' | b'\\' | b'.')
}

/// True when `.` at `i` is a statement/object terminator, not part of PN_LOCAL or an IRI.
///
/// Turtle allows `.` inside local names (`ex:foo.bar`) and absolute IRIs (`<http://a.b/c>`).
/// A terminating `.` is never both preceded and followed by name characters.
pub(crate) fn is_turtle_terminating_dot(bytes: &[u8], i: usize) -> bool {
    if bytes.get(i) != Some(&b'.') {
        return false;
    }
    let prev_name = i > 0 && is_turtle_name_char(bytes[i - 1]) && bytes[i - 1] != b'.';
    let next_name = bytes.get(i + 1).is_some_and(|b| is_turtle_name_char(*b) && *b != b'.');
    !(prev_name && next_name)
}

#[derive(Clone, Copy)]
enum TurtleStringKind {
    ShortDouble,
    ShortSingle,
    LongDouble,
    LongSingle,
}

/// Walk from subject start to the terminating `.` of this statement.
///
/// Tracks `"…"`, `'…'`, `"""…"""`, `'''…'''` strings, IRIs (`<...>`), `#` line comments,
/// brackets, and parens. Does not treat `.` inside IRIs, comments, strings, or PN_LOCAL
/// names (`ex:foo.bar`) as terminators.
pub(crate) fn statement_end_byte(source_text: &str, start: usize) -> Option<usize> {
    let bytes = source_text.as_bytes();
    if start >= bytes.len() {
        return None;
    }
    let mut i = start;
    let mut bracket_depth = 0i32;
    let mut paren_depth = 0i32;
    let mut string_kind: Option<TurtleStringKind> = None;
    let mut in_iri = false;
    let mut in_comment = false;
    let mut escape = false;

    while i < bytes.len() {
        let b = bytes[i];
        if in_comment {
            if b == b'\n' {
                in_comment = false;
            }
            i += 1;
            continue;
        }
        if let Some(kind) = string_kind {
            match kind {
                TurtleStringKind::ShortDouble | TurtleStringKind::ShortSingle => {
                    let quote = match kind {
                        TurtleStringKind::ShortDouble => b'"',
                        _ => b'\'',
                    };
                    if escape {
                        escape = false;
                    } else if b == b'\\' {
                        escape = true;
                    } else if b == quote {
                        string_kind = None;
                    }
                    i += 1;
                    continue;
                }
                TurtleStringKind::LongDouble => {
                    if bytes.get(i..i + 3) == Some(br#"""""#) {
                        string_kind = None;
                        i += 3;
                        continue;
                    }
                    i += 1;
                    continue;
                }
                TurtleStringKind::LongSingle => {
                    if bytes.get(i..i + 3) == Some(br"'''") {
                        string_kind = None;
                        i += 3;
                        continue;
                    }
                    i += 1;
                    continue;
                }
            }
        }
        if in_iri {
            if b == b'>' {
                in_iri = false;
            }
            i += 1;
            continue;
        }

        if bytes.get(i..i + 3) == Some(br#"""""#) {
            string_kind = Some(TurtleStringKind::LongDouble);
            i += 3;
            continue;
        }
        if bytes.get(i..i + 3) == Some(br"'''") {
            string_kind = Some(TurtleStringKind::LongSingle);
            i += 3;
            continue;
        }

        match b {
            b'#' => in_comment = true,
            b'"' => string_kind = Some(TurtleStringKind::ShortDouble),
            b'\'' => string_kind = Some(TurtleStringKind::ShortSingle),
            b'<' => in_iri = true,
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' => paren_depth += 1,
            b')' => paren_depth = paren_depth.saturating_sub(1),
            b'.' if bracket_depth == 0
                && paren_depth == 0
                && is_turtle_terminating_dot(bytes, i) =>
            {
                return Some(i + 1);
            }
            _ => {}
        }
        i += 1;
    }
    None
}

#[derive(Debug, Clone, Copy)]
pub struct ByteRange {
    pub start: u64,
    pub end: u64,
}

fn line_start_byte(source_text: &str, line_idx: usize) -> u64 {
    let mut offset = 0usize;
    for (i, line) in source_text.lines().enumerate() {
        if i == line_idx {
            return offset as u64;
        }
        offset = line_byte_offset_after(source_text, offset, line);
    }
    0
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
    use ontocore_core::{Entity, EntityKind};

    fn ex_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".into(), "http://example.org/people#".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ])
    }

    fn clinic_ns() -> BTreeMap<String, String> {
        BTreeMap::from([("ex".into(), "http://example.org/clinic#".into())])
    }

    #[test]
    fn finds_entity_line() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let loc = find_entity_block(ttl, "http://example.org/people#Person", "Person", &ex_ns());
        assert!(loc.line.is_some());
        let line = ttl.lines().nth(loc.line.unwrap() as usize - 1).unwrap();
        assert!(line.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn subject_not_domain_mention() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let range =
            find_subject_statement(ttl, "http://example.org/people#Person", "Person", &ex_ns())
                .expect("subject");
        let line = ttl[..range.start as usize].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let subject_line = &ttl[line..];
        assert!(subject_line.starts_with("ex:Person a"));
    }

    #[test]
    fn patient_block_includes_multiline_restriction() {
        let ttl = include_str!("../../../fixtures/complex-classes.ttl");
        let entity = Entity {
            iri: "http://example.org/clinic#Patient".into(),
            short_name: "Patient".into(),
            kind: EntityKind::Class,
            ontology_id: String::new(),
            source_location: find_entity_block(
                ttl,
                "http://example.org/clinic#Patient",
                "Patient",
                &clinic_ns(),
            ),
            labels: vec![],
            comments: vec![],
            deprecated: false,
            obo_id: None,
        };
        let range = entity_block_range(ttl, &entity, &clinic_ns()).expect("block");
        let block = &ttl[range.start as usize..range.end as usize];
        assert!(block.contains("owl:Restriction"));
        assert!(block.contains("owl:someValuesFrom"));
        assert!(block.trim_end().ends_with('.'));
    }

    #[test]
    fn add_label_targets_class_block_not_trailing_triple() {
        use crate::patch::{apply_patches_to_text, PatchOp};
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/people#Person".into(),
            value: "Human".into(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        let person_block_start = preview.find("ex:Person a owl:Class").expect("class decl");
        let human_pos = preview.find("Human").expect("label");
        assert!(human_pos > person_block_start);
        let trailing = preview.find("ex:Person rdfs:subClassOf ex:Thing");
        if let Some(t) = trailing {
            assert!(human_pos < t, "label must be in class block, not trailing triple");
        }
    }

    #[test]
    fn statement_end_respects_dots_in_absolute_iris() {
        let ttl = "<http://example.org/people#Person> a owl:Class ;\n    rdfs:label \"Person\" .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        let block = &ttl[..end];
        assert!(block.contains("rdfs:label"));
        assert!(block.trim_end().ends_with('.'));
        assert!(!block.ends_with("example."));
    }

    #[test]
    fn statement_end_respects_dots_in_comments() {
        let ttl = "ex:Person a owl:Class ; # see docs.\n    rdfs:label \"Person\" .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        let block = &ttl[..end];
        assert!(block.contains("rdfs:label"));
        assert!(block.contains("# see docs."));
    }

    #[test]
    fn statement_end_respects_dots_in_local_names() {
        let ttl = "ex:foo.bar a owl:Class .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        assert_eq!(&ttl[..end], "ex:foo.bar a owl:Class .");
    }

    #[test]
    fn statement_end_respects_dots_in_long_strings() {
        let ttl = "ex:Person a owl:Class ;\n    rdfs:comment '''See Dr. Smith.''' .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        let block = &ttl[..end];
        assert!(block.contains("Dr. Smith"));
        assert!(block.trim_end().ends_with('.'));
    }
}
