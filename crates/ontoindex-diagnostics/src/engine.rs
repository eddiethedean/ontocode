use crate::input::DiagnosticInput;
use crate::rules::{
    broken_imports, duplicate_labels, missing_labels, orphan_classes, parse_errors,
    undefined_prefixes,
};
use ontoindex_core::Diagnostic;
use std::collections::HashMap;
use std::path::PathBuf;

/// Collect all diagnostics for a catalog snapshot.
pub fn collect_diagnostics(input: &DiagnosticInput<'_>) -> Vec<Diagnostic> {
    collect_diagnostics_with_sources(input, &HashMap::new())
}

/// Collect diagnostics, optionally using in-memory source text overrides (LSP open buffers).
pub fn collect_diagnostics_with_sources(
    input: &DiagnosticInput<'_>,
    source_overrides: &HashMap<PathBuf, String>,
) -> Vec<Diagnostic> {
    let source = |path: &std::path::Path| -> String {
        if let Some(text) = source_overrides.get(path) {
            return text.clone();
        }
        std::fs::read_to_string(path).unwrap_or_default()
    };

    let mut diagnostics = Vec::new();
    diagnostics.extend(parse_errors(input, &source));
    diagnostics.extend(broken_imports(input, &source));
    diagnostics.extend(undefined_prefixes(input, &source));
    diagnostics.extend(duplicate_labels(input, &source));
    diagnostics.extend(missing_labels(input, &source));
    diagnostics.extend(orphan_classes(input, &source));
    diagnostics
}
