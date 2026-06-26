use std::process::Command;

mod support;

#[test]
fn cli_classify_el_json() {
    let bin = support::ontoindex_binary();
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        support::fixture_workspace().join("reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    let output = Command::new(&bin)
        .args(["classify", workspace.to_str().unwrap(), "--profile", "el", "--format", "json"])
        .output()
        .expect("run classify");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json output");
    assert_eq!(json.get("profile_used").and_then(|v| v.as_str()), Some("el"));
    assert_eq!(json.get("consistent").and_then(|v| v.as_bool()), Some(true));
}

#[test]
fn cli_dl_profile_reports_error() {
    let bin = support::ontoindex_binary();
    let workspace = support::fixture_workspace();
    let output = Command::new(&bin)
        .args(["classify", workspace.to_str().unwrap(), "--profile", "dl"])
        .output()
        .expect("run classify");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("OntoLogos") || stderr.contains("1.0"));
}
