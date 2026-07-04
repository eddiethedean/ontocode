//! Compare ontology workspaces or git refs with semantic diff.

use ontocore::diff::{diff_git_refs, format_diff_text, parse_git_range};
use ontocore::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let ws = Workspace::open(&fixtures)?;
    println!("Indexed {} classes", ws.stats().class_count);

    if let Ok((left, right)) = parse_git_range("HEAD..WORKTREE") {
        if let Ok(diff) = diff_git_refs(&fixtures, &left, &right) {
            println!("{}", format_diff_text(&diff, false));
        }
    }

    Ok(())
}
