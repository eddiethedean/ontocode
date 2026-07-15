# Releasing OntoCode / OntoCore

Maintainer checklist for publishing crates, binaries, and the VS Code extension.

## Version bump

1. Update `[workspace.package].version` in root [Cargo.toml on GitHub](https://github.com/eddiethedean/ontocode/blob/main/Cargo.toml)
2. Update **`docs/TAGGED_RELEASE`** in the release commit that cuts that tag — public install pins must equal the **published** GitHub Release version (never bump pins on a working branch before the tag exists)
3. Update `extension/package.json` and `extension/webview-ui/package.json` `version`
3. Update [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [docs/changelog.md](changelog.md)
4. Regenerate [NOTICES on GitHub](https://github.com/eddiethedean/ontocode/blob/main/NOTICES) if dependencies changed (`cargo license` recommended)
5. Sync user-facing docs (see checklist below)

## Documentation sync checklist (every release)

### Tier-1 capability truth (must match SHIPPED — do these first)

Source of truth: **[docs/SHIPPED.md](SHIPPED.md)** and **[docs/supported-formats.md](supported-formats.md)**. Every Tier-1 surface below must agree on writable formats, current tagged version, and evaluate-pack caveats (byte-identical XML is a non-goal; write-back itself must not be described as planned/read-only after it ships).

- [ ] **[docs/SHIPPED.md](SHIPPED.md)** — canonical capability matrix (update first)
- [ ] **[docs/supported-formats.md](supported-formats.md)** — format write-back matrix
- [ ] [Root README on GitHub](https://github.com/eddiethedean/ontocode/blob/main/README.md) — “Editable today”, install pins, migration pointer for this release
- [ ] [docs/index.md](index.md) — hero version, write-back warning, capability table
- [ ] [docs/guides/first-success.md](guides/first-success.md) — banners, step 4, and troubleshooting agree
- [ ] [docs/faq.md](faq.md) — Protégé `.owl` edit answer, gaps list, formats
- [ ] [extension/README.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/extension/README.md) — “Editable today” / What’s included
- [ ] [docs/guides/owl-xml-workflow.md](guides/owl-xml-workflow.md) + [mkdocs.yml](https://github.com/eddiethedean/ontocode/blob/main/mkdocs.yml) Interop nav label
- [ ] [docs/guides/protege-decision.md](guides/protege-decision.md), [production-readiness.md](guides/production-readiness.md), [enterprise-eval.md](guides/enterprise-eval.md), [procurement-appendix.md](guides/procurement-appendix.md), [protege-coexistence.md](guides/protege-coexistence.md), [protege-migration.md](guides/protege-migration.md)
- [ ] [docs/troubleshooting.md](troubleshooting.md), [docs/vscode-install.md](vscode-install.md), [docs/start.md](start.md)
- [ ] [docs/patch-reference.md](patch-reference.md), [docs/cli-reference.md](cli-reference.md), [docs/lsp-api.md](lsp-api.md)
- [ ] [docs/roadmap.md](roadmap.md) + [ROADMAP.md](https://github.com/eddiethedean/ontocode/blob/main/ROADMAP.md) — tagged release must be **Shipped**, not Planned
- [ ] [docs/migration/README.md](migration/README.md) + this release’s `docs/migration/vN.md`
- [ ] Run `./scripts/check-doc-versions.sh` (also enforced in CI) — includes stale write-back claim greps

### Full sync (after Tier-1)

- [ ] [docs/getting-started.md](getting-started.md) — release binary examples / `cargo install` pin
- [ ] [docs/errors.md](errors.md) / [docs/workspace-limits.md](workspace-limits.md) — behavior changes
- [ ] [docs/guides/production-evidence.md](guides/production-evidence.md) — self-benchmark protocol
- [ ] [docs/guides/governance.md](guides/governance.md) — sustainability / support policy
- [ ] [docs/guides/platform-compatibility.md](guides/platform-compatibility.md) — VS Code / OS matrix
- [ ] [docs/guides/release-timeline.md](guides/release-timeline.md) — non-commitment timeline
- [ ] [docs/guides/enterprise-deployment.md](guides/enterprise-deployment.md) — air-gap / CI rollout
- [ ] [docs/guides/performance-sizing.md](guides/performance-sizing.md) — sizing tiers
- [ ] [docs/guides/lgpl-compliance.md](guides/lgpl-compliance.md) — legal review pack
- [ ] [security.md](security.md) / [SECURITY.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/SECURITY.md) — supported versions table
- [ ] [docs/changelog.md](changelog.md) — mirror recent releases from CHANGELOG.md
- [ ] [docs/webview-protocol.md](webview-protocol.md) — React panel message protocol
- [ ] [docs/ontocode/feature-tour.md](ontocode/feature-tour.md), [graph-view.md](ontocode/graph-view.md), [semantic-diff.md](ontocode/semantic-diff.md), [obo-workflow.md](guides/obo-workflow.md), [robot-interop.md](guides/robot-interop.md)
- [ ] [docs/authoring.md](authoring.md), [docs/concepts.md](concepts.md), [docs/ontocore/lsp.md](ontocore/lsp.md)
- [ ] Crate README “Current version” / `--version` pins (`crates/ontocore*`)
- [ ] [docs/migration/v0.20.md](migration/v0.20.md) — when applicable
- [ ] [docs/migration/v0.21.md](migration/v0.21.md) — RDF/XML + OWL/XML write-back (add a row for each new minor)
- [ ] [docs/guides/plugins.md](guides/plugins.md) — plugin authoring when plugin surface changes
- [ ] [docs/design/PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) — status columns if features shipped (banner if historical)
- [ ] [docs/design/ARCHITECTURE.md](design/ARCHITECTURE.md) / [OWL_AUTHORING_SPEC.md](design/OWL_AUTHORING_SPEC.md) — shipped vs target banners
- [ ] [docs/design/LICENSES.md](design/LICENSES.md) — dependency sections
- [ ] Run `./scripts/build-docs.sh` locally before tagging
- [ ] Build tutorial pack: `./scripts/package-tutorial-zip.sh` and attach `ontocode-tutorial.zip` to the GitHub Release
- [ ] Ensure **CI is green on the release commit** before tagging (the release workflow **requires** a successful `ci.yml` run on that SHA; it does not re-run the full test suite)

## Tag and publish

Push a tag matching `[workspace.package].version` in `Cargo.toml`:

```bash
git tag v0.25.0   # must match [workspace.package].version in Cargo.toml
git push origin v0.25.0
```

The [release workflow on GitHub](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/release.yml):

1. **Preflight (tag gates):** tag↔version match, doc version sync, rustfmt, and a green `ci.yml` run on the tagged SHA (waits briefly if CI is still in progress)
2. **LSP matrix** (parallel with publish after preflight): multi-platform `ontocore-lsp` binaries
3. **Publish crates.io** (starts after preflight; overlaps LSP builds): workspace crates in dependency order with `--no-verify`, retrying only on 429 / index lag (no fixed inter-crate sleep — crates.io allows a burst of 30 version updates)
4. **Package + GitHub Release** (after LSP matrix): Linux x64 `ontocore` CLI, per-platform LSP archives, multi-platform VSIX, Open VSX (if `OVSX_PAT` is set), `SHA256SUMS` + `NOTICES`

Requires the `CARGO_REGISTRY_TOKEN` repository secret. For Open VSX (Cursor), set `OVSX_PAT` — see [marketplace-publish.md](marketplace-publish.md).

## Published crates (dependency order)

`ontocore-core` → `ontocore-parser` → `ontocore-owl` → `ontocore-obo` → `ontocore-edit` → `ontocore-diagnostics` → `ontocore-catalog` → `ontocore-diff` → `ontocore-docs` → `ontocore-swrl` → `ontocore-refactor` → `ontocore-query` → `ontocore-reasoner` → `ontocore-robot` → `ontocore-plugin` (+ plugin crates) → `ontocore-lsp` → `ontocore` → `ontocore-cli`

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
