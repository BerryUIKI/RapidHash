# Accessibility

RapidHash targets WCAG 2.2 AA principles where applicable to a desktop
application and follows native platform accessibility conventions.

## Requirements

- Every operation is available by keyboard.
- Focus order is predictable and visible.
- Controls have accessible names, roles, states, and descriptions.
- Status is never communicated by color alone.
- Text and essential graphics meet contrast requirements.
- The interface supports operating-system scaling without clipped content.
- Tables expose headers, row identity, selection, sorting, and status to
  assistive technology.
- Live progress announcements are throttled; completion and errors are announced.
- Motion follows reduced-motion preferences.
- Digest text can wrap, scroll, and be copied without requiring pointer input.
- Error messages identify the affected control or item and suggest recovery.

## Localization and Layout

Layouts accommodate text expansion and do not encode meaning through word order.
Bidirectional text is isolated so an untrusted path cannot visually reorder
surrounding status text. Paths have an accessible logical representation even
when their visual rendering is escaped.

## Testing

Release testing includes keyboard-only operation, 200% scaling, high contrast,
reduced motion, automated semantic checks, and representative screen readers on
tier-one platforms. Known platform limitations are documented in release notes.
