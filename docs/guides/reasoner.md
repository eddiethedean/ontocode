# Reasoner guide (v0.6)

OntoCode v0.6 adds **OWL EL classification** via [OntoLogos](https://github.com/eddiethedean/ontologos) 0.9.0, with optional **RL** and **RDFS** profiles.

## Run in VS Code

1. Index the workspace (`OntoCode: Index Workspace`).
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

- `classify` exits non-zero when unsatisfiable classes are found.
- `dl` / `auto` return an error until OntoLogos 1.0.

## LSP

Custom methods: `ontoindex/runReasoner`, `ontoindex/getExplanation`. See [LSP API](../lsp-api.md).
