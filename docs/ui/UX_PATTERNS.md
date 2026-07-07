# UX Patterns

## 1. Purpose

Reusable interaction patterns ensure every workspace feels consistent.

## 2. Selection Pattern

### Behavior

- Click selects.
- Shift-click extends.
- Cmd/Ctrl-click toggles.
- Selection updates Current Focus.
- Inspector follows selection.
- Graph and explorer reveal selection.

### Accessibility

Selection state must be exposed to screen readers.

## 3. Inline Editing Pattern

- Double-click or Enter begins editing.
- Enter commits.
- Escape cancels.
- Validation runs live.
- Undo restores previous semantic value.

## 4. Drag and Drop Pattern

- Drag previews semantic effect.
- Invalid drops show reason.
- Drop creates undoable command.
- Multi-object drag supported where meaningful.

## 5. Inspector Pattern

Inspector cards answer:

- What is this?
- What is wrong?
- Where is it used?
- What can I do next?

Do not turn the inspector into the main editor.

## 6. Command Palette Pattern

Every command has:

- Name.
- Description.
- Scope.
- Shortcut if available.
- Current enablement reason.
- Optional preview.

## 7. Search Pattern

Search supports:

- Fuzzy name search.
- IRI search.
- Label search.
- Relationship search.
- Diagnostics search.
- Query search.
- Documentation search.

Results are grouped and keyboard navigable.

## 8. Empty State Pattern

Every empty state explains:

1. What this area is.
2. Why it is empty.
3. What the user can do next.

## 9. Loading Pattern

Use skeletons for predictable content and progress indicators for long operations.

## 10. Error Pattern

Errors must include:

- Human-readable explanation.
- Affected entity.
- Suggested fix.
- Navigation target.
- Copy diagnostics action.

## 11. AI Suggestion Pattern

AI suggestions include:

- Recommendation.
- Reason.
- Confidence.
- Preview.
- Apply.
- Dismiss.

AI changes are never silent.

## 12. Refactoring Pattern

Refactorings always provide:

- Analysis.
- Preview.
- Reasoning impact.
- Apply.
- Undo.

## 13. Review Pattern

Semantic review surfaces changes by meaning:

- Added entities.
- Removed entities.
- Changed relationships.
- Reasoning regressions.
- Documentation gaps.
