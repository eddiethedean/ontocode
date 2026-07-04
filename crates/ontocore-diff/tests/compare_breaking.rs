use ontocore_catalog::IndexBuilder;
use ontocore_core::AXIOM_KIND_SUB_CLASS_OF;
use ontocore_diff::{diff_catalogs, BreakingReason, EntityChangeKind};

#[test]
fn removed_subclass_emits_breaking() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\nex:B a owl:Class .\nex:A rdfs:subClassOf ex:B .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\nex:B a owl:Class .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    assert!(
        diff.breaking_changes.iter().any(|b| matches!(b.reason, BreakingReason::RemovedSuperclass))
            || diff.axiom_changes.iter().any(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF),
        "expected subclass removal in diff"
    );
}

#[test]
fn new_deprecation_reported() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class ; owl:deprecated true .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    assert!(
        diff.entity_changes.iter().any(|c| c.kind == EntityChangeKind::Deprecated),
        "expected deprecation change"
    );
}
