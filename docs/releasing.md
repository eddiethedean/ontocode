# Releasing OntoCode / OntoIndex

Maintainer checklist for publishing crates, binaries, and the VS Code extension.

## Version bump

1. Update `[workspace.package].version` in root [Cargo.toml on GitHub](https://github.com/eddiethedean/ontocode/blob/main/Cargo.toml)
2. Update `extension/package.json` `version`
3. Update [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [docs/changelog.md](changelog.md)
4. Regenerate [NOTICES on GitHub](https://github.com/eddiethedean/ontocode/blob/main/NOTICES) if dependencies changed (`cargo license` recommended)
5. Sync user-facing docs (see checklist below)

## Documentation sync checklist (every release)

- [ ] **[docs/SHIPPED.md](SHIPPED.md)** — canonical capability matrix (update first)
- [ ] [Root README on GitHub](https://github.com/eddiethedean/ontocode/blob/main/README.md) — version, link to SHIPPED.md, Read the Docs badge
- [ ] [docs/index.md](index.md) — hero version, capability table, documentation map
- [ ] [extension/README.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/extension/README.md) — "What's included", command table
- [ ] [docs/vscode-install.md](vscode-install.md) — recommended version, commands
- [ ] [docs/getting-started.md](getting-started.md) — release binary examples (Path D)
- [ ] [docs/faq.md](faq.md) — API version, Protégé comparison
- [ ] [docs/errors.md](errors.md) / [docs/workspace-limits.md](workspace-limits.md) — behavior changes
- [ ] [docs/guides/enterprise-eval.md](guides/enterprise-eval.md) — shipped capabilities
- [ ] [docs/guides/production-readiness.md](guides/production-readiness.md) — pilot criteria
- [ ] [docs/guides/enterprise-deployment.md](guides/enterprise-deployment.md) — air-gap / CI rollout
- [ ] [docs/guides/performance-sizing.md](guides/performance-sizing.md) — sizing tiers
- [ ] [docs/guides/lgpl-compliance.md](guides/lgpl-compliance.md) — legal review pack
- [ ] [security.md](security.md) / [SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — supported versions table
- [ ] [docs/changelog.md](changelog.md) — mirror recent releases from CHANGELOG.md
- [ ] [docs/lsp-api.md](lsp-api.md) — new methods or error codes
- [ ] [docs/design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) — status columns if features shipped
- [ ] [docs/design/ARCHITECTURE.md](design/ARCHITECTURE.md) / [OWL_AUTHORING_SPEC.md](design/OWL_AUTHORING_SPEC.md) — shipped vs target banners
- [ ] [docs/design/LICENSES.md](design/LICENSES.md) — dependency sections
- [ ] Run `mkdocs build --strict` locally before tagging
- [ ] Run `./scripts/check-doc-versions.sh` (also enforced in CI)

## Tag and publish

Push a tag matching `[workspace.package].version` in `Cargo.toml`:

```bash
git tag v0.6.0   # must match [workspace.package].version in Cargo.toml
git push origin v0.6.0
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

`ontoindex-core` → `ontoindex-parser` → `ontoindex-owl` → `ontoindex-diagnostics` → `ontoindex-catalog` → `ontoindex-query` → `ontoindex-reasoner` → `ontoindex-lsp` → `ontoindex-cli`

## VS Code Marketplace

See [marketplace-publish.md](marketplace-publish.md).

## Verify artifacts

[release-integrity.md](release-integrity.md) — SHA256SUMS, `--locked` installs.

## Security

Report vulnerabilities per [security.md](security.md) — not via public issues.

## Read the Docs

The documentation site is built with [MkDocs](https://www.mkdocs.org/) and hosted at [ontocode-vs.readthedocs.io](https://ontocode-vs.readthedocs.io/en/latest/).

1. Read the Docs project slug: **`ontocode-vs`** (this sets the `*.readthedocs.io` subdomain; it cannot be renamed after import — the display name can be “OntoCode” in RTD settings).
2. RTD reads [`.readthedocs.yaml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.readthedocs.yaml) and installs [docs/requirements.txt](requirements.txt).
3. `mkdocs.yml` `site_url` must match the live subdomain (`https://ontocode-vs.readthedocs.io/`).
4. Pushes to `main` rebuild the `latest` version; tags can publish versioned docs.

Local preview: `pip install -r docs/requirements.txt && mkdocs serve`.
