use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReasonerError {
    #[error("core error: {0}")]
    Core(#[from] ontocore_core::OntoCoreError),

    #[error("load error in {path}: {message}")]
    Load { path: std::path::PathBuf, message: String },

    #[error("unsupported profile: {0}")]
    UnsupportedProfile(String),

    #[error("ontology error: {0}")]
    Ontology(String),

    #[error("classification error: {0}")]
    Classify(String),

    #[error("explanation error: {0}")]
    Explain(String),

    #[error("no classification result cached; run reasoner first")]
    NotClassified,

    #[error("class not found: {0}")]
    ClassNotFound(String),

    #[error("explanation not available for class: {0}")]
    ExplanationUnavailable(String),
}

pub type Result<T> = std::result::Result<T, ReasonerError>;
