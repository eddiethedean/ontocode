# Releasing OntoCode / OntoIndex

Maintainer checklist for publishing crates, binaries, and the VS Code extension.

## Version bump

1. Update `[workspace.package].version` in root [Cargo.toml on GitHub](https://github.com/eddiethedean/ontocode/blob/main/Cargo.toml)
2. Update `extension/package.json` `version`
3. Update [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md)
4. Regenerate [NOTICES](../NOTICES) if dependencies changed (`cargo license` recommended)
5. Sync user-facing docs (see checklist below)

## Documentation sync checklist (every release)

- [ ] [Root README on GitHub](https://github.com/eddiethedean/ontocode/blob/main/README.md) — version, "What ships", architecture diagram, Read the Docs badge
- [ ] [docs index](index.md) — shipped version labels
- [ ] [extension/README.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/extension/README.md) — "What's included"
- [ ] [docs/vscode-install.md](vscode-install.md) — recommended version
- [ ] [security.md](security.md) / [SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — supported versions table
- [ ] [docs/lsp-api.md](lsp-api.md) / [docs/sql-reference.md](sql-reference.md) — version headers
- [ ] [docs/design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) — status columns if features shipped
- [ ] [docs/design/LICENSES.md](design/LICENSES.md) — dependency sections

## Tag and publish

Push a tag matching `[workspace.package].version` in `Cargo.toml`:

```bash
git tag v0.4.0
git push origin v0.4.0
```

The [release workflow on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml):

1. Verifies packages and runs tests
2. Publishes workspace crates to [crates.io](https://crates.io/) in dependency order
3. Creates a GitHub Release with:
   - `ontoindex` Linux x64 binary
   - `ontoindex-lsp` per-platform archives
   - Multi-platform `ontocode-*.vsix`

Requires the `CARGO_REGISTRY_TOKEN` repository secret.

## Published crates (dependency order)

`ontoindex-core` → `ontoindex-parser` → `ontoindex-owl` → `ontoindex-diagnostics` → `ontoindex-catalog` → `ontoindex-query` → `ontoindex-lsp` → `ontoindex-cli`

## VS Code Marketplace

See [marketplace-publish.md](marketplace-publish.md).

## Verify artifacts

[release-integrity.md](release-integrity.md) — SHA256SUMS, `--locked` installs.

## Security

Report vulnerabilities per [security.md](security.md) — not via public issues.

## Read the Docs

The documentation site is built with [MkDocs](https://www.mkdocs.org/) and hosted on [Read the Docs](https://onto-code.readthedocs.io/).

1. Import the GitHub repository at [readthedocs.org/dashboard/import](https://readthedocs.org/dashboard/import/) (project slug: `onto-code`).
2. RTD reads [`.readthedocs.yaml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.readthedocs.yaml) and installs [docs/requirements.txt](requirements.txt).
3. Pushes to `main` rebuild the `latest` version; tags can publish versioned docs.

Local preview: `pip install -r docs/requirements.txt && mkdocs serve`.
