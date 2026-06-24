//! Horned-OWL facade for OntoIndex: load, catalog bridge, and patch write-back.
//!
//! Published as [`ontoindex-owl`](https://crates.io/crates/ontoindex-owl).

mod bridge;
mod error;
mod load;
pub mod manchester;
pub mod patch;
mod span;

pub use bridge::{bridge_ontology, OwlBridgeResult};
pub use error::{OwlError, Result};
pub use load::{load_from_quads, load_turtle_text, supports_horned_load};
pub use manchester::{
    class_expression_to_manchester, class_expression_to_turtle_fragment, expression_tree_json,
    parse_class_expression, ManchesterDiagnostic, ManchesterParseOutput,
};
pub use patch::{
    apply_patches, apply_patches_to_text, ApplyPatchResult, PatchDiagnostic, PatchEntityKind,
    PatchOp,
};
