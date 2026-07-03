mod support;

use ontocore_catalog::IndexBuilder;
use ontocore_core::{EntityKind, AXIOM_KIND_SUB_CLASS_OF};
use ontocore_owl::{load_turtle_text, supports_horned_load};
use ontocore_parser::parse_ontology_text;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[test]
fn owl_oxigraph_consistency_on_fixtures() {
    let workspace = support::fixture_workspace();
    for entry in fs::read_dir(&workspace).expect("read fixtures") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("ttl") {
            continue;
        }
        let format = ontocore_core::OntologyFormat::Turtle;
        assert!(supports_horned_load(format));
        let doc_id = "doc-test";
        let text = fs::read_to_string(&path).expect("read ttl");
        let parsed = parse_ontology_text(&path, format, doc_id, &text, text.as_bytes())
            .expect("oxigraph parse");

        let owl = load_turtle_text(&path, doc_id, &text, parsed.quads(), &parsed.namespaces)
            .expect("horned load");

        let ox_entities: BTreeSet<String> = parsed
            .entities
            .iter()
            .filter(|e| e.kind != EntityKind::Ontology && e.kind != EntityKind::Other)
            .map(|e| e.iri.clone())
            .collect();
        let horned_entities: BTreeSet<String> = owl
            .bridge
            .entities
            .iter()
            .filter(|e| e.kind != EntityKind::Ontology && e.kind != EntityKind::Other)
            .map(|e| e.iri.clone())
            .collect();

        assert_eq!(ox_entities, horned_entities, "entity IRI mismatch in {}", path.display());

        let ox_subclass: BTreeSet<(String, String)> = parsed
            .axioms
            .iter()
            .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF)
            .map(|a| (a.subject.clone(), a.object.clone()))
            .collect();
        let horned_subclass: BTreeSet<(String, String)> = owl
            .bridge
            .axioms
            .iter()
            .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF)
            .map(|a| (a.subject.clone(), a.object.clone()))
            .collect();

        let ox_named: BTreeSet<_> =
            ox_subclass.iter().filter(|(_, obj)| !obj.starts_with("_:")).cloned().collect();
        let horned_named: BTreeSet<_> = horned_subclass
            .iter()
            .filter(|(_, obj)| obj.starts_with("http://") || obj.starts_with("https://"))
            .cloned()
            .collect();
        assert_eq!(ox_named, horned_named, "named subclass edges mismatch in {}", path.display());

        for (subject, object) in &ox_subclass {
            if object.starts_with("_:") {
                assert!(
                    horned_subclass.iter().any(|(s, o)| s == subject && !o.starts_with("http")),
                    "missing complex subclass for {subject} in {}",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn patch_roundtrip_label_and_subclass() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("roundtrip.ttl");
    fs::copy(support::fixture_workspace().join("example.ttl"), &path).unwrap();

    let namespaces = BTreeMap::from([
        ("ex".to_string(), "http://example.org/people#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
    ]);

    let patches = vec![
        ontocore_owl::PatchOp::AddLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Human".to_string(),
        },
        ontocore_owl::PatchOp::AddSubClassOf {
            entity_iri: "http://example.org/people#Employee".to_string(),
            parent_iri: "http://example.org/people#Person".to_string(),
        },
    ];

    // create Employee first
    let create = vec![ontocore_owl::PatchOp::CreateEntity {
        entity_iri: "http://example.org/people#Employee".to_string(),
        kind: ontocore_owl::PatchEntityKind::Class,
    }];
    ontocore_owl::apply_patches(&path, &create, false, &namespaces).expect("create");

    let result = ontocore_owl::apply_patches(&path, &patches, false, &namespaces).expect("patch");
    assert!(result.applied);

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("reindex");
    let person = catalog.find_entity("http://example.org/people#Person").expect("Person");
    assert!(person.labels.iter().any(|l| l.contains("Human")));

    let hierarchy = catalog.class_hierarchy();
    assert!(hierarchy
        .parents
        .get("http://example.org/people#Employee")
        .is_some_and(|p| p.contains(&"http://example.org/people#Person".to_string())));
}
