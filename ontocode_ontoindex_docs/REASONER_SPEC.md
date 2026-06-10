# REASONER_SPEC.md

## 1. Purpose

Reasoner support is required for Protégé replacement status.

OntoCode must support classification, consistency checking, inferred hierarchy browsing, and explanation workflows.

## 2. Reasoner Adapter Model

Reasoners are external components accessed through adapters.

Required adapters by v1.0:

- ELK
- HermiT

Desired adapters:

- Pellet
- RDFox
- custom command adapter

## 3. Reasoner Operations

```rust
pub trait ReasonerAdapter {
    fn name(&self) -> &str;
    fn classify(&self, input: ReasonerInput) -> Result<ClassificationResult>;
    fn check_consistency(&self, input: ReasonerInput) -> Result<ConsistencyResult>;
    fn unsatisfiable_classes(&self, input: ReasonerInput) -> Result<Vec<EntityIri>>;
    fn explain(&self, input: ExplanationRequest) -> Result<ExplanationResult>;
}
```

## 4. User Workflows

### 4.1 Run Classification

User selects:

`OntoCode: Run Reasoner`

Output:

- inferred class hierarchy
- changed inferred relationships
- unsatisfiable classes
- warnings/errors

### 4.2 Inspect Unsatisfiable Class

User clicks an unsatisfiable class.

OntoCode shows:

- class
- asserted axioms
- inferred conflicts
- explanation if available

### 4.3 Compare Asserted vs Inferred Hierarchy

Explorer can toggle:

- asserted hierarchy
- inferred hierarchy
- combined hierarchy

## 5. Implementation Strategy

Initial implementation should use external JVM-based reasoners through command adapters.

Later implementation may support native Rust reasoners if available.

## 6. Caching

Reasoner results should be cached by:

- ontology catalog hash
- reasoner version
- reasoner options

## 7. v1.0 Requirements

- at least one production-capable reasoner adapter
- unsatisfiable class reporting
- inferred hierarchy display
- reasoner errors surfaced in VS Code
