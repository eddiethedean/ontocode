//! SWRL validation convenience test (crate unit tests cover unbound vars).

use ontocore_swrl::{parse_swrl_rule_json, validate_rule, SwrlSeverity};

#[test]
fn validates_basic_class_property_rule_json() {
    let json = r#"{
      "id": "person-to-human",
      "body": [
        { "kind": "class", "class": "http://example.org/swrl#Person", "arg": { "variable": "x" } },
        {
          "kind": "object_property",
          "property": "http://example.org/swrl#hasPet",
          "subject": { "variable": "x" },
          "object": { "variable": "y" }
        }
      ],
      "head": [
        { "kind": "class", "class": "http://example.org/swrl#Human", "arg": { "variable": "x" } }
      ],
      "enabled": true
    }"#;
    let rule = parse_swrl_rule_json(json).expect("parse");
    let diags = validate_rule(&rule);
    assert!(
        !diags.iter().any(|d| matches!(d.severity, SwrlSeverity::Error)),
        "expected no errors: {diags:?}"
    );
}

#[test]
fn rules_from_fixture_ttl() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/swrl/basic_class_property.ttl");
    let text = std::fs::read_to_string(&path).expect("read fixture");
    let rules = ontocore_swrl::rules_from_turtle_document(&text);
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].id.as_deref(), Some("person-to-human"));
    assert_eq!(rules[0].body.len(), 2);
    assert_eq!(rules[0].head.len(), 1);
}
