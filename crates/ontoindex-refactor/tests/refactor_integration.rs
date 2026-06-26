use ontoindex_catalog::IndexBuilder;
use ontoindex_refactor::{
    apply_refactor_plan, find_usages, preview_extract_module, preview_migrate_namespace,
    preview_move_entity, preview_rename_iri,
};
use std::path::PathBuf;
use tempfile::TempDir;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/refactor")
        .canonicalize()
        .expect("fixture dir")
}

fn build_catalog(dir: &std::path::Path) -> ontoindex_catalog::OntologyCatalog {
    IndexBuilder::new().workspace(dir).build().expect("index")
}

#[test]
fn find_usages_across_files() {
    let catalog = build_catalog(&fixture_dir());
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    assert!(!usages.is_empty());
    assert!(usages.iter().any(|u| u.file.ends_with("org.ttl")));
    assert!(usages.iter().any(|u| u.file.ends_with("people.ttl")));
}

#[test]
fn rename_iri_across_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    for name in ["org.ttl", "people.ttl"] {
        std::fs::copy(fixture_dir().join(name), ws.join(name)).unwrap();
    }
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org/org#Person",
        "http://example.org/org#Human",
    )
    .expect("plan");
    assert!(!plan.changes.is_empty());
    apply_refactor_plan(&plan, false).expect("apply");
    let org_text = std::fs::read_to_string(ws.join("org.ttl")).unwrap();
    assert!(org_text.contains("ex:Human"));
    assert!(!org_text.contains("ex:Person"));
}

#[test]
fn migrate_namespace_updates_prefix_and_entity_iris() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org#",
        "http://example.org/v2/org#",
    )
    .expect("plan");
    let preview =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("people change");
    assert!(preview.preview_text.contains("http://example.org/v2/org#"));
    assert!(preview.preview_text.contains("v2/org#"));
    apply_refactor_plan(&plan, false).expect("apply");
    let text = std::fs::read_to_string(ws.join("people.ttl")).unwrap();
    assert!(text.contains("http://example.org/v2/org#"));
    assert!(!text.contains("http://example.org/org#Person"));
    assert!(text.contains("ex:Person") || text.contains("v2"));
}

#[test]
fn extract_multiple_entities_same_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string(), "http://example.org/org#Agent".to_string()],
        &out,
        false,
    )
    .expect("plan");
    apply_refactor_plan(&plan, false).expect("apply");
    let module = std::fs::read_to_string(&out).unwrap();
    assert!(module.contains("ex:Person"));
    assert!(module.contains("ex:Agent"));
    let source = std::fs::read_to_string(ws.join("people.ttl")).unwrap();
    assert!(!source.contains("ex:Person"));
    assert!(!source.contains("ex:Agent"));
}

#[test]
fn extract_module_leave_stub_uses_prefixed_curie() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        true,
    )
    .expect("plan");
    let source_change =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("source change");
    assert!(source_change.preview_text.contains("ex:Person"));
    assert!(source_change.preview_text.contains("owl:deprecated true"));
}

#[test]
fn move_entity_between_files() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    std::fs::write(ws.join("target.ttl"), "@prefix ex: <http://example.org/org#> .\n").unwrap();
    let catalog = build_catalog(ws);
    let plan =
        preview_move_entity(&catalog, "http://example.org/org#Agent", &ws.join("target.ttl"))
            .expect("plan");
    assert_eq!(plan.changes.len(), 2);
    apply_refactor_plan(&plan, false).expect("apply");
    let target = std::fs::read_to_string(ws.join("target.ttl")).unwrap();
    assert!(target.contains("ex:Agent"));
}

#[test]
fn extract_module_creates_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
    )
    .expect("plan");
    apply_refactor_plan(&plan, false).expect("apply");
    assert!(out.exists());
    let module = std::fs::read_to_string(&out).unwrap();
    assert!(module.contains("ex:Person"));
}
