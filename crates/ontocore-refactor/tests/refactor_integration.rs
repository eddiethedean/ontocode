use ontocore_catalog::IndexBuilder;
use ontocore_refactor::{
    apply_refactor_plan, apply_refactor_plan_checked, find_usages, preview_extract_module,
    preview_migrate_namespace, preview_move_entity, preview_rename_iri,
    validate_refactor_plan_paths, RefactorError,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/refactor")
        .canonicalize()
        .expect("fixture dir")
}

fn empty_overrides() -> HashMap<PathBuf, String> {
    HashMap::new()
}

fn workspace_roots(path: &Path) -> Vec<PathBuf> {
    vec![path.canonicalize().unwrap_or_else(|_| path.to_path_buf())]
}

fn build_catalog(dir: &std::path::Path) -> ontocore_catalog::OntologyCatalog {
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
fn find_usages_rejects_person_substring_in_person_type() {
    let catalog = build_catalog(&fixture_dir());
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    assert!(!usages.iter().any(|u| u.context.contains("PersonType")));
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
        &empty_overrides(),
    )
    .expect("plan");
    assert!(!plan.changes.is_empty());
    apply_refactor_plan(&plan, false, ws).expect("apply");
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
        &empty_overrides(),
    )
    .expect("plan");
    let preview =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("people change");
    assert!(preview.preview_text.contains("http://example.org/v2/org#"));
    assert!(preview.preview_text.contains("v2/org#"));
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let text = std::fs::read_to_string(ws.join("people.ttl")).unwrap();
    assert!(text.contains("http://example.org/v2/org#"));
    assert!(!text.contains("http://example.org/org#Person"));
    assert!(text.contains("ex:Person") || text.contains("v2"));
}

#[test]
fn migrate_namespace_preserves_slash_prefix_terminator() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    let ttl = r#"@prefix ex: <http://example.org/org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
<http://example.org/org/Person> a owl:Class .
"#;
    std::fs::write(ws.join("slash.ttl"), ttl).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org/",
        "http://example.org/v2/org/",
        &empty_overrides(),
    )
    .expect("plan");
    let preview = plan.changes.iter().find(|c| c.path.ends_with("slash.ttl")).expect("change");
    assert!(preview.preview_text.contains("<http://example.org/v2/org/>"));
    assert!(preview.preview_text.contains("@prefix ex: <http://example.org/v2/org/>"));
    assert!(!preview.preview_text.contains("<http://example.org/v2/org#>"));
}

#[test]
fn migrate_namespace_renames_multiple_angle_bracket_iris_in_one_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/org#Person> a owl:Class .
<http://example.org/org#Agent> a owl:Class .
"#;
    std::fs::write(ws.join("multi.ttl"), ttl).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org#",
        "http://example.org/v2/org#",
        &empty_overrides(),
    )
    .expect("plan");
    let preview = plan.changes.iter().find(|c| c.path.ends_with("multi.ttl")).expect("change");
    assert!(preview.preview_text.contains("<http://example.org/v2/org#Person>"));
    assert!(preview.preview_text.contains("<http://example.org/v2/org#Agent>"));
    assert!(!preview.preview_text.contains("<http://example.org/org#Person>"));
    assert!(!preview.preview_text.contains("<http://example.org/org#Agent>"));
}

#[test]
fn validate_refactor_plan_rejects_paths_outside_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path().canonicalize().unwrap();
    let outside = std::env::temp_dir().join("ontocode-outside-refactor.ttl");
    let plan = ontocore_refactor::RefactorPlan {
        changes: vec![ontocore_refactor::FileChange {
            path: outside.clone(),
            preview_text: "bad".to_string(),
            original_text: String::new(),
            hunks: vec![],
        }],
        warnings: vec![],
    };
    let err = validate_refactor_plan_paths(&ws, &plan).unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
    let _ = std::fs::remove_file(outside);
}

#[test]
fn move_entity_rejects_canonical_same_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let same = ws.join("./people.ttl");
    let err = preview_move_entity(
        &catalog,
        "http://example.org/org#Agent",
        &same,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
}

