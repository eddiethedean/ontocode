use ontocore_reasoner::{run_dl_query, DlQueryMode, ReasonerId, WorkspaceInputLoader};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[test]
fn dl_query_named_class_returns_hierarchy() {
    let input = WorkspaceInputLoader::new(fixture_dir()).load().expect("load fixtures");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
    let result = run_dl_query(
        ReasonerId::Rl,
        &input,
        "ex:ClinicPerson",
        &namespaces,
        DlQueryMode::Inferred,
    )
    .expect("dl query");
    assert!(!result.subclasses.is_empty(), "expected subclasses of ClinicPerson");
    assert!(
        result.subclasses.iter().any(|iri| iri.contains("Patient") || iri.contains("Staff")),
        "subclasses={:?}",
        result.subclasses
    );
}

#[test]
fn dl_query_parses_complex_expression() {
    let input = WorkspaceInputLoader::new(fixture_dir()).load().expect("load fixtures");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
    let result = run_dl_query(
        ReasonerId::Dl,
        &input,
        "ex:hasRecord some ex:MedicalRecord",
        &namespaces,
        DlQueryMode::Inferred,
    )
    .expect("complex dl query");
    assert_eq!(result.normalized.is_empty(), false);
    assert!(!result.query_class_iri.is_empty());
}
