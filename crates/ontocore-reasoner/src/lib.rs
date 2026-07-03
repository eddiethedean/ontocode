//! OntoLogos-backed reasoner facade for OntoCore (v0.7).
//!
//! Published as [`ontocore-reasoner`](https://crates.io/crates/ontocore-reasoner).

mod adapter;
mod cache;
mod el;
mod error;
mod explain;
mod hierarchy;
mod input;
mod rdfs;
mod result;
mod rl;
mod runner;
mod stub;

pub use adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
pub use cache::{ReasonerCache, ReasonerCacheStore};
pub use error::{ReasonerError, Result};
pub use input::{ReasonerInput, WorkspaceInputLoader};
pub use result::{
    ClassificationResult, ConsistencyResult, ExplanationRequest, ExplanationResult,
    ExplanationStep, InferredHierarchy, ReasonerSnapshot, ReasonerWarning,
};
pub use runner::{classify, explain};
