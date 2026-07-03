# PLUGIN_SPEC.md

## 1. Purpose

The plugin system allows users and organizations to extend OntoCore/OntoCode without modifying the core project.

## 2. Plugin Types

### 2.1 Validator Plugins
Add custom diagnostics.

Examples:

- require labels on every class
- enforce naming conventions
- forbid deprecated imports
- validate organization-specific annotation properties

### 2.2 Exporter Plugins
Generate output formats.

Examples:

- Markdown
- HTML
- CSV reports
- JSON catalogs
- SHACL shapes
- custom documentation portals

### 2.3 Reasoner Plugins
Integrate **native** reasoners (Rust binary or WASM). JVM subprocess reasoners are **not supported** ([ADR-0014](adr/0014-rust-native-reasoners-only.md)).

Examples:

- custom Rust reasoner binary implementing the plugin protocol
- WASM reasoner module (future)
- organization-specific validation reasoners

Built-in adapters (`el`, `dl`, `rl`, `rdfs`, `auto`) ship in `ontocore-reasoner` as thin wrappers over [OntoLogos](https://github.com/eddiethedean/ontologos) — see [REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0015](adr/0015-adopt-ontologos-reasoner.md).

Built-in SHACL adapter (P1) wraps [`rudof`](https://crates.io/crates/rudof) — see [SHACL_SPEC.md](SHACL_SPEC.md), [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

### 2.4 Query Function Plugins
Add functions to the SQL-like query layer.

Examples:

- `ontology_depth(iri)`
- `descendants(iri)`
- `ancestor_path(iri)`
- `is_deprecated(iri)`

### 2.5 UI Plugins
Future extension point for VS Code views or custom inspectors.

## 3. Plugin Manifest

```toml
[plugin]
name = "example-validator"
version = "0.1.0"
kind = "validator"

[capabilities]
diagnostics = true
quick_fixes = true
```

## 4. Validator Interface

```rust
pub trait ValidatorPlugin {
    fn name(&self) -> &str;
    fn validate(&self, catalog: &OntologyCatalog) -> Vec<Diagnostic>;
}
```

## 5. Exporter Interface

```rust
pub trait ExporterPlugin {
    fn name(&self) -> &str;
    fn export(&self, catalog: &OntologyCatalog, options: ExportOptions) -> Result<ExportResult>;
}
```

## 6. Stability

v1.0 plugin APIs should be semver-stable.

Before v1.0, plugin APIs may change.

## 7. v1.0 reference plugins (P1)

Ship with v1.0 as examples and optional builtins:

| Plugin | Kind | Purpose |
|--------|------|---------|
| `naming-convention-validator` | Validator | Enforce IRI/label naming rules |
| `markdown-docs-exporter` | Exporter | Markdown ontology docs |
| `shacl-validator` | Validator | SHACL via adapter ([SHACL_SPEC.md](SHACL_SPEC.md)) |

These demonstrate the plugin API; they do not replace Protégé's plugin catalog (P2 in [PROTEGE_PARITY.md](PROTEGE_PARITY.md)).
