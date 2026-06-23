mod golden;
mod support;

use std::path::PathBuf;

use ontoindex_catalog::IndexBuilder;
use ontoindex_core::ParseStatus;
use ontoindex_query::{query_catalog, sparql_catalog, QueryError};
use support::fixture_catalog;

#[test]
fn indexes_fixture_ontology() {
    let stats = fixture_catalog().data().stats();

    assert_eq!(stats.ontology_count, 2);
    assert_eq!(stats.class_count, 4);
    assert_eq!(stats.object_property_count, 1);
    assert_eq!(stats.data_property_count, 1);
    assert_eq!(stats.annotation_property_count, 1);
    assert_eq!(stats.individual_count, 2);
    assert_eq!(stats.error_count, 0);
    assert!(stats.axiom_count >= 1);
    assert!(stats.annotation_count > 0);
    assert!(stats.triple_count > 0);
}

#[test]
fn validate_fails_on_parse_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("bad.ttl"), "@prefix ex: <http://ex/> .\nex:a a owl:Class .\n")
        .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
    assert_eq!(catalog.data().stats().error_count, 1);

    let doc = catalog.data().documents.iter().find(|d| d.parse_status == ParseStatus::Error);
    assert!(doc.is_some(), "expected parse error document");
    assert!(doc.unwrap().parse_message.is_some());
}

#[test]
fn valid_file_still_indexes_when_sibling_has_parse_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(support::fixture_workspace().join("example.ttl"), dir.path().join("good.ttl"))
        .unwrap();
    std::fs::write(dir.path().join("bad.ttl"), "@prefix ex: <http://ex/> .\nex:a a owl:Class .\n")
        .unwrap();

    let stats = IndexBuilder::new().workspace(dir.path()).build().expect("build").data().stats();

    assert_eq!(stats.ontology_count, 2);
    assert_eq!(stats.error_count, 1);
    assert_eq!(stats.class_count, 3);
}

#[test]
fn sql_diagnostics_table() {
    let dir = tempfile::tempdir().unwrap();
    for name in ["lint-broken-import.ttl", "lint-duplicate-labels.ttl", "lint-orphan.ttl"] {
        std::fs::copy(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/diagnostics").join(name),
            dir.path().join(name),
        )
        .unwrap();
    }

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
    let result = query_catalog(&catalog, "SELECT code, severity FROM diagnostics")
        .expect("diagnostics query");

    let codes: Vec<_> = result.rows.iter().filter_map(|r| r.get("code").cloned()).collect();
    assert!(codes.iter().any(|c| c == "broken_import"));
    assert!(codes.iter().any(|c| c == "duplicate_label"));
    assert!(codes.iter().any(|c| c == "orphan_class"));
    assert!(codes.iter().any(|c| c == "missing_label"));
}

#[test]
fn validate_reports_diagnostics() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/diagnostics/lint-broken-import.ttl"),
        dir.path().join("bad.ttl"),
    )
    .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
    let errors = catalog
        .data()
        .diagnostics
        .iter()
        .filter(|d| d.severity == ontoindex_core::DiagnosticSeverity::Error)
        .count();
    assert!(errors >= 1);
}

#[test]
fn classes_snapshot() {
    golden::assert_golden_snapshot(
        "fixtures",
        "SELECT short_name, labels FROM classes",
        &PathBuf::from("tests/golden/snapshots/classes.tsv"),
    );
}

#[test]
fn sql_query_with_filter() {
    let catalog = fixture_catalog();

    let result =
        query_catalog(&catalog, "SELECT short_name FROM classes WHERE short_name = 'Person'")
            .expect("filtered query");

    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0].get("short_name").map(String::as_str), Some("Person"));
}

#[test]
fn sql_query_properties_and_axioms_tables() {
    let catalog = fixture_catalog();

    let properties =
        query_catalog(&catalog, "SELECT short_name, kind FROM properties").expect("properties");
    assert_eq!(properties.rows.len(), 3);

    let axioms = query_catalog(&catalog, "SELECT axiom_kind FROM axioms").expect("axioms");
    assert!(axioms
        .rows
        .iter()
        .any(|row| row.get("axiom_kind").map(String::as_str) == Some("SubClassOf")));

    let imports = query_catalog(&catalog, "SELECT import_iri FROM imports").expect("imports");
    assert!(!imports.rows.is_empty());
}

#[test]
fn sql_query_unknown_table_returns_error() {
    let catalog = fixture_catalog();

    let err = query_catalog(&catalog, "SELECT * FROM missing_table").unwrap_err();
    assert!(matches!(err, QueryError::Sql(msg) if msg.contains("unknown table")));
}

#[test]
fn sql_query_non_select_returns_error() {
    let catalog = fixture_catalog();

    let err = query_catalog(&catalog, "INSERT INTO classes VALUES ('x')").unwrap_err();
    assert!(matches!(err, QueryError::Sql(msg) if msg.contains("only SELECT")));
}

#[test]
fn sparql_query_triples() {
    let catalog = fixture_catalog();

    let result = sparql_catalog(
        &catalog,
        "SELECT ?s WHERE { ?s a <http://www.w3.org/2002/07/owl#NamedIndividual> }",
    )
    .expect("sparql query");

    assert_eq!(result.rows.len(), 2);
    let subjects: Vec<_> = result.rows.iter().filter_map(|row| row.get("s").cloned()).collect();
    assert!(subjects.iter().any(|s| s.contains("alice")));
    assert!(subjects.iter().any(|s| s.contains("acme")));
}

#[test]
fn sparql_ask_query_returns_boolean() {
    let catalog = fixture_catalog();

    let result = sparql_catalog(
        &catalog,
        "ASK { <http://example.org/people#Person> a <http://www.w3.org/2002/07/owl#Class> }",
    )
    .expect("sparql ask");

    assert_eq!(result.columns, vec!["boolean"]);
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0].get("boolean").map(String::as_str), Some("true"));
}

#[test]
fn sparql_malformed_query_returns_error() {
    let catalog = fixture_catalog();

    let err = sparql_catalog(&catalog, "SELECT ?s WHERE { ?s ?p ?o").unwrap_err();
    assert!(matches!(err, QueryError::Sparql(_)));
}

#[test]
fn sql_json_and_csv_export() {
    let catalog = fixture_catalog();
    let result = query_catalog(&catalog, "SELECT short_name FROM classes").expect("query");

    let json = ontoindex_query::sql::to_json(&result).expect("json export");
    assert!(json.contains("Person"));

    let csv = ontoindex_query::sql::to_csv(&result).expect("csv export");
    assert!(csv.contains("short_name"));
    assert!(csv.contains("Person"));
}
