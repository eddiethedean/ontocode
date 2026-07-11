//! Protégé-style round-trip tests (OWL_AUTHORING_SPEC §5) + v0.18 workflow fixtures.

use ontocore::Workspace;
use ontocore_core::DiagnosticCode;
use ontocore_owl::{apply_patches_to_text, PatchOp};
use ontocore_reasoner::{classify, ReasonerId, WorkspaceInputLoader};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn roundtrip_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/protege-roundtrip")
}

#[test]
fn protege_roundtrip_properties_domain_range() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let path = dir.join("properties.ttl");
    let text = std::fs::read_to_string(&path).expect("read ttl");
    let ns = BTreeMap::from([
        ("ex".to_string(), "http://example.org/props#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string()),
    ]);
    let result = apply_patches_to_text(
        &text,
        &[PatchOp::AddComment {
            entity_iri: "http://example.org/props#hasAge".to_string(),
            value: "Age in years".to_string(),
        }],
        true,
        &ns,
    )
    .expect("patch");
    assert!(result.preview_text.is_some());
    let detail =
        ws.catalog().entity_detail("http://example.org/props#hasAge").expect("hasAge detail");
    assert!(detail.axioms.iter().any(|a| a.kind == "domain"));
    let _ = detail.characteristics.functional;
}

#[test]
fn protege_roundtrip_individuals_indexed() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws.catalog().entity_detail("http://example.org/people#Alice").expect("Alice");
    assert!(detail.axioms.iter().any(|a| a.kind == "class_assertion"));
    assert!(detail.axioms.iter().any(|a| a.kind == "object_property_assertion"));
}

#[test]
fn protege_roundtrip_people_classes_and_labels() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let person = ws.catalog().entity_detail("http://example.org/people#Person").expect("Person");
    assert_eq!(person.entity.short_name, "Person");
    assert!(
        !person.entity.labels.is_empty(),
        "expected Person label, got {:?}",
        person.entity.labels
    );
    assert!(ws.catalog().find_entity("http://example.org/people#worksFor").is_some());
}

#[test]
fn protege_roundtrip_property_chains_indexed() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws
        .catalog()
        .entity_detail("http://example.org/chains#hasGrandparent")
        .expect("hasGrandparent");
    assert!(
        detail.axioms.iter().any(|a| a.kind == "property_chain"),
        "expected property_chain axiom, got {:?}",
        detail.axioms.iter().map(|a| &a.kind).collect::<Vec<_>>()
    );
}

#[test]
fn protege_roundtrip_annotations_indexed() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws.catalog().entity_detail("http://example.org/ann#Concept").expect("Concept");
    assert!(!detail.annotations.is_empty() || !detail.axioms.is_empty());
    assert!(ws.catalog().find_entity("http://example.org/ann#seeAlso").is_some());
}

#[test]
fn protege_roundtrip_owl_rdfxml_horned() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    assert!(ws.catalog().find_entity("http://example.org/org#Department").is_some());
}

#[test]
fn protege_roundtrip_owx_horned() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws
        .catalog()
        .entity_detail("http://example.org/org#Department")
        .expect("Department from example.owx");
    assert_eq!(detail.entity.short_name, "Department");
}

#[test]
fn protege_workflow_classify_people_el() {
    let dir = roundtrip_dir();
    let input = WorkspaceInputLoader::new(&dir).load().expect("load reasoner input");
    let result = classify(ReasonerId::El, &input, false).expect("classify");
    assert!(result.consistent, "people fixture should be consistent");
    assert!(!input.content_hash.is_empty());
}

#[test]
fn protege_workflow_broken_import_fixture_diagnosed() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/diagnostics");
    let broken = fixtures.join("lint-broken-import.ttl");
    if !broken.exists() {
        return;
    }
    let ws = Workspace::open(&fixtures).expect("open diagnostics fixtures");
    let diags = ws.diagnostics();
    assert!(
        diags.iter().any(|d| {
            matches!(d.code, DiagnosticCode::BrokenImport)
                || d.message.to_lowercase().contains("import")
        }),
        "expected broken import diagnostic, got {:?}",
        diags.iter().map(|d| (&d.code, &d.message)).collect::<Vec<_>>()
    );
}
