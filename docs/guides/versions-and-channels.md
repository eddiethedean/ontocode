# Versions and channels

**How to pick the right OntoCode / OntoCore build.** Pin production and CI to a **tagged** release; do not follow `main` docs alone.

## Source of truth

| Source | What it means |
|--------|----------------|
| [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/ontocode/blob/main/docs/TAGGED_RELEASE) | Canonical public install version (today: **0.22.0**) |
| [GitHub Releases](https://github.com/eddiethedean/ontocode/releases) | VSIX, Linux CLI tarball, multi-platform LSP, tutorial zip, checksums |
| [crates.io](https://crates.io/crates/ontocore-cli) | Published Rust crates and `cargo install ontocore-cli` |
| [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) | Extension for VS Code (manual publish; usually matches the tag within hours) |
| [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) | Extension for Cursor / Open VSX clients |
| Read the Docs `latest` | Built from the default branch (`main`) — may describe work **after** the last tag |

## How to check which version you have

| Surface | How |
|---------|-----|
| **VS Code / Cursor extension** | Extensions view → **OntoCode** → version under the title (or `ontocode.ontocode` in the extension details) |
| **CLI** | `ontocore --version` (or `ontocore -V`) |
| **Language server** | **Output → OntoCore Language Server** often logs the binary path; or run the bundled `ontocore-lsp` with `--version` if you know its path |

If Marketplace / Open VSX is behind GitHub, install the release VSIX for the tag you need — see below.

## Recommended installs (v0.22.0)

| Goal | Command / link |
|------|----------------|
| VS Code | Marketplace **or** download `ontocode-v0.22.0.vsix` from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases/tag/v0.22.0) |
| Cursor | [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) or the same VSIX |
| CLI (pinned) | `cargo install ontocore-cli --locked --version 0.22.0` |
| CLI (Linux, no compile) | `ontocore-v0.22.0-x86_64-unknown-linux-gnu.tar.gz` from GitHub Releases |
| Tutorial files (offline) | `ontocode-tutorial.zip` from the same GitHub Release |

Always pin: bare `cargo install ontocore-cli` resolves to the **latest** crates.io version and can jump without your review.

## When Marketplace lags GitHub

Marketplace and Open VSX publishes are **manual** steps after the release workflow finishes (see [Marketplace publish](../marketplace-publish.md)). If the store listing is behind the GitHub tag:

1. Prefer the **GitHub Release VSIX** for the version you need.
2. Keep CI on crates.io with an explicit `--version`.
3. Re-check Marketplace/Open VSX before org-wide rollout.

For capability truth by version, use [What ships today](../SHIPPED.md). For upgrades, use [Migration guides](../migration/README.md). Maintainer notes: [Marketplace publish](../marketplace-publish.md).
