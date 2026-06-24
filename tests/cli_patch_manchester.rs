mod support;

use std::fs;
use std::process::Command;

#[test]
fn patch_complex_subclass_manchester() {
    let workspace = support::fixture_workspace();
    let src = workspace.join("complex-classes.ttl");
    let tmp = tempfile::tempdir().expect("tmpdir");
    let dst = tmp.path().join("complex-classes.ttl");
    fs::copy(&src, &dst).expect("copy fixture");

    let patch = serde_json::json!([{
        "op": "add_complex_sub_class_of",
        "entity_iri": "http://example.org/clinic#Patient",
        "manchester": "ex:hasRecord some ex:MedicalRecord"
    }]);

    let patch_file = tmp.path().join("patch.json");
    fs::write(&patch_file, serde_json::to_string(&patch).unwrap()).unwrap();

    let bin = support::ontoindex_binary();
    let output = Command::new(&bin)
        .args(["patch", dst.to_str().unwrap(), patch_file.to_str().unwrap(), "--preview"])
        .output()
        .expect("run patch");
    assert!(output.status.success(), "patch failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("owl:Restriction") || stdout.contains("preview"));
}
