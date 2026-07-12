# Mutation testing baseline (path_jail + OWL patch)

Greenfield, **not** a PR gate. Run locally or via `.github/workflows/mutants.yml`
(`workflow_dispatch` / weekly cron):

```bash
cargo install cargo-mutants --locked
./scripts/run-mutants.sh --timeout 120 --jobs 2
```

## Baseline (2026-07-11, after oracle + extract-path tests)

### `ontocore-core` / `path_jail.rs`

| Metric | First run | After stronger tests |
|--------|-----------|----------------------|
| Mutants tested | 35 | 35 |
| Caught | 24 | **31** |
| Missed | 11 | **4** |
| Miss rate | ~31% | **~11%** |

Remaining misses (non-blocking): `||`/`&&` edges in `resolve_path_in_workspace` / `is_path_within`, `is_path_within_lexical → true`, and one `ensure_extract_path_within` match-arm delete (covered by sibling parent-escape checks).

Critical escape paths covered by unit tests: `ensure_extract_path_within` (`..`, absolute, empty),
symlink escape, sibling prefix trap, multi-root outside reject, `discover_git_repo_root`.

### `ontocore-owl` / `patch.rs`

Run via `./scripts/run-mutants.sh` (second step). Success-path ops for the ten catalog oracles
in `tests/owl_patch_oracles.rs` should kill no-op `apply_one_patch` arms; adversarial
IRI/CURIE string tests remain in `patch.rs` for injection mutants.

Artifacts: `mutants.out/` (gitignored locally when present).
