use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageKind {
    EntityDeclaration,
    AxiomSubject,
    AxiomObject,
    AnnotationSubject,
    AnnotationObject,
    Import,
    TextReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub iri: String,
    pub referenced_iri: String,
    pub file: PathBuf,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub start_byte: Option<u64>,
    pub end_byte: Option<u64>,
    pub kind: UsageKind,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hunk {
    pub start_byte: u64,
    pub end_byte: u64,
    pub old_text: String,
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: PathBuf,
    pub preview_text: String,
    pub original_text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorPlan {
    pub changes: Vec<FileChange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RefactorRequest {
    RenameIri {
        from_iri: String,
        to_iri: String,
    },
    MigrateNamespace {
        from_base: String,
        to_base: String,
    },
    MoveEntity {
        entity_iri: String,
        target_file: PathBuf,
    },
    ExtractModule {
        entity_iris: Vec<String>,
        output_file: PathBuf,
        #[serde(default)]
        leave_stub: bool,
    },
}
