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
6. **Marketplace visuals:** `extension/media/explorer-preview.png` exists (sync from `docs/media/explorer-preview.png` when the docs image changes) and `package.json` includes a `screenshots` entry
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

1. Update root README **Choose your path → VS Code** to link Marketplace listing first, VSIX as fallback
2. Add Marketplace badge to README
3. Tag release and attach VSIX for users who prefer offline install

## Token handling

Store `VSCE_PAT` or `AZURE_DEVOPS_EXT_PAT` in CI secrets only; never commit tokens.
