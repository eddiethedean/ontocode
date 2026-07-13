//! Adapter smoke for RDF/XML write-back via Transaction.

use ontocore_edit::{apply_transaction_to_text_as, EditFormat, Transaction};
use ontocore_owl::PatchOp;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/protege-roundtrip")
        .join(name);
    std::fs::read_to_string(path).expect("fixture")
}

#[test]
fn rdf_xml_transaction_set_label() {
    let source = fixture("organization.owl");
    let txn = Transaction::from_turtle(vec![PatchOp::SetLabel {
        entity_iri: "http://example.org/org#Department".into(),
        value: "Department Via Txn".into(),
    }]);
    let result =
        apply_transaction_to_text_as(&txn, &source, true, &BTreeMap::new(), EditFormat::RdfXml)
            .expect("apply");
    let text = result.preview_text.expect("preview");
    assert!(text.contains("Department Via Txn"));
}

#[test]
fn owl_xml_transaction_create_class() {
    let source = fixture("example.owx");
    let txn = Transaction::from_turtle(vec![
        PatchOp::CreateEntity {
            entity_iri: "http://example.org/org#Team".into(),
            kind: ontocore_owl::PatchEntityKind::Class,
        },
        PatchOp::SetLabel {
            entity_iri: "http://example.org/org#Team".into(),
            value: "Team".into(),
        },
    ]);
    let result =
        apply_transaction_to_text_as(&txn, &source, true, &BTreeMap::new(), EditFormat::OwlXml)
            .expect("apply");
    let text = result.preview_text.expect("preview");
    assert!(text.contains("Team") || text.contains("#Team"));
}
