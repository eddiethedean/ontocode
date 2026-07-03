//! Workspace refactoring for OntoCore (v0.8).
//!
//! Published as [`ontocore-refactor`](https://crates.io/crates/ontocore-refactor).

mod apply;
mod error;
mod model;
mod rename;
mod source;
mod text;
mod usages;

pub use apply::{
    apply_refactor_plan, apply_refactor_plan_checked, apply_refactor_plan_checked_with_overrides,
    plan_touches_path, plans_equivalent, validate_refactor_plan_paths,
};
pub use error::{RefactorError, Result};
pub use model::{FileChange, Hunk, RefactorPlan, RefactorRequest, Usage, UsageKind};
pub use rename::{
    preview_extract_module, preview_migrate_namespace, preview_move_entity, preview_refactor,
    preview_rename_iri,
};
pub use usages::{find_usages, find_usages_with_overrides};
