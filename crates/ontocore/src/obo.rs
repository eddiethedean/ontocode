//! OBO Format patch write-back (re-export).

pub use ontocore_obo::{
    apply_patches, apply_patches_to_text, atomic_write, ApplyPatchResult, OboError, OboPatchOp,
    PatchDiagnostic,
};
