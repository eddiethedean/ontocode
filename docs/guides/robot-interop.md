# ROBOT interop

OntoCore v0.19.0 wraps the [ROBOT](http://robot.obolibrary.org/) CLI for validate, merge, and report workflows. ROBOT runs as an **external Java process** — OntoCore does not embed a JVM.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Prerequisites

| Requirement | Notes |
|-------------|-------|
| **Java** | ROBOT requires a JRE on the agent or developer machine |
| **`robot` on PATH** | Or set an explicit path (see below) |
| **OntoCore 0.19.0+** | `ontocore robot` subcommand or LSP `ontocore/runRobot` |

Install ROBOT from [robot.obolibrary.org](http://robot.obolibrary.org/).

## CLI

```bash
# Validate
ontocore robot validate path/to/ontology.owl

# Merge
ontocore robot merge --inputs a.owl --inputs b.owl --output merged.owl

# Report
ontocore robot report path/to/ontology.owl --report report.tsv
```

Override the executable:

```bash
ontocore robot validate demo.obo --robot-path /opt/robot/robot.jar
```

Full flag reference: [CLI reference](../cli-reference.md#robot).

## VS Code

Set **`ontocode.robotPath`** in settings to the `robot` executable or JAR when it is not on `PATH`. Trusted workspaces only (ignored in Restricted Mode).

LSP clients can call `ontocore/runRobot` — see [LSP API](../lsp-api.md).

## CI recipe

```yaml
- name: Install OntoCore
  run: cargo install ontocore-cli --locked --version 0.20.0

- name: OntoCore validate
  run: ontocore validate ./ontologies

- name: ROBOT validate
  run: ontocore robot validate ./ontologies/core.owl
```

Example with OBO fixtures: [`examples/obo-workflow/`](https://github.com/eddiethedean/ontocode/tree/main/examples/obo-workflow).

## Security note

ROBOT spawns a child process with user-supplied arguments. Run in trusted CI agents only; do not expose `runRobot` over an unauthenticated network LSP bridge.

## Related

- [OBO workflow guide](obo-workflow.md)
- [CI integration](../ci-integration.md)
- [Enterprise evaluation](enterprise-eval.md)
