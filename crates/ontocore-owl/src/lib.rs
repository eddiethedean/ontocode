//! Horned-OWL facade for OntoCore: load, catalog bridge, and patch write-back.
//!
//! Published as [`ontocore-owl`](https://crates.io/crates/ontocore-owl).

mod bridge;
mod error;
mod load;
pub mod manchester;
pub mod patch;
mod span;

pub use bridge::{bridge_ontology, OwlBridgeResult};
pub use error::{OwlError, Result};
pub use load::{load_from_quads, load_owx_text, load_turtle_text, supports_horned_load};
pub use manchester::{
    class_expression_to_manchester, class_expression_to_turtle_fragment, expression_tree_json,
    parse_class_expression, ManchesterDiagnostic, ManchesterParseOutput,
};
pub use patch::{
    apply_patches, apply_patches_to_text, atomic_write, is_safe_iri, validate_prefix,
    ApplyPatchResult, PatchDiagnostic, PatchEntityKind, PatchOp,
};
pub use span::{
    all_entity_statement_ranges, entity_block_range, entity_primary_block_range,
    is_in_comment_or_string, namespaces_for_text, prefixes_from_turtle, short_name_from_iri,
    ByteRange,
};
