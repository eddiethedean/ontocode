use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
pub enum ParseStatus {
    Ok,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axiom {
    pub id: String,
    pub ontology_id: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub axiom_kind: String,
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
