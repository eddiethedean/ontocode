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

#[test]
fn dl_query_asserted_mode_returns_named_class_instances() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("abox.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:Person a owl:Class .\n",
            "ex:Employee a owl:Class ; rdfs:subClassOf ex:Person .\n",
            "ex:alice a owl:NamedIndividual , ex:Person .\n",
            "ex:bob a owl:NamedIndividual , ex:Employee .\n",
            "ex:carol a owl:NamedIndividual , owl:Thing .\n",
        ),
    )
    .unwrap();
    let input = WorkspaceInputLoader::new(ws).load().expect("load");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org#".to_string());
    let result = run_dl_query(
        ReasonerId::Rl,
        &input,
        "ex:Person",
        &namespaces,
        DlQueryMode::Asserted,
    )
    .expect("asserted dl query");
    assert!(
        result.instances.iter().any(|i| i.ends_with("#alice")),
        "alice should be asserted instance: {:?}",
        result.instances
    );
    assert!(
        result.instances.iter().any(|i| i.ends_with("#bob")),
        "bob (Employee subclass) should be included: {:?}",
        result.instances
    );
    assert!(
        !result.instances.iter().any(|i| i.ends_with("#carol")),
        "carol is only Thing: {:?}",
        result.instances
    );
    assert!(
        !result.warnings.iter().any(|w| w.contains("instances require inferred")),
        "unexpected warning: {:?}",
        result.warnings
    );
}

#[test]
fn dl_query_asserted_mode_warns_for_anonymous_instances() {
    let input = WorkspaceInputLoader::new(fixture_dir()).load().expect("load fixtures");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
    let result = run_dl_query(
        ReasonerId::Rl,
        &input,
        "ex:hasRecord some ex:MedicalRecord",
        &namespaces,
        DlQueryMode::Asserted,
    )
    .expect("anonymous asserted dl query");
    assert!(result.instances.is_empty());
    assert!(
        result.warnings.iter().any(|w| w.contains("anonymous")),
        "expected anonymous warning: {:?}",
        result.warnings
    );
}
