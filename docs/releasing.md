# Releasing OntoCode / OntoCore

Maintainer checklist for publishing crates, binaries, and the VS Code extension.

## Version bump

1. Update `[workspace.package].version` in root [Cargo.toml on GitHub](https://github.com/eddiethedean/ontocode/blob/main/Cargo.toml)
2. Update `extension/package.json` and `extension/webview-ui/package.json` `version`
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
- [ ] [docs/guides/protege-coexistence.md](guides/protege-coexistence.md) — must match SHIPPED
- [ ] [docs/guides/protege-decision.md](guides/protege-decision.md) — decision matrix
- [ ] [docs/guides/production-evidence.md](guides/production-evidence.md) — self-benchmark protocol
- [ ] [docs/guides/governance.md](guides/governance.md) — sustainability / support policy
- [ ] [docs/guides/platform-compatibility.md](guides/platform-compatibility.md) — VS Code / OS matrix
- [ ] [docs/guides/release-timeline.md](guides/release-timeline.md) — non-commitment timeline
- [ ] [docs/guides/production-readiness.md](guides/production-readiness.md) — pilot criteria
- [ ] [docs/guides/enterprise-deployment.md](guides/enterprise-deployment.md) — air-gap / CI rollout
- [ ] [docs/guides/performance-sizing.md](guides/performance-sizing.md) — sizing tiers
- [ ] [docs/guides/lgpl-compliance.md](guides/lgpl-compliance.md) — legal review pack
- [ ] [security.md](security.md) / [SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — supported versions table
- [ ] [docs/changelog.md](changelog.md) — mirror recent releases from CHANGELOG.md
- [ ] [docs/lsp-api.md](lsp-api.md) — new methods or error codes
- [ ] [docs/webview-protocol.md](webview-protocol.md) — React panel message protocol
- [ ] [docs/ontocode/graph-view.md](ontocode/graph-view.md), [docs/ontocode/semantic-diff.md](ontocode/semantic-diff.md), [obo-workflow.md](guides/obo-workflow.md), [robot-interop.md](guides/robot-interop.md)
- [ ] [docs/migration/v0.8.md](migration/v0.8.md) — upgrade notes when applicable
- [ ] [docs/migration/v0.9.md](migration/v0.9.md) — OntoCore identity upgrade notes when applicable
- [ ] [docs/migration/v0.10.md](migration/v0.10.md) — semantic workspace upgrade notes when applicable
- [ ] [docs/migration/v0.11.md](migration/v0.11.md) — editor depth upgrade notes when applicable
- [ ] [docs/migration/v0.12.md](migration/v0.12.md) — authoring parity upgrade notes when applicable
- [ ] [docs/migration/v0.13.md](migration/v0.13.md) — OntoUI platform upgrade notes when applicable
- [ ] [docs/migration/v0.14.md](migration/v0.14.md) — plugin host MVP upgrade notes when applicable
- [ ] [ROADMAP.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) and [docs/roadmap.md](roadmap.md) — keep shipped/planned sections in sync
- [ ] [docs/guides/plugins.md](guides/plugins.md) — plugin authoring when plugin surface changes
- [ ] [docs/design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) — status columns if features shipped
- [ ] [docs/design/ARCHITECTURE.md](design/ARCHITECTURE.md) / [OWL_AUTHORING_SPEC.md](design/OWL_AUTHORING_SPEC.md) — shipped vs target banners
- [ ] [docs/design/LICENSES.md](design/LICENSES.md) — dependency sections
- [ ] Run `mkdocs build --strict` locally before tagging
- [ ] Run `./scripts/check-doc-versions.sh` (also enforced in CI)
- [ ] Ensure **CI is green on the release commit** before tagging (the release workflow does not re-run the full test suite)

## Tag and publish

Push a tag matching `[workspace.package].version` in `Cargo.toml`:

```bash
git tag v0.16.0   # must match [workspace.package].version in Cargo.toml
git push origin v0.16.0
```

The [release workflow on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml):

1. Builds multi-platform `ontocore-lsp` binaries (matrix job)
2. Verifies the tag matches `Cargo.toml` and release binaries build
3. Publishes workspace crates to [crates.io](https://crates.io/) in dependency order
4. Creates a GitHub Release with:
   - `ontocore` Linux x64 binary
   - `ontocore-lsp` per-platform archives
   - Multi-platform `ontocode-*.vsix`

Requires the `CARGO_REGISTRY_TOKEN` repository secret. For Open VSX (Cursor), set `OVSX_PAT` — see [marketplace-publish.md](marketplace-publish.md).

## Published crates (dependency order)

`ontocore-core` → `ontocore-parser` → `ontocore-owl` → `ontocore-obo` → `ontocore-diagnostics` → `ontocore-catalog` → `ontocore-diff` → `ontocore-docs` → `ontocore-refactor` → `ontocore-query` → `ontocore-reasoner` → `ontocore-robot` → `ontocore-lsp` → `ontocore-plugin` → `ontocore` → `ontocore-cli`

## VS Code Marketplace and Open VSX

!!! warning "P0 — manual Marketplace publish"
    The [release workflow](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml) publishes crates.io, GitHub Release assets, and **Open VSX** automatically. **VS Code Marketplace publish is manual** — run `npx vsce publish` after the tag (see [marketplace-publish.md](marketplace-publish.md)).

See [marketplace-publish.md](marketplace-publish.md). Open VSX listing: [open-vsx.org/extension/ontocode/ontocode](https://open-vsx.org/extension/ontocode/ontocode).

## Verify artifacts

[release-integrity.md](release-integrity.md) — SHA256SUMS, `--locked` installs.

## Security

Report vulnerabilities per [security.md](security.md) — not via public issues.

## Read the Docs

The documentation site is built with [MkDocs](https://www.mkdocs.org/) and hosted at [ontocode-vs.readthedocs.io](https://ontocode-vs.readthedocs.io/en/latest/).

1. Read the Docs project slug: **`ontocode-vs`** (this sets the `*.readthedocs.io` subdomain; it cannot be renamed after import — the display name can be “OntoCode” in RTD settings).
2. RTD reads [`.readthedocs.yaml` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.readthedocs.yaml) and installs [docs/requirements.txt](requirements.txt).
3. `mkdocs.yml` `site_url` must match the live subdomain (`https://ontocode-vs.readthedocs.io/`).
4. Pushes to `main` rebuild the `latest` version.

### Versioning model

| RTD version | Git ref | Audience |
|-------------|---------|----------|
| `latest` | `main` | In-development docs |
| `stable` | Latest semver tag (auto) | Default for current release users |
| `v0.13.0` | Tag `v0.13.0` | Frozen docs for that release |
| `release-v0.13.0` | Branch `release/v0.13.0` | Release stabilization before tagging |

RTD slugifies branch names by replacing `/` with `-` (`release/v0.13.0` → `release-v0.13.0`).

### Automated activation (GitHub Actions)

The [readthedocs workflow](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/readthedocs.yml) activates and builds a matching RTD version when you push:

- a **release branch** (`release/v*`), or
- a **semver tag** (`v*.*.*`).

**One-time setup:** add repository secret `READTHEDOCS_API_TOKEN` ([RTD account token](https://app.readthedocs.org/account/tokens/)) with access to project `ontocode-vs`.

To activate an existing release branch immediately (e.g. after adding the secret):

1. **Actions → Read the Docs → Run workflow**, or
2. Locally: `READTHEDOCS_API_TOKEN=… ./scripts/readthedocs-activate-version.sh release/v0.13.0`

### RTD dashboard automation rules (recommended backup)

Automation rules are not configurable in `.readthedocs.yaml`; add these once under **Admin → Automation rules** so new refs activate even if the GitHub Action is unavailable:

| Description | Match | Type | Action |
|-------------|-------|------|--------|
| Activate release branches | Custom: `^release/v\d+\.\d+\.\d+$` | Branch | Activate version |
| Activate semver tags | SemVer versions | Tag | Activate version |

RTD rules apply only to **new** refs created after the rule is saved. Use the GitHub Action or `readthedocs-activate-version.sh` for branches that already exist (e.g. `release/v0.13.0`).

Local preview: `pip install -r docs/requirements.txt && mkdocs serve`.
