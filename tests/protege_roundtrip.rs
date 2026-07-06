//! Protégé-style round-trip tests (OWL_AUTHORING_SPEC §5).

use ontocore::Workspace;
use ontocore_owl::{apply_patches_to_text, PatchOp};
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
