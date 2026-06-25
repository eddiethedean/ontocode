# OBO + ROBOT workflow example

Mixed OBO workspace demonstrating OntoIndex indexing alongside ROBOT CI steps.

## Files

- `demo.obo` — minimal OBO ontology (two terms)

## Local commands

```bash
# From repository root
cargo run -- inspect examples/obo-workflow
cargo run -- validate examples/obo-workflow
cargo run -- query examples/obo-workflow "SELECT obo_id, labels FROM entities"

# ROBOT (requires Java + robot on PATH)
cargo run -- robot validate examples/obo-workflow/demo.obo
```

## CI recipe

```yaml
- name: OntoIndex validate
  run: cargo install ontoindex-cli --locked && ontoindex validate examples/obo-workflow

- name: ROBOT validate
  run: ontoindex robot validate examples/obo-workflow/demo.obo
```

See [OBO workflow guide](https://onto-code.readthedocs.io/en/latest/guides/obo-workflow/).
