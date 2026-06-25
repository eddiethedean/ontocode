# Reasoner guide

OntoCode ships **OWL EL classification** via [OntoLogos](https://github.com/eddiethedean/ontologos) 0.9.0, with optional **RL** and **RDFS** profiles (since v0.6.0).

## Run in VS Code

1. Index the workspace (**OntoCode: Index Workspace**).
2. Run **`OntoCode: Run Reasoner`** — opens the Reasoner Results panel.
3. Use **`OntoCode: Set Hierarchy Mode`** to switch the Classes tree between **asserted**, **inferred**, and **combined** hierarchies.
4. Click an unsatisfiable class in the panel to open **`OntoCode: Show Explanation`** (EL-first).

Settings:

| Setting | Default | Purpose |
|---------|---------|---------|
| `ontocode.reasoner.default` | `el` | Profile for Run Reasoner |
| `ontocode.reasoner.autoProfile` | `true` | Profile-detection warnings |
| `ontocode.hierarchy.mode` | `asserted` | Explorer hierarchy display |

**DL** and **auto** profiles are disabled until OntoLogos 1.0.0 ships on crates.io.

## CLI / CI

```bash
ontoindex classify ./my-ontologies --profile el --format json
ontoindex explain ./my-ontologies --class 'http://example.org/onto#Invalid' --profile el
```

- `classify` exits non-zero when unsatisfiable classes are found — see [workspace-limits.md](../workspace-limits.md).
- `dl` / `auto` return an error until OntoLogos 1.0.

CI example: [ci-integration.md](../ci-integration.md).

## LSP

Custom methods: `ontoindex/runReasoner`, `ontoindex/getExplanation`. See [LSP API](../lsp-api.md).

## Profiles

| Profile | Engine | Typical use |
|---------|--------|-------------|
| `el` | `ontologos-el` | OWL EL ontologies (default) |
| `rl` | `ontologos-rl` | OWL RL materialization |
| `rdfs` | `ontologos-rdfs` | RDFS entailment |
| `dl` | stub | Requires OntoLogos 1.0 |
| `auto` | stub | Requires OntoLogos 1.0 |

## Dual-stack note

OntoIndex keeps **two in-memory models** in v0.6:

- **Oxigraph + Horned-OWL** — authoritative for indexing, SPARQL/SQL, Turtle write-back, asserted hierarchy.
- **OntoLogos** — loads workspace Turtle/RDF files separately for classification.

Axiom counts and some constructs may differ from Protégé until the Horned-OWL → OntoLogos bridge ships (v1.0 backlog). Profile warnings in the Reasoner Results panel flag constructs outside the selected profile.

## Known limitations

| Limitation | Notes |
|------------|-------|
| DL / auto | Stubbed with clear error until OntoLogos 1.0 |
| Explanations | EL-first; require unsatisfiable class + prior classify |
| Hierarchy toggle | **Inferred** / **combined** need a successful reasoner run first |
| Parse coverage | OntoLogos 0.9 partial OWL mapping — see [Protégé parity](../design/PROTEGE_PARITY.md) |

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| `dl` / `auto` error | Use `el`, `rl`, or `rdfs` |
| Explorer unchanged after classify | Run **Set Hierarchy Mode** → inferred or combined |
| Empty explanation | Class may not be unsatisfiable; run reasoner first |
| Results differ from Protégé | Expected for partial mappings; check profile warnings |

More: [troubleshooting.md](../troubleshooting.md) · [FAQ](../faq.md).

## Related

- [What ships today](../SHIPPED.md)
- [CLI reference](../cli-reference.md)
- [Errors reference](../errors.md)
