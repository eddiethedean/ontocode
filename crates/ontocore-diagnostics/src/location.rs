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
    let mut needles = vec![iri.to_string(), format!("<{iri}>"), format!(":{short_name}")];
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            needles.push(format!("{prefix}:{short_name}"));
        }
    }
    needles
}
