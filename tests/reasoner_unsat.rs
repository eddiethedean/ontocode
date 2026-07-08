//! EL classification must detect unsatisfiable classes in the reasoner-unsat fixture.

use ontocore_reasoner::{classify, ReasonerId, WorkspaceInputLoader};
use std::path::PathBuf;

fn unsat_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/reasoner-unsat.ttl"),
        workspace.join("reasoner-unsat.ttl"),
    )
    .expect("copy fixture");
    (dir, workspace)
}

#[test]
fn el_classify_detects_unsatisfiable_fixture() {
    let (_dir, workspace) = unsat_workspace();
    let catalog =
        ontocore_catalog::IndexBuilder::new().workspace(&workspace).build().expect("index");
    let input =
        WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::El, &input, false).expect("classify");

    assert_eq!(result.profile_used, "el");
    assert!(!result.consistent, "expected inconsistent ontology");
    assert!(
        !result.unsatisfiable.is_empty(),
        "expected at least one unsatisfiable class, got {:?}",
        result.unsatisfiable
    );
    assert!(
        result.unsatisfiable.iter().any(|iri| iri.contains("Invalid") || iri.contains("Nothing")),
        "expected unsatisfiable class related to Invalid or Nothing: {:?}",
        result.unsatisfiable
    );
}
