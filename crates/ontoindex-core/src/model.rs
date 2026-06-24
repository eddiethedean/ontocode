use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Wire format (LSP JSON) uses snake_case via serde; SQL CLI uses [`OntologyFormat::as_str`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OntologyFormat {
    Turtle,
    RdfXml,
    Owl,
    JsonLd,
    NTriples,
    NQuads,
    TriG,
    Unknown,
}

impl OntologyFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "ttl" => Self::Turtle,
            "rdf" => Self::RdfXml,
            "owl" => Self::Owl,
            "jsonld" | "json-ld" => Self::JsonLd,
            "nt" => Self::NTriples,
            "nq" => Self::NQuads,
            "trig" => Self::TriG,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Turtle => "turtle",
            Self::RdfXml => "rdf_xml",
            Self::Owl => "owl",
            Self::JsonLd => "json_ld",
            Self::NTriples => "n_triples",
            Self::NQuads => "n_quads",
            Self::TriG => "trig",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseStatus {
    Ok,
    Warning,
    Error,
}

impl ParseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Class,
    ObjectProperty,
    DataProperty,
    AnnotationProperty,
    Individual,
    Ontology,
    Other,
}

impl EntityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::ObjectProperty => "object_property",
            Self::DataProperty => "data_property",
            Self::AnnotationProperty => "annotation_property",
            Self::Individual => "individual",
            Self::Ontology => "ontology",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: Option<u64>,
    pub column: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_byte: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_byte: Option<u64>,
}

impl SourceLocation {
    pub fn is_empty(&self) -> bool {
        self.line.is_none()
            && self.column.is_none()
            && self.start_byte.is_none()
            && self.end_byte.is_none()
    }

    pub fn at_line_col(line: u64, column: u64) -> Self {
        Self { line: Some(line), column: Some(column), start_byte: None, end_byte: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

impl DiagnosticSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    ParseError,
    BrokenImport,
    UndefinedPrefix,
    DuplicateLabel,
    MissingLabel,
    OrphanClass,
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::BrokenImport => "broken_import",
            Self::UndefinedPrefix => "undefined_prefix",
            Self::DuplicateLabel => "duplicate_label",
            Self::MissingLabel => "missing_label",
            Self::OrphanClass => "orphan_class",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub file: PathBuf,
    pub range: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_iri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyDocument {
    pub id: String,
    pub path: PathBuf,
    pub format: OntologyFormat,
    pub base_iri: Option<String>,
    pub imports: Vec<String>,
    pub namespaces: BTreeMap<String, String>,
    pub parse_status: ParseStatus,
    pub content_hash: String,
    pub modified_time: u64,
    pub parse_message: Option<String>,
    pub parse_error_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub iri: String,
    pub short_name: String,
    pub kind: EntityKind,
    pub ontology_id: String,
    pub source_location: SourceLocation,
    pub labels: Vec<String>,
    pub comments: Vec<String>,
    pub deprecated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub ontology_id: String,
    #[serde(default, skip_serializing_if = "SourceLocation::is_empty")]
    pub source_location: SourceLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axiom {
    pub id: String,
    pub ontology_id: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub axiom_kind: String,
    #[serde(default, skip_serializing_if = "SourceLocation::is_empty")]
    pub source_location: SourceLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    pub prefix: String,
    pub iri: String,
    pub ontology_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub ontology_id: String,
    pub import_iri: String,
}

/// Snake_case axiom kind stored in [`Axiom::axiom_kind`] and SQL `axioms.axiom_kind`.
pub const AXIOM_KIND_SUB_CLASS_OF: &str = "sub_class_of";
pub const AXIOM_KIND_EQUIVALENT_CLASS: &str = "equivalent_class";

#[cfg(test)]
mod tests {
    use super::ParseStatus;

    #[test]
    fn parse_status_as_str_matches_serde() {
        assert_eq!(ParseStatus::Ok.as_str(), "ok");
        assert_eq!(ParseStatus::Warning.as_str(), "warning");
        assert_eq!(ParseStatus::Error.as_str(), "error");
    }
}
