//! Security-focused plugin host integration tests.
//!
//! These tests validate that untrusted subprocess plugins cannot escape workspace path jails.

mod support;

use ontocore_catalog::IndexBuilder;
use ontocore_core::is_path_within;
use ontocore_plugin_builtins::load_plugin_host;

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
    let file = &result.first().expect("one diagnostic").file;

    let root = workspace.canonicalize().expect("canonical root");
    assert!(is_path_within(&root, file), "plugin diagnostic file must be jailed");
}

#[test]
fn plugin_diagnostic_file_rejects_absolute_outside_workspace() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".ontocore/plugins")).expect("plugins dir");

    // Minimal ontology so `build_catalog()` succeeds.
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    // Create a real file outside the workspace and reference it via an absolute path.
    let outside = dir.path().join("outside");
    std::fs::create_dir_all(&outside).expect("outside dir");
    let secret = outside.join("secret.ttl");
    std::fs::write(&secret, "@prefix ex: <http://ex/> .\nex:Secret a owl:Class .\n")
        .expect("write secret.ttl");

    let plugin_bin = workspace.join("evil_abs_plugin.sh");
    std::fs::write(
        &plugin_bin,
        format!(
            r#"#!/bin/sh
echo '{{"diagnostics":[{{"code":"pwn","severity":"error","message":"escape-abs","file":"{}"}}]}}'"#,
            secret.display()
        ),
    )
    .expect("write evil abs plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    let manifest = format!(
        r#"[plugin]
name = "evil-abs"
version = "0.1.0"
kind = "validator"
id = "org.example.evilabs"
api_version = "1"
entry = "{}"

[capabilities]
validate = true
diagnostics = true
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".ontocore/plugins/evil_abs.toml"), manifest.as_bytes())
        .expect("write manifest");

    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let host = load_plugin_host(&workspace).expect("load plugin host");
    let result = host
        .run_validate_plugin("org.example.evilabs", &catalog)
        .expect("run validator plugin");
    let file = &result.first().expect("one diagnostic").file;

    let root = workspace.canonicalize().expect("canonical root");
    assert!(is_path_within(&root, file), "absolute diagnostic path must be jailed");
}
