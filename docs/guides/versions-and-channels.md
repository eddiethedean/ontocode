# Versions and channels

**How to pick the right OntoCode / OntoCore build.** Pin production and CI to a **tagged** release; do not follow `main` docs alone.

## Source of truth

| Source | What it means |
|--------|----------------|
| [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/ontocode/blob/main/docs/TAGGED_RELEASE) | Canonical public install version (today: **0.26.2**) |
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

## Recommended installs (v0.26.2)

| Goal | Command / link |
|------|----------------|
| VS Code | Marketplace **or** download `ontocode-v0.26.2.vsix` from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases/tag/v0.26.2) |
| Cursor | [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) or the same VSIX |
| CLI (pinned) | `cargo install ontocore-cli --locked --version 0.26.2` |
| CLI (Linux, no compile) | `ontocore-v0.26.2-x86_64-unknown-linux-gnu.tar.gz` from GitHub Releases |
| Tutorial files (offline) | Prefer the curl/PowerShell samples in [First success](first-success.md). Optional: `ontocode-tutorial.zip` from the same GitHub Release when attached |

Always pin: bare `cargo install ontocore-cli` resolves to the **latest** crates.io version and can jump without your review.

## When Marketplace lags GitHub

Marketplace publish is always **manual** after the release workflow finishes. **Open VSX** publishes automatically when `OVSX_PAT` is set (see [Marketplace publish](../marketplace-publish.md)). Either store can lag the GitHub tag by hours or longer.

### Playbook: store version ≠ latest tag

1. Check the [latest GitHub Release](https://github.com/eddiethedean/ontocode/releases/latest) tag (example: `v0.26.2`).
2. In VS Code / Cursor: **Extensions → OntoCode** — note the installed version.
3. If the store is older than the tag you need:
   - Download `ontocode-v<version>.vsix` from that Release.
   - **Extensions → … → Install from VSIX…**
   - Reload the window.
4. Confirm the extension version matches the tag under the Extension details.
5. Keep CI pinned with `cargo install ontocore-cli --locked --version <tag>` (or the Linux tarball) — do not wait for Marketplace for automation.

Re-check Marketplace / Open VSX before an org-wide rollout. Capability truth by version: [What ships today](../SHIPPED.md). Upgrades: [Migration guides](../migration/README.md).

Maintainer notes: [Marketplace publish](../marketplace-publish.md).
