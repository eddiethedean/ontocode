# Cursor prompt: Add AI action lifecycle state machine (stub)

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[AI_ORCHESTRATION.md](../platform/AI_ORCHESTRATION.md)`
- [ ] [0006-ai-preview-before-apply.md](../adr/0006-ai-preview-before-apply.md)

## Non-goals

- Real LLM providers
- Network calls

## Current state

- No AI state in store today

## Tasks

1. Add ai slice to WorkspaceStore with lifecycle enum (Idle, ProposalGenerated, PreviewShown, Applied)
2. Add actions: proposePatch, showPreview, approve, reject (no network)
3. Gate approve() to require PreviewShown state
4. Add aiStore.test.ts for lifecycle transitions

## Acceptance criteria

- [ ] Cannot approve from Idle
- [ ] approve only from PreviewShown
- [ ] Tests pass

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not add API keys or fetch calls

## References

- [Cursor prompts index](README.md)
