# Publishing OntoCode to the VS Code Marketplace

OntoCode is on the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode). This checklist is for maintainers publishing new versions.

## Prerequisites

- [Visual Studio Marketplace publisher](https://marketplace.visualstudio.com/manage) account (`ontocode` publisher id in `extension/package.json`)
- Personal Access Token with Marketplace **Manage** scope
- `vsce` installed (`npm i -g @vscode/vsce` or use project devDependency)
- Multi-platform release VSIX built by [release workflow on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml)

## Pre-publish checks

1. Version bumped in `extension/package.json` and root `Cargo.toml` workspace version
2. [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) updated
3. [extension/README.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/extension/README.md) and [docs/vscode-install.md](vscode-install.md) mention Marketplace install and current version
4. User docs synced per [releasing.md](releasing.md) checklist
5. `npm test` and `cargo test --workspace` pass
6. **Marketplace visuals:** run `./scripts/render-explorer-preview.sh` after editing `docs/media/explorer-preview.svg` (syncs to `extension/media/explorer-preview.png`); `package.json` includes a `screenshots` entry
7. `extension/README.md` hero image uses `media/explorer-preview.png` (not a docs-only path)
8. Short description avoids overstating Protégé parity — point to [Protégé migration guide](guides/protege-migration.md) instead

## Publish command

```bash
cd extension
npm ci && npm run compile
npx vsce publish --no-dependencies
```

For a one-off VSIX without publishing:

```bash
npx vsce package --no-dependencies
```

## After publish

1. Update root README **Choose your path → VS Code** to link Marketplace and Open VSX listings first, VSIX as fallback
2. Verify Open VSX and Marketplace badges on README and [docs/index.md](index.md) (Marketplace: `https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg` — shields.io `visual-studio-marketplace` badges are retired)
3. Tag release and attach VSIX for users who prefer offline install

## Token handling

Store `VSCE_PAT` or `AZURE_DEVOPS_EXT_PAT` in CI secrets only; never commit tokens.

## Open VSX (Cursor and other Open VSX clients)

From v0.11.1, the [release workflow on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml) publishes the same VSIX to [Open VSX](https://open-vsx.org/) after packaging.

### Prerequisites

- [Open VSX publisher](https://open-vsx.org/user-settings/publisher) namespace `ontocode` (claim before first release)
- Personal Access Token from Open VSX user settings
- Repository secret `OVSX_PAT` with publish scope

### Manual publish (emergency)

```bash
npm install -g ovsx
ovsx publish dist/ontocode-v0.11.1.vsix -p "$OVSX_PAT"
```

### After Open VSX publish

1. Verify listing at [open-vsx.org/extension/ontocode/ontocode](https://open-vsx.org/extension/ontocode/ontocode) (badge: `https://img.shields.io/open-vsx/v/ontocode/ontocode`)
2. Confirm Cursor Extensions search finds **OntoCode**
3. Document Cursor install path in [vscode-install.md](vscode-install.md) Option E
