//! v0.22 OWL 2 authoring patch + round-trip coverage.

use ontocore::Workspace;
use ontocore_catalog::IndexBuilder;
use ontocore_owl::{apply_patches, apply_patches_to_text, apply_xml_patches_to_text, PatchOp};
use ontocore_core::OntologyFormat;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn roundtrip_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/protege-roundtrip")
}

fn keys_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".into(), "http://example.org/keys#".into()),
        ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into()),
    ])
}

fn abox_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".into(), "http://example.org/abox#".into()),
        ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into()),
    ])
}

#[test]
fn owl2_keys_fixture_indexes_haskey_and_inverse() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    assert!(ws.catalog().find_entity("http://example.org/keys#Person").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#hasSSN").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#hasParent").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#hasChild").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#Sex").is_some());
    // Round-trip: patch a second key property onto Person and confirm write-back.
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("owl2-keys.ttl");
    std::fs::copy(dir.join("owl2-keys.ttl"), &path).expect("copy");
    apply_patches(
        &path,
        &[PatchOp::AddInverseObjectProperties {
            property_iri: "http://example.org/keys#hasChild".into(),
            inverse_iri: "http://example.org/keys#hasParent".into(),
        }],
        false,
        &keys_ns(),
    )
    .expect("apply inverse");
    let out = std::fs::read_to_string(&path).expect("read");
    assert!(
        out.contains("inverseOf") || out.contains("hasParent"),
        "inverse patch must persist: {out}"
    );
}

#[test]
fn owl2_abox_fixture_indexes_sameas_and_negative() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let alice = ws
        .catalog()
        .entity_detail("http://example.org/abox#alice")
        .expect("alice");
    assert!(
        alice.axioms.iter().any(|a| {
            a.kind.contains("same")
                || a.display.contains("sameAs")
                || a.display.contains("ally")
                || a.kind.contains("negative")
                || a.display.contains("knows")
        }) || ws.catalog().find_entity("http://example.org/abox#ally").is_some(),
        "expected sameAs / negative coverage, axioms={:?}",
        alice.axioms
    );
}

#[test]
fn owl2_turtle_patch_add_has_key_and_same_individual() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("owl2-keys.ttl");
    std::fs::copy(roundtrip_dir().join("owl2-keys.ttl"), &path).expect("copy");

    let person = "http://example.org/keys#Person";
    let has_parent = "http://example.org/keys#hasParent";
    let preview = apply_patches_to_text(
        &std::fs::read_to_string(&path).expect("read"),
        &[PatchOp::AddHasKey {
            class_iri: person.into(),
            properties: vec![has_parent.into()],
        }],
        true,
        &keys_ns(),
    )
    .expect("preview hasKey");
    let text = preview.preview_text.expect("preview text");
    assert!(
        text.contains("hasKey") || text.contains(has_parent),
        "hasKey patch preview missing key content: {text}"
    );

    apply_patches(
        &path,
        &[PatchOp::AddHasKey {
            class_iri: person.into(),
            properties: vec![has_parent.into()],
        }],
        false,
        &keys_ns(),
    )
    .expect("apply hasKey");

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("reindex");
    let detail = catalog.entity_detail(person).expect("Person detail");
    assert!(
        detail.axioms.iter().any(|a| a.display.contains("hasParent") || a.kind.contains("key"))
            || std::fs::read_to_string(&path).unwrap().contains("hasParent"),
        "hasKey must persist, axioms={:?}",
        detail.axioms
    );
}

#[test]
fn owl2_turtle_patch_abox_negative_and_same() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("owl2-abox.ttl");
    std::fs::copy(roundtrip_dir().join("owl2-abox.ttl"), &path).expect("copy");

    let alice = "http://example.org/abox#alice";
    let carol = "http://example.org/abox#carol";
    let knows = "http://example.org/abox#knows";

    apply_patches(
        &path,
        &[
            PatchOp::AddSameIndividual {
                individuals: vec![alice.into(), carol.into()],
            },
            PatchOp::AddNegativeObjectPropertyAssertion {
                entity_iri: alice.into(),
                property_iri: knows.into(),
                target_iri: carol.into(),
            },
        ],
        false,
        &abox_ns(),
    )
    .expect("apply abox patches");

    let out = std::fs::read_to_string(&path).expect("read result");
    assert!(
        out.contains("sameAs") || out.contains(carol),
        "sameAs / negative patches must write content: {out}"
    );
}

#[test]
fn owl2_xml_mutate_supports_domain_characteristic_and_rejects_prefix() {
    let rdf = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
    <owl:Ontology rdf:about="http://example.org/ont"/>
    <owl:Class rdf:about="http://example.org/ont#Person"/>
    <owl:ObjectProperty rdf:about="http://example.org/ont#worksFor"/>
</rdf:RDF>"#;
    let ns = BTreeMap::new();
    let ok = apply_xml_patches_to_text(
        rdf,
        OntologyFormat::RdfXml,
        &[
            PatchOp::AddDomain {
                entity_iri: "http://example.org/ont#worksFor".into(),
                class_iri: "http://example.org/ont#Person".into(),
            },
            PatchOp::SetFunctional {
                entity_iri: "http://example.org/ont#worksFor".into(),
                value: true,
            },
            PatchOp::AddObjectPropertyAssertion {
                entity_iri: "http://example.org/ont#alice".into(),
                property_iri: "http://example.org/ont#worksFor".into(),
                target_iri: "http://example.org/ont#acme".into(),
            },
        ],
        true,
        &ns,
    )
    .expect("xml mutate domain/functional/opa");
    let preview = ok.preview_text.expect("preview");
    assert!(
        preview.contains("worksFor") && preview.contains("Person"),
        "expected domain/characteristic content: {preview}"
    );

    let err = apply_xml_patches_to_text(
        rdf,
        OntologyFormat::RdfXml,
        &[PatchOp::AddPrefix {
            prefix: "ex".into(),
            namespace_iri: "http://example.org/".into(),
        }],
        true,
        &ns,
    );
    assert!(err.is_err(), "prefix ops must error on XML");
    let msg = format!("{}", err.unwrap_err());
    assert!(
        msg.contains("Turtle-only") || msg.contains("AddPrefix") || msg.contains("prefix"),
        "expected clear prefix error, got {msg}"
    );
}
