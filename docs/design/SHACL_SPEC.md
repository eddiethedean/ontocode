# SHACL Validation Specification (v1.0 P1)

> **Status:** target design (P1 per [PROTEGE_PARITY.md](PROTEGE_PARITY.md))
>
> Competes with Protégé SHACL plugins via adapter, not built-in SHACL engine.

## 1. Purpose

Run SHACL shape validation against indexed ontologies/data and surface violations in the VS Code Problems panel.

## 2. Adapter model

```rust
pub trait ShaclValidatorAdapter {
    fn name(&self) -> &str;
    fn validate(&self, input: ShaclValidationInput) -> Result<ShaclValidationResult>;
}
```

Initial adapter: wrap external SHACL engine (e.g. TopBraid SHACL CLI or Apache Jena `shacl validate` — TBD at implementation).

## 3. User workflow

1. User adds `shapes/` directory or configures shape file paths
2. `OntoCode: Validate SHACL` or included in `OntoCode: Validate Workspace`
3. Violations appear in Problems panel with focus node, shape, and message
4. Optional code action: jump to focus node in ontology file

## 4. Plugin integration

Reference **SHACL validator plugin** per [PLUGIN_SPEC.md](PLUGIN_SPEC.md).

## 5. Milestone

Ship as **P1** reference plugin at v1.0; not a release blocker.
