# Contributing

Thank you for contributing to OntoCode and OntoIndex.

This page summarizes where to start. The full guide lives in the repository root:

**[CONTRIBUTING.md](https://github.com/eddiethedean/ontocode/blob/main/CONTRIBUTING.md)**

## Quick commands

```bash
# Rust
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Extension
cd extension && npm ci && npm test

# Documentation site (MkDocs)
pip install -r docs/requirements.txt
mkdocs serve
mkdocs build --strict
```

## Documentation changes

- User guides: `docs/`
- Design specs and ADRs: `docs/design/`
- On release, follow the checklist in [releasing.md](releasing.md)

Pull requests should update docs when user-facing behavior or public APIs change.
