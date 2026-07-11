//! EL classification must detect unsatisfiable classes in the reasoner-unsat fixture.

use ontocore_reasoner::{
    classify, explain, explain_alternatives, ExplanationRequest, ReasonerId, WorkspaceInputLoader,
};
use std::path::PathBuf;

fn unsat_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/reasoner-unsat.ttl"),
        workspace.join("reasoner-unsat.ttl"),
    )
    .expect("copy fixture");
    (dir, workspace)
}

#[test]
fn el_classify_detects_unsatisfiable_fixture() {
    let (_dir, workspace) = unsat_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::El, &input, false).expect("classify");

    assert_eq!(result.profile_used, "el");
    assert!(!result.consistent, "expected inconsistent ontology");
    assert!(
        !result.unsatisfiable.is_empty(),
        "expected at least one unsatisfiable class, got {:?}",
        result.unsatisfiable
    );
    assert!(
        result.unsatisfiable.iter().any(|iri| iri.contains("Invalid") || iri.contains("Nothing")),
        "expected unsatisfiable class related to Invalid or Nothing: {:?}",
        result.unsatisfiable
    );
}

#[test]
fn auto_classify_reports_concrete_el_profile() {
    let (_dir, workspace) = unsat_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::Auto, &input, false).expect("classify");
    assert_eq!(result.profile_used, "el");
    assert!(!result.consistent);
}

#[test]
fn auto_cli_and_lsp_explain_match_concrete_engine() {
    let (_dir, workspace) = unsat_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let classification = classify(ReasonerId::Auto, &input, false).expect("classify");
    let concrete = ReasonerId::parse(&classification.profile_used).expect("concrete profile");
    assert_eq!(concrete, ReasonerId::El);

    let class_iri = classification
        .unsatisfiable
        .iter()
        .find(|iri| iri.contains("Invalid"))
        .cloned()
        .unwrap_or_else(|| classification.unsatisfiable[0].clone());
    let request = ExplanationRequest { class_iri };

    // CLI path (adapter explain) and LSP path (explain_alternatives) must agree,
    // and both must match the concrete engine Auto selected — not a hard-coded DL path.
    let cli = explain(ReasonerId::Auto, &input, &request);
    let lsp = explain_alternatives(ReasonerId::Auto, &input, &request, 5);
    let via_concrete_explain = explain(concrete, &input, &request);
    let via_concrete_alts = explain_alternatives(concrete, &input, &request, 5);

    assert_eq!(
        format!("{cli:?}"),
        format!("{via_concrete_explain:?}"),
        "CLI Auto explain must match the concrete engine Auto classified with"
    );
    assert_eq!(
        format!("{lsp:?}"),
        format!("{via_concrete_alts:?}"),
        "LSP Auto explain_alternatives must match the concrete engine Auto classified with"
    );

    match (&cli, &lsp) {
        (Ok(cli_result), Ok(alts)) => {
            assert!(!alts.is_empty());
            assert_eq!(cli_result.text, alts[0].text);
            assert_eq!(cli_result.class_iri, alts[0].class_iri);
        }
        (Err(cli_err), Err(lsp_err)) => {
            assert_eq!(format!("{cli_err}"), format!("{lsp_err}"));
        }
        _ => panic!("CLI explain and LSP explain_alternatives diverged: cli={cli:?} lsp={lsp:?}"),
    }
}
