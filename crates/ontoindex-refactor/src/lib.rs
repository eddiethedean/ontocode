//! Workspace refactoring for OntoIndex (v0.8).
//!
//! Published as [`ontoindex-refactor`](https://crates.io/crates/ontoindex-refactor).

mod apply;
mod error;
mod model;
mod rename;
mod text;
mod usages;

pub use apply::{apply_refactor_plan, apply_refactor_plan_checked, plan_touches_path};
pub use error::{RefactorError, Result};
pub use model::{FileChange, Hunk, RefactorPlan, RefactorRequest, Usage, UsageKind};
pub use rename::{
    preview_extract_module, preview_migrate_namespace, preview_move_entity, preview_refactor,
    preview_rename_iri,
};
pub use usages::find_usages;
