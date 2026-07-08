//! CLI integration test for `ontocore diff --pr-summary`.

mod support;

use std::process::Command;

#[test]
fn diff_pr_summary_emits_markdown_between_directories() {
    let left = tempfile::tempdir().unwrap();
    let right = tempfile::tempdir().unwrap();
    let base = support::fixture_workspace().join("example.ttl");
    std::fs::copy(&base, left.path().join("example.ttl")).unwrap();
    std::fs::copy(&base, right.path().join("example.ttl")).unwrap();
    std::fs::write(
        right.path().join("extra.ttl"),
        "@prefix ex: <http://example.org/people#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<http://example.org/extra> a owl:Ontology .\nex:ExtraClass a owl:Class .\n",
    )
    .unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "ontocore-cli",
            "--",
            "diff",
            "--left-ref",
            left.path().to_str().unwrap(),
            "--right-ref",
            right.path().to_str().unwrap(),
            "--pr-summary",
        ])
        .output()
        .expect("spawn ontocore diff");

    assert!(output.status.success(), "diff failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("## Ontology changes"), "expected PR summary header");
    assert!(stdout.contains("ontocore diff --pr-summary"));
    assert!(
        stdout.contains("Entities") || stdout.contains("entity"),
        "expected entity section in summary: {stdout}"
    );
}