#[test]
fn move_entity_rejects_path_outside_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let outside = TempDir::new().unwrap();
    let target = outside.path().join("secret.ttl");
    let err = preview_move_entity(
        &catalog,
        "http://example.org/org#Agent",
        &target,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
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
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let module = std::fs::read_to_string(&out).unwrap();
    assert!(module.contains("ex:Person"));
    assert!(module.contains("ex:Agent"));
    let source = std::fs::read_to_string(ws.join("people.ttl")).unwrap();
    assert!(!source.contains("ex:Person"));
    assert!(!source.contains("ex:Agent"));
}

#[test]
fn extract_module_preserves_existing_output_content() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let out = ws.join("module.ttl");
    std::fs::write(&out, "@prefix ex: <http://example.org/org#> .\nex:Existing a owl:Class .\n")
        .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    let out_change = plan.changes.iter().find(|c| c.path == out).expect("output change");
    assert!(out_change.preview_text.contains("ex:Existing"));
    assert!(out_change.preview_text.contains("ex:Person"));
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let written = std::fs::read_to_string(&out).unwrap();
    assert!(written.contains("ex:Existing"));
    assert!(written.contains("ex:Person"));
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
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    let source_change =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("source change");
    assert!(source_change.preview_text.contains("ex:Person"));
    assert!(source_change.preview_text.contains("owl:deprecated true"));
    assert!(source_change.preview_text.contains("a owl:Class"));
}

#[test]
fn move_entity_between_files() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    std::fs::write(ws.join("target.ttl"), "@prefix ex: <http://example.org/org#> .\n").unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_move_entity(
        &catalog,
        "http://example.org/org#Agent",
        &ws.join("target.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    assert_eq!(plan.changes.len(), 2);
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let target = std::fs::read_to_string(ws.join("target.ttl")).unwrap();
    assert!(target.contains("ex:Agent"));
}

#[test]
fn extract_module_validates_nonexistent_output_path() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    validate_refactor_plan_paths(ws, &plan).expect("nonexistent output path is in workspace");
    let roots = workspace_roots(ws);
    apply_refactor_plan_checked(&plan, false, Some(&roots)).expect("apply with validation");
    assert!(out.exists());
}

#[test]
fn rename_iri_renames_default_prefix_curie() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("default.ttl"),
        concat!(
            "@prefix : <http://example.org/org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            ":Person a owl:Class .\n"
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org/org#Person",
        "http://example.org/org#Human",
        &empty_overrides(),
    )
    .expect("plan");
    assert!(!plan.changes.is_empty());
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let text = std::fs::read_to_string(ws.join("default.ttl")).unwrap();
    assert!(text.contains(":Human a owl:Class"));
    assert!(!text.contains(":Person a owl:Class"));
}

#[test]
fn move_entity_across_multi_root_workspace() {
    let tmp = TempDir::new().unwrap();
    let root_a = tmp.path().join("folder-a");
    let root_b = tmp.path().join("folder-b");
    std::fs::create_dir_all(&root_a).unwrap();
    std::fs::create_dir_all(&root_b).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), root_a.join("people.ttl")).unwrap();
    let roots =
        vec![root_a.canonicalize().expect("root a"), root_b.canonicalize().expect("root b")];
    let catalog = IndexBuilder::new()
        .workspace(roots[0].clone())
        .scan_roots(roots.clone())
        .build()
        .expect("index");
    let target = roots[1].join("moved.ttl");
    let plan = preview_move_entity(
        &catalog,
        "http://example.org/org#Person",
        &target,
        &empty_overrides(),
        &roots,
    )
    .expect("plan across secondary root");
    apply_refactor_plan_checked(&plan, false, Some(&roots)).expect("apply");
    let moved = std::fs::read_to_string(&target).expect("target file");
    assert!(moved.contains("ex:Person"));
    let source = std::fs::read_to_string(root_a.join("people.ttl")).unwrap();
    assert!(!source.contains("ex:Person"));
}
