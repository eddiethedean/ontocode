//! Semantic diff between ontology catalogs and Git refs.

mod compare;
mod format;
mod git;
mod model;

pub use compare::{apply_unsat_diff, diff_catalogs, diff_directories, DiffError, Result};
pub use format::{format_diff_json, format_diff_markdown, format_diff_text};
pub use git::{catalog_at_git_ref, diff_git_refs, parse_git_range, GitDiffSpec};
pub use model::{
    AnnotationChange, AxiomChange, BreakingChange, BreakingReason, DiffResult, DiffSummaryCounts,
    EntityChange, EntityChangeKind, ImportChange, InferenceChange,
};
