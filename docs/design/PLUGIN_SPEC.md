# PLUGIN_SPEC.md

## 1. Purpose

The plugin system allows users and organizations to extend OntoIndex/OntoCode without modifying the core project.

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
Integrate external reasoners.

Examples:

- ELK
- HermiT
- Pellet
- RDFox
- custom enterprise reasoners

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
