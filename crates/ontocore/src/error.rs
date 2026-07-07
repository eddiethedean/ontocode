//! Unified error type for common OntoCore façade operations.

use thiserror::Error;

/// Aggregates errors from OntoCore façade modules for embedders that prefer one enum.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Catalog(#[from] ontocore_catalog::CatalogError),
    #[error(transparent)]
    Query(#[from] ontocore_query::QueryError),
    #[error(transparent)]
    Graph(#[from] ontocore_catalog::GraphError),
    #[error(transparent)]
    Reasoner(#[from] ontocore_reasoner::ReasonerError),
    #[error(transparent)]
    Export(#[from] ontocore_docs::ExportError),
    #[error(transparent)]
    Owl(#[from] ontocore_owl::OwlError),
    #[error(transparent)]
    Obo(#[from] ontocore_obo::OboError),
}
