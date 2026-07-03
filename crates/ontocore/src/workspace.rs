//! High-level workspace API for OntoCore.
//!
//! **Pre-1.0:** `Workspace` and its convenience methods are experimental and may change
//! until v0.10 stabilizes the public API.

use crate::catalog::{CatalogError, EntityDetail, IndexBuilder, OntologyCatalog};
use crate::query::{query_catalog, sparql_catalog, QueryError, QueryResult, SparqlResult};
use ontoindex_core::Diagnostic;
use std::path::{Path, PathBuf};

/// An indexed ontology workspace.
pub struct Workspace {
    root: PathBuf,
    catalog: OntologyCatalog,
}

impl Workspace {
    /// Scan and index an ontology workspace on disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CatalogError> {
        let root = path.as_ref().to_path_buf();
        let catalog = IndexBuilder::new().workspace(&root).build()?;
        Ok(Self { root, catalog })
    }

    /// Workspace root path passed to [`Self::open`].
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Indexed catalog (SQL, SPARQL, entity API).
    pub fn catalog(&self) -> &OntologyCatalog {
        &self.catalog
    }

    /// Lint and parse diagnostics collected during indexing.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.catalog.data().diagnostics.clone()
    }

    /// Run a SQL query against virtual ontology tables.
    pub fn query(&self, sql: &str) -> Result<QueryResult, QueryError> {
        query_catalog(&self.catalog, sql)
    }

    /// Run SPARQL against the indexed triple store.
    pub fn sparql(&self, sparql: &str) -> Result<SparqlResult, QueryError> {
        sparql_catalog(&self.catalog, sparql)
    }

    /// Search entities by IRI fragment, short name, or label (case-insensitive).
    pub fn search(&self, term: &str) -> Result<Vec<EntityDetail>, CatalogError> {
        let needle = term.to_ascii_lowercase();
        if needle.is_empty() {
            return Ok(Vec::new());
        }

        let mut matches: Vec<EntityDetail> = self
            .catalog
            .data()
            .entities
            .iter()
            .filter(|entity| entity_matches_term(entity, &needle))
            .filter_map(|entity| self.catalog.entity_detail(&entity.iri))
            .collect();

        matches.sort_by(|a, b| a.entity.short_name.cmp(&b.entity.short_name));
        matches.dedup_by(|a, b| a.entity.iri == b.entity.iri);
        Ok(matches)
    }
}

fn entity_matches_term(entity: &ontoindex_core::Entity, needle: &str) -> bool {
    if entity.iri.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if entity.short_name.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if entity
        .obo_id
        .as_ref()
        .is_some_and(|id| id.to_ascii_lowercase().contains(needle))
    {
        return true;
    }
    entity.labels.iter().any(|label| label.to_ascii_lowercase().contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
    }

    #[test]
    fn workspace_open_fixtures() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        assert!(ws.catalog().data().stats().class_count > 0);
    }

    #[test]
    fn workspace_query_classes() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        let result = ws.query("SELECT short_name FROM classes").expect("query");
        assert!(!result.rows.is_empty());
    }

    #[test]
    fn workspace_search_person() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        let hits = ws.search("Person").expect("search");
        assert!(!hits.is_empty());
    }
}
