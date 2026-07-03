mod support;

use ontocore_reasoner::{classify, ReasonerId, WorkspaceInputLoader};
use std::path::PathBuf;

fn el_only_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    (dir, workspace)
}

#[test]
fn el_classify_el_fixture_workspace() {
    let (_dir, workspace) = el_only_workspace();
    let catalog =
        ontocore_catalog::IndexBuilder::new().workspace(&workspace).build().expect("index");
    let input =
        WorkspaceInputLoader::new(&workspace).load(catalog.class_hierarchy()).expect("load");
    let result = classify(ReasonerId::El, &input, false).expect("classify");

    assert_eq!(result.profile_used, "el");
    assert!(result.consistent);
}

#[test]
fn dl_profile_classifies_el_fixture_workspace() {
    let (_dir, workspace) = el_only_workspace();
    let catalog =
        ontocore_catalog::IndexBuilder::new().workspace(&workspace).build().expect("index");
    let input =
        WorkspaceInputLoader::new(&workspace).load(catalog.class_hierarchy()).expect("load");
    let result = classify(ReasonerId::Dl, &input, false).expect("classify");

    assert_eq!(result.profile_used, "dl");
    assert!(result.consistent);
}
