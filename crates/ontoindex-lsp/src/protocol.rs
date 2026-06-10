use ontoindex_catalog::{CatalogStats, ClassHierarchy, EntityDetail};
use ontoindex_core::{Entity, OntologyDocument};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IndexWorkspaceParams {
    #[serde(rename = "workspaceUri")]
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
}

#[derive(Debug, Deserialize)]
pub struct GetEntityParams {
    pub iri: String,
}

#[derive(Debug, Serialize)]
pub struct GetEntityResult {
    pub detail: EntityDetail,
}

#[derive(Debug, Serialize)]
pub struct OntoIndexError {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
    pub user_action: Option<String>,
}

impl OntoIndexError {
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
}
