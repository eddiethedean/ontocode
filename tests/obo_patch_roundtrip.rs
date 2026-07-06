//! OBO patch round-trip integration test.

use ontocore_obo::{apply_patches_to_text, OboPatchOp};
use std::path::PathBuf;

fn obo_fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/obo-workflow/example.obo")
}

#[test]
fn obo_patch_add_synonym_roundtrip() {
    let path = obo_fixture();
    if !path.exists() {
        return;
    }
    let source = std::fs::read_to_string(&path).expect("read obo");
    let term_id = "EX:001";
    let result = apply_patches_to_text(
        &source,
        &[OboPatchOp::AddSynonym {
            term_id: term_id.to_string(),
            value: "test synonym".to_string(),
            scope: "exact".to_string(),
        }],
        true,
    )
    .expect("preview patch");
    let preview = result.preview_text.expect("preview text");
    assert!(preview.contains("synonym: test synonym"));
    let validated = fastobo::from_str(&preview);
    assert!(validated.is_ok(), "patched OBO validates: {validated:?}");
}
