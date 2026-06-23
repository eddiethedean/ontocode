# SHACL Validation Specification (v1.0 P1)

> **Status:** target design (P1 per [PROTEGE_PARITY.md](PROTEGE_PARITY.md))
>
> Native Rust SHACL via [`rudof`](https://crates.io/crates/rudof) (shapes-rs) — not an in-tree SHACL engine ([ADR-0016](adr/0016-dependency-first-implementation.md)).

## 1. Purpose

Run SHACL shape validation against indexed ontologies/data and surface violations in the VS Code Problems panel.

## 2. Adapter model

```rust
pub trait ShaclValidatorAdapter {
    fn name(&self) -> &str;
    fn validate(&self, input: ShaclValidationInput) -> Result<ShaclValidationResult>;
}
```

**Initial adapter:** [`rudof`](https://crates.io/crates/rudof) (MIT OR Apache-2.0) — RDF shapes implementation from [shapes-rs](https://github.com/weso/shapes-rs).

OntoIndex owns: shape path configuration, data graph selection from catalog/Oxigraph, mapping violations to LSP diagnostics.

**Excluded:** JVM SHACL engines (TopBraid, Jena `shacl validate`) as default adapters — prefer Rust-native per [ADR-0016](adr/0016-dependency-first-implementation.md).

## 3. User workflow

1. User adds `shapes/` directory or configures shape file paths
2. `OntoCode: Validate SHACL` or included in `OntoCode: Validate Workspace`
3. Violations appear in Problems panel with focus node, shape, and message
4. Optional code action: jump to focus node in ontology file

## 4. Plugin integration

Reference **SHACL validator plugin** per [PLUGIN_SPEC.md](PLUGIN_SPEC.md) — built-in adapter wraps `rudof`.

## 5. Milestone

Ship as **P1** reference plugin at v1.0; not a release blocker.

## Related

- [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md)
