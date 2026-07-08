//! Security-focused plugin host integration tests.
//!
//! These tests validate that untrusted subprocess plugins cannot escape workspace path jails.

mod support;

use ontocore_catalog::IndexBuilder;
use ontocore_plugin_builtins::load_plugin_host;
use std::path::{Component, Path, PathBuf};

fn is_within(root: &Path, candidate: &Path) -> bool {
    candidate.starts_with(root)
}

/// Lexically normalize a path by resolving `.` and `..` segments without touching the filesystem.
///
/// This is intentionally *not* `canonicalize()`: security checks must work even when the target
/// path doesn't exist or is not readable on the current machine (e.g. CI runners).
fn normalize_lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[test]
fn plugin_diagnostic_file_allows_workspace_escape_via_dotdot() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".ontocore/plugins")).expect("plugins dir");

    // Minimal ontology so `build_catalog()` succeeds.
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    // Create a subprocess plugin script that emits a diagnostic pointing outside the workspace.
    let plugin_bin = workspace.join("evil_plugin.sh");
    std::fs::write(
        &plugin_bin,
        r#"#!/bin/sh
echo '{"diagnostics":[{"code":"pwn","severity":"error","message":"escape","file":"../../etc/passwd"}]}'"#,
    )
    .expect("write evil plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    // Plugin manifest. Use an absolute entry path so discovery resolves it directly.
    let manifest = format!(
        r#"[plugin]
name = "evil"
version = "0.1.0"
kind = "validator"
id = "org.example.evil"
api_version = "1"
entry = "{}"

[capabilities]
validate = true
diagnostics = true
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".ontocore/plugins/evil.toml"), manifest.as_bytes())
        .expect("write manifest");

    // Run via the Rust API (avoids coupling to the CLI argument contract).
    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let host = load_plugin_host(&workspace).expect("load plugin host");
    let result =
        host.run_validate_plugin("org.example.evil", &catalog).expect("run validator plugin");
    let file = result.first().expect("one diagnostic").file.display().to_string();

    let root = workspace.canonicalize().expect("canonical root");
    let resolved = normalize_lexical(Path::new(&file));

    assert!(
        !is_within(&root, &resolved),
        "expected plugin diagnostic to escape workspace; root={}, resolved={}",
        root.display(),
        resolved.display()
    );
}
