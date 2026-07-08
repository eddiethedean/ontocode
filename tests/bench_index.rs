//! Index + query smoke benchmarks (CI-friendly, no external downloads required).

use std::path::PathBuf;
use std::time::Instant;

use ontocore_catalog::IndexBuilder;
use ontocore_query::query_catalog;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[test]
fn bench_index_smoke() {
    let start = Instant::now();
    let catalog = IndexBuilder::new().workspace(fixtures_dir()).build().expect("index fixtures");
    let index_ms = start.elapsed().as_millis();
    eprintln!("bench_index_smoke: index fixtures in {index_ms}ms");
    assert!(index_ms < 30_000, "fixture index should finish within 30s (got {index_ms}ms)");

    let query_start = Instant::now();
    let result = query_catalog(&catalog, "SELECT short_name FROM classes").expect("query classes");
    let query_ms = query_start.elapsed().as_millis();
    eprintln!("bench_index_smoke: query classes -> {} rows in {query_ms}ms", result.rows.len());
    assert!(!result.rows.is_empty());
    assert!(query_ms < 5_000);
}

#[test]
fn bench_axiom_tables_smoke() {
    let catalog = IndexBuilder::new().workspace(fixtures_dir()).build().expect("index");
    for table in [
        "restrictions",
        "domain_axioms",
        "range_axioms",
        "equivalent_class_axioms",
        "disjoint_class_axioms",
    ] {
        let sql = format!("SELECT * FROM {table}");
        let result = query_catalog(&catalog, &sql).expect(&sql);
        eprintln!("bench_axiom_tables_smoke: {table} -> {} rows", result.rows.len());
    }
}
