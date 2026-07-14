//! Core SWRL built-ins registry (`swrlb:`).

pub const SWRLB_NS: &str = "http://www.w3.org/2003/11/swrlb#";

/// Built-ins OntoCode validates and documents for v0.23 (execution may still be partial).
pub const SUPPORTED_BUILTINS: &[&str] = &[
    "equal",
    "notEqual",
    "lessThan",
    "lessThanOrEqual",
    "greaterThan",
    "greaterThanOrEqual",
    "add",
    "subtract",
    "multiply",
    "divide",
    "stringEqualIgnoreCase",
    "stringConcat",
    "substring",
    "stringLength",
    "contains",
    "startsWith",
    "endsWith",
    "matches",
    "booleanNot",
];

pub fn is_supported_builtin(predicate_iri: &str) -> bool {
    let local = predicate_iri
        .strip_prefix(SWRLB_NS)
        .or_else(|| predicate_iri.rsplit(['#', '/']).next())
        .unwrap_or(predicate_iri);
    SUPPORTED_BUILTINS.contains(&local)
}

#[allow(dead_code)]
pub fn builtin_iri(local: &str) -> String {
    format!("{SWRLB_NS}{local}")
}
