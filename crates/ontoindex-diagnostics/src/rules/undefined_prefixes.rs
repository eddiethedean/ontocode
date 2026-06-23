use crate::location::find_prefix_in_source;
use crate::input::DiagnosticInput;
use ontoindex_core::{Diagnostic, DiagnosticCode, DiagnosticSeverity, ParseStatus};
use std::collections::BTreeSet;
use std::path::Path;

pub fn undefined_prefixes(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for doc in data.documents {
        if doc.parse_status == ParseStatus::Error {
            continue;
        }
        let text = source(&doc.path);
        let declared: BTreeSet<&str> = doc.namespaces.keys().map(String::as_str).collect();
        let builtins = builtin_prefixes();

        for cap in QNAME_RE.captures_iter(&text) {
            let full = cap.get(0).map(|m| m.as_str()).unwrap_or("");
            if full.contains("://") {
                continue;
            }
            let prefix = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if prefix.is_empty() || declared.contains(prefix) || builtins.contains(prefix) {
                continue;
            }
            let range = find_prefix_in_source(&text, prefix);
            let message = format!("undefined prefix: {prefix}:");
            if diagnostics.iter().any(|d: &Diagnostic| {
                d.file == doc.path
                    && d.code == DiagnosticCode::UndefinedPrefix
                    && d.message == message
            }) {
                continue;
            }
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::UndefinedPrefix,
                severity: DiagnosticSeverity::Error,
                message,
                file: doc.path.clone(),
                range,
                entity_iri: None,
                quick_fix: None,
            });
        }
    }
    diagnostics
}

fn builtin_prefixes() -> BTreeSet<&'static str> {
    [
        "rdf", "rdfs", "owl", "xsd", "xml", "sh", "skos", "dc", "dcterms", "foaf", "schema",
    ]
    .into_iter()
    .collect()
}

static QNAME_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"([A-Za-z][\w-]*):[A-Za-z_][\w-]*").expect("qname regex")
});
