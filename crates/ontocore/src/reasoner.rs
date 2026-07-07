//! Reasoning integration via OntoLogos.

pub use ontocore_reasoner::{
    classify, explain, ClassificationResult, ConsistencyResult, ExplanationRequest,
    ExplanationResult, ExplanationStep, InferredHierarchy, ReasonerAdapter, ReasonerCache,
    ReasonerCacheStore, ReasonerError, ReasonerId, ReasonerInput, ReasonerProfile,
    ReasonerSnapshot, ReasonerWarning, WorkspaceInputLoader,
};
