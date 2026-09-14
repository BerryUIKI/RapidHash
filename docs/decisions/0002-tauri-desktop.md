# ADR-0002: Use Tauri 2 for the Desktop Application

- Status: Accepted
- Date: 2026-09-14
- Owners: RapidHash maintainers

## Context

The desktop application needs polished tables, drag-and-drop, localization,
accessibility, responsive progress, and consistent product design across Windows,
macOS, and Linux while reusing the Rust core.

## Decision

Use Tauri 2 as the desktop shell and React with TypeScript for presentation.
Expose narrow typed commands and events from Rust. Use the system webview and a
restrictive capability and Content Security Policy configuration.

## Alternatives Considered

- **egui or iced:** an all-Rust stack, but less mature for the desired document,
  table, accessibility, and localization experience.
- **Slint:** promising native-focused UI, but a smaller ecosystem for this
  project's complex result-table interactions.
- **Electron:** mature and consistent, but carries a substantially larger runtime
  and duplicates responsibilities already handled by the Rust core.
- **Separate native UIs:** best platform fidelity but unsustainable duplication
  for the expected maintainer capacity.

## Consequences

Frontend and Rust development can evolve independently behind typed boundaries.
Platform webview differences require testing, Node tooling becomes a development
dependency, and Tauri permissions are part of the security boundary.
