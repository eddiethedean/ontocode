# Accessibility Specification

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Standard

OntoCode targets WCAG 2.2 AA.

## 2. Keyboard Accessibility

Every feature must be keyboard accessible.

Required:

- visible focus indicators
- logical tab order
- keyboard shortcuts
- escape routes
- no keyboard traps

## 3. Screen Readers

Expose:

- entity names
- roles
- selected state
- diagnostics
- relationship summaries
- command descriptions

## 4. Color and Contrast

- Minimum contrast follows WCAG AA.
- Error state never relies on color alone.
- High contrast theme supported.

## 5. Motion

Respect reduced motion.

Disable or simplify:

- graph animations
- panel transitions
- focus zoom animations

## 6. Graph Accessibility

Provide alternative graph navigation:

- node list
- relationship table
- keyboard traversal
- textual neighborhood summary

## 7. AI Accessibility

AI outputs must be readable, structured, and navigable.

## 8. Testing

- automated accessibility checks
- keyboard-only test scripts
- screen reader smoke tests
- high contrast snapshots
