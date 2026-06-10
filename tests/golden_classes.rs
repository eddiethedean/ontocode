mod golden;

use std::path::PathBuf;

#[test]
fn classes_snapshot() {
    golden::assert_golden_snapshot(
        "fixtures",
        "SELECT short_name, labels FROM classes",
        &PathBuf::from("tests/golden/snapshots/classes.tsv"),
    );
}
