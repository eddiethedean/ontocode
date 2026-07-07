# Cursor prompt: Reasoning diagnostics store integration

## Prerequisites

Read:

- [ ] `platform/REASONING_COMPILER.md`
- [ ] `ui/REASONING_EXPERIENCE.md`

## Non-goals

- Auto classify on save
- Problems panel UI

## Current state

- Reasoner panels exist; no reasoning slice in store

## Tasks

1. Add reasoning slice (profile, lastRun, unsatisfiable, hierarchyMode)
2. On reasoner complete message, update store and emit ReasoningCompleted
3. Add reasoningStore.test.ts

## Acceptance criteria

- [ ] Store updates on mock reasoner result
- [ ] ReasoningCompleted emitted

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not change Ontologos integration in Rust

## References

- [Cursor prompts index](README.md)
