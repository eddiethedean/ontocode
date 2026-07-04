use ontocore_core::ensure_extract_path_within;
use ontocore_diff::parse_git_range;

#[test]
fn reject_path_escape_in_extract() {
    let dir = tempfile::tempdir().unwrap();
    assert!(ensure_extract_path_within(dir.path(), "../outside.ttl").is_err());
    assert!(ensure_extract_path_within(dir.path(), "nested/../../outside.ttl").is_err());
}

#[test]
fn parse_two_dot_range() {
    let (left, right) = parse_git_range("main..feature").unwrap();
    assert_eq!(left, "main");
    assert_eq!(right, "feature");
}

#[test]
fn parse_triple_dot_range() {
    let (left, right) = parse_git_range("main...feature").unwrap();
    assert_eq!(left, "main...feature");
    assert_eq!(right, "TRIPLE_DOT");
}
