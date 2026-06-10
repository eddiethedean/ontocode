pub mod sparql;
pub mod sql;

pub use sparql::{run_sparql, SparqlResult};
pub use sql::{run_sql, QueryResult};

use ontoindex_catalog::OntologyCatalog;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("SQL error: {0}")]
    Sql(String),

    #[error("SPARQL error: {0}")]
    Sparql(String),

    #[error("export error: {0}")]
    Export(String),
}

pub type Result<T> = std::result::Result<T, QueryError>;

pub trait QueryableCatalog {
    fn catalog(&self) -> &OntologyCatalog;
}

impl QueryableCatalog for OntologyCatalog {
    fn catalog(&self) -> &OntologyCatalog {
        self
    }
}

pub fn query_catalog(catalog: &OntologyCatalog, sql: &str) -> Result<QueryResult> {
    run_sql(catalog, sql)
}

pub fn sparql_catalog(catalog: &OntologyCatalog, sparql: &str) -> Result<SparqlResult> {
    run_sparql(catalog, sparql)
}
