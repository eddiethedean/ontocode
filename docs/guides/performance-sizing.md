# Performance and sizing

Guidance for sizing OntoIndex workspaces and planning pilots. **Formal benchmark suite is a v1.0 backlog item** — validate performance on your own corpora before production rollout.

Hard limits: [workspace limits](../workspace-limits.md). Pilot criteria: [production readiness](production-readiness.md).

## Hard limits (enforced)

| Resource | Cap | Failure mode |
|----------|-----|--------------|
| Ontology files per workspace | 10,000 | Index error |
| Single file size | 50 MB | Skip / error |
| Total RDF triples | 20,000,000 | Index error |
| Triples per file | 5,000,000 | Per-file error |
| Entities | 1,000,000 | Index error |
| SQL/SPARQL result rows | 100,000 | **Silent truncation** |
| Open LSP document buffers | 256 | LSP limit |

## Sizing tiers (qualitative)

Use these tiers to choose pilot scope. **Run `ontoindex inspect` on your repo** to measure actual counts.

| Tier | Files | Triples (order of magnitude) | Typical profile | v0.7 fit |
|------|-------|------------------------------|-----------------|----------|
| **Small** | 1–20 | &lt; 100k | Single-domain Turtle, EL | Excellent |
| **Medium** | 20–500 | 100k–5M | Multi-file imports, mixed formats | Good — monitor index time |
| **Large** | 500–10k | 5M–20M | Enterprise taxonomy, heavy imports | Pilot required — approach caps |
| **Extra-large** | &gt; 10k files or &gt; 20M triples | — | Full OBO, massive SKOS | **Not supported** — split workspace or wait for incremental indexing |

## Reference measurement (tutorial fixtures)

Measured with `ontoindex inspect fixtures --format json` on release **0.7.0** (repository tutorial corpus):

| Metric | Value |
|--------|-------|
| Ontology files | 5 |
| Classes | 16 |
| Triples | 88 |
| Diagnostic errors | 0 |

Use this as a **smoke-test baseline only** — not representative of production ontologies.

## Recommended pilot benchmarks

Run on a **representative clone** of your production ontology tree:

```bash
# Replace with your ontology root
ONTO=/path/to/ontologies
VERSION=0.7.0

# Catalog stats
time ./ontoindex-v${VERSION}-x86_64-unknown-linux-gnu inspect "$ONTO" --format json

# Validation (full index + lint)
time ./ontoindex-v${VERSION}-x86_64-unknown-linux-gnu validate "$ONTO"

# Optional: classification
time ./ontoindex-v${VERSION}-x86_64-unknown-linux-gnu classify "$ONTO" --profile el --format json
```

Record:

| Measurement | Why |
|-------------|-----|
| Wall-clock `inspect` | Index + catalog build time |
| Wall-clock `validate` | CI gate latency budget |
| Wall-clock `classify` | Reasoner cost (loads OntoLogos model separately) |
| Peak RSS memory | CI runner sizing (use `/usr/bin/time -v` on Linux) |
| `triple_count`, `class_count` | Compare to tier table |

**Acceptance suggestion:** CI `validate` completes within your pipeline stage budget (e.g. &lt; 5 minutes) on `main` branch corpora.

## Memory model (v0.7)

Documentation and architecture specs describe **multiple in-memory representations**:

| Model | Used for |
|-------|----------|
| Oxigraph triple store | SPARQL, triple counts |
| Horned-OWL catalog | Turtle axioms, write-back, explorer |
| OntoLogos ontology | `classify` / reasoner (separate load) |

Running **reasoner + full index** uses more memory than `validate` alone. Size CI runners accordingly when adding `classify` jobs.

## Query performance notes

| Pattern | Guidance |
|---------|----------|
| SQL on catalog tables | Fast for filtered `SELECT` on single virtual table |
| `SELECT *` on large tables | May hit 100k row truncation — use `WHERE` |
| SPARQL graph scan | Cost scales with triple count; always use `LIMIT` in exploration |
| Re-index on each CLI invocation | `query`, `sparql`, `validate` re-index workspace each run |

For very large repos in CI, point commands at a **subdirectory**:

```bash
ontoindex validate ./src/ontologies
```

## VS Code interactive use

| Factor | Impact |
|--------|--------|
| Initial workspace index | Proportional to file count and triple count |
| Re-index on save | Debounced; large unsaved buffers count toward 50 MB/file cap |
| Query Workbench | Same row cap as CLI; watch `truncated` in UI |
| Multi-root workspace | Only **first** folder indexed — appears as "performance" issues if misconfigured |

## When to split workspaces

Consider splitting when:

- File count approaches **10,000**
- Total triples approach **20M**
- Full OBO or biomedical imports dominate — may exceed limits ([workspace limits](../workspace-limits.md))
- CI time exceeds pipeline budget — validate per submodule in matrix jobs

## Future work

- v1.0: formal performance benchmarks document ([Protégé parity P1](../design/PROTEGE_PARITY.md))
- v0.9: incremental indexing ([roadmap](../design/ROADMAP.md))

## Related

- [Workspace limits](../workspace-limits.md)
- [CI integration](../ci-integration.md)
- [Best practices](best-practices.md)
- [Production readiness](production-readiness.md)
