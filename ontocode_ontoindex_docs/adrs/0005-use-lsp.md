# ADR-0005 — Use Language Server Protocol for Editor Integration

## Status
Accepted

## Context
OntoCode should be VS Code-native but should not lock the backend to VS Code forever.

## Decision
Expose editor features through an OntoIndex language server.

## Consequences
Other editors can integrate later, and the architecture stays clean.