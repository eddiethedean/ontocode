# Install the OntoCore CLI (macOS and Windows)

> **Canonical install:** [Install](../install.md). **Most users never need this page.** The [OntoCode VS Code / Cursor extension](../vscode-install.md) bundles `ontocore-lsp`. Install the extension for IDE work and skip compiling the CLI unless you need `ontocore` for CI, scripting, or local validation outside the editor.

Release CLI tarballs are **Linux x64 only** (`x86_64-unknown-linux-gnu`). On **macOS** and **Windows**, install from crates.io with Rust. On **Linux arm64** (and other non-x64 targets), there is no tarball — use `cargo install` below or the LSP bundled in the VSIX ([platform compatibility](platform-compatibility.md)).

Canonical pin: **`0.26.1`** ([TAGGED_RELEASE](../TAGGED_RELEASE)). See also [Install CLI & CI (detail)](../install-cli-ci.md) and [CI integration](../ci-integration.md) (Linux CI prefers the release tarball).

## Prerequisites

| Requirement | macOS | Windows |
|-------------|-------|---------|
| Rust toolchain | **1.88+** via [rustup](https://rustup.rs/) | **1.88+** via rustup |
| System tools | Xcode Command Line Tools: `xcode-select --install` | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ workload) |
| Disk / time | Expect **15–30+ minutes** cold compile and several GB for cargo cache | Same |

### Apple Silicon notes

- Install the **aarch64** Rust toolchain via rustup on Apple Silicon Macs (default on current macOS).
- Ensure `~/.cargo/bin` is on your `PATH` (rustup installer usually configures this).

### Verify tools

```bash
rustc --version   # 1.88 or newer
cargo --version
```

macOS:

```bash
xcode-select -p   # should print a Developer path
```

## Install

```bash
cargo install ontocore-cli --locked --version 0.26.1
```

Confirm:

```bash
ontocore --help
which ontocore   # typically ~/.cargo/bin/ontocore
```

!!! warning "Wrong crate name"
    The GitHub repo is `ontocode`. The crate and CLI are **`ontocore-cli`** / **`ontocore`**. Do not run `cargo install ontocode`.

## First commands

Use **your** ontology directory (there is no `fixtures/` outside a git clone):

```bash
ontocore validate /path/to/your/ontologies
ontocore query /path/to/your/ontologies "SELECT * FROM classes"
```

## When to use the extension instead

| Goal | Prefer |
|------|--------|
| Browse / edit in the IDE | [VS Code extension](../vscode-install.md) |
| Local CI-like validate/query without VS Code | This CLI install |
| Linux CI in GitHub Actions | [Release tarball](../ci-integration.md) (faster than compile) |
| Custom `ontocode.lspPath` without compiling | Prebuilt **`ontocore-lsp-*`** archives on [GitHub Releases](https://github.com/eddiethedean/ontocode/releases/tag/v0.26.1) (macOS, Windows, Linux x64/arm64) — set `ontocode.lspPath` to the extracted binary (trusted workspace) |

## Optional: prebuilt language server (skip `cargo install` for LSP)

Release assets include platform-specific `ontocore-lsp` binaries (for example `ontocore-lsp-*-apple-darwin.tar.gz`, `ontocore-lsp-*-pc-windows-msvc.zip`, Linux gnu/musl and arm64 variants). Download, verify against `SHA256SUMS`, extract, and point `ontocode.lspPath` at the binary. Most users should keep the **bundled** LSP from the VSIX and skip this.

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| `ontocore: command not found` | Add `~/.cargo/bin` to `PATH`; open a new terminal |
| Compile errors on Windows | Install MSVC Build Tools with the C++ desktop workload |
| Compile errors on macOS | Re-run `xcode-select --install`; update Xcode CLT |
| Wrong version after install | Re-run with `--version 0.26.1 --locked`; check `ontocore --version` |
| Slow installs | Expected on first cold build; subsequent upgrades are faster |

## Related

- [Install CLI & CI (detail)](../install-cli-ci.md)
- [Which artifact?](which-artifact.md)
- [Release integrity](../release-integrity.md)
- [Versions and channels](versions-and-channels.md)
