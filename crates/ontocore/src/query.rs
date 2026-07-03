//! SQL virtual tables and SPARQL over an indexed catalog.

pub use ontoindex_query::{
    query_catalog, sparql_catalog, run_sparql, run_sql, QueryError, QueryResult, QueryableCatalog,
    Result, SparqlResult,
};
