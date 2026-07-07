# Design Tokens

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

Design tokens are the canonical source of truth for visual styling.

No hard-coded visual values should appear in application components.

## 2. Token Categories

- Color
- Typography
- Spacing
- Radius
- Border
- Shadow
- Motion
- Z-index
- Component states
- Semantic entity colors

## 3. Spacing

| Token | Value |
|---|---:|
| space.0 | 0 |
| space.1 | 4px |
| space.2 | 8px |
| space.3 | 12px |
| space.4 | 16px |
| space.5 | 24px |
| space.6 | 32px |
| space.7 | 48px |
| space.8 | 64px |

## 4. Typography

| Token | Size | Weight | Use |
|---|---:|---:|---|
| font.display | 28px | 700 | Product headings |
| font.title | 20px | 650 | Workspace titles |
| font.heading | 16px | 650 | Sections |
| font.body | 13px | 400 | General UI |
| font.caption | 12px | 400 | Supporting text |
| font.code | 13px | 400 | Editors and identifiers |

## 5. Radius

| Token | Value |
|---|---:|
| radius.sm | 4px |
| radius.md | 8px |
| radius.lg | 12px |
| radius.pill | 999px |

## 6. Motion

| Token | Duration | Use |
|---|---:|---|
| motion.fast | 150ms | Small feedback |
| motion.normal | 200ms | Panel transitions |
| motion.slow | 250ms | Large transitions |

## 7. Semantic Colors

Semantic colors are named by meaning, not appearance.

- color.status.success
- color.status.warning
- color.status.error
- color.status.info
- color.entity.class
- color.entity.property
- color.entity.individual
- color.entity.annotation
- color.entity.query
- color.entity.ai
- color.entity.reasoning

## 8. Component State Tokens

Every interactive component supports:

- default
- hover
- active
- focused
- disabled
- selected
- error
- loading

## 9. Theme Requirements

Support:

- dark
- light
- high contrast
- reduced motion
- user accent color

## 10. Distribution

Tokens are generated as:

- CSS variables
- TypeScript constants
- JSON
- Figma token source
