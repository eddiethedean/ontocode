# Cursor prompt: Unify refactoring preview with store

## Prerequisites

Read:

- [ ] `platform/SEMANTIC_REFACTORING.md`
- [ ] `ui/SEMANTIC_REFACTORING.md`

## Non-goals

- Merge classes refactor
- Undo stack

## Current state

- extension/webview-ui/src/panels/RefactorPreview.tsx

## Tasks

1. Add refactoring slice: pending preview, clear on apply/cancel
2. RefactorPreview reads pending from store
3. Apply still uses existing LSP applyRefactor message
4. Add RefactorPreview.test.tsx store integration

## Acceptance criteria

- [ ] Preview required before apply button enabled
- [ ] Cancel clears pending

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not change Rust refactor ops

## References

- [Cursor prompts index](README.md)
