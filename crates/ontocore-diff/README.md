# ontocore-diff

> Part of **OntoCore** (semantic workspace engine).

Semantic ontology diff for [OntoCore](https://github.com/eddiethedean/ontocode) — compare indexed catalogs, version refs, or directories with breaking-change heuristics.

## Install

```toml
ontocore-diff = "0.17"
```

## Quick example

```rust
use std::path::Path;
use ontocore_diff::{diff_git_refs, format_diff_markdown};

let diff = diff_git_refs(Path::new("/path/to/repo"), "main", "feature")?;
println!("{}", format_diff_markdown(&diff, false));
```

CLI: `ontocore diff main..feature`, `ontocore diff --breaking-only HEAD..WORKTREE`.

Re-exported from `ontocore::diff` and `Workspace::diff` / `diff_against_path`.

## Documentation

- [Migration v0.9 → v0.10](https://github.com/eddiethedean/ontocode/blob/main/docs/migration/v0.10.md)
- [Migration v0.10 → v0.11](https://github.com/eddiethedean/ontocode/blob/main/docs/migration/v0.11.md)
- [Semantic diff spec](https://github.com/eddiethedean/ontocode/blob/main/docs/design/SEMANTIC_DIFF_SPEC.md)
- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)

## License

MIT OR Apache-2.0
