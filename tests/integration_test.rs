use ontoindex_catalog::IndexBuilder;
use ontoindex_query::{query_catalog, sparql_catalog};

#[test]
fn indexes_fixture_ontology() {
    let catalog = IndexBuilder::new().workspace("fixtures").build().expect("index fixtures");

    let stats = catalog.data().stats();
    assert_eq!(stats.ontology_count, 1);
    assert_eq!(stats.class_count, 2);
    assert_eq!(stats.object_property_count, 1);
    assert_eq!(stats.individual_count, 2);
    assert_eq!(stats.error_count, 0);
}

#[test]
fn sql_query_classes() {
    let catalog = IndexBuilder::new().workspace("fixtures").build().expect("index fixtures");

    let result =
        query_catalog(&catalog, "SELECT short_name, labels FROM classes").expect("sql query");

    assert_eq!(result.columns, vec!["short_name", "labels"]);
    assert_eq!(result.rows.len(), 2);

    let names: Vec<String> =
        result.rows.iter().map(|row| row.get("short_name").cloned().unwrap_or_default()).collect();
    assert!(names.contains(&"Person".to_string()));
    assert!(names.contains(&"Organization".to_string()));
}

#[test]
fn sql_query_with_filter() {
    let catalog = IndexBuilder::new().workspace("fixtures").build().expect("index fixtures");

    let result =
        query_catalog(&catalog, "SELECT short_name FROM classes WHERE short_name = 'Person'")
            .expect("filtered query");

    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0].get("short_name").map(String::as_str), Some("Person"));
}

#[test]
fn sparql_query_triples() {
    let catalog = IndexBuilder::new().workspace("fixtures").build().expect("index fixtures");

    let result = sparql_catalog(
        &catalog,
        "SELECT ?s WHERE { ?s a <http://www.w3.org/2002/07/owl#NamedIndividual> }",
    )
    .expect("sparql query");

    assert_eq!(result.rows.len(), 2);
}
