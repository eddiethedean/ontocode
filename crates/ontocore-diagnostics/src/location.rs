use ontocore_core::SourceLocation;
use std::collections::BTreeMap;

pub fn find_in_source(source_text: &str, needles: &[String]) -> SourceLocation {
    for (line_idx, line) in source_text.lines().enumerate() {
        for needle in needles {
            if let Some(col) = line.find(needle) {
                return SourceLocation::at_line_col((line_idx + 1) as u64, col as u64);
            }
        }
    }
    SourceLocation::default()
}

pub fn find_prefix_in_source(source_text: &str, prefix: &str) -> SourceLocation {
    let needles = vec![
        format!("@prefix {prefix}:"),
        format!("@PREFIX {prefix}:"),
        format!("PREFIX {prefix}:"),
        format!("{prefix}:"),
    ];
    find_in_source(source_text, &needles)
}

pub fn entity_needles(
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut needles = vec![iri.to_string(), format!("<{iri}>")];
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            needles.push(format!("{prefix}:{short_name}"));
        }
    }
    // Bare `:LocalName` last — it is a substring of `{prefix}:{short_name}`.
    needles.push(format!(":{short_name}"));
    needles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn entity_needles_prefixed_match_before_bare_local() {
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/ex#".to_string());
        let needles = entity_needles("http://example.org/ex#Person", "Person", &namespaces);
        let line = "ex:Person a owl:Class .";
        let loc = find_in_source(line, &needles);
        assert_eq!(loc.line, Some(1));
        assert_eq!(loc.column, Some(0));
    }

    #[test]
    fn entity_needles_bare_local_still_matches_default_prefix() {
        let namespaces = BTreeMap::new();
        let needles = entity_needles("http://example.org/ex#Person", "Person", &namespaces);
        let line = ":Person a owl:Class .";
        let loc = find_in_source(line, &needles);
        assert_eq!(loc.line, Some(1));
        assert_eq!(loc.column, Some(0));
    }
}
