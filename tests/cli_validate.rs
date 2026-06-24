use std::path::{Path, PathBuf};
use std::process::Command;

mod support;

pub fn ontoindex_binary() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_ontoindex") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return candidate;
        }
    }

    find_ontoindex_binary_in_target().unwrap_or_else(|| {
        let target_dir = std::env::var("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));
        panic!(
            "ontoindex binary not found under {} (run `cargo build -p ontoindex-cli` first)",
            target_dir.display()
        );
    })
}

fn find_ontoindex_binary_in_target() -> Option<PathBuf> {
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));

    for subdir in ["debug", "release"] {
        let candidate = target_dir.join(subdir).join("ontoindex");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

#[test]
fn validate_exits_zero_on_clean_fixtures() {
    let fixtures = support::fixture_workspace();
    let output = Command::new(ontoindex_binary())
        .args(["validate", fixtures.to_str().expect("fixture path")])
        .output()
        .expect("spawn ontoindex validate");

    assert!(
        output.status.success(),
        "expected exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK:"), "expected success message, got: {stdout}");
}

#[test]
fn validate_exits_zero_when_only_warnings() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/diagnostics/lint-orphan.ttl"),
        dir.path().join("orphan.ttl"),
    )
    .unwrap();

    let output = Command::new(ontoindex_binary())
        .args(["validate", dir.path().to_str().expect("temp path")])
        .output()
        .expect("spawn ontoindex validate");

    assert!(
        output.status.success(),
        "expected exit 0 for warnings-only workspace, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("WARN")
            || stderr.contains("orphan_class")
            || stderr.contains("missing_label"),
        "expected warning diagnostics, got: {stderr}"
    );
}

#[test]
fn validate_exits_nonzero_on_diagnostic_errors() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/diagnostics/lint-broken-import.ttl"),
        dir.path().join("bad.ttl"),
    )
    .unwrap();

    let output = Command::new(ontoindex_binary())
        .args(["validate", dir.path().to_str().expect("temp path")])
        .output()
        .expect("spawn ontoindex validate");

    assert!(!output.status.success(), "expected non-zero exit on broken import fixture");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("broken_import"), "expected broken_import in stderr, got: {stderr}");
}
