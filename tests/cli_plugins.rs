//! CLI plugin commands integration test.

mod support;

use std::process::Command;

#[test]
fn cli_plugins_list_json() {
    let workspace =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/plugin-workspace");
    // Invoke via `cargo run -p ontocore-cli` so tests don't depend on a prebuilt binary
    // in `target/debug/ontocore` (which may be stale in CI/local).
    let output = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "ontocore-cli",
            "--",
            "plugins",
            "list",
            workspace.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .expect("run ontocore plugins list");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ontocode.naming-validator"));
}
