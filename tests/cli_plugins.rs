//! CLI plugin commands integration test.

mod support;

#[test]
fn cli_plugins_list_json() {
    let workspace =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/plugin-workspace");
    let output = support::ontocore_cmd()
        .args(["plugins", "list", workspace.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("run ontocore plugins list");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ontocode.naming-validator"));
}
