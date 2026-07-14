//! OBO Format 1.4 patch write-back (ADR-0019).

mod apply;
mod error;
mod patch;
mod remap;

pub use apply::{
    apply_patches, apply_patches_to_text, atomic_write, ApplyPatchResult, PatchDiagnostic,
};
pub use error::{OboError, Result};
pub use patch::OboPatchOp;
pub use remap::remap_obo_id_in_text;
