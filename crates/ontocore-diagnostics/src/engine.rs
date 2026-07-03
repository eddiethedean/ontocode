use crate::input::DiagnosticInput;
use crate::rules::{
    broken_imports, duplicate_labels, missing_labels, orphan_classes, parse_errors,
    undefined_prefixes,
};
use ontocore_core::{Diagnostic, DiagnosticCode, DiagnosticSeverity, SourceLocation};
use std::cell::RefCell;
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
    let io_failures: RefCell<Vec<Diagnostic>> = RefCell::new(Vec::new());

    let source = |path: &std::path::Path| -> String {
        if let Some(text) = source_overrides.get(path) {
            return text.clone();
        }
        if let Ok(canonical) = path.canonicalize() {
            if let Some(text) = source_overrides.get(&canonical) {
                return text.clone();
            }
        }
        for (override_path, text) in source_overrides {
            if override_path.canonicalize().ok().as_ref() == path.canonicalize().ok().as_ref() {
                return text.clone();
            }
        }
        match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) => {
                io_failures.borrow_mut().push(Diagnostic {
                    code: DiagnosticCode::IoReadError,
                    severity: DiagnosticSeverity::Warning,
                    message: format!("could not read file for lint analysis: {err}"),
                    file: path.to_path_buf(),
                    range: SourceLocation::default(),
                    entity_iri: None,
                    quick_fix: None,
                });
                String::new()
            }
        }
    };

    let mut diagnostics = Vec::new();
    diagnostics.extend(parse_errors(input, &source));
    diagnostics.extend(broken_imports(input, &source));
    diagnostics.extend(undefined_prefixes(input, &source));
    diagnostics.extend(duplicate_labels(input, &source));
    diagnostics.extend(missing_labels(input, &source));
    diagnostics.extend(orphan_classes(input, &source));
    diagnostics.extend(io_failures.into_inner());
    diagnostics
}
