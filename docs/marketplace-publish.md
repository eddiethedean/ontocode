# Publishing OntoCode to the VS Code Marketplace

OntoCode is not yet on the Marketplace. This checklist is for maintainers when publication is ready.

## Prerequisites

- [Visual Studio Marketplace publisher](https://marketplace.visualstudio.com/manage) account (`ontocode` publisher id in `extension/package.json`)
- Personal Access Token with Marketplace **Manage** scope
- `vsce` installed (`npm i -g @vscode/vsce` or use project devDependency)
- Multi-platform release VSIX built by [`.github/workflows/release.yml`](../.github/workflows/release.yml)

## Pre-publish checks

1. Version bumped in `extension/package.json` and root `Cargo.toml` workspace version
2. [CHANGELOG.md](../CHANGELOG.md) updated
3. [extension/README.md](../extension/README.md) and [docs/vscode-install.md](vscode-install.md) mention Marketplace install
4. `npm test` and `cargo test --workspace` pass
5. Screenshot or preview image in README (see `docs/media/explorer-preview.svg`)

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
