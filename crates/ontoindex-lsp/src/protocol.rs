use ontoindex_catalog::{CatalogStats, ClassHierarchy, EntityDetail};
use ontoindex_core::{Diagnostic, Entity, OntologyDocument};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticSummary {
    pub code: String,
    pub severity: String,
    pub message: String,
    pub file: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_iri: Option<String>,
}

impl From<&Diagnostic> for DiagnosticSummary {
    fn from(d: &Diagnostic) -> Self {
        Self {
            code: d.code.as_str().to_string(),
            severity: d.severity.as_str().to_string(),
            message: d.message.clone(),
            file: d.file.display().to_string(),
            line: d.range.line,
            column: d.range.column,
            entity_iri: d.entity_iri.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IndexWorkspaceParams {
    /// Workspace root URI (`file://…`). Accepts legacy camelCase `workspaceUri` during migration.
    #[serde(alias = "workspaceUri", default)]
    pub workspace_uri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IndexWorkspaceResult {
    pub stats: CatalogStats,
    pub indexed_at: u64,
}

#[derive(Debug, Serialize)]
pub struct CatalogSnapshot {
    pub documents: Vec<OntologyDocument>,
    pub entities: Vec<Entity>,
    pub hierarchy: ClassHierarchy,
    pub diagnostics: Vec<DiagnosticSummary>,
}

#[derive(Debug, Deserialize)]
pub struct GetEntityParams {
    pub iri: String,
}

#[derive(Debug, Serialize)]
pub struct GetEntityResult {
    pub detail: EntityDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApplyAxiomPatchParams {
    pub document_uri: String,
    pub patches: Vec<ontoindex_owl::PatchOp>,
    #[serde(default)]
    pub preview_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub sql: String,
}

#[derive(Debug, Deserialize)]
pub struct SparqlParams {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct TabularQueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ParseManchesterParams {
    pub expression: String,
    pub axiom_kind: String,
    #[serde(default)]
    pub entity_iri: Option<String>,
    #[serde(default)]
    pub document_uri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ManchesterCompletions {
    pub classes: Vec<String>,
    pub object_properties: Vec<String>,
    pub data_properties: Vec<String>,
    pub datatypes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ParseManchesterResult {
    pub normalized: String,
    pub turtle_fragment: String,
    pub tree: serde_json::Value,
    pub diagnostics: Vec<ontoindex_owl::PatchDiagnostic>,
    pub completions: ManchesterCompletions,
}

#[derive(Debug, Serialize)]
pub struct ApplyAxiomPatchResult {
    #[serde(flatten)]
    pub patch: ontoindex_owl::ApplyPatchResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detail: Option<EntityDetail>,
}

/// LSP JSON error payload for custom `ontoindex/*` methods (not [`ontoindex_core::OntoIndexError`]).
#[derive(Debug, Serialize)]
pub struct LspErrorPayload {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
    pub user_action: Option<String>,
}

impl LspErrorPayload {
    pub fn not_indexed() -> Self {
        Self {
            code: "NOT_INDEXED".to_string(),
            message: "Workspace has not been indexed yet".to_string(),
            recoverable: true,
            user_action: Some("Run OntoCode: Index Workspace".to_string()),
        }
    }

    pub fn not_found(iri: &str) -> Self {
        Self {
            code: "ENTITY_NOT_FOUND".to_string(),
            message: format!("Entity not found: {iri}"),
            recoverable: true,
            user_action: None,
        }
    }

    pub fn index_failed(message: String) -> Self {
        Self {
            code: "INDEX_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check ontology files for parse errors".to_string()),
        }
    }

    pub fn patch_invalid(message: String) -> Self {
        Self {
            code: "PATCH_INVALID".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check patch parameters and entity IRIs".to_string()),
        }
    }

    pub fn unsupported_format(message: String) -> Self {
        Self {
            code: "UNSUPPORTED_FORMAT".to_string(),
            message,
            recoverable: true,
            user_action: Some("Save as Turtle (.ttl) for write-back".to_string()),
        }
    }

    pub fn query_failed(message: String) -> Self {
        Self {
            code: "QUERY_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check query syntax and virtual table names".to_string()),
        }
    }

    pub fn manchester_invalid(message: String) -> Self {
        Self {
            code: "MANCHESTER_INVALID".to_string(),
            message,
            recoverable: true,
            user_action: Some("Fix the Manchester class expression".to_string()),
        }
    }
}
