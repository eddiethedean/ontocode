# Release integrity

How to verify OntoCode / OntoIndex release artifacts from [GitHub Releases](https://github.com/eddiethedean/ontocode/releases).

## Checksums

Each release includes `SHA256SUMS` with SHA-256 hashes of:

- `ontoindex-*.tar.gz` (CLI binary)
- `ontoindex-lsp-*.tar.gz` / `.zip` (per-platform LSP)
- `ontocode-*.vsix` (VS Code extension)

Verify after download:

```bash
shasum -a 256 -c SHA256SUMS
```

On Linux you may use `sha256sum -c SHA256SUMS` instead.

## crates.io

Rust crates are published from CI using a restricted `CARGO_REGISTRY_TOKEN`. Install with:

```bash
cargo install ontoindex-cli --locked
```

Prefer `--locked` so dependency versions match the published crate.

## VS Code extension

1. Download the `.vsix` from the release matching your platform (multi-platform VSIX bundles `ontoindex-lsp`).
2. Verify against `SHA256SUMS`.
3. Install via **Extensions → Install from VSIX…**

`ontocode.lspPath` is a **trusted-admin** setting. In VS Code **Restricted Mode** (untrusted workspace), the extension ignores workspace `lspPath` and uses the bundled server.

## Dependency auditing

CI runs `cargo audit` on the Rust workspace. Report vulnerable dependencies via [SECURITY.md](../SECURITY.md).

## Future: signed artifacts

Code signing and Sigstore attestations for release binaries are planned. Until then, use checksums and install from the official GitHub Releases page only.
