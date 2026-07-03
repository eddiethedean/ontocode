use ontocore_core::document_lookup::normalize_iri;
use ontocore_owl::{namespaces_for_text, short_name_from_iri};
use std::collections::BTreeMap;

/// Build replacement needles for an IRI in Turtle source (reserved for future prefix-aware renames).
#[allow(dead_code)]
pub fn iri_replacement_needles(
    iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<(String, String)> {
    let short = short_name_from_iri(iri);
    let mut needles = vec![(format!("<{iri}>"), iri.to_string())];
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            needles.push((format!("{prefix}:{short}"), format!("{prefix}:{short}")));
        }
    }
    needles
}

pub fn normalize_namespace_base(base: &str) -> String {
    normalize_iri(base)
}

pub fn remap_iri(iri: &str, from_base: &str, to_base: &str) -> Option<String> {
    let from = normalize_namespace_base(from_base);
    let to = normalize_namespace_base(to_base);
    if iri == from || iri.starts_with(&format!("{from}#")) || iri.starts_with(&format!("{from}/")) {
        let suffix = if iri.len() > from.len() { &iri[from.len()..] } else { "" };
        Some(format!("{to}{suffix}"))
    } else {
        None
    }
}

/// Replace all occurrences of `old_iri` with `new_iri` in Turtle text, preserving prefix forms.
pub fn replace_iri_in_text(
    text: &str,
    old_iri: &str,
    new_iri: &str,
    declared_namespaces: &BTreeMap<String, String>,
) -> (String, Vec<(usize, usize, String, String)>) {
    let namespaces = namespaces_for_text(text, declared_namespaces);
    let old_short = short_name_from_iri(old_iri);
    let new_short = short_name_from_iri(new_iri);
    let mut hunks = Vec::new();
    let mut result = text.to_string();

    let mut replacements: Vec<(String, String)> =
        vec![(format!("<{old_iri}>"), format!("<{new_iri}>"))];
    if old_iri != new_iri {
        replacements.push((old_iri.to_string(), new_iri.to_string()));
    }

    for (prefix, ns) in &namespaces {
        if old_iri.starts_with(ns) && !prefix.is_empty() {
            let old_token = format!("{prefix}:{old_short}");
            let new_ns = namespace_for_iri(new_iri, &namespaces).unwrap_or_else(|| {
                new_iri
                    .rsplit_once('#')
                    .or_else(|| new_iri.rsplit_once('/'))
                    .map(|(base, _)| base.to_string())
                    .unwrap_or_default()
            });
            let new_prefix =
                prefix_for_namespace(&new_ns, &namespaces).unwrap_or_else(|| prefix.clone());
            let new_token = format!("{new_prefix}:{new_short}");
            if old_token != new_token {
                replacements.push((old_token, new_token));
            }
        }
    }

    for (old, new) in &replacements {
        if old == new {
            continue;
        }
        let mut search_from = 0usize;
        while let Some(pos) = result[search_from..].find(old) {
            let start = search_from + pos;
            let end = start + old.len();
            if !is_safe_replacement_boundary(&result, start, end) {
                search_from = end;
                continue;
            }
            hunks.push((start, end, old.clone(), new.clone()));
            result.replace_range(start..end, new);
            search_from = start + new.len();
        }
    }

    (result, hunks)
}

fn namespace_for_iri(iri: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    let mut best: Option<(usize, String)> = None;
    for ns in namespaces.values() {
        if iri.starts_with(ns) {
            let len = ns.len();
            if best.as_ref().is_none_or(|(l, _)| len > *l) {
                best = Some((len, ns.clone()));
            }
        }
    }
    best.map(|(_, ns)| ns)
}

fn prefix_for_namespace(ns: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    namespaces.iter().find(|(_, v)| normalize_iri(v) == normalize_iri(ns)).map(|(p, _)| p.clone())
}

/// True when `needle` at `col` in `line` is a standalone Turtle token (not a substring).
pub fn is_token_match_at(line: &str, needle: &str, col: usize) -> bool {
    if !line[col..].starts_with(needle) {
        return false;
    }
    is_safe_replacement_boundary(line, col, col + needle.len())
}

pub fn is_safe_replacement_boundary(text: &str, start: usize, end: usize) -> bool {
    let before = text.as_bytes().get(start.wrapping_sub(1)).copied();
    let after = text.as_bytes().get(end).copied();
    let ok_before = before.is_none_or(|b| !b.is_ascii_alphanumeric() && b != b':' && b != b'#');
    let ok_after =
        after.is_none_or(|b| !b.is_ascii_alphanumeric() && b != b':' && b != b'#' && b != b'/');
    ok_before && ok_after
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_iri_in_angle_brackets() {
        let ttl = "@prefix ex: <http://example.org#> .\nex:Foo a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, hunks) =
            replace_iri_in_text(ttl, "http://example.org#Foo", "http://example.org#Bar", &ns);
        assert!(!hunks.is_empty());
        assert!(out.contains("ex:Bar"));
        assert!(!out.contains("ex:Foo"));
    }

    #[test]
    fn replace_iri_does_not_corrupt_slash_extended_iri() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   ex:Role a owl:Class .\n\
                   <http://example.org#Person/role> a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Agent", &ns);
        assert!(out.contains("<http://example.org#Person/role>"));
        assert!(!out.contains("<http://example.org#Agent/role>"));
    }
}
