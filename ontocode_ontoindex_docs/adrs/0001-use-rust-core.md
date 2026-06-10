# ADR-0001 — Use Rust for OntoIndex Core

## Status
Accepted

## Context
The backend needs to be fast, embeddable, safe, portable, and suitable for CLI, LSP, and future Python/WASM bindings.

## Decision
Use Rust as the implementation language for OntoIndex.

## Consequences
Rust enables high performance and safe concurrency, but increases implementation complexity compared with Python.